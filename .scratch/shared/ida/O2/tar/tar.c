// Function: main @ 0xa9f0
long long main(long long a1, char **a2, char **a3)
{
  char **v3; // r15
  long long v4; // r14
  char *v5; // rax
  void *v6; // rax
  char *v7; // rax
  long long v8; // rax
  const char *v9; // rdi
  char *v10; // rax
  long long *v11; // rdx
  long long v12; // rsi
  long long v13; // rdi
  int v14; // ebx
  int i; // eax
  unsigned long long v16; // rax
  int *v17; // rbp
  char *v18; // rbp
  size_t v19; // r12
  long long v20; // rdx
  int *v21; // rbp
  char **v22; // rdx
  int v23; // ecx
  int v24; // ebx
  double v25; // xmm0_8
  double v26; // xmm1_8
  int *v27; // rbp
  char **v28; // r12
  int v29; // eax
  long long *v30; // rax
  char *v31; // rbp
  long long *v32; // rbx
  char v33; // al
  char *v34; // rax
  long long v35; // rax
  void *v36; // rdi
  long long v37; // rsi
  long long v38; // rax
  long long *v39; // rax
  long long v40; // r8
  long long *v41; // rdi
  char **v42; // rsi
  const char *v43; // rsi
  const char **v44; // rbx
  char *v45; // r12
  const char **v46; // rbx
  const char **v47; // r12
  const char *v48; // rdi
  const char *v49; // rdi
  char *v50; // rbx
  char *v51; // r12
  char *v52; // rbp
  const char *v53; // rax
  const char *v54; // r14
  const char *v55; // r13
  long long v56; // r12
  char *v57; // rax
  long long v58; // rbp
  FILE *v60; // rax
  char **v61; // rbp
  char *v62; // rax
  char *v63; // rax
  char *v64; // rax
  unsigned int v65; // r12d
  char *v66; // rax
  long long v67; // rax
  char *v68; // rax
  long long v69; // r12
  char *v70; // rax
  long long v71; // r13
  char *v72; // rax
  long long v73; // rax
  const char *v74; // r12
  char *v75; // rax
  long long v76; // rax
  long long v77; // rax
  int *v78; // [rsp+8h] [rbp-1D0h]
  char **v79; // [rsp+8h] [rbp-1D0h]
  int v80; // [rsp+14h] [rbp-1C4h]
  char **v81; // [rsp+18h] [rbp-1C0h]
  char v82[4]; // [rsp+2Ch] [rbp-1ACh] BYREF
  int v83; // [rsp+30h] [rbp-1A8h] BYREF
  long long v84; // [rsp+38h] [rbp-1A0h]
  long long v85; // [rsp+40h] [rbp-198h]
  long long v86; // [rsp+48h] [rbp-190h]
  int v87; // [rsp+50h] [rbp-188h] BYREF
  const char *v88; // [rsp+58h] [rbp-180h]
  long long v89; // [rsp+60h] [rbp-178h]
  long long v90; // [rsp+68h] [rbp-170h]
  int *v91; // [rsp+70h] [rbp-168h] BYREF
  void *ptr; // [rsp+78h] [rbp-160h]
  short v93; // [rsp+80h] [rbp-158h]
  char v94; // [rsp+82h] [rbp-156h]
  void *v95; // [rsp+88h] [rbp-150h]
  long long v96; // [rsp+90h] [rbp-148h]
  long long v97; // [rsp+A0h] [rbp-138h] BYREF
  long long *v98; // [rsp+A8h] [rbp-130h]
  long long v99; // [rsp+B0h] [rbp-128h]
  char src[3]; // [rsp+195h] [rbp-43h] BYREF
  unsigned long long v101; // [rsp+198h] [rbp-40h]

  v3 = a2;
  v4 = (int)a1;
  v101 = __readfsqword(0x28u);
  sub_C740(a1, a2, a3);
  sub_4AF40(*a2);
  setlocale(6, "");
  bindtextdomain("tar", "/usr/local/share/locale");
  textdomain("tar");
  status = 2;
  qword_82D50 = (long long (*)(long long))sub_10C20;
  dword_82D58 = 0;
  sub_4C760(0, 7);
  v5 = dcgettext(0, "stdout", 5);
  sub_40160(v5);
  if ( (unsigned int)sub_4E3A0() )
  {
    if ( qword_82D50 )
      ((void (*)(void))qword_82D50)();
    v68 = dcgettext(0, "failed to assert availability of the standard file descriptors", 5);
    error(0, 0, "%s", v68);
LABEL_255:
    sub_16FD0();
  }
  qword_82C18 = 10;
  v6 = (void *)sub_4ECC0(80);
  qword_82830 = 0;
  ::ptr = v6;
  signal(17, 0);
  v93 = 0;
  v91 = &v83;
  v83 = 1;
  v84 = 0;
  v85 = 0;
  v86 = 0;
  ptr = 0;
  v94 = 0;
  v95 = 0;
  v96 = 0;
  sub_3F7C0("tar", off_7FB00);
  v95 = getenv("SIMPLE_BACKUP_SUFFIX");
  v7 = getenv("POSIXLY_CORRECT");
  dword_82BA8 = 0;
  dword_82BA4 = 0;
  dword_82BA0 = 20;
  nbytes = 10240;
  byte_82A38 = v7 != 0;
  v8 = sub_40920();
  byte_827E8 = 1;
  qword_82B60 = v8;
  dword_82A7C = 0;
  *(long long *)&xmmword_82B00 = 0x8000000000000000LL;
  *((long long *)&xmmword_82B00 + 1) = -1;
  *(long long *)&xmmword_82AE0 = 0x8000000000000000LL;
  *((long long *)&xmmword_82AE0 + 1) = -1;
  dword_82AD4 = 8;
  dword_82A84 = 1;
  dword_82A80 = 0;
  dword_827E4 = 0;
  dword_82ABC = -1;
  qword_82AC0 = 0;
  dword_82B54 = -1;
  qword_82B58 = 0;
  byte_82B20 = 1;
  dword_82B24 = -1;
  dword_827FC = -1;
  if ( (int)a1 > 1 )
  {
    v9 = a2[1];
    if ( *v9 != 45 )
    {
      src[0] = 45;
      v28 = a2 + 2;
      src[2] = 0;
      v29 = v4 + strlen(v9);
      v80 = v29 - 1;
      v30 = (long long *)sub_4ECC0(8LL * v29);
      v31 = a2[1];
      v81 = (char **)v30;
      v32 = v30 + 1;
      *v30 = *a2;
      v33 = *v31;
      v79 = &a2[v4];
      if ( !*v31 )
      {
LABEL_111:
        if ( v79 > v28 )
        {
          v41 = v32;
          v42 = v28;
          do
            *v41++ = (long long)*v42++;
          while ( v42 < v79 );
          v32 += ((unsigned long long)((char *)v79 - 1 - (char *)v28) >> 3) + 1;
        }
        *v32 = 0;
        v3 = v81;
        LODWORD(v4) = v80;
        goto LABEL_4;
      }
      while ( 1 )
      {
        src[1] = v33;
        v35 = sub_4EE30(src);
        v36 = off_7FB20;
        *v32 = v35;
        v37 = (unsigned int)*v31;
        v38 = sub_2D3C0(v36, v37);
        if ( !v38 )
        {
          if ( !off_7FB40 )
            goto LABEL_110;
          v39 = *off_7FB40;
          if ( !*off_7FB40 )
            goto LABEL_110;
          while ( 1 )
          {
            v38 = sub_2D3C0(*v39, v37);
            if ( v38 )
              break;
            v39 = *(long long **)(v40 + 32);
            if ( !v39 )
              goto LABEL_110;
          }
        }
        if ( *(long long *)(v38 + 16) )
        {
          if ( v79 <= v28 )
          {
            if ( qword_82D50 )
              ((void (*)(void))qword_82D50)();
            v65 = *v31;
            v66 = dcgettext(0, "Old option '%c' requires an argument.", 5);
            error(0, 0, v66, v65);
            sub_2DEA0(2);
          }
          v34 = *v28;
          ++v31;
          v32 += 2;
          ++v28;
          *(v32 - 1) = (long long)v34;
          v33 = *v31;
          if ( !*v31 )
            goto LABEL_111;
        }
        else
        {
LABEL_110:
          v33 = *++v31;
          ++v32;
          if ( !v33 )
            goto LABEL_111;
        }
      }
    }
  }
LABEL_4:
  v10 = getenv("TAR_OPTIONS");
  v88 = "TAR_OPTIONS";
  v87 = 0;
  v89 = 0;
  v90 = 0;
  if ( !v10 )
    goto LABEL_11;
  v99 = 1;
  if ( (unsigned int)sub_375A0(v10, &v97, 33558086) )
  {
    if ( qword_82D50 )
      ((void (*)(void))qword_82D50)();
    v69 = sub_358F0(&v97);
    v70 = dcgettext(0, "cannot split TAR_OPTIONS: %s", 5);
    error(0, 0, v70, v69);
    goto LABEL_255;
  }
  if ( v97 )
  {
    v11 = v98;
    v78 = v91;
    v12 = (unsigned int)(v99 + v97);
    *v98 = qword_82DD0;
    v91 = &v87;
    if ( (unsigned int)argp_parse(&off_7FB20, v12, v11, 40, v82, &v91) )
      abort();
    v91 = v78;
    if ( dword_82630 )
    {
      if ( qword_82D50 )
        ((void (*)(void))qword_82D50)();
      v74 = v88;
      v75 = dcgettext(0, "non-option arguments in %s", 5);
      error(0, 0, v75, v74);
      sub_2DEA0(2);
    }
    v97 = 0;
  }
  sub_35800(&v97);
LABEL_11:
  v13 = (long long)&off_7FB20;
  v14 = argp_parse(&off_7FB20, (unsigned int)v4, v3, 8, &v87, &v91);
  if ( v14 )
    exit(2);
  if ( (char)v93 )
  {
    if ( dword_82BA8 == 3 )
    {
      v13 = (long long)"v7";
      sub_2E430("v7");
    }
    else
    {
      dword_82AA8 = -1;
    }
  }
  for ( i = v87; (int)v4 > i; v87 = i )
  {
    v13 = (long long)v3[i];
    sub_28130(v13);
    i = v87 + 1;
  }
  if ( !dword_82BA4 )
  {
    if ( HIBYTE(v93) )
      dword_82BA4 = 4;
    else
      dword_82BA4 = 6;
  }
  if ( (pattern && dword_82BA8 == 3 || byte_82B50 || byte_82B10 || byte_82A88) && ((1 << dword_82BA4) & 0x54) == 0 )
  {
    if ( qword_82D50 )
      ((void (*)(void))qword_82D50)();
    v43 = "GNU features wanted on incompatible archive format";
    goto LABEL_227;
  }
  if ( qword_82B38 )
  {
    if ( !dword_82630 )
    {
      if ( qword_82D50 )
        ((void (*)(void))qword_82D50)();
      v43 = "--occurrence is meaningless without a file list";
      goto LABEL_227;
    }
    v13 = (unsigned int)dword_82BA8;
    if ( (byte_69A60[4 * dword_82BA8] & 0x10) == 0 )
    {
      if ( qword_82BC8 && *(int *)qword_82BC8 == 1 )
      {
        v77 = sub_2DD30((unsigned int)dword_82BA8);
        sub_2E2F0("--occurrence", v77);
      }
      qword_82B38 = 0;
    }
  }
  v16 = qword_82830;
  if ( !qword_82830 )
  {
    v61 = (char **)::ptr;
    v13 = (long long)"TAPE";
    qword_82830 = 1;
    *v61 = getenv("TAPE");
    if ( !*(long long *)::ptr )
      *(long long *)::ptr = "-";
    v16 = qword_82830;
  }
  if ( v16 > 1 && !byte_82B10 )
  {
    if ( qword_82D50 )
      qword_82D50(v13);
    v43 = "Multiple archive files require '-M' option";
    goto LABEL_227;
  }
  if ( qword_82B28 )
  {
    if ( xmmword_82B00 < 0 )
      goto LABEL_39;
    v17 = (int *)qword_82BD0;
    v13 = qword_82BD0;
    if ( (unsigned int)sub_2DCA0(qword_82BD0, qword_82BD8) )
      sub_2E2F0("--listed-incremental", "--newer");
    if ( *v17 != 1 )
    {
      xmmword_82B00 = 0;
      goto LABEL_39;
    }
    qword_82B28 = 0;
  }
  if ( dword_82B24 != -1 )
  {
    if ( qword_82D50 )
      qword_82D50(v13);
    v63 = dcgettext(0, "--level is meaningless without --listed-incremental", 5);
    v13 = 0;
    error(0, 0, v63);
  }
LABEL_39:
  v18 = pattern;
  if ( pattern )
  {
    if ( (dword_82BA4 & 0xFFFFFFFB) == 2 )
    {
      v13 = (long long)pattern;
      v19 = byte_82B10 == 0 ? 99LL : 81LL;
      if ( strlen(pattern) > v19 )
      {
        if ( qword_82D50 )
        {
          qword_82D50(v13);
          v18 = pattern;
        }
        v71 = sub_4CE40(v18);
        v72 = dcngettext(
                0,
                "%s: Volume label is too long (limit is %lu byte)",
                "%s: Volume label is too long (limit is %lu bytes)",
                v19,
                5);
        error(0, 0, v72, v71, v19);
        sub_2DEA0(2);
      }
    }
  }
  if ( byte_82A50 )
  {
    if ( byte_82B10 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot verify multi-volume archives";
      goto LABEL_227;
    }
    if ( qword_82B70 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot verify compressed archives";
      goto LABEL_227;
    }
    v13 = (unsigned int)dword_82BA8;
    if ( (byte_69A60[4 * dword_82BA8] & 2) == 0 )
    {
      if ( qword_82BE0 && *(int *)qword_82BE0 == 1 )
      {
        v67 = sub_2DD30((unsigned int)dword_82BA8);
        sub_2E2F0("--verify", v67);
      }
      byte_82A50 = 0;
    }
  }
  else if ( qword_82B70 )
  {
    if ( byte_82B10 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot use multi-volume compressed archives";
      goto LABEL_227;
    }
    if ( (byte_69A60[4 * dword_82BA8] & 4) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot update compressed archives";
      goto LABEL_227;
    }
    if ( dword_82BA8 == 2 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot concatenate compressed archives";
      goto LABEL_227;
    }
  }
  if ( dword_82AF0 == 2 && xmmword_82AE0 < 0 )
  {
    if ( qword_82D50 )
      qword_82D50(v13);
    v43 = "--clamp-mtime needs a date specified using --mtime";
    goto LABEL_227;
  }
  if ( !HIBYTE(v93) || dword_82BA4 == 4 )
  {
    if ( dword_82A9C > 0 )
    {
      if ( dword_82BA4 == 4 )
      {
        if ( dword_82AA0 > 0 )
          goto LABEL_91;
        goto LABEL_125;
      }
      if ( (byte_69A60[4 * dword_82BA8] & 1) != 0 )
        goto LABEL_58;
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "--acls can be used only on POSIX archives";
LABEL_227:
      v64 = dcgettext(0, v43, 5);
      error(0, 0, v64);
      sub_2DEA0(2);
    }
    if ( dword_82AA0 <= 0 )
      goto LABEL_125;
    if ( dword_82BA4 == 4 )
      goto LABEL_91;
    v20 = (unsigned int)dword_82BA8;
LABEL_120:
    if ( (byte_69A60[4 * v20] & 1) != 0 )
      goto LABEL_59;
    if ( qword_82D50 )
      qword_82D50(v13);
    v43 = "--selinux can be used only on POSIX archives";
    goto LABEL_227;
  }
  v20 = (unsigned int)dword_82BA8;
  if ( (byte_69A60[4 * dword_82BA8] & 1) == 0 )
  {
    if ( qword_82D50 )
      qword_82D50(v13);
    v43 = "--pax-option can be used only on POSIX archives";
    goto LABEL_227;
  }
  if ( dword_82A9C <= 0 )
  {
    if ( dword_82AA0 <= 0 )
      goto LABEL_125;
    goto LABEL_120;
  }
LABEL_58:
  if ( dword_82AA0 > 0 )
    goto LABEL_59;
LABEL_125:
  if ( dword_82A98 > 0 && dword_82BA4 != 4 )
  {
    if ( (byte_69A60[4 * dword_82BA8] & 1) == 0 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "--xattrs can be used only on POSIX archives";
      goto LABEL_227;
    }
LABEL_59:
    if ( byte_82A78 )
      goto LABEL_65;
LABEL_60:
    if ( !byte_82AAC )
      goto LABEL_65;
    v13 = (unsigned int)dword_82BA8;
    if ( (byte_69A60[4 * dword_82BA8] & 1) != 0 )
      goto LABEL_65;
    goto LABEL_62;
  }
LABEL_91:
  if ( !byte_82A78 )
    goto LABEL_60;
  v13 = (unsigned int)dword_82BA8;
  if ( (byte_69A60[4 * dword_82BA8] & 1) == 0 )
  {
    if ( qword_82BE8 && *(int *)qword_82BE8 == 1 )
    {
      v76 = sub_2DD30((unsigned int)dword_82BA8);
      sub_2E2F0("--starting-file", v76);
    }
    byte_82A78 = 0;
    if ( byte_82AAC )
    {
LABEL_62:
      if ( qword_82BF0 && *(int *)qword_82BF0 == 1 )
      {
        v73 = sub_2DD30(v13);
        sub_2E2F0("--same-order", v73);
      }
      byte_82AAC = 0;
    }
  }
LABEL_65:
  if ( !byte_82AD0 )
    goto LABEL_70;
  if ( byte_82B92 )
  {
    v21 = (int *)qword_82BF8;
    v13 = qword_82BF8;
    if ( (unsigned int)sub_2DCA0(qword_82BF8, qword_82C00) )
      sub_2E2F0("--one-top-level", "--absolute-names");
    if ( *v21 != 1 )
    {
      byte_82AD0 = 0;
      goto LABEL_70;
    }
    byte_82B92 = 0;
  }
  if ( !qword_82AC8 )
  {
    v13 = sub_40250(*(long long *)::ptr);
    qword_82AC8 = (char *)sub_2B8B0((void *)v13);
    free((void *)v13);
    if ( !qword_82AC8 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR";
      goto LABEL_227;
    }
  }
LABEL_70:
  if ( byte_82ABA )
    dword_82B34 = 3;
  if ( byte_82B69 )
  {
    v22 = &off_80900;
    if ( dword_82B88 != 2 )
    {
      v23 = 592128;
      goto LABEL_75;
    }
    v14 = 0x40000;
    goto LABEL_177;
  }
  if ( dword_82B88 != 2 )
  {
    LODWORD(v22) = 657664;
LABEL_177:
    LODWORD(v22) = v14 | (unsigned int)v22;
    v23 = (unsigned int)v22 | 0x10000;
    v14 = ((unsigned char)byte_82B69 ^ 1) << 8;
    goto LABEL_75;
  }
  LODWORD(v22) = 919808;
  v23 = 985344;
  v14 = 256;
LABEL_75:
  flag = v14;
  v24 = dword_82BA8;
  dword_82808 = (int)v22;
  dword_82804 = v23;
  if ( dword_82BA8 == 9 )
  {
    if ( !dword_82630 )
      ++dword_82A54;
  }
  else if ( byte_82B91 )
  {
    dword_82A54 = 2;
  }
  v25 = *(double *)&qword_82A70;
  if ( *(double *)&qword_82A70 != 0.0 )
  {
    v26 = (nbytes & 0x8000000000000000LL) != 0LL
        ? (double)(int)(nbytes & 1 | (nbytes >> 1)) + (double)(int)(nbytes & 1 | (nbytes >> 1))
        : (double)(int)nbytes;
    if ( v26 > *(double *)&qword_82A70 )
    {
      if ( qword_82D50 )
        qword_82D50(v13);
      v43 = "Volume length cannot be less than record size";
      goto LABEL_227;
    }
  }
  if ( byte_82AAC && qword_82B28 )
  {
    v27 = (int *)qword_82BF0;
    v13 = qword_82BF0;
    if ( (unsigned int)sub_2DCA0(qword_82BF0, qword_82BD0) )
      sub_2E2F0("--preserve-order", "--listed-incremental");
    if ( *v27 == 1 )
      qword_82B28 = 0;
    else
      byte_82AAC = 0;
  }
  switch ( v24 )
  {
    case 1:
    case 2:
    case 8:
      v46 = (const char **)::ptr;
      v47 = (const char **)((char *)::ptr + 8 * qword_82830);
      qword_82828 = (long long)::ptr;
      if ( ::ptr >= v47 )
        goto LABEL_139;
      while ( 1 )
      {
        v48 = *v46;
        if ( !strcmp(*v46, "-") )
          break;
        qword_82828 = (long long)++v46;
        if ( v46 >= v47 )
          goto LABEL_139;
      }
      if ( qword_82D50 )
        qword_82D50(v48);
      v43 = "Options '-Aru' are incompatible with '-f -'";
      goto LABEL_227;
    case 3:
      if ( !dword_82630 )
      {
        if ( qword_82D50 )
          qword_82D50(v13);
        v43 = "Cowardly refusing to create an empty archive";
        goto LABEL_227;
      }
      if ( v94 )
      {
        if ( qword_82830 )
        {
          v58 = *(long long *)::ptr;
          if ( strcmp(*(const char **)::ptr, "-") )
            sub_2B880(v58, qword_82B70, v25);
        }
      }
LABEL_139:
      if ( filename )
      {
        qword_81F28 = fopen(filename, "w");
        if ( !qword_81F28 )
          sub_32EC0(filename, v25);
      }
      else
      {
        if ( byte_82A6A )
          v60 = stderr;
        else
          v60 = stdout;
        qword_81F28 = v60;
      }
      v49 = (const char *)v95;
      qword_82828 = (long long)::ptr;
      if ( v95 )
        qword_82DB8 = (char *)sub_4EE30(v95);
      if ( byte_82B84 )
      {
        v49 = "--backup";
        dword_82B80 = sub_40040("--backup", v96, v25);
        if ( !dword_82B80 || byte_82A6A || qword_82A60 )
          byte_82B84 = 0;
      }
      sub_10BA0(v25);
      v50 = (char *)ptr;
      if ( ptr )
      {
        do
        {
          v52 = v50;
          v50 = *(char **)v50;
          if ( dword_82A54 )
          {
            v53 = (const char *)sub_23240(*((long long *)v52 + 1), *((long long *)v52 + 2), 1);
            v54 = (const char *)*((long long *)v52 + 4);
            v55 = v53;
            v51 = (char *)v54;
            if ( !strcmp(v54, v53) )
              goto LABEL_147;
            if ( qword_82D50 )
            {
              qword_82D50(v54);
              v54 = (const char *)*((long long *)v52 + 4);
            }
            v56 = *((long long *)v52 + 3);
            v57 = dcgettext(0, "Option %s: Treating date '%s' as %s", 5);
            error(0, 0, v57, v56, v54, v55);
          }
          v51 = (char *)*((long long *)v52 + 4);
LABEL_147:
          free(v51);
          v49 = v52;
          free(v52);
        }
        while ( v50 );
      }
      sub_28180();
      if ( qword_82A48 )
        sub_DBF0();
      switch ( dword_82BA8 )
      {
        case 0:
          if ( qword_82D50 )
            qword_82D50(v49);
          v43 = "You must specify one of the '-Acdtrux', '--delete' or '--test-label' options";
          goto LABEL_227;
        case 1:
        case 2:
        case 8:
          sub_31350();
          goto LABEL_162;
        case 3:
          sub_16020();
          goto LABEL_162;
        case 4:
          sub_16780();
          goto LABEL_162;
        case 5:
          sub_110C0();
          sub_23E80(sub_11680);
          goto LABEL_162;
        case 6:
          sub_1A0B0();
          sub_23E80(sub_1A1E0);
          sub_1A080();
          goto LABEL_162;
        case 7:
          sub_23E80(sub_24580);
          goto LABEL_162;
        case 9:
          sub_24680();
          goto LABEL_162;
        default:
LABEL_162:
          sub_10FA0();
          if ( byte_82A69 )
            sub_CC30();
          if ( dword_82C20 )
            sub_148B0();
          if ( qword_82A48 )
            sub_DCF0();
          free(::ptr);
          sub_320A0();
          sub_281E0();
          if ( dword_82D58 == 2 )
          {
            v62 = dcgettext(0, "Exiting with failure status due to previous errors", 5);
            error(0, 0, v62);
          }
          if ( qword_81F28 == stdout )
          {
            sub_40180();
          }
          else if ( ((stderr->_flags & 0x20) != 0 || fclose(stderr)) && dword_82D58 <= 1 )
          {
            dword_82D58 = 2;
          }
          return (unsigned int)dword_82D58;
      }
    case 5:
    case 6:
    case 7:
    case 9:
      v44 = (const char **)::ptr;
      v45 = (char *)::ptr + 8 * qword_82830;
      qword_82828 = (long long)::ptr;
      if ( v45 > ::ptr )
      {
        do
        {
          if ( !strcmp(*v44, "-") )
            sub_2DF30("-f", v25);
          qword_82828 = (long long)++v44;
        }
        while ( v45 > (char *)v44 );
      }
      goto LABEL_139;
    default:
      goto LABEL_139;
  }
}



// Function: print_stats @ 0xbf60
long long print_stats(long long a1, const char *a2, double a3)
{
  int v4; // ebx
  double v5; // xmm2_8
  const char *v6; // r12
  unsigned long long v7; // rdi
  const char *v8; // rax
  char *v10; // rax
  unsigned long long v11; // rdi
  char v12[656]; // [rsp+10h] [rbp-548h] BYREF
  char v13[664]; // [rsp+2A0h] [rbp-2B8h] BYREF
  unsigned long long v14; // [rsp+538h] [rbp-20h]

  v4 = 0;
  v14 = __readfsqword(0x28u);
  if ( a2 && *a2 )
  {
    v10 = dcgettext(0, a2, 5);
    v4 = __fprintf_chk(a1, 1, "%s: ", v10);
  }
  if ( *(double *)&qword_81F78 > 0.0
    && (v5 = a3 / *(double *)&qword_81F78, a3 / *(double *)&qword_81F78 < 1.844674407370955e19) )
  {
    if ( v5 >= 9.223372036854776e18 )
      v11 = (unsigned int)(int)(v5 - 9.223372036854776e18) ^ 0x8000000000000000LL;
    else
      v11 = (unsigned int)(int)v5;
    v6 = (const char *)sub_44440(v11, v13, 432, 1, 1);
  }
  else
  {
    v6 = "?";
  }
  if ( a3 >= 9.223372036854776e18 )
    v7 = (unsigned int)(int)(a3 - 9.223372036854776e18) ^ 0x8000000000000000LL;
  else
    v7 = (unsigned int)(int)a3;
  v8 = (const char *)sub_44440(v7, v12, 432, 1, 1);
  return v4 + (unsigned int)__fprintf_chk(a1, (char *)&dword_0 + 1, "%.0f (%s, %s/s)", a3, v8, v6);
}



// Function: init_buffer @ 0xc100
char *init_buffer()
{
  long long v0; // rbp
  char *v1; // rax
  char *result; // rax

  v0 = dword_82178;
  v1 = (char *)qword_82180[dword_82178];
  if ( !v1 )
  {
    qword_82180[v0] = sub_265A0(&qword_82190 + dword_82178, nbytes);
    v1 = (char *)qword_82180[dword_82178];
  }
  s1 = v1;
  src = v1;
  result = &v1[512 * (long long)dword_82BA0];
  qword_81F50 = result;
  return result;
}



// Function: _flush_write @ 0xc180
unsigned long long _flush_write()
{
  unsigned long long v0; // r12
  void **v1; // r14
  void **v2; // rdx
  void **v3; // rbx
  long long v5; // rsi
  unsigned long long v6; // rcx
  unsigned long long v7; // rax
  unsigned long long v8; // r13
  long long v9; // r13
  void *v10; // rax
  void **v11; // rbp

  sub_10CF0(1);
  if ( *(double *)&qword_82A70 == 0.0 || *(double *)&qword_821A0 < *(double *)&qword_82A70 )
  {
    v0 = nbytes;
    if ( !byte_82A30 )
      v0 = sub_2C140(*(double *)&qword_82A70);
    if ( v0 )
    {
      if ( byte_82B10 )
      {
        v1 = (void **)qword_82148;
        v2 = (void **)qword_82148;
        if ( !dword_82138 )
        {
          while ( v2 )
          {
            v3 = (void **)*v2;
            if ( !*v2 || v0 < (long long)v3[1] << 9 )
            {
              v5 = (long long)v2[1];
              v6 = v0 - (v5 << 9);
              v7 = (unsigned long long)v2[4];
              v8 = (unsigned long long)v2[5] + (v6 >> 9);
              if ( v7 <= v6 )
                v6 = (unsigned long long)v2[4];
              v2[5] = (void *)v8;
              v9 = v5 + v8;
              v10 = (void *)(v7 - v6);
              v2[4] = v10;
              if ( v10 )
              {
                v3 = v2;
                v9 = v5;
              }
              if ( v3 != v1 && v1 )
              {
                do
                {
                  v11 = v1;
                  v1 = (void **)*v1;
                  free(v11[2]);
                  free(v11);
                }
                while ( v1 && v3 != v1 );
              }
              qword_82148 = v1;
              if ( !v1 )
                qword_82140 = 0;
              for ( ; v3; v3 = (void **)*v3 )
              {
                v3[1] = (char *)v3[1] - v9;
                v3[5] = 0;
              }
              return v0;
            }
            v2 = (void **)*v2;
          }
        }
      }
    }
    return v0;
  }
  else
  {
    *__errno_location() = 28;
    return 0;
  }
}



// Function: add_chunk_header @ 0xc400
unsigned long long add_chunk_header(long long a1)
{
  void *v2; // rax
  long long v3; // rdx
  long long v4; // rax
  void *ptr[55]; // [rsp+0h] [rbp-1D8h] BYREF
  unsigned long long v6; // [rsp+1B8h] [rbp-20h]

  v6 = __readfsqword(0x28u);
  if ( dword_82BA4 == 4 )
  {
    memset(ptr, 0, sizeof(ptr));
    v2 = *(void **)(a1 + 16);
    LODWORD(ptr[14]) = 33188;
    ptr[1] = v2;
    ptr[0] = v2;
    HIDWORD(ptr[14]) = getuid();
    LODWORD(ptr[15]) = getgid();
    ptr[0] = (void *)sub_1CDF0(ptr, "%d/GNUFileParts/%f.%n", dword_7F014);
    ptr[1] = ptr[0];
    ptr[17] = *(void **)(a1 + 32);
    ptr[35] = ptr[17];
    v3 = sub_13570(ptr);
    if ( !v3 )
      abort();
    v4 = sub_13200(0, ptr, v3);
    sub_13230(v4);
    free(ptr[0]);
  }
  return v6 - __readfsqword(0x28u);
}



// Function: buffer_write_global_xheader @ 0xc660
long long buffer_write_global_xheader()
{
  return sub_1DD60(&unk_820E8);
}



// Function: mv_begin_read @ 0xc670
long long mv_begin_read(void **a1)
{
  long long result; // rax

  if ( byte_82B10 )
    return sub_C500(*a1);
  return result;
}



// Function: mv_end @ 0xc6a0
void mv_end()
{
  void **v0; // rbx
  void **v1; // rbp

  if ( byte_82B10 )
  {
    v0 = (void **)qword_82148;
    if ( qword_82148 )
    {
      do
      {
        v1 = v0;
        v0 = (void **)*v0;
        free(v1[2]);
        free(v1);
      }
      while ( v0 );
    }
    qword_82148 = 0;
    qword_82140 = 0;
  }
}



// Function: mv_size_left @ 0xc710
void *mv_size_left(long long a1)
{
  void *result; // rax

  result = qword_82148;
  if ( qword_82148 )
    *((long long *)qword_82148 + 4) = a1;
  return result;
}



// Function: clear_read_error_count @ 0xc730
void clear_read_error_count()
{
  dword_82164 = 0;
}



// Function: set_start_time @ 0xc740
long long set_start_time()
{
  long long result; // rax

  result = sub_43220(&tp);
  stru_82A10 = (struct timespec)_mm_loadu_si128((const __m128i *)&tp);
  stru_82A00 = stru_82A10;
  return result;
}



// Function: compute_duration @ 0xc770
unsigned long long compute_duration()
{
  struct timespec v1; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v2; // [rsp+18h] [rbp-10h]

  v2 = __readfsqword(0x28u);
  sub_43220(&v1);
  *(double *)&qword_81F78 = (double)(LODWORD(v1.tv_nsec) - LODWORD(stru_82A00.tv_nsec)) / 1000000000.0
                          + (double)(LODWORD(v1.tv_sec) - LODWORD(stru_82A00.tv_sec))
                          + *(double *)&qword_81F78;
  sub_43220(&stru_82A00);
  return v2 - __readfsqword(0x28u);
}



// Function: first_decompress_program @ 0xc810
char *first_decompress_program(int *a1)
{
  int v2; // r8d
  int v3; // edx
  int *v4; // rcx
  int v5; // eax
  long long v6; // rsi

  if ( qword_82B70 )
    return qword_82B70;
  v2 = dword_81F70;
  if ( !dword_81F70 )
    return qword_82B70;
  *a1 = 0;
  v3 = dword_7F020[0];
  if ( dword_7F020[0] )
  {
    v4 = (int *)&unk_7F038;
    v5 = 0;
    while ( 1 )
    {
      v6 = v5++;
      if ( v2 == v3 )
        break;
      v3 = *v4;
      v4 += 6;
      if ( !v3 )
        goto LABEL_10;
    }
    *a1 = v5;
    return *(char **)&dword_7F020[6 * v6 + 2];
  }
  else
  {
    v5 = 0;
LABEL_10:
    *a1 = v5;
    return 0;
  }
}



// Function: next_decompress_program @ 0xc890
long long next_decompress_program(int *a1)
{
  long long v1; // rax
  int *v2; // rdx
  int v3; // ecx
  long long v5; // rsi

  if ( qword_82B70 )
    return 0;
  v1 = *a1;
  v2 = &dword_7F020[6 * v1];
  v3 = *v2;
  if ( *v2 )
  {
    while ( 1 )
    {
      v5 = (int)v1;
      LODWORD(v1) = v1 + 1;
      if ( dword_81F70 == v3 )
        break;
      v3 = v2[6];
      v2 += 6;
      if ( !v3 )
        goto LABEL_3;
    }
    *a1 = v1;
    return *(long long *)&dword_7F020[6 * v5 + 2];
  }
  else
  {
LABEL_3:
    *a1 = v1;
    return 0;
  }
}



// Function: format_total_stats @ 0xc900
long long format_total_stats(_IO_FILE *a1, const char **a2, unsigned char a3, int a4)
{
  int v6; // eax
  double v8; // xmm0_8
  int v9; // r12d
  char *IO_write_ptr; // rax
  int v11; // r12d
  char *v12; // rax
  const char *v13; // rsi
  int v14; // r12d
  signed long long v15; // rax
  double v16; // xmm0_8
  double v17; // xmm0_8
  unsigned long long v18; // rdi
  const char *v19; // rax
  unsigned int v20; // r12d
  char *v21; // rax
  int v23; // eax
  double v24; // xmm0_8
  char *v25; // rax
  size_t v26; // rdx
  size_t v27; // rdx
  size_t v28; // rdx
  char *v29; // rax
  char v30[24]; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v31; // [rsp+28h] [rbp-30h]

  v31 = __readfsqword(0x28u);
  if ( (unsigned int)dword_82BA8 > 7 )
  {
    if ( dword_82BA8 != 8 )
      goto LABEL_26;
    goto LABEL_23;
  }
  if ( (unsigned int)dword_82BA8 <= 4 )
  {
    if ( dword_82BA8 == 4 )
    {
      v6 = nbytes * qword_81F38;
      if ( ((nbytes * qword_81F38) & 0x8000000000000000LL) != 0LL )
      {
        v27 = v6 & 1 | ((nbytes * qword_81F38) >> 1);
        v8 = (double)(int)v27 + (double)(int)v27;
      }
      else
      {
        v8 = (double)v6;
      }
      v9 = sub_BF60((long long)a1, *a2, v8);
      IO_write_ptr = a1->_IO_write_ptr;
      if ( IO_write_ptr >= a1->_IO_write_end )
      {
        __overflow(a1, a3);
      }
      else
      {
        a1->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = a3;
      }
      v11 = sub_BF60((long long)a1, a2[1], *(double *)&qword_821A8 + *(double *)&qword_821A0) + v9 + 1;
      v12 = a1->_IO_write_ptr;
      if ( v12 >= a1->_IO_write_end )
      {
        __overflow(a1, a3);
      }
      else
      {
        a1->_IO_write_ptr = v12 + 1;
        *v12 = a3;
      }
      v13 = a2[2];
      v14 = v11 + 1;
      if ( v13 && *v13 )
      {
        v29 = dcgettext(0, v13, 5);
        v14 += __fprintf_chk(a1, 1, "%s: ", v29);
      }
      v15 = nbytes * (qword_81F38 - qword_822D8);
      if ( v15 < 0 )
      {
        v28 = v15 & 1 | ((nbytes * (qword_81F38 - qword_822D8)) >> 1);
        v16 = (double)(int)v28 + (double)(int)v28;
      }
      else
      {
        v16 = (double)(int)v15;
      }
      v17 = v16 - (*(double *)&qword_821A8 + *(double *)&qword_821A0);
      if ( v17 >= 9.223372036854776e18 )
        v18 = (unsigned int)(int)(v17 - 9.223372036854776e18) ^ 0x8000000000000000LL;
      else
        v18 = (unsigned int)(int)v17;
      v19 = (const char *)sub_45370(v18, v30);
      v20 = __fprintf_chk(a1, 1, "%s", v19) + v14;
      if ( a4 )
        goto LABEL_18;
      return v20;
    }
    if ( !dword_82BA8 )
LABEL_26:
      abort();
LABEL_23:
    v20 = sub_BF60((long long)a1, a2[1], *(double *)&qword_821A8 + *(double *)&qword_821A0);
    if ( !a4 )
      return v20;
    goto LABEL_18;
  }
  v23 = nbytes * qword_81F38;
  if ( ((nbytes * qword_81F38) & 0x8000000000000000LL) != 0LL )
  {
    v26 = v23 & 1 | ((nbytes * qword_81F38) >> 1);
    v24 = (double)(int)v26 + (double)(int)v26;
  }
  else
  {
    v24 = (double)v23;
  }
  v25 = dcgettext(0, *a2, 5);
  v20 = sub_BF60((long long)a1, v25, v24);
  if ( a4 )
  {
LABEL_18:
    v21 = a1->_IO_write_ptr;
    if ( v21 >= a1->_IO_write_end )
    {
      __overflow(a1, (unsigned char)a4);
    }
    else
    {
      a1->_IO_write_ptr = v21 + 1;
      *v21 = a4;
    }
    ++v20;
  }
  return v20;
}



// Function: print_total_stats @ 0xcc30
long long print_total_stats()
{
  return sub_C900(stderr, (const char **)off_7D140, 0xAu, 10);
}



// Function: reset_eof @ 0xcc80
char *reset_eof()
{
  char *result; // rax

  if ( byte_82161 )
  {
    byte_82161 = 0;
    dword_81F40 = 1;
    src = s1;
    result = (char *)s1 + 512 * (long long)dword_82BA0;
    qword_81F50 = result;
  }
  return result;
}



// Function: xclose @ 0xcd20
int xclose(int a1)
{
  int result; // eax
  char *v2; // rax

  result = close(a1);
  if ( result )
  {
    v2 = dcgettext(0, "(pipe)", 5);
    return sub_32D60(v2);
  }
  return result;
}



// Function: archive_read_error @ 0xcd60
long long archive_read_error()
{
  long long v0; // rdi
  long long result; // rax
  const char *v2; // rsi
  char *v3; // rax

  v0 = *(long long *)qword_82828;
  sub_32F00(*(long long *)qword_82828);
  if ( !qword_82170 )
  {
    if ( qword_82D50 )
      qword_82D50(v0);
    v2 = "At beginning of tape, quitting now";
LABEL_7:
    v3 = dcgettext(0, v2, 5);
    error(0, 0, v3);
    sub_16FD0();
  }
  result = (unsigned int)dword_82164++;
  if ( (int)result > 10 )
  {
    if ( qword_82D50 )
      qword_82D50(v0);
    v2 = "Too many errors, quitting";
    goto LABEL_7;
  }
  return result;
}



// Function: short_read @ 0xcdf0
unsigned long long short_read(unsigned long long a1)
{
  unsigned long long v1; // rbx
  char *v2; // r12
  size_t v3; // rbp
  long long v4; // rdi
  long long v6; // rdi
  char *v7; // rax
  unsigned long long v8; // r12
  char *v9; // rax
  struct stat v10; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v11; // [rsp+98h] [rbp-30h]

  v1 = a1;
  v11 = __readfsqword(0x28u);
  v2 = (char *)s1 + a1;
  v3 = nbytes - a1;
  if ( nbytes != a1 && (dword_81B80 & 0x400000) != 0 && !(qword_82170 | v3 & 0x1FF) && a1 )
  {
    v6 = (unsigned int)fd;
    if ( fstat(fd, &v10) )
    {
      sub_26460(*(long long *)qword_82828);
    }
    else if ( (v10.st_mode & 0xB000) == 0x2000 )
    {
      if ( qword_82D50 )
        qword_82D50(v6);
      v7 = dcngettext(0, "Record size = %lu block", "Record size = %lu blocks", v1 >> 9, 5);
      error(0, 0, v7, v1 >> 9);
    }
  }
  if ( (v3 & 0x1FF) != 0 )
    goto LABEL_15;
LABEL_4:
  if ( v3 && v1 && byte_82160 )
  {
    do
    {
      while ( 1 )
      {
        v4 = (unsigned int)fd;
        if ( fd <= 0x3FFFFFFF )
          break;
        v4 = (unsigned int)(fd - 0x40000000);
        v1 = sub_34460(v4, v2, v3);
        if ( v1 != -1 )
          goto LABEL_12;
LABEL_9:
        sub_CD60();
      }
      v1 = sub_4D570(fd, v2, v3);
      if ( v1 == -1 )
        goto LABEL_9;
LABEL_12:
      if ( !v1 )
        break;
      if ( !byte_82160 )
      {
        v8 = nbytes - v3;
        if ( qword_82D50 )
          qword_82D50(v4);
        v9 = dcngettext(0, "Unaligned block (%lu byte) in archive", "Unaligned block (%lu bytes) in archive", v8, 5);
        error(0, 0, v9, v8);
        sub_16FD0();
      }
      v3 -= v1;
      v2 += v1;
      if ( (v3 & 0x1FF) == 0 )
        goto LABEL_4;
LABEL_15:
      ;
    }
    while ( v1 );
  }
  ++qword_81F38;
  qword_81F50 = (char *)s1 + ((nbytes - v3) & 0xFFFFFFFFFFFFFE00LL);
  return v11 - __readfsqword(0x28u);
}



// Function: flush_archive @ 0xd000
long long flush_archive()
{
  int v0; // eax
  char *v2; // r8
  long long v3; // rdx
  signed long long v4; // rcx
  char *v5; // rdi
  bool v6; // al
  int *v7; // rax
  int v8; // edi
  __off_t v9; // r12
  long long v10; // rdi
  __off_t v11; // r12
  __off_t v12; // rax
  char *v13; // rax
  long long v14; // rax
  bool v15; // al
  short v16[2]; // [rsp+0h] [rbp-18h] BYREF
  int v17; // [rsp+4h] [rbp-14h]
  unsigned long long v18; // [rsp+8h] [rbp-10h]

  v18 = __readfsqword(0x28u);
  v0 = dword_81F40;
  if ( dword_81F40 )
  {
    v2 = s1;
    v3 = dword_82BA0;
    v4 = ((char *)qword_81F50 - (char *)s1) >> 9;
    goto LABEL_6;
  }
  if ( !byte_82CF8 )
  {
    src = s1;
    qword_82170 += ((char *)qword_81F50 - (char *)s1) >> 9;
    qword_81F50 = (char *)s1 + 512 * (long long)dword_82BA0;
    return qword_82150();
  }
  dword_81F40 = 1;
  byte_82CF8 = 0;
  v16[0] = 4;
  v17 = 1;
  if ( fd <= 0x3FFFFFFF )
    v6 = ioctl(fd, 0x40086D01u, v16) >= 0;
  else
    v6 = (int)sub_348B0((unsigned int)(fd - 0x40000000), 1074294017, v16) >= 0;
  if ( !v6 )
  {
    v7 = __errno_location();
    v8 = fd;
    if ( *v7 != 5
      || (fd <= 0x3FFFFFFF
        ? (v15 = ioctl(fd, 0x40086D01u, v16) >= 0)
        : (v15 = (int)sub_348B0((unsigned int)(fd - 0x40000000), 1074294017, v16) >= 0),
          v8 = fd,
          !v15) )
    {
      if ( v8 <= 0x3FFFFFFF )
        v9 = lseek(v8, 0, 1);
      else
        v9 = sub_34720((unsigned int)(v8 - 0x40000000), 0, 1);
      v10 = (unsigned int)fd;
      v11 = v9 - ((char *)qword_81F50 - (char *)s1);
      if ( v11 < 0 )
        v11 = 0;
      if ( fd > 0x3FFFFFFF )
      {
        v10 = (unsigned int)(fd - 0x40000000);
        v12 = sub_34720(v10, v11, 0);
      }
      else
      {
        v12 = lseek(fd, v11, 0);
      }
      if ( v11 != v12 )
      {
        if ( qword_82D50 )
          qword_82D50(v10);
        v13 = dcgettext(0, "Cannot backspace archive file; it may be unreadable without -i", 5);
        error(0, 0, v13);
        v2 = s1;
        if ( s1 == (void *)qword_82CF0 )
          goto LABEL_27;
        memset(s1, 0, qword_82CF0 - (long long)s1);
      }
    }
  }
  v2 = s1;
LABEL_27:
  v3 = dword_82BA0;
  v14 = ((char *)qword_81F50 - v2) >> 9;
  v4 = v14;
  if ( v14 < dword_82BA0 )
  {
    memset(qword_81F50, 0, (dword_82BA0 - v14) << 9);
    qword_81F50 = (char *)s1 + 512 * (long long)dword_82BA0;
    return v18 - __readfsqword(0x28u);
  }
  v0 = dword_81F40;
LABEL_6:
  v5 = src;
  qword_82170 += v4;
  src = v2;
  qword_81F50 = &v2[512 * v3];
  switch ( v0 )
  {
    case 1:
      return qword_82158(v5 - v2);
    case 2:
      abort();
    case 0:
      return qword_82150();
  }
  return v18 - __readfsqword(0x28u);
}



// Function: add_volume_label @ 0xd530
unsigned long long add_volume_label()
{
  const char *v0; // rbx
  size_t v1; // rbp
  size_t v2; // rax
  void *v3; // rbp
  long long v5; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-20h]

  v6 = __readfsqword(0x28u);
  v0 = (const char *)sub_45370(dword_7F014, &v5);
  v1 = strlen(pattern);
  v2 = strlen(v0);
  v3 = (void *)sub_4ECC0(v1 + v2 + 9);
  __sprintf_chk(v3, 1, -1, "%s %s %s", pattern, "Volume", v0);
  if ( dword_82BA4 == 4 )
    sub_1DA70("GNU.volume.label");
  else
    ((void (*)(void *, long long))sub_D3C0)(v3, 1);
  free(v3);
  return v6 - __readfsqword(0x28u);
}



// Function: check_compressed_archive @ 0xd600
long long check_compressed_archive(bool *a1, long long a2)
{
  bool *v2; // rbx
  const char *v3; // rbp
  char v4; // r12
  void *v5; // rax
  const char *v6; // rdx
  bool v7; // al
  long long result; // rax
  unsigned int *v9; // rbx
  const void *v10; // rsi
  size_t v11; // rdx
  char v12; // [rsp+7h] [rbp-21h] BYREF
  unsigned long long v13; // [rsp+8h] [rbp-20h]

  v2 = a1;
  v3 = (const char *)s1;
  v4 = byte_82160;
  v13 = __readfsqword(0x28u);
  byte_82160 = 1;
  if ( !a1 )
    v2 = (bool *)&v12;
  v5 = src;
  qword_81F50 = s1;
  v6 = (const char *)s1;
  if ( s1 == src )
  {
    if ( byte_82161 )
    {
      *v2 = 1;
      byte_82160 = v4;
      goto LABEL_10;
    }
    sub_D000(a1, a2, s1);
    v5 = src;
    v6 = (const char *)qword_81F50;
    if ( src == qword_81F50 )
    {
      byte_82161 = 1;
      v3 = (const char *)s1;
      v7 = 1;
      goto LABEL_5;
    }
    v3 = (const char *)s1;
  }
  v7 = v5 == 0;
LABEL_5:
  *v2 = v7;
  byte_82160 = v4;
  if ( v3 != v6 && (!strcmp(v3 + 257, "ustar") || !strcmp(v3 + 257, "ustar  ")) )
  {
    result = sub_22480(v3, 1);
    if ( (int)result == 1 )
      return result;
    v3 = (const char *)s1;
  }
LABEL_10:
  v9 = (unsigned int *)&unk_7D190;
  v10 = &unk_630B4;
  v11 = 2;
  while ( memcmp(v3, v10, v11) )
  {
    v9 += 6;
    if ( v9 == (unsigned int *)algn_7D250 )
      return 0;
    v11 = *((long long *)v9 + 1);
    v10 = (const void *)*((long long *)v9 + 2);
  }
  return *v9;
}



// Function: seek_archive @ 0xd780
__off_t seek_archive(signed long long a1)
{
  signed long long v1; // rbx
  signed long long v2; // rdx
  unsigned long long v3; // rax
  long long v4; // rdi
  long long v5; // rbp
  __off_t v6; // rsi
  __off_t v7; // r8
  long long v8; // r8
  long long v9; // rcx
  long long v10; // r8
  char *v12; // rax

  v1 = (char *)src - (char *)s1;
  v2 = (dword_82BA0 - (((char *)src - (char *)s1) >> 9)) << 9;
  if ( v2 >= a1 )
    return 0;
  v3 = a1 - v2;
  if ( a1 - v2 < nbytes )
    return 0;
  v4 = (unsigned int)fd;
  v5 = qword_82170;
  v6 = nbytes * (v3 / nbytes);
  if ( fd <= 0x3FFFFFFF )
  {
    v7 = lseek(fd, v6, 1);
  }
  else
  {
    v4 = (unsigned int)(fd - 0x40000000);
    v7 = sub_34720(v4, v6, 1);
  }
  if ( v7 < 0 )
    return v7;
  if ( v7 % nbytes )
  {
    if ( qword_82D50 )
      qword_82D50(v4);
    v12 = dcgettext(0, "rmtlseek not stopped at a record boundary", 5);
    error(0, 0, v12);
    sub_16FD0();
  }
  v8 = v7 >> 9;
  v9 = v8 - dword_82BA0;
  v10 = v8 - (v5 + (v1 >> 9));
  qword_82170 = v9;
  qword_81F38 += v10 / dword_82BA0;
  src = qword_81F50;
  return v10;
}



// Function: close_archive @ 0xd8d0
void close_archive(long long a1, long long a2, long long a3)
{
  long long v3; // rsi
  void **v4; // rbx
  void **v5; // rbp

  if ( byte_82CF8 || dword_81F40 == 1 )
  {
    do
      sub_D000(a1, a2, a3);
    while ( src > s1 );
  }
  sub_C770();
  if ( byte_82A50 )
    sub_11F60();
  if ( fd <= 0x3FFFFFFF )
  {
    if ( !close(fd) )
      goto LABEL_7;
  }
  else if ( !(unsigned int)sub_34350((unsigned int)(fd - 0x40000000)) )
  {
    goto LABEL_7;
  }
  sub_32D60(*(long long *)qword_82828);
LABEL_7:
  v3 = (unsigned char)byte_82161;
  sub_2BEE0(dword_82168);
  sub_2FC00(&qword_82840, v3);
  free(qword_82190);
  free(qword_82198);
  v4 = (void **)qword_82148;
  if ( qword_82148 )
  {
    do
    {
      v5 = v4;
      v4 = (void **)*v4;
      free(v5[2]);
      free(v5);
    }
    while ( v4 );
  }
  qword_82148 = 0;
  qword_82140 = 0;
}



// Function: write_fatal_details @ 0xda10
void write_fatal_details()
{
  long long v0; // rdi

  sub_33510();
  if ( fd <= 0x3FFFFFFF )
  {
    if ( !close(fd) )
      goto LABEL_3;
  }
  else if ( !(unsigned int)sub_34350((unsigned int)(fd - 0x40000000)) )
  {
    goto LABEL_3;
  }
  sub_32D60(*(long long *)qword_82828);
LABEL_3:
  v0 = (unsigned int)dword_82168;
  sub_2BEE0(dword_82168);
  sub_16FD0(v0, 0);
}



// Function: archive_write_error @ 0xda70
void archive_write_error(long long a1)
{
  int *v1; // rax
  int v2; // r12d
  int *v3; // rbx

  if ( byte_82A69 )
  {
    v1 = __errno_location();
    v2 = *v1;
    v3 = v1;
    sub_CC30();
    *v3 = v2;
  }
  sub_DA10(*(long long *)qword_82828, a1, nbytes);
}



// Function: simple_flush_write @ 0xdac0
unsigned long long simple_flush_write()
{
  unsigned long long result; // rax

  result = sub_C180();
  if ( nbytes != result )
    sub_DA70(result);
  ++qword_81F30;
  *(double *)&qword_821A0 = (double)(int)result + *(double *)&qword_821A0;
  return result;
}



// Function: init_volume_number @ 0xdbf0
int init_volume_number()
{
  FILE *v0; // rax
  FILE *v1; // rbp
  FILE *v2; // rdi
  int *v3; // rax
  long long v4; // r12
  char *v5; // rax

  v0 = fopen(qword_82A48, "r");
  if ( v0 )
  {
    v1 = v0;
    v2 = v0;
    if ( (unsigned int)__isoc99_fscanf(v0, "%d", &dword_7F010) != 1 || dword_7F010 < 0 )
    {
      if ( qword_82D50 )
        qword_82D50(v2);
      v4 = sub_4CE40(qword_82A48);
      v5 = dcgettext(0, "%s: contains invalid volume number", 5);
      error(0, 0, v5, v4);
      sub_16FD0(0, 0);
    }
    if ( (v1->_flags & 0x20) != 0 )
    {
      sub_32F00(qword_82A48);
      LODWORD(v3) = fclose(v1);
      if ( !(int)v3 )
        return (int)v3;
    }
    else
    {
      LODWORD(v3) = fclose(v1);
      if ( !(int)v3 )
        return (int)v3;
    }
    LODWORD(v3) = sub_32D60(qword_82A48);
  }
  else
  {
    v3 = __errno_location();
    if ( *v3 != 2 )
      LODWORD(v3) = sub_32EA0(qword_82A48);
  }
  return (int)v3;
}



// Function: closeout_volume_number @ 0xdcf0
int closeout_volume_number()
{
  FILE *v0; // rax
  FILE *v1; // rbp
  int result; // eax

  v0 = fopen(qword_82A48, "w");
  if ( !v0 )
    return sub_32EA0(qword_82A48);
  v1 = v0;
  __fprintf_chk(v0, 1, "%d\n", dword_7F010);
  if ( (v1->_flags & 0x20) == 0 )
  {
    result = fclose(v1);
    if ( !result )
      return result;
    return sub_32D60(qword_82A48);
  }
  sub_334F0(qword_82A48);
  result = fclose(v1);
  if ( result )
    return sub_32D60(qword_82A48);
  return result;
}



// Function: gnu_flush_write @ 0xe570
unsigned long long gnu_flush_write(long long a1, void *a2)
{
  signed long long v3; // r13
  void **v4; // rbx
  int v5; // eax
  long long *v7; // rdi
  char *v8; // rbp
  double v9; // xmm0_8
  char *v10; // r12
  long long v11; // rdx
  long long *v12; // r13
  const char *v13; // r15
  size_t v14; // rax
  unsigned int v15; // edx
  long long v16; // r15
  char *v17; // rax
  char *v18; // rsi
  int v19; // r15d
  char *v20; // rax
  void *v21; // rcx
  void *v22; // rdx
  void **v23; // rdi
  long long v24; // rdx
  void *v25; // r13
  void **v26; // r14
  void **v27; // r15
  size_t v28; // rbx
  const void *v29; // rsi
  void *v30; // rdi
  char *v31; // rdx
  unsigned long long v32; // rax
  char *v33; // rbx
  char *v34; // rdi
  long long v35; // rdx
  char *v36; // rax
  unsigned long long v37; // r13
  char *v38; // rax
  char *v39; // [rsp+8h] [rbp-50h]
  unsigned long long v40; // [rsp+18h] [rbp-40h]

  v40 = __readfsqword(0x28u);
  qword_82158 = (long long (*)(long long))sub_DAC0;
  v3 = sub_C180();
  if ( v3 != nbytes && !byte_82B10 )
    goto LABEL_81;
  if ( v3 )
    ++qword_81F30;
  v4 = (void **)qword_82148;
  *(double *)&qword_821A0 = (double)(int)v3 + *(double *)&qword_821A0;
  if ( v3 != nbytes )
  {
    while ( v4 && *v4 && (unsigned long long)v3 >= *((long long *)*v4 + 1) << 9 )
      v4 = (void **)*v4;
    if ( (v3 & 0x1FF) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(a1);
      v38 = dcgettext(0, "write did not end on a block boundary", 5);
      error(0, 0, v38);
      dword_82D58 = 2;
    }
    else if ( v3 >= 0 || (v5 = *__errno_location(), (unsigned int)(v5 - 5) <= 1) || v5 == 28 )
    {
      if ( !(unsigned char)sub_DD80(1) )
        goto LABEL_16;
      v7 = (long long *)&unk_81F80;
      sub_2FC00(&unk_81F80, a2);
      if ( ++dword_7F010 < 0 )
        sub_C310();
      v8 = (char *)(a1 - v3);
      v9 = *(double *)&qword_821A8 + *(double *)&qword_821A0;
      qword_821A0 = 0;
      ++dword_7F014;
      v10 = (char *)s1 + v3;
      dword_82178 = dword_82178 == 0;
      qword_821A8 = *(long long *)&v9;
      sub_C100();
      dword_82138 = 1;
      if ( pattern )
        sub_D530();
      if ( v4 )
      {
        if ( dword_82BA4 == 4 )
        {
          sub_1DA70("GNU.volume.filename");
          sub_1DA70("GNU.volume.size");
          a2 = &unk_81F80;
          v7 = (long long *)"GNU.volume.offset";
          sub_1DA70("GNU.volume.offset");
          v21 = qword_81F50;
        }
        else
        {
          v12 = src;
          if ( src == qword_81F50 )
          {
            v12 = 0;
            if ( !byte_82161 )
            {
              sub_D000(&unk_81F80, a2, v11);
              v12 = src;
              if ( src == qword_81F50 )
              {
                byte_82161 = 1;
                v12 = 0;
              }
            }
          }
          v13 = (const char *)v4[2];
          v14 = strlen(v13);
          v15 = v14;
          if ( v14 > 0x64 )
          {
            if ( qword_82D50 )
            {
              qword_82D50(v13);
              v13 = (const char *)v4[2];
            }
            v16 = sub_4CE40(v13);
            v17 = dcgettext(0, "%s: file name too long to be stored in a GNU multivolume header, truncated", 5);
            error(0, 0, v17, v16);
            v15 = 100;
          }
          *v12 = 0;
          v12[63] = 0;
          memset(
            (void *)((unsigned long long)(v12 + 1) & 0xFFFFFFFFFFFFFFF8LL),
            0,
            8LL * (((unsigned int)v12 - (((int)v12 + 8) & 0xFFFFFFF8) + 512) >> 3));
          v18 = (char *)v4[2];
          if ( v15 >= 8 )
          {
            *v12 = *(long long *)v18;
            *(long long *)((char *)v12 + v15 - 8) = *(long long *)&v18[v15 - 8];
            qmemcpy(
              (void *)((unsigned long long)(v12 + 1) & 0xFFFFFFFFFFFFFFF8LL),
              (const void *)(v18 - ((char *)v12 - ((unsigned long long)(v12 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
              8LL * ((v15 + (int)v12 - (((int)v12 + 8) & 0xFFFFFFF8)) >> 3));
          }
          else if ( (v15 & 4) != 0 )
          {
            *(int *)v12 = *(int *)v18;
            *(int *)((char *)v12 + v15 - 4) = *(int *)&v18[v15 - 4];
          }
          else if ( v15 )
          {
            *(char *)v12 = *v18;
            if ( (v15 & 2) != 0 )
              *(short *)((char *)v12 + v15 - 2) = *(short *)&v18[v15 - 2];
          }
          *((char *)v12 + 156) = 77;
          sub_12F80(v4[4], (char *)v12 + 124, 12);
          sub_12F80((char *)v4[3] - (char *)v4[4], (char *)v12 + 369, 12);
          a2 = v12;
          v7 = &qword_82840;
          v19 = dword_82A54;
          dword_82A54 = 0;
          sub_13320(&qword_82840, v12, -1);
          v20 = (char *)src;
          dword_82A54 = v19;
          if ( v12 >= src )
          {
            v20 = (char *)src + (((char *)v12 - (char *)src) & 0xFFFFFFFFFFFFFE00LL) + 512;
            src = v20;
          }
          v21 = qword_81F50;
          if ( qword_81F50 < v20 )
            goto LABEL_33;
        }
      }
      else
      {
        v21 = qword_81F50;
      }
      v22 = src;
      if ( src == v21 )
      {
        v22 = 0;
        if ( !byte_82161 )
        {
          sub_D000(v7, a2, 0);
          v22 = src;
          if ( src == qword_81F50 )
          {
            byte_82161 = 1;
            v22 = 0;
          }
        }
      }
      sub_13200(1, &unk_81F80, v22);
      v23 = (void **)&unk_81F80;
      sub_2FC00(&unk_81F80, &unk_81F80);
      if ( v4 )
      {
        v23 = v4;
        sub_C400((long long)v4);
      }
      v25 = src;
      if ( src == qword_81F50 )
      {
        v25 = 0;
        if ( !byte_82161 )
        {
          sub_D000(v23, &unk_81F80, v24);
          v25 = src;
          if ( src == qword_81F50 )
          {
            byte_82161 = 1;
            v25 = 0;
          }
        }
      }
      v26 = (void **)qword_82148;
      v39 = s1;
      if ( qword_82148 )
      {
        do
        {
          if ( v4 == v26 )
            break;
          v27 = v26;
          v26 = (void **)*v26;
          free(v27[2]);
          free(v27);
        }
        while ( v26 );
      }
      qword_82148 = v26;
      if ( !v26 )
        qword_82140 = 0;
      for ( ; v4; v4 = (void **)*v4 )
      {
        v4[1] = (char *)v4[1] + (((char *)v25 - v39) >> 9);
        v4[5] = 0;
      }
      dword_82138 = 0;
      v28 = (char *)qword_81F50 - (char *)v25;
      if ( (unsigned long long)v8 > (char *)qword_81F50 - (char *)v25 )
      {
        while ( 1 )
        {
          v29 = v10;
          v30 = v25;
          v10 += v28;
          memcpy(v25, v29, v28);
          v31 = (char *)src;
          v8 -= v28;
          v32 = (unsigned long long)v25 + ((v28 - 1) & 0xFFFFFFFFFFFFFE00LL);
          if ( v32 >= (unsigned long long)src )
          {
            v31 = (char *)src + ((v32 - (long long)src) & 0xFFFFFFFFFFFFFE00LL) + 512;
            src = v31;
          }
          v33 = qword_81F50;
          if ( qword_81F50 < v31 )
            break;
          v25 = src;
          if ( qword_81F50 == v31 )
          {
            v25 = 0;
            if ( !byte_82161 )
            {
              sub_D000(v30, v29, v31);
              v25 = src;
              v33 = qword_81F50;
              if ( src == qword_81F50 )
              {
                byte_82161 = 1;
                v25 = 0;
              }
            }
          }
          v28 = v33 - (char *)v25;
          if ( (unsigned long long)v8 <= v28 )
            goto LABEL_57;
        }
      }
      else
      {
LABEL_57:
        memcpy(v25, v10, (size_t)v8);
        v34 = &v8[(long long)v25];
        memset(&v8[(long long)v25], 0, v28 - (long long)v8);
        v36 = (char *)src;
        v37 = (unsigned long long)v25 + ((unsigned long long)(v8 - 1) & 0xFFFFFFFFFFFFFE00LL);
        if ( v37 >= (unsigned long long)src )
        {
          v36 = (char *)src + ((v37 - (long long)src) & 0xFFFFFFFFFFFFFE00LL) + 512;
          src = v36;
        }
        if ( qword_81F50 >= v36 )
        {
          if ( qword_81F50 == v36 && !byte_82161 )
          {
            sub_D000(v34, 0, v35);
            if ( src == qword_81F50 )
              byte_82161 = 1;
          }
          goto LABEL_16;
        }
      }
LABEL_33:
      abort();
    }
LABEL_81:
    sub_DA70(v3);
  }
LABEL_16:
  qword_82158 = (long long (*)(long long))gnu_flush_write;
  return v40 - __readfsqword(0x28u);
}



// Function: drop_volume_label_suffix @ 0xf3f0
void *drop_volume_label_suffix(const char *src)
{
  size_t v1; // rax
  const char *v2; // rbx
  const unsigned short *v3; // rdx
  const char *v4; // rbx
  size_t v6; // rbx
  void *v7; // r12

  v1 = strlen(src);
  if ( !v1 )
    return 0;
  v2 = &src[v1 - 1];
  if ( src >= v2 )
    return 0;
  v3 = *__ctype_b_loc();
  while ( (v3[*(unsigned char *)v2] & 0x800) != 0 )
  {
    if ( src == --v2 )
      return 0;
  }
  if ( src >= v2 )
    return 0;
  v4 = v2 - 7;
  if ( v4 <= src || memcmp(v4, " Volume ", 8u) )
    return 0;
  v6 = v4 - src;
  v7 = (void *)sub_4ECC0(v6 + 1);
  memcpy(v7, src, v6);
  *((char *)v7 + v6) = 0;
  return v7;
}



// Function: match_volume_label @ 0xf4b0
unsigned long long match_volume_label(char **a1, long long a2, long long a3)
{
  const char *v3; // rbp
  char *v4; // rdi
  long long v5; // rbx
  long long v6; // r12
  char *v7; // rax
  const char *v8; // rax
  char *v9; // rbp
  int v10; // ebx
  char *v12; // rbp
  char v13; // al
  long long v14; // r12
  char *v15; // rax
  long long v16; // rax
  char v17[360]; // [rsp+0h] [rbp-1D8h] BYREF
  char v18[80]; // [rsp+168h] [rbp-70h] BYREF
  unsigned long long v19; // [rsp+1B8h] [rbp-20h]

  v3 = qword_81F18;
  v19 = __readfsqword(0x28u);
  if ( qword_81F18 )
    goto LABEL_2;
  v12 = src;
  if ( src == qword_81F50 )
  {
    if ( byte_82161 )
      goto LABEL_20;
    sub_D000(a1, a2, a3);
    v12 = src;
    if ( src == qword_81F50 )
    {
      byte_82161 = 1;
      goto LABEL_20;
    }
  }
  if ( !v12 )
    goto LABEL_20;
  v13 = v12[156];
  if ( v13 == 86 )
  {
    a1 = &qword_81F18;
    sub_25160(&qword_81F18, v12, 100);
  }
  else if ( v13 == 103 )
  {
    sub_2FB40(v17);
    v16 = sub_231E0(v12 + 124, 12);
    sub_1DB80(v18, v12, v16);
    sub_1D8A0(v17);
    a1 = (char **)v17;
    sub_2FC00(v17, v12);
  }
  v3 = qword_81F18;
  if ( !qword_81F18 )
  {
LABEL_20:
    if ( qword_82D50 )
      qword_82D50(a1);
    v14 = sub_4D2D0(pattern);
    v15 = dcgettext(0, "Archive not labeled to match %s", 5);
    error(0, 0, v15, v14);
    sub_16FD0(0, 0);
  }
LABEL_2:
  v4 = pattern;
  if ( fnmatch(pattern, v3, 0) )
  {
    if ( !byte_82B10
      || (v4 = (char *)v3, v8 = (const char *)sub_F3F0(v3), (v9 = (char *)v8) == 0)
      || (v4 = (char *)v8, v10 = fnmatch(v8, pattern, 0), free(v9), v10) )
    {
      if ( qword_82D50 )
        qword_82D50(v4);
      v5 = sub_4D2B0(1, pattern);
      v6 = sub_4D2B0(0, qword_81F18);
      v7 = dcgettext(0, "Volume %s does not match %s", 5);
      error(0, 0, v7, v6, v5);
      sub_16FD0(0, 0);
    }
  }
  return v19 - __readfsqword(0x28u);
}



// Function: flush_write @ 0xf6f0
long long flush_write()
{
  return qword_82158(nbytes);
}



// Function: open_archive @ 0xf710
unsigned long long open_archive(long long a1, long long a2, double a3)
{
  int v3; // ebx
  int v4; // ebp
  char *v5; // rsi
  unsigned long long v6; // rdi
  char *v7; // r15
  long long v8; // rdx
  __pid_t v10; // eax
  char *v11; // rdx
  const void **v12; // r13
  char *v13; // r14
  int v14; // r12d
  unsigned int v15; // eax
  char *v16; // rax
  char *v17; // rax
  int v18; // eax
  __pid_t v19; // eax
  int *v20; // rax
  int v21; // r12d
  int *v22; // rbx
  long long *v23; // rax
  int v24; // ebp
  char *v25; // rax
  int v26; // eax
  int *v27; // rax
  char *v28; // rax
  int v29; // eax
  const char *v30; // rsi
  char *v31; // rax
  int v32; // eax
  char *v33; // rax
  char *v34; // rax
  int *v35; // rdx
  int v36; // esi
  char *v37; // rax
  unsigned int v38; // [rsp+Ch] [rbp-4Ch]
  bool v39; // [rsp+17h] [rbp-41h] BYREF
  unsigned long long v40; // [rsp+18h] [rbp-40h]

  v40 = __readfsqword(0x28u);
  qword_82150 = sub_EC80;
  qword_82158 = (long long (*)(long long))sub_E570;
  if ( !nbytes )
  {
    if ( qword_82D50 )
      qword_82D50(a1);
    v30 = "Invalid value for record_size";
    goto LABEL_94;
  }
  if ( !qword_82830 )
  {
    if ( qword_82D50 )
      qword_82D50(a1);
    v30 = "No archive name given";
    goto LABEL_94;
  }
  v3 = a1;
  v4 = 0;
  sub_2FC00(&qword_82840, a2);
  dword_82178 = 0;
  sub_C100();
  v5 = "-";
  if ( (int)a1 != 2 )
    v4 = a1;
  v6 = *(long long *)ptr;
  dword_81F40 = v4;
  if ( !strcmp((const char *)v6, "-") )
  {
    v6 = v4 != 0;
    if ( isatty(v6) )
    {
      if ( qword_82D50 )
        qword_82D50(v6);
      if ( !v4 )
      {
        v11 = dcgettext(0, "Refusing to read archive contents from terminal (missing -f option?)", 5);
        goto LABEL_33;
      }
      v30 = "Refusing to write archive contents to terminal (missing -f option?)";
LABEL_94:
      v11 = dcgettext(0, v30, 5);
LABEL_33:
      error(0, 0, v11);
      sub_16FD0(0, 0);
    }
  }
  v7 = qword_82B70;
  qword_81F38 = 0;
  byte_82160 = byte_82AB9;
  if ( qword_82B70 )
  {
    switch ( v3 )
    {
      case 1:
        dword_82168 = sub_2C180();
        if ( !filename )
        {
          v5 = "-";
          v6 = *(long long *)ptr;
          if ( !strcmp(*(const char **)ptr, "-") )
            qword_81F28 = stderr;
        }
        break;
      case 2:
        abort();
      case 0:
        v10 = sub_2C570();
        byte_82160 = 1;
        dword_82168 = v10;
        qword_81F50 = s1;
        break;
    }
    goto LABEL_11;
  }
  v12 = (const void **)ptr;
  v5 = "-";
  v13 = *(char **)ptr;
  v6 = *(long long *)ptr;
  v14 = strcmp(*(const char **)ptr, "-");
  if ( !v14 )
  {
    byte_82160 = 1;
    if ( byte_82A50 )
    {
      if ( qword_82D50 )
        qword_82D50(v6);
      v30 = "Cannot verify stdin/stdout archive";
      goto LABEL_94;
    }
    if ( v3 == 1 )
    {
      fd = 1;
      if ( filename )
        goto LABEL_68;
    }
    else
    {
      if ( v3 != 2 )
      {
        if ( !v3 )
        {
          fd = 0;
          v6 = (unsigned long long)&v39;
          v15 = sub_D600(&v39, (long long)"-");
          if ( v15 > 1 )
          {
            if ( qword_82D50 )
            {
              v38 = v15;
              qword_82D50(&v39);
              v15 = v38;
            }
            v35 = dword_7F020;
            while ( 1 )
            {
              v36 = *v35;
              if ( !*v35 )
                goto LABEL_140;
              v35 += 6;
              if ( v15 == v36 )
                break;
              ++v14;
            }
            v7 = *(char **)&dword_7F020[6 * v14 + 4];
LABEL_140:
            v37 = dcgettext(0, "Archive is compressed. Use %s option", 5);
            error(0, 0, v37, v7);
            sub_16FD0(0, 0);
          }
          if ( v39 )
          {
            if ( qword_82D50 )
              qword_82D50(&v39);
            v16 = dcgettext(0, "This does not look like a tar archive", 5);
            v5 = 0;
            v6 = 0;
            error(0, 0, v16);
            dword_82D58 = 2;
          }
        }
        goto LABEL_11;
      }
      fd = 0;
      byte_81F20 = 1;
      qword_81F50 = s1;
      if ( filename )
      {
LABEL_68:
        if ( (unsigned char)sub_2BD50() )
          goto LABEL_69;
LABEL_60:
        v20 = __errno_location();
        v21 = *v20;
        v22 = v20;
        goto LABEL_61;
      }
    }
    qword_81F28 = stderr;
    goto LABEL_68;
  }
  switch ( v3 )
  {
    case 1:
      v24 = 0;
      if ( byte_82B84 )
      {
        v24 = 1;
        sub_25C10(v13);
        v12 = (const void **)ptr;
        v13 = *(char **)ptr;
      }
      if ( byte_82A50 )
      {
        if ( byte_82D70
          || (v25 = strchr(v13, 58), qword_82D78 = (long long)v25, v13 = (char *)*v12, !v25)
          || v25 <= v13
          || memchr(*v12, 47, v25 - v13) )
        {
          v5 = (char *)(&dword_40 + 2);
          v6 = (unsigned long long)v13;
          v26 = open(v13, 66, 438);
        }
        else
        {
          v5 = (char *)(&dword_40 + 2);
          v6 = (unsigned long long)v13;
          v26 = sub_33A80(v13, 66, 0x40000000, qword_82AB0);
        }
      }
      else if ( byte_82D70
             || (v31 = strchr(v13, 58), qword_82D78 = (long long)v31, v13 = (char *)*v12, !v31)
             || v31 <= v13
             || memchr(*v12, 47, v31 - v13) )
      {
        v5 = (char *)(&qword_1B0 + 6);
        v6 = (unsigned long long)v13;
        v26 = creat(v13, 0x1B6u);
      }
      else
      {
        v5 = (char *)(&dword_40 + 1);
        v6 = (unsigned long long)v13;
        v26 = sub_33A80(v13, 65, 0x40000000, qword_82AB0);
      }
      fd = v26;
      if ( v26 < 0 )
        goto LABEL_83;
      if ( v26 > 0x3FFFFFFF )
      {
        sub_2BE10();
        sub_2BDC0();
        qword_81F30 = 0;
LABEL_16:
        if ( pattern )
        {
          if ( byte_82B10 )
          {
            sub_D530();
          }
          else if ( dword_82BA4 == 4 )
          {
            sub_1DA70("GNU.volume.label");
          }
          else
          {
            sub_D3C0(pattern, v5, v8);
          }
        }
        goto LABEL_19;
      }
      if ( !(unsigned char)sub_2BD50() )
      {
LABEL_83:
        v27 = __errno_location();
        v21 = *v27;
        v22 = v27;
        if ( v24 )
          sub_25EA0();
LABEL_61:
        v23 = ptr;
        *v22 = v21;
        sub_32EC0(*v23, a3);
      }
LABEL_69:
      sub_2BE10();
      sub_2BDC0();
      goto LABEL_14;
    case 2:
      if ( byte_82D70
        || (v28 = strchr(v13, 58), qword_82D78 = (long long)v28, v13 = (char *)*v12, !v28)
        || v28 <= v13
        || memchr(*v12, 47, v28 - v13) )
      {
        v5 = (char *)(&dword_40 + 2);
        v29 = open(v13, 66, 438);
      }
      else
      {
        v5 = (char *)(&dword_40 + 2);
        v29 = sub_33A80(v13, 66, 0x40000000, qword_82AB0);
      }
      v6 = 0;
      fd = v29;
      if ( (unsigned int)sub_D600(0, 66) > 1 )
      {
        if ( qword_82D50 )
          qword_82D50(0);
        v30 = "Cannot update compressed archives";
        goto LABEL_94;
      }
      break;
    case 0:
      if ( byte_82D70
        || (v17 = strchr(v13, 58), qword_82D78 = (long long)v17, v13 = (char *)*v12, !v17)
        || v17 <= v13
        || memchr(*v12, 47, v17 - v13) )
      {
        v5 = 0;
        v6 = (unsigned long long)v13;
        v18 = open(v13, 0, 438);
      }
      else
      {
        v5 = 0;
        v6 = (unsigned long long)v13;
        v18 = sub_33A80(v13, 0, 0x40000000, qword_82AB0);
      }
      fd = v18;
      if ( v18 == -1 )
        goto LABEL_60;
      if ( byte_82B10 )
        goto LABEL_58;
      if ( qword_82B70 )
      {
LABEL_55:
        if ( v18 <= 0x3FFFFFFF )
        {
          v6 = (unsigned int)v18;
          close(v18);
        }
        else
        {
          v6 = (unsigned int)(v18 - 0x40000000);
          sub_34350(v6);
        }
        byte_82161 = 0;
        v19 = sub_2C570();
        byte_82160 = 1;
        dword_82168 = v19;
        v18 = fd;
LABEL_58:
        qword_81F38 = 0;
        qword_81F50 = s1;
LABEL_59:
        fd = v18;
        if ( v18 < 0 )
          goto LABEL_60;
        sub_C350();
        break;
      }
      v6 = (unsigned long long)&v39;
      v32 = sub_D600(&v39, 0);
      if ( v32 )
      {
        if ( v32 == 1 )
        {
          if ( v39 )
          {
            if ( qword_82D50 )
              qword_82D50(&v39);
            v33 = dcgettext(0, "This does not look like a tar archive", 5);
            v5 = 0;
            v6 = 0;
            error(0, 0, v33);
            dword_82D58 = 2;
          }
LABEL_110:
          v18 = fd;
          goto LABEL_59;
        }
        dword_81F70 = v32;
      }
      else
      {
        if ( v39 )
        {
          if ( qword_82D50 )
            qword_82D50(&v39);
          v34 = dcgettext(0, "This does not look like a tar archive", 5);
          error(0, 0, v34);
          dword_82D58 = 2;
        }
        v5 = 0;
        v6 = *(long long *)ptr;
        sub_2B880(*(long long *)ptr, 0, a3);
        if ( !qword_82B70 )
          goto LABEL_110;
      }
      v18 = fd;
      goto LABEL_55;
  }
LABEL_11:
  if ( fd < 0 )
    goto LABEL_60;
  if ( fd > 0x3FFFFFFF )
  {
    sub_2BE10();
    sub_2BDC0();
    if ( v3 )
      goto LABEL_14;
LABEL_22:
    if ( src == qword_81F50 && !byte_82161 )
    {
      sub_D000(v6, v5, v8);
      if ( src == qword_81F50 )
        byte_82161 = 1;
    }
    goto LABEL_23;
  }
  if ( !(unsigned char)sub_2BD50() )
    goto LABEL_60;
  sub_2BE10();
  sub_2BDC0();
  if ( !v3 )
    goto LABEL_22;
LABEL_14:
  if ( (unsigned int)(v3 - 1) > 1 )
    goto LABEL_19;
  qword_81F30 = 0;
  if ( v3 == 1 )
    goto LABEL_16;
LABEL_23:
  if ( pattern )
    sub_F4B0((char **)v6, (long long)v5, v8);
LABEL_19:
  sub_43220(&stru_82A10);
  stru_82A00 = (struct timespec)_mm_loadu_si128((const __m128i *)&stru_82A10);
  return v40 - __readfsqword(0x28u);
}



// Function: copy_string_unquote @ 0x100a0
char *copy_string_unquote(void *a1)
{
  const char *v1; // rax
  char v2; // bl
  char *v3; // r12
  size_t v5; // rax
  size_t v6; // rbp

  v1 = (const char *)sub_4EE30(a1);
  v2 = *v1;
  v3 = (char *)v1;
  if ( (*v1 == 34 || v2 == 39) && (v5 = strlen(v1), v6 = v5, v3[v5 - 1] == v2) )
  {
    memmove(v3, v3 + 1, v5 - 2);
    v3[v6 - 2] = 0;
    sub_251B0(v3);
    return v3;
  }
  else
  {
    sub_251B0(v3);
    return v3;
  }
}



// Function: getwidth @ 0x10110
long long getwidth(FILE *a1)
{
  int v1; // eax
  long long result; // rax
  char *v3; // rdi
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  v1 = fileno(a1);
  if ( ioctl(v1, 0x5413u, &v4) || (result = WORD1(v4), !WORD1(v4)) )
  {
    v3 = getenv("COLUMNS");
    if ( !v3 )
      return 80;
    result = strtol(v3, 0, 10);
    if ( result <= 0 )
      return 80;
  }
  return result;
}



// Function: format_checkpoint_string @ 0x101a0
long long format_checkpoint_string(FILE *a1, unsigned long long a2, char *a3, unsigned char a4, unsigned int a5, double a6)
{
  long long v7; // r13
  unsigned char v11; // al
  char *v12; // rdx
  char *v13; // r13
  char v15; // al
  char *IO_write_ptr; // rax
  unsigned char v17; // dl
  char *v18; // rax
  const char *v19; // r15
  FILE *v20; // rsi
  const char *v21; // r15
  char *v22; // rax
  long long v23; // rdx
  char *v24; // rcx
  char *v25; // rdi
  signed long long v26; // r8
  const char *v27; // r9
  char v28; // al
  struct tm *v29; // rax
  long long v30; // rax
  char *v31; // r9
  char *v32; // rax
  const char **v33; // rsi
  unsigned long long v34; // r15
  char *v35; // rax
  char *v36; // rax
  long long v37; // rcx
  long long v38; // rdi
  char *s; // [rsp+8h] [rbp-1A0h]
  long long v40; // [rsp+10h] [rbp-198h]
  signed long long v42; // [rsp+20h] [rbp-188h]
  const char *v43; // [rsp+20h] [rbp-188h]
  char *v44; // [rsp+20h] [rbp-188h]
  long long v45; // [rsp+20h] [rbp-188h]
  signed long long v46; // [rsp+20h] [rbp-188h]
  char *v47; // [rsp+28h] [rbp-180h]
  char *v48; // [rsp+28h] [rbp-180h]
  char *v49; // [rsp+28h] [rbp-180h]
  struct timeval tv; // [rsp+30h] [rbp-178h] BYREF
  long long v51[4]; // [rsp+40h] [rbp-168h] BYREF
  long long v52[30]; // [rsp+60h] [rbp-148h] BYREF
  char v53[24]; // [rsp+150h] [rbp-58h] BYREF
  unsigned long long v54; // [rsp+168h] [rbp-40h]

  v7 = a5;
  v54 = __readfsqword(0x28u);
  if ( a4 )
  {
    s = dcgettext(0, "write", 5);
    v40 = sub_45370(v7, v53);
    if ( !a3 )
      a3 = dcgettext(0, "Write checkpoint %u", 5);
  }
  else
  {
    s = dcgettext(0, "read", 5);
    v40 = sub_45370(v7, v53);
    if ( !a3 )
      a3 = dcgettext(0, "Read checkpoint %u", 5);
  }
  v11 = *a3;
  if ( *a3 )
  {
    do
    {
      if ( v11 == 37 )
      {
        v15 = a3[1];
        v13 = a3 + 1;
        if ( v15 == 123 )
        {
          v21 = a3 + 2;
          v22 = strchr(v21, 125);
          v24 = v22;
          if ( v22 )
          {
            v25 = nptr;
            v26 = v22 - v13;
            if ( v22 - v13 > (unsigned long long)qword_821C0 )
            {
              qword_821C0 = v22 - v13;
              v46 = v22 - v13;
              v49 = v22;
              v36 = (char *)sub_4ECE0(nptr, v22 - v13, v23, v22);
              v24 = v49;
              v26 = v46;
              nptr = v36;
              v25 = v36;
            }
            v47 = v24;
            v42 = v26;
            memcpy(v25, v21, v26 - 1);
            v13 = v47 + 1;
            nptr[v42 - 1] = 0;
            v27 = nptr;
            if ( nptr )
            {
              v28 = v47[1];
              if ( v28 != 42 )
              {
                switch ( v28 )
                {
                  case 'T':
                    v44 = nptr;
                    sub_C770();
                    v52[7] = ",";
                    LODWORD(v30) = sub_375A0(v44, v52, 17988);
                    v31 = v44;
                    if ( (int)v30 )
                    {
                      if ( qword_82D50 )
                      {
                        qword_82D50(v44);
                        v31 = v44;
                      }
                      v48 = v31;
                      v45 = sub_358F0(v52);
                      v32 = dcgettext(0, "cannot split string '%s': %s", 5);
                      error(0, 0, v32, v48, v45);
                      v33 = (const char **)&off_7F140;
                      dword_82D58 = 2;
                    }
                    else
                    {
                      v37 = v52[0];
                      v33 = (const char **)v51;
                      if ( !v52[0] )
                        goto LABEL_54;
                      v38 = v52[1];
                      v30 = 0;
                      v33 = (const char **)v51;
                      do
                      {
                        v51[v30] = *(long long *)(v38 + 8 * v30);
                        ++v30;
                      }
                      while ( v30 != v37 );
                      if ( (int)v30 != 3 )
                      {
LABEL_54:
                        v30 = (int)v30;
                        do
                          v51[v30++] = 0;
                        while ( (int)v30 != 3 );
                      }
                    }
                    a2 += (int)sub_C900(a1, v33, 0x2Cu, 0);
                    sub_35800(v52);
                    goto LABEL_10;
                  case 'c':
LABEL_26:
                    a2 += (int)format_checkpoint_string(a1, a2, "%{%Y-%m-%d %H:%M:%S}t: %ds, %{read,wrote}T%*\r", a4, a5);
                    goto LABEL_10;
                  case 'd':
LABEL_27:
                    sub_C770();
                    a2 += (int)__fprintf_chk(a1, 1, "%.0f", a6);
                    goto LABEL_10;
                  case 's':
LABEL_23:
                    v19 = s;
                    v20 = a1;
                    goto LABEL_24;
                  case 't':
LABEL_36:
                    v43 = v27;
                    gettimeofday(&tv, 0);
                    v29 = localtime(&tv.tv_sec);
                    a2 += sub_43180(a1, v43, v29, 0, (unsigned int)(1000 * LODWORD(tv.tv_usec)));
                    goto LABEL_10;
                  case 'u':
LABEL_25:
                    v19 = (const char *)v40;
                    v20 = a1;
LABEL_24:
                    fputs_unlocked(v19, v20);
                    a2 += strlen(v19);
                    break;
                  default:
                    goto LABEL_18;
                }
                goto LABEL_10;
              }
              v34 = strtol(nptr, 0, 10);
              goto LABEL_43;
            }
          }
LABEL_18:
          IO_write_ptr = a1->_IO_write_ptr;
          if ( IO_write_ptr >= a1->_IO_write_end )
          {
            __overflow(a1, 37);
          }
          else
          {
            a1->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = 37;
          }
          v17 = *v13;
          v18 = a1->_IO_write_ptr;
          if ( v18 >= a1->_IO_write_end )
          {
            __overflow(a1, v17);
          }
          else
          {
            a1->_IO_write_ptr = v18 + 1;
            *v18 = v17;
          }
          a2 += 2LL;
        }
        else
        {
          if ( v15 == 42 )
          {
            v34 = sub_10110(a1);
LABEL_43:
            if ( a2 < v34 )
            {
              do
              {
                v35 = a1->_IO_write_ptr;
                if ( v35 < a1->_IO_write_end )
                {
                  a1->_IO_write_ptr = v35 + 1;
                  *v35 = 32;
                }
                else
                {
                  __overflow(a1, 32);
                }
                ++a2;
              }
              while ( a2 != v34 );
            }
            goto LABEL_10;
          }
          switch ( v15 )
          {
            case 'T':
              sub_C770();
              a2 += (int)sub_C900(a1, (const char **)&off_7F140, 0x2Cu, 0);
              break;
            case 'c':
              goto LABEL_26;
            case 'd':
              goto LABEL_27;
            case 's':
              goto LABEL_23;
            case 't':
              v27 = "%c";
              goto LABEL_36;
            case 'u':
              goto LABEL_25;
            default:
              goto LABEL_18;
          }
        }
      }
      else
      {
        v12 = a1->_IO_write_ptr;
        if ( v12 >= a1->_IO_write_end )
        {
          __overflow(a1, v11);
        }
        else
        {
          a1->_IO_write_ptr = v12 + 1;
          *v12 = v11;
        }
        ++a2;
        v13 = a3;
        if ( *a3 == 13 )
        {
          dword_821D8 = 1;
          a2 = 0;
        }
      }
LABEL_10:
      v11 = v13[1];
      a3 = v13 + 1;
    }
    while ( v11 );
  }
  fflush_unlocked(a1);
  return (unsigned int)a2;
}



// Function: checkpoint_finish_compile @ 0x10ba0
long long checkpoint_finish_compile()
{
  long long result; // rax

  result = (unsigned int)dword_82260;
  if ( !dword_82260 )
  {
    if ( !dword_82B78 || qword_82270 )
      return result;
    sub_10710("echo");
    result = (unsigned int)dword_82260;
  }
  if ( (int)result == 1 )
  {
    sigprocmask(0, &set, 0);
    result = (unsigned int)dword_82B78;
    if ( !dword_82B78 )
      dword_82B78 = 10;
    dword_82260 = 2;
  }
  return result;
}



// Function: checkpoint_flush_actions @ 0x10c20
void checkpoint_flush_actions()
{
  long long i; // rbx
  FILE *v1; // rbp
  long long j; // r12
  char *IO_write_ptr; // rdx
  char *v4; // rax

  for ( i = qword_82270; i; i = *(long long *)i )
  {
    while ( 1 )
    {
      if ( *(int *)(i + 8) == 3 )
      {
        v1 = qword_821D0;
        if ( qword_821D0 )
        {
          if ( dword_821D8 )
            break;
        }
      }
      i = *(long long *)i;
      if ( !i )
        return;
    }
    for ( j = sub_10110(qword_821D0); j; --j )
    {
      IO_write_ptr = v1->_IO_write_ptr;
      if ( IO_write_ptr >= v1->_IO_write_end )
      {
        __overflow(v1, 32);
        v1 = qword_821D0;
      }
      else
      {
        v1->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 32;
      }
    }
    v4 = v1->_IO_write_ptr;
    if ( v4 >= v1->_IO_write_end )
    {
      __overflow(v1, 13);
      v1 = qword_821D0;
    }
    else
    {
      v1->_IO_write_ptr = v4 + 1;
      *v4 = 13;
    }
    fflush_unlocked(v1);
  }
}



// Function: checkpoint_run @ 0x10cf0
unsigned long long checkpoint_run(unsigned char a1, double a2)
{
  long long i; // rbx
  FILE *v5; // rdi
  char *IO_write_ptr; // rax
  int v7; // eax
  char *v8; // rax
  FILE *v9; // rdi
  char *v10; // rax
  int sig; // [rsp+4h] [rbp-44h] BYREF
  unsigned long long v12; // [rsp+8h] [rbp-40h]

  v12 = __readfsqword(0x28u);
  if ( dword_82B78 )
  {
    if ( !(++dword_82278 % (unsigned int)dword_82B78) )
    {
      for ( i = qword_82270; i; i = *(long long *)i )
      {
        switch ( *(int *)(i + 8) )
        {
          case 0:
            IO_write_ptr = qword_81F28->_IO_write_ptr;
            if ( IO_write_ptr >= qword_81F28->_IO_write_end )
            {
              __overflow(qword_81F28, 46);
            }
            else
            {
              qword_81F28->_IO_write_ptr = IO_write_ptr + 1;
              *IO_write_ptr = 46;
            }
            fflush_unlocked(qword_81F28);
            break;
          case 1:
            v9 = qword_821D0;
            if ( qword_821D0 || (qword_821D0 = fopen("/dev/tty", "w"), (v9 = qword_821D0) != 0) )
            {
              v10 = v9->_IO_write_ptr;
              if ( v10 >= v9->_IO_write_end )
              {
                __overflow(v9, 7);
              }
              else
              {
                v9->_IO_write_ptr = v10 + 1;
                *v10 = 7;
              }
              fflush_unlocked(qword_821D0);
            }
            break;
          case 2:
            a2 = __fprintf_chk(stderr, 1, "%s: ", (const char *)qword_82DD0);
            sub_101A0(stderr, v7, *(char **)(i + 16), a1, dword_82278, a2);
            v8 = stderr->_IO_write_ptr;
            if ( v8 >= stderr->_IO_write_end )
            {
              __overflow(stderr, 10);
            }
            else
            {
              stderr->_IO_write_ptr = v8 + 1;
              *v8 = 10;
            }
            break;
          case 3:
            v5 = qword_821D0;
            if ( qword_821D0 || (qword_821D0 = fopen("/dev/tty", "w"), (v5 = qword_821D0) != 0) )
              sub_101A0(v5, 0, *(char **)(i + 16), a1, dword_82278, a2);
            break;
          case 4:
            sleep(*(int *)(i + 16));
            break;
          case 5:
            sub_2D260(*(long long *)(i + 16), *(long long *)qword_82828, (unsigned int)dword_82278);
            break;
          case 6:
            sub_C770();
            sub_CC30();
            break;
          case 7:
            sigwait(&set, &sig);
            break;
          default:
            continue;
        }
      }
    }
  }
  return v12 - __readfsqword(0x28u);
}



// Function: checkpoint_finish @ 0x10fa0
void checkpoint_finish()
{
  if ( dword_82B78 )
  {
    sub_10C20();
    if ( qword_821D0 )
      fclose(qword_821D0);
  }
}



// Function: process_noop @ 0x10fe0
long long process_noop()
{
  return 1;
}



// Function: report_difference @ 0x11120
unsigned long long report_difference(long long a1, long long a2, ...)
{
  const char *v2; // rax
  double v3; // xmm0_8
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-C0h]

  v6 = __readfsqword(0x28u);
  if ( a2 )
  {
    v2 = (const char *)sub_250F0(1, *(long long *)(a1 + 8));
    v3 = __fprintf_chk(qword_81F28, 1, "%s: ", v2);
    va_start(va, a2);
    ((void (*)(FILE *, long long, long long, __va_list_tag *, double))__vfprintf_chk)(qword_81F28, 1, a2, va, v3);
    __fprintf_chk(qword_81F28, 1, "\n");
  }
  sub_2FD00(1);
  return v6 - __readfsqword(0x28u);
}



// Function: process_rawdata @ 0x11250
long long process_rawdata(size_t nbytes, void *s1)
{
  long long v2; // rax
  long long v3; // r13
  char *v4; // rax
  long long result; // rax
  int v6; // r8d
  char *v7; // rax

  v2 = sub_25FE0(dword_82290, qword_82288, nbytes);
  if ( nbytes == v2 )
  {
    v6 = memcmp(s1, qword_82288, nbytes);
    result = 1;
    if ( v6 )
    {
      v7 = dcgettext(0, "Contents differ", 5);
      sub_11120((long long)&qword_82840, (long long)v7);
      return 0;
    }
  }
  else
  {
    v3 = v2;
    if ( v2 == -1 )
    {
      sub_32F00(file);
      sub_11120((long long)&qword_82840, 0);
      return 0;
    }
    else
    {
      v4 = dcngettext(0, "Could only read %lu of %lu byte", "Could only read %lu of %lu bytes", nbytes, 5);
      sub_11120((long long)&qword_82840, (long long)v4, v3, nbytes);
      return 0;
    }
  }
  return result;
}



// Function: diff_symlink @ 0x11340
unsigned long long diff_symlink()
{
  size_t v0; // rax
  char *v1; // rbp
  size_t v2; // r12
  size_t v3; // r13
  ssize_t v4; // rax
  char *v5; // rax
  char v7[1032]; // [rsp+0h] [rbp-438h] BYREF
  unsigned long long v8; // [rsp+408h] [rbp-30h]

  v8 = __readfsqword(0x28u);
  v0 = strlen(from);
  v1 = v7;
  v2 = v0;
  v3 = v0 + 1;
  if ( v0 > 0x3FF )
    v1 = (char *)sub_4ECC0(v0 + 1);
  v4 = readlinkat(oldfd, file, v1, v3);
  if ( v4 < 0 )
  {
    if ( *__errno_location() == 2 )
      sub_33160(file);
    else
      sub_33140(file);
    sub_11120((long long)&qword_82840, 0);
  }
  else if ( v2 != v4 || memcmp(from, v1, v2) )
  {
    v5 = dcgettext(0, "Symlink differs", 5);
    sub_11120((long long)&qword_82840, (long long)v5);
  }
  if ( v1 != v7 )
    free(v1);
  return v8 - __readfsqword(0x28u);
}



// Function: diff_dir @ 0x11460
unsigned long long diff_dir()
{
  char *v0; // rbp
  const char *v2; // rsi
  char *v3; // rax
  struct stat v4; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v5; // [rsp+98h] [rbp-20h]

  v0 = file;
  v5 = __readfsqword(0x28u);
  if ( !(unsigned int)sub_25FC0(file, &v4) )
  {
    if ( (v4.st_mode & 0xF000) == 0x4000 )
    {
      if ( (((unsigned short)dword_828B0 ^ LOWORD(v4.st_mode)) & 0xFFF) == 0 )
        return v5 - __readfsqword(0x28u);
      v2 = "Mode differs";
    }
    else
    {
      v2 = "File type differs";
    }
    v3 = dcgettext(0, v2, 5);
    sub_11120((long long)&qword_82840, (long long)v3);
    return v5 - __readfsqword(0x28u);
  }
  if ( *__errno_location() == 2 )
    sub_33430(v0);
  else
    sub_33410(v0);
  sub_11120((long long)&qword_82840, 0);
  return v5 - __readfsqword(0x28u);
}



// Function: diff_link @ 0x11560
unsigned long long diff_link()
{
  char *v0; // rbp
  char *v1; // r14
  char *v3; // rdi
  long long v4; // r12
  char *v5; // rax
  struct stat v6; // [rsp+0h] [rbp-158h] BYREF
  struct stat buf; // [rsp+90h] [rbp-C8h] BYREF
  unsigned long long v8; // [rsp+128h] [rbp-30h]

  v0 = file;
  v8 = __readfsqword(0x28u);
  if ( (unsigned int)sub_25FC0(file, &v6) )
  {
    v3 = v0;
    if ( *__errno_location() == 2 )
    {
LABEL_10:
      sub_33430(v3);
      goto LABEL_8;
    }
LABEL_7:
    sub_33410(v3);
LABEL_8:
    sub_11120((long long)&qword_82840, 0);
    return v8 - __readfsqword(0x28u);
  }
  v1 = from;
  if ( (unsigned int)sub_25FC0(from, &buf) )
  {
    v3 = v1;
    if ( *__errno_location() == 2 )
      goto LABEL_10;
    goto LABEL_7;
  }
  if ( !(unsigned char)sub_2C0F0(&v6, &buf) )
  {
    v4 = sub_250F0(0, from);
    v5 = dcgettext(0, "Not linked to %s", 5);
    sub_11120((long long)&qword_82840, (long long)v5, v4);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: diff_archive @ 0x11680
unsigned long long diff_archive()
{
  long long *v0; // rdi
  char v1; // bl
  long long v2; // r12
  char *v3; // rax
  char *v5; // r12
  const char *v6; // rsi
  char *v7; // rax
  char *v8; // rax
  char *v9; // rbp
  char v10; // dl
  int v11; // ecx
  const char *v12; // rsi
  char *v13; // rax
  int v14; // eax
  int v15; // r12d
  long long v16; // rax
  const char *v17; // rbx
  char *v18; // r14
  size_t v19; // rax
  char *v20; // r12
  long long v21; // rax
  __off_t v22; // r12
  bool v23; // of
  long long v24; // rax
  int v25; // eax
  int v26; // r13d
  int v27; // eax
  char *v28; // rax
  char *v29; // rdi
  char *v30; // rax
  char *v31; // rax
  char *v32; // rax
  char *v33; // rax
  char *v34; // rax
  long long (*v35)(); // rdx
  struct stat v36; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v37; // [rsp+98h] [rbp-30h]

  v0 = (long long *)qword_82408;
  v37 = __readfsqword(0x28u);
  sub_CCD0(qword_82408);
  if ( dword_82A54 )
  {
    if ( byte_82280 )
    {
      v8 = dcgettext(0, "Verify ", 5);
      __fprintf_chk(qword_81F28, 1, v8);
    }
    v0 = &qword_82840;
    sub_23BC0(&qword_82840, qword_82408, -1);
  }
  v1 = *(char *)(qword_82408 + 156);
  switch ( v1 )
  {
    case 0:
    case 48:
    case 55:
    case 83:
      goto LABEL_9;
    case 49:
      return sub_11560();
    case 50:
      return sub_11340();
    case 51:
    case 52:
    case 54:
      v9 = file;
      if ( (unsigned int)sub_25FC0(file, &v36) )
      {
        v29 = v9;
        if ( *__errno_location() == 2 )
LABEL_85:
          sub_33430(v29);
        else
          sub_33410(v9);
LABEL_70:
        sub_11120((long long)&qword_82840, 0);
        return v37 - __readfsqword(0x28u);
      }
      v10 = *(char *)(qword_82408 + 156);
      v11 = v36.st_mode & 0xF000;
      if ( v10 == 51 )
      {
        if ( v11 != 0x2000 )
          goto LABEL_67;
LABEL_76:
        v12 = "Device number differs";
        if ( dev != v36.st_rdev )
        {
LABEL_22:
          v13 = dcgettext(0, v12, 5);
          sub_11120((long long)&qword_82840, (long long)v13);
          return v37 - __readfsqword(0x28u);
        }
LABEL_20:
        if ( (((unsigned short)dword_828B0 ^ LOWORD(v36.st_mode)) & 0xFFF) == 0 )
          return v37 - __readfsqword(0x28u);
        v12 = "Mode differs";
        goto LABEL_22;
      }
      if ( v10 != 52 )
      {
        if ( v11 == 4096 )
          goto LABEL_20;
        goto LABEL_67;
      }
      if ( v11 == 24576 )
        goto LABEL_76;
LABEL_67:
      v28 = dcgettext(0, "File type differs", 5);
      sub_11120((long long)&qword_82840, (long long)v28);
      return v37 - __readfsqword(0x28u);
    case 53:
    case 68:
      if ( !(unsigned char)sub_20FC0(&qword_82840) )
        return sub_11460();
      if ( dword_829E8 )
        goto LABEL_28;
      v14 = sub_14950(qword_829D8, qword_82840, (unsigned int)dword_82808);
      v15 = v14;
      if ( v14 < 0 )
      {
        v35 = sub_26370;
      }
      else
      {
        if ( !fstat(v14, (struct stat *)&qword_82898) )
        {
          dword_829E8 = v15;
LABEL_28:
          v16 = sub_1F810(&qword_82840);
          v17 = (const char *)sub_1FED0(v16);
          if ( v17 )
          {
            v18 = qword_829D0;
            while ( 1 )
            {
              while ( 1 )
              {
                do
                {
                  if ( !*v18 )
                  {
LABEL_80:
                    if ( !*v17 )
                      return sub_11460();
LABEL_81:
                    v30 = dcgettext(0, "Contents differ", 5);
                    sub_11120((long long)&qword_82840, (long long)v30);
                    return sub_11460();
                  }
                }
                while ( (unsigned char)(*v18 - 68) > 0x15u );
                if ( ((1LL << (*v18 - 68)) & 0x114000) != 0 )
                  goto LABEL_80;
                if ( ((1LL << (*v18 - 68)) & 0x200400) == 0 )
                  break;
                if ( !strchr("YN", *v17) || strcmp(v18 + 1, v17 + 1) )
                  goto LABEL_81;
LABEL_36:
                v19 = strlen(v18) + 1;
                v18 += v19;
                v17 += v19;
              }
              if ( ((1 << (*v18 - 68)) & 1) != 0 )
              {
                if ( strcmp(v18, v17) )
                  goto LABEL_81;
                goto LABEL_36;
              }
            }
          }
          sub_10FF0(sub_10FE0);
          return sub_11460();
        }
        close(v15);
        v35 = (long long (*)())sub_26460;
      }
      sub_26490(qword_82840, 0, v35);
      return sub_11460();
    case 77:
      if ( byte_82850 )
        return sub_11460();
      v20 = file;
      if ( (unsigned int)sub_25FC0(file, &v36) )
      {
        v29 = v20;
        if ( *__errno_location() == 2 )
          goto LABEL_85;
        sub_33410(v20);
        goto LABEL_70;
      }
      if ( (v36.st_mode & 0xF000) != 0x8000 )
        goto LABEL_13;
      v21 = sub_231E0(qword_82408 + 369, 12);
      v22 = v21;
      if ( v21 < 0 )
        goto LABEL_74;
      v23 = __OFADD__(qword_828C8, v21);
      v24 = qword_828C8 + v21;
      if ( v23 || v36.st_size != v24 )
        goto LABEL_74;
      v25 = __openat_2((unsigned int)oldfd, file, (unsigned int)dword_82808);
      v26 = v25;
      if ( v25 < 0 )
      {
        sub_32EA0(file);
        goto LABEL_73;
      }
      if ( lseek(v25, v22, 0) < 0 )
      {
        sub_33200(file, v22);
        sub_11120((long long)&qword_82840, 0);
      }
      else
      {
        sub_10FF0(sub_11250);
      }
      if ( close(v26) )
        sub_32D60(file);
      return v37 - __readfsqword(0x28u);
    case 86:
      return v37 - __readfsqword(0x28u);
    default:
      if ( qword_82D50 )
      {
        qword_82D50(v0);
        v1 = *(char *)(qword_82408 + 156);
      }
      v2 = sub_4CE40(file);
      v3 = dcgettext(0, "%s: Unknown file type '%c', diffed as normal file", 5);
      error(0, 0, v3, v2, (unsigned int)v1);
      dword_82D58 = 2;
LABEL_9:
      if ( byte_82850 )
        return sub_11460();
      v5 = file;
      if ( (unsigned int)sub_25FC0(file, &v36) )
      {
        if ( *__errno_location() == 2 )
          sub_33430(v5);
        else
          sub_33410(v5);
LABEL_73:
        sub_11120((long long)&qword_82840, 0);
        sub_24660();
        return v37 - __readfsqword(0x28u);
      }
      if ( (v36.st_mode & 0xF000) == 0x8000 )
      {
        if ( ((dword_828B0 ^ v36.st_mode) & 0xFFF) != 0 )
        {
          v32 = dcgettext(0, "Mode differs", 5);
          sub_11120((long long)&qword_82840, (long long)v32);
        }
        if ( !(unsigned char)sub_2C0D0(&v36, &qword_82898) )
        {
          v31 = dcgettext(0, "Uid differs", 5);
          sub_11120((long long)&qword_82840, (long long)v31);
        }
        if ( !(unsigned char)sub_2C0E0(&v36, &qword_82898) )
        {
          v34 = dcgettext(0, "Gid differs", 5);
          sub_11120((long long)&qword_82840, (long long)v34);
        }
        if ( (unsigned int)sub_2FCC0(v36.st_mtim.tv_sec, v36.st_mtim.tv_nsec, xmmword_82938, *(&xmmword_82938 + 1)) )
        {
          v33 = dcgettext(0, "Mod time differs", 5);
          sub_11120((long long)&qword_82840, (long long)v33);
        }
        if ( *(char *)(qword_82408 + 156) == 83 || v36.st_size == qword_828C8 )
        {
          v27 = __openat_2((unsigned int)oldfd, v5, (unsigned int)dword_82808);
          dword_82290 = v27;
          if ( v27 < 0 )
          {
            sub_32EA0(v5);
            sub_24660();
            sub_11120((long long)&qword_82840, 0);
          }
          else
          {
            if ( byte_82960 )
              sub_2B690((unsigned int)v27, &qword_82840);
            else
              sub_10FF0(sub_11250);
            if ( dword_82B88 == 1
              && v36.st_size
              && (unsigned int)sub_26100(
                                 (unsigned int)dword_82290,
                                 (unsigned int)oldfd,
                                 v5,
                                 v36.st_atim.tv_sec,
                                 v36.st_atim.tv_nsec) )
            {
              sub_334B0(v5);
            }
            if ( close(dword_82290) )
              sub_32D60(v5);
          }
          return v37 - __readfsqword(0x28u);
        }
LABEL_74:
        v6 = "Size differs";
      }
      else
      {
LABEL_13:
        v6 = "File type differs";
      }
      v7 = dcgettext(0, v6, 5);
      sub_11120((long long)&qword_82840, (long long)v7);
      sub_24660();
      return v37 - __readfsqword(0x28u);
  }
}



// Function: verify_volume @ 0x11f60
unsigned long long verify_volume(long long a1)
{
  int v1; // eax
  int *v2; // rax
  int v3; // edi
  int v4; // r14d
  char *v5; // rax
  char *v6; // rax
  char *v7; // rax
  long long v8; // rax
  long long v9; // r12
  char *v10; // rax
  unsigned int v11; // r12d
  char *v12; // rax
  int v14; // eax
  short v15[2]; // [rsp+8h] [rbp-60h] BYREF
  int v16; // [rsp+Ch] [rbp-5Ch]
  char v17[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v18; // [rsp+28h] [rbp-40h]

  v18 = __readfsqword(0x28u);
  if ( (unsigned char)sub_335F0() )
  {
    if ( qword_82D50 )
      qword_82D50(a1);
    v5 = dcgettext(0, "Archive contains file names with leading prefixes removed.", 5);
    a1 = 0;
    error(0, 0, v5);
    if ( !(unsigned char)sub_30E80() )
      goto LABEL_21;
  }
  else if ( !(unsigned char)sub_30E80() )
  {
    goto LABEL_3;
  }
  if ( qword_82D50 )
    qword_82D50(a1);
  v6 = dcgettext(0, "Archive contains transformed file names.", 5);
  error(0, 0, v6);
LABEL_21:
  if ( qword_82D50 )
    qword_82D50(0);
  v7 = dcgettext(0, "Verification may fail to locate original files.", 5);
  error(0, 0, v7);
LABEL_3:
  sub_1F6E0();
  if ( !qword_82288 )
  {
    qword_82288 = (void *)sub_265A0(v15, nbytes);
    if ( qword_82B28 )
      sub_203B0();
  }
  fsync(fd);
  ioctl(fd, 0x24Bu);
  v16 = 1;
  v15[0] = 2;
  if ( fd <= 0x3FFFFFFF )
    v1 = ioctl(fd, 0x40086D01u, v15);
  else
    v1 = sub_348B0((unsigned int)(fd - 0x40000000), 1074294017, v15);
  if ( v1 >= 0 )
    goto LABEL_10;
  v2 = __errno_location();
  v3 = fd;
  if ( *v2 != 5 )
  {
    if ( fd > 0x3FFFFFFF )
      goto LABEL_9;
LABEL_38:
    if ( !lseek(v3, 0, 0) )
      goto LABEL_10;
LABEL_39:
    sub_332B0(*(long long *)ptr);
    return v18 - __readfsqword(0x28u);
  }
  if ( fd > 0x3FFFFFFF )
    v14 = sub_348B0((unsigned int)(fd - 0x40000000), 1074294017, v15);
  else
    v14 = ioctl(fd, 0x40086D01u, v15);
  v3 = fd;
  if ( v14 >= 0 )
    goto LABEL_10;
  if ( fd <= 0x3FFFFFFF )
    goto LABEL_38;
LABEL_9:
  if ( sub_34720((unsigned int)(v3 - 0x40000000), 0, 0) )
    goto LABEL_39;
LABEL_10:
  byte_82280 = 1;
  dword_81F40 = 0;
  sub_F6E0();
  while ( 1 )
  {
    v4 = sub_22570(&qword_82408, &qword_82840, 0);
    if ( v4 != 5 )
    {
      if ( v4 == 4 )
        break;
      goto LABEL_13;
    }
    v11 = 0;
    do
    {
      ++v11;
      sub_CCD0(qword_82408);
      v4 = sub_22570(&qword_82408, &qword_82840, 0);
    }
    while ( v4 == 5 );
    if ( qword_82D50 )
      qword_82D50(&qword_82408);
    v12 = dcngettext(
            0,
            "VERIFY FAILURE: %d invalid header detected",
            "VERIFY FAILURE: %d invalid headers detected",
            (int)v11,
            5);
    error(0, 0, v12, v11);
    dword_82D58 = 2;
    if ( v4 == 4 )
      break;
LABEL_13:
    if ( v4 == 3 )
    {
      sub_CCD0(qword_82408);
      if ( !byte_82B51 )
      {
        if ( (unsigned int)sub_22570(&qword_82408, &qword_82840, 0) == 3 )
          break;
        if ( (dword_81B80 & 1) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(&qword_82408);
          v8 = sub_CC60();
          v9 = sub_45370(v8, v17);
          v10 = dcgettext(0, "A lone zero block at %s", 5);
          error(0, 0, v10, v9);
        }
      }
    }
    else
    {
      sub_22C60(qword_82408, &qword_82840, &dword_82400, 1);
      sub_11680();
      sub_2FC00(&qword_82840, &qword_82840);
    }
  }
  dword_81F40 = 1;
  byte_82280 = 0;
  return v18 - __readfsqword(0x28u);
}



// Function: hash_link @ 0x12380
unsigned long long hash_link(long long *a1, unsigned long long a2)
{
  return (a1[1] ^ *a1) % a2;
}



// Function: compare_links @ 0x123a0
bool compare_links(_OWORD *a1, _OWORD *a2)
{
  return *a2 - *a1 == 0;
}



// Function: to_chars_subst @ 0x123c0
long long to_chars_subst(
        int a1,
        int a2,
        long long a3,
        int a4,
        long long (*a5)(unsigned int *),
        int a6,
        long long a7,
        long long a8)
{
  long long v10; // r15
  long long v11; // r12
  long long v12; // rbp
  long long v13; // rax
  const char *v14; // r15
  long long v15; // rdi
  long long v16; // rax
  long long v17; // r13
  long long v18; // r12
  long long v19; // rax
  long long v20; // rdi
  long long v21; // rbx
  char *v22; // rax
  char *v24; // rax
  unsigned int v27; // [rsp+3Ch] [rbp-BCh] BYREF
  char v28[32]; // [rsp+40h] [rbp-B8h] BYREF
  char v29; // [rsp+60h] [rbp-98h] BYREF
  char v30[32]; // [rsp+61h] [rbp-97h] BYREF
  char v31[32]; // [rsp+81h] [rbp-77h] BYREF
  char v32[23]; // [rsp+A1h] [rbp-57h] BYREF
  unsigned long long v33; // [rsp+B8h] [rbp-40h]

  v33 = __readfsqword(0x28u);
  if ( a2 )
  {
    if ( (unsigned long long)(8 * (a7 - 1)) > 0x3F )
    {
      v11 = -1;
      v10 = 0x8000000000000000LL;
      v12 = sub_45370(-1, v28);
    }
    else
    {
      v10 = 1LL << (8 * ((unsigned char)a7 - 1));
      v11 = v10 - 1;
      v12 = sub_45370(v10 - 1, v28);
    }
    v13 = sub_45370(v10, v31);
    *(char *)(v13 - 1) = 45;
    v14 = (const char *)(v13 - 1);
LABEL_5:
    if ( a1 )
      goto LABEL_6;
LABEL_16:
    v15 = a3;
    v17 = sub_45370(a3, &v29);
    if ( a5 )
      goto LABEL_7;
    goto LABEL_17;
  }
  if ( (unsigned long long)(3 * a7 - 3) > 0x3F )
  {
    v11 = -1;
    v14 = "0";
    v12 = sub_45370(-1, v28);
    goto LABEL_5;
  }
  v14 = "0";
  v11 = (1LL << (3 * (unsigned char)a7 - 3)) - 1;
  v12 = sub_45370(v11, v28);
  if ( !a1 )
    goto LABEL_16;
LABEL_6:
  v15 = -a3;
  v16 = sub_45370(-a3, v30);
  *(char *)(v16 - 1) = 45;
  v17 = v16 - 1;
  if ( a5 )
  {
LABEL_7:
    v18 = a5(&v27) & v11;
    v27 = (unsigned char)v27 & (dword_82BA4 == 6);
    if ( v27 )
      v18 = -v18;
    v19 = sub_45370(v18, v32);
    v20 = v27;
    v21 = v19;
    if ( v27 )
    {
      *(char *)(v19 - 1) = 45;
      v21 = v19 - 1;
    }
    if ( qword_82D50 )
      qword_82D50(v20);
    v22 = dcgettext(0, "value %s out of %s range %s..%s; substituting %s", 5);
    error(0, 0, v22, v17, a8, v14, v12, v21);
    return sub_12680(v27, v18, a4, 0, a6, a7, a8);
  }
LABEL_17:
  if ( qword_82D50 )
    qword_82D50(v15);
  v24 = dcgettext(0, "value %s out of %s range %s..%s", 5);
  error(0, 0, v24, v17, a8, v14, v12);
  dword_82D58 = 2;
  return 0;
}



// Function: open_failure_recover @ 0x12990
long long open_failure_recover(long long a1)
{
  int *v1; // rax
  unsigned int v2; // r12d
  long long v4; // rax
  long long v5; // rax
  int v6; // edx
  long long v7; // rdi

  v1 = __errno_location();
  LOBYTE(v1) = a1 != 0 && *v1 == 24;
  v2 = (unsigned int)v1;
  if ( !(char)v1 )
    return v2;
  v4 = *(long long *)(a1 + 408);
  if ( v4 )
  {
    v5 = *(long long *)(v4 + 408);
    while ( v5 )
    {
      while ( 1 )
      {
        v6 = *(int *)(v5 + 424);
        v7 = v5;
        v5 = *(long long *)(v5 + 408);
        if ( v6 <= 0 )
          break;
        if ( !v5 || *(int *)(v5 + 424) <= 0 )
        {
          sub_2FB80(v7);
          return v2;
        }
      }
    }
  }
  return 0;
}



// Function: unknown_file_error @ 0x12a20
void unknown_file_error(long long a1)
{
  long long v1; // r12
  char *v2; // rax

  if ( (dword_81B80 & 0x20) == 0 )
  {
    if ( byte_82B52 )
      return;
    goto LABEL_7;
  }
  if ( qword_82D50 )
    qword_82D50(a1);
  v1 = sub_4CE40(a1);
  v2 = dcgettext(0, "%s: Unknown file type; file ignored", 5);
  error(0, 0, v2, v1);
  if ( !byte_82B52 )
LABEL_7:
    sub_2FD00(2);
}



// Function: ensure_slash @ 0x12ac0
const char *ensure_slash(const char **a1)
{
  const char *v1; // rbp
  long long v2; // rdx
  long long v3; // rcx
  size_t i; // rbx
  char *v5; // rax
  long long v6; // r13
  long long v7; // rax
  const char *result; // rax

  v1 = *a1;
  for ( i = strlen(*a1); ; --i )
  {
    if ( !i )
    {
      v5 = (char *)v1;
      v6 = 1;
      goto LABEL_6;
    }
    if ( v1[i - 1] != 47 )
      break;
  }
  v5 = (char *)&v1[i];
  v6 = i + 1;
LABEL_6:
  if ( *v5 != 47 )
  {
    v7 = sub_4ECE0(v1, i + 2, v2, v3);
    *a1 = (const char *)v7;
    v5 = (char *)(i + v7);
  }
  *v5 = 47;
  result = *a1;
  (*a1)[v6] = 0;
  return result;
}



// Function: write_short_name @ 0x12da0
long long *write_short_name(long long a1)
{
  long long *v2; // rax
  int v3; // ecx
  long long *v4; // r8
  void *v5; // rdi
  long long v6; // rax
  long long v7; // rcx
  char v8; // dl

  v2 = (long long *)sub_D360();
  v3 = (int)v2;
  *v2 = 0;
  v4 = v2;
  v2[63] = 0;
  v5 = (void *)((unsigned long long)(v2 + 1) & 0xFFFFFFFFFFFFFFF8LL);
  v6 = 0;
  memset(v5, 0, 8LL * ((unsigned int)(v3 - (int)v5 + 512) >> 3));
  v7 = *(long long *)(a1 + 8);
  do
  {
    v8 = *(char *)(v7 + v6);
    *((char *)v4 + v6) = v8;
    if ( !v8 )
      break;
    ++v6;
  }
  while ( v6 != 100 );
  if ( dword_82BA4 == 2 )
    *((char *)v4 + 99) = 0;
  return v4;
}



// Function: add_exclusion_tag @ 0x12e10
size_t add_exclusion_tag(char *s, int a2, long long a3)
{
  long long v4; // rbx
  long long v5; // rax
  size_t result; // rax

  v4 = sub_4ECC0(40);
  v5 = qword_822D0;
  *(long long *)v4 = s;
  *(int *)(v4 + 16) = a2;
  *(long long *)(v4 + 24) = a3;
  *(long long *)(v4 + 32) = v5;
  result = strlen(s);
  qword_822D0 = v4;
  *(long long *)(v4 + 8) = result;
  return result;
}



// Function: cachedir_file_p @ 0x12f10
bool cachedir_file_p(int a1)
{
  ssize_t v1; // r8
  bool result; // al
  long long _0[9]; // [rsp+0h] [rbp+0h] BYREF

  _0[7] = __readfsqword(0x28u);
  v1 = read(a1, _0, 0x2Bu);
  result = 0;
  if ( v1 == 43 )
    return memcmp(_0, "Signature: 8a477f597d28d172789f06886806bc55", 0x2Bu) == 0;
  return result;
}



// Function: write_eot @ 0x12fe0
long long write_eot()
{
  long long *v0; // r8
  void *v1; // rbp
  size_t v2; // rax

  v0 = (long long *)sub_D360();
  *v0 = 0;
  v0[63] = 0;
  memset(
    (void *)((unsigned long long)(v0 + 1) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)v0 - (((int)v0 + 8) & 0xFFFFFFF8) + 512) >> 3));
  sub_CCD0(v0);
  v1 = (void *)sub_D360();
  v2 = sub_CD10(v1);
  memset(v1, 0, v2);
  return sub_CCD0(v1);
}



// Function: start_private_header @ 0x13050
long long *start_private_header(long long a1, unsigned long long a2, long long a3)
{
  long long *v5; // rax
  int v6; // ecx
  long long *v7; // r12
  void *v8; // rdi
  long long v9; // rax
  char v10; // dl
  int v11; // esi
  unsigned long long v12; // rcx
  char *v13; // rax
  char v14; // dl
  char *v15; // rax
  char *v16; // rax

  v5 = (long long *)sub_D360();
  v6 = (int)v5;
  *v5 = 0;
  v7 = v5;
  v5[63] = 0;
  v8 = (void *)((unsigned long long)(v5 + 1) & 0xFFFFFFFFFFFFFFF8LL);
  v9 = 0;
  memset(v8, 0, 8LL * ((unsigned int)(v6 - (int)v8 + 512) >> 3));
  do
  {
    v10 = *(char *)(a1 + v9);
    *((char *)v7 + v9) = v10;
    if ( !v10 )
      break;
    ++v9;
  }
  while ( v9 != 100 );
  if ( dword_82BA4 == 2 )
    *((char *)v7 + 99) = 0;
  sub_12680(a2 >> 63, a2, 8, 0, (int)v7 + 124, 12, (long long)"off_t");
  v11 = -1;
  if ( (unsigned long long)a3 <= 0x1FFFFFFFFLL )
    v11 = a3;
  if ( a3 < 0 )
    v11 = 0;
  sub_12680(0, v11, 8, 0, (int)v7 + 136, 12, (long long)"time_t");
  if ( (unsigned int)(dword_82BA4 - 3) <= 1 || (v12 = 33188, dword_82BA4 == 6) )
    v12 = 420;
  *((char *)v7 + 107) = 0;
  v13 = (char *)v7 + 106;
  do
  {
    v14 = v12;
    --v13;
    v12 >>= 3;
    v13[1] = (v14 & 7) + 48;
  }
  while ( (char *)v7 + 99 != v13 );
  *((char *)v7 + 115) = 0;
  v15 = (char *)v7 + 114;
  do
    *v15-- = 48;
  while ( v15 != (char *)v7 + 107 );
  *((char *)v7 + 123) = 0;
  v16 = (char *)v7 + 122;
  do
    *v16-- = 48;
  while ( v16 != (char *)v7 + 115 );
  strcpy((char *)v7 + 257, "ustar");
  *(short *)((char *)v7 + 263) = 12336;
  return v7;
}



// Function: simple_finish_header @ 0x13230
long long simple_finish_header(unsigned char *a1)
{
  int v1; // edx
  char *v2; // r9
  unsigned char *v3; // rax
  int v4; // ecx
  unsigned long long v5; // r10
  char *v6; // rax
  unsigned char *v8; // rax
  char v9; // dl

  v1 = 0;
  *(long long *)(a1 + 148) = 0x2020202020202020LL;
  v2 = a1 + 148;
  v3 = a1;
  do
  {
    v4 = *v3++;
    v1 += v4;
  }
  while ( v3 != a1 + 512 );
  v5 = v1;
  if ( v1 <= (int)&loc_3FFFE + 1 )
  {
    a1[154] = 0;
    v8 = a1 + 153;
    do
    {
      v9 = v5;
      --v8;
      v5 >>= 3;
      v8[1] = (v9 & 7) + 48;
    }
    while ( a1 + 147 != v8 );
  }
  else
  {
    if ( (dword_82BA4 & 0xFFFFFFFB) != 2 )
    {
      sub_123C0(0, 0, v1, 8, 0, (int)v2, 7, (long long)"uintmax_t");
      return sub_CCD0(a1);
    }
    a1[148] = 0x80;
    v6 = a1 + 154;
    do
    {
      *v6-- = v5;
      v5 >>= 8;
    }
    while ( v6 != v2 );
  }
  return sub_CCD0(a1);
}



// Function: finish_header @ 0x13320
long long finish_header(long long a1, unsigned char *a2, long long a3)
{
  int v3; // eax
  unsigned long long v4; // rax
  long long v5; // rcx
  unsigned char *v7; // rax

  if ( dword_82A54 )
  {
    v3 = a2[156];
    if ( (char)v3 != 75 )
    {
      v4 = (unsigned int)(v3 - 76);
      if ( (unsigned char)v4 > 0x2Cu || (v5 = 0x100008000001LL, !_bittest64(&v5, v4)) )
      {
        dword_82400 = dword_82BA4;
        sub_23BC0(a1, a2, a3);
      }
    }
  }
  if ( *(long long *)(a1 + 376) || !*(long long *)(a1 + 360) )
    return sub_13230(a2);
  v7 = (unsigned char *)sub_12C70(0, a1, a2);
  return sub_13230(v7);
}



// Function: write_gnu_long_link @ 0x133d0
long long write_gnu_long_link(long long a1, const char *a2, char a3)
{
  const char *v3; // r12
  size_t v5; // rax
  size_t v6; // rbp
  size_t v7; // r13
  long long *v8; // r14
  long long v9; // rcx
  long long i; // rax
  char v11; // dl
  long long v12; // rcx
  long long j; // rax
  char v14; // dl
  char *v15; // r14
  size_t v16; // rbx
  const char *v17; // rsi

  v3 = a2;
  v5 = strlen(a2);
  v6 = v5 + 1;
  v7 = v5;
  v8 = sub_13050((long long)"././@LongLink", v5 + 1, 0);
  if ( !byte_82AD2 )
  {
    v9 = qword_822A8;
    if ( !qword_822A8 )
    {
      sub_27EB0(0);
      sub_27F40(0);
      v9 = qword_822A8;
    }
    for ( i = 0; i != 32; ++i )
    {
      v11 = *(char *)(v9 + i);
      *((char *)v8 + i + 265) = v11;
      if ( !v11 )
        break;
    }
    *((char *)v8 + 296) = 0;
    v12 = qword_822A0;
    for ( j = 0; j != 32; ++j )
    {
      v14 = *(char *)(v12 + j);
      *((char *)v8 + j + 297) = v14;
      if ( !v14 )
        break;
    }
    *((char *)v8 + 328) = 0;
  }
  *((char *)v8 + 156) = a3;
  *(long long *)((char *)v8 + 257) = 0x20207261747375LL;
  sub_13320(a1, (unsigned char *)v8, -1);
  v15 = (char *)sub_D360();
  v16 = sub_CD10(v15);
  if ( v6 > v16 )
  {
    do
    {
      v17 = v3;
      v3 += v16;
      memcpy(v15, v17, v16);
      v6 -= v16;
      sub_CCD0(&v15[(v16 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      v15 = (char *)sub_D360();
      v16 = sub_CD10(v15);
    }
    while ( v6 > v16 );
    v7 = v6 - 1;
  }
  memcpy(v15, v3, v6);
  memset(&v15[v6], 0, v16 - v6);
  return sub_CCD0(&v15[v7 & 0xFFFFFFFFFFFFFE00LL]);
}



// Function: start_header @ 0x13570
long long *start_header(long long a1)
{
  int v2; // ebp
  long long v3; // r13
  long long v4; // r12
  size_t v5; // rax
  long long *v6; // r12
  unsigned int v7; // ebp
  unsigned int v8; // eax
  char *v9; // r9
  unsigned long long v10; // rax
  char *v11; // rdx
  char v12; // cl
  unsigned int v13; // ecx
  char *v14; // r9
  unsigned long long v15; // rdx
  char *v16; // rax
  unsigned int v17; // eax
  char *v18; // r9
  unsigned long long v19; // rdx
  char *v20; // rax
  unsigned long long v21; // rsi
  unsigned long long v22; // rbp
  int v23; // esi
  unsigned long long v24; // rdi
  unsigned int v25; // edx
  const char *v26; // rbp
  long long i; // rax
  char v28; // dl
  long long v29; // rcx
  long long j; // rax
  char v31; // dl
  unsigned long long v32; // rax
  bool v33; // zf
  long long v34; // rbp
  unsigned long long v36; // rdx
  char *v37; // rax
  char *v38; // rax
  char v39; // cl
  char *v40; // rax
  char v41; // cl
  char *v42; // rax
  char v43; // cl
  unsigned long long v44; // rax
  unsigned int v45; // edi
  int v46; // r13d
  unsigned int v47; // r13d
  int v48; // esi
  unsigned int v49; // edi
  unsigned int v50; // esi
  unsigned int v51; // edi
  long long v52; // r14
  long long v53; // r12
  char *v54; // rax
  long long v55; // rcx
  long long v56; // r12
  char *v57; // rax
  unsigned long long v58; // rbp
  long long *v59; // rax
  int *v60; // rcx
  char *v61; // rsi
  const char *v62; // rbp
  long long v63; // r12
  char *v64; // rax
  char v65; // dl
  void *src; // [rsp+10h] [rbp-58h] BYREF
  void *v67; // [rsp+18h] [rbp-50h] BYREF
  __m128i v68; // [rsp+20h] [rbp-48h]
  unsigned long long v69; // [rsp+38h] [rbp-30h]

  v2 = dword_82BA4;
  v3 = *(long long *)(a1 + 8);
  v69 = __readfsqword(0x28u);
  src = 0;
  v67 = 0;
  if ( dword_82BA4 == 4 )
  {
    if ( !(unsigned char)sub_31A80(v3) )
    {
      sub_1DA70((char *)"path");
      v6 = sub_12DA0(a1);
      goto LABEL_7;
    }
    v2 = dword_82BA4;
    v3 = *(long long *)(a1 + 8);
  }
  v4 = v3;
  v5 = strlen((const char *)v3);
  if ( v2 == 2 )
  {
    if ( v5 > 0x63 )
LABEL_111:
      sub_133D0(a1, (const char *)v3, 76);
    goto LABEL_6;
  }
  if ( v5 <= 0x64 )
  {
LABEL_6:
    v6 = sub_12DA0(a1);
    goto LABEL_7;
  }
  switch ( v2 )
  {
    case 1:
      if ( qword_82D50 )
      {
        qword_82D50(v3);
        v4 = *(long long *)(a1 + 8);
      }
      v56 = sub_4CE40(v4);
      v57 = dcgettext(0, "%s: file name is too long (max %d); not dumped", 5);
      error(0, 0, v57, v56, 99);
      goto LABEL_142;
    case 3:
    case 5:
      if ( v5 > 0x100 )
      {
        if ( qword_82D50 )
          qword_82D50(v3);
        v63 = sub_4CE40(v3);
        v64 = dcgettext(0, "%s: file name is too long (max %d); not dumped", 5);
        error(0, 0, v64, v63, 256);
LABEL_142:
        dword_82D58 = 2;
        return 0;
      }
      if ( v5 > 0x9C )
      {
        v65 = *(char *)(v3 + 155);
        v52 = 155;
      }
      else
      {
        v52 = v5 - 1;
        if ( *(char *)(v3 + v5 - 1) != 47 )
          goto LABEL_118;
        v65 = *(char *)(v3 + v5 - 2);
        v52 = v5 - 2;
      }
      while ( v65 != 47 )
      {
LABEL_118:
        if ( !--v52 )
          goto LABEL_119;
        v65 = *(char *)(v3 + v52);
      }
      v58 = v5 - 1 - v52;
      if ( v58 - 1 > 0x63 )
      {
LABEL_119:
        if ( qword_82D50 )
          qword_82D50(v3);
        v53 = sub_4CE40(v3);
        v54 = dcgettext(0, "%s: file name is too long (cannot be split); not dumped", 5);
        v55 = v53;
        v6 = 0;
        error(0, 0, v54, v55);
        dword_82D58 = 2;
        return v6;
      }
      v59 = (long long *)sub_D360();
      *v59 = 0;
      v6 = v59;
      v59[63] = 0;
      memset(
        (void *)((unsigned long long)(v59 + 1) & 0xFFFFFFFFFFFFFFF8LL),
        0,
        8LL * (((unsigned int)v59 - (((int)v59 + 8) & 0xFFFFFFF8) + 512) >> 3));
      v60 = (int *)((char *)v59 + 345);
      if ( (unsigned int)v52 >= 8 )
      {
        *(long long *)((char *)v59 + 345) = *(long long *)v3;
        *(long long *)((char *)v60 + (unsigned int)v52 - 8) = *(long long *)(v3 + (unsigned int)v52 - 8);
        qmemcpy(
          (void *)(((unsigned long long)v59 + 353) & 0xFFFFFFFFFFFFFFF8LL),
          (const void *)(v3 - ((long long)v60 - (((unsigned long long)v59 + 353) & 0xFFFFFFFFFFFFFFF8LL))),
          8LL * (((unsigned int)v52 + (int)v60 - (((int)v59 + 353) & 0xFFFFFFF8)) >> 3));
      }
      else if ( (v52 & 4) != 0 )
      {
        *v60 = *(int *)v3;
        *(int *)((char *)v60 + (unsigned int)v52 - 4) = *(int *)(v3 + (unsigned int)v52 - 4);
      }
      else if ( (int)v52 )
      {
        *(char *)v60 = *(char *)v3;
        if ( (v52 & 2) != 0 )
          *(short *)((char *)v60 + (unsigned int)v52 - 2) = *(short *)(v3 + (unsigned int)v52 - 2);
      }
      v61 = (char *)(v3 + v52 + 1);
      if ( v58 >= 8 )
      {
        *v59 = *(long long *)v61;
        *(long long *)((char *)v59 + v58 - 8) = *(long long *)&v61[v58 - 8];
        qmemcpy(
          (void *)((unsigned long long)(v59 + 1) & 0xFFFFFFFFFFFFFFF8LL),
          (const void *)(v61 - ((char *)v59 - ((unsigned long long)(v59 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
          8 * (((unsigned long long)v59 + v58 - ((unsigned long long)(v59 + 1) & 0xFFFFFFFFFFFFFFF8LL)) >> 3));
      }
      else if ( (v58 & 4) != 0 )
      {
        *(int *)v59 = *(int *)v61;
        *(int *)((char *)v59 + v58 - 4) = *(int *)&v61[v58 - 4];
      }
      else if ( v58 )
      {
        *(char *)v59 = *v61;
        if ( (v58 & 2) != 0 )
          *(short *)((char *)v59 + v58 - 2) = *(short *)&v61[v58 - 2];
      }
      break;
    case 4:
      sub_1DA70((char *)"path");
      goto LABEL_6;
    case 6:
      goto LABEL_111;
    default:
      goto LABEL_126;
  }
LABEL_7:
  if ( !v6 )
    return 0;
  sub_24E10(*(unsigned int *)(a1 + 116), a1 + 116, &src);
  sub_24EE0(*(unsigned int *)(a1 + 120), a1 + 120, &v67);
  if ( qword_82B18 )
  {
    v7 = *(int *)(a1 + 112);
    v8 = sub_45CE0(v7, (v7 & 0xF000) == 0x4000, (unsigned int)dword_82B14, qword_82B18, 0) | v7 & 0xFFFFF000;
    *(int *)(a1 + 112) = v8;
  }
  else
  {
    v8 = *(int *)(a1 + 112);
  }
  v9 = (char *)v6 + 100;
  if ( (dword_82BA4 & 0xFFFFFFFD) == 1 )
  {
    *((char *)v6 + 107) = 0;
    v10 = v8 & 0xFFF;
    v11 = (char *)v6 + 106;
    do
    {
      v12 = v10;
      --v11;
      v10 >>= 3;
      v11[1] = (v12 & 7) + 48;
    }
    while ( (char *)v6 + 99 != v11 );
    goto LABEL_13;
  }
  if ( (unsigned int)(dword_82BA4 - 3) <= 1 || dword_82BA4 == 6 )
  {
    v36 = v8 & 0xFFF;
  }
  else
  {
    v36 = v8;
    if ( v8 > 0x1FFFFF )
    {
      if ( dword_82BA4 == 2 )
      {
        *((char *)v6 + 100) = 0x80;
        v37 = (char *)v6 + 107;
        do
        {
          *v37-- = v36;
          v36 >>= 8;
        }
        while ( v9 != v37 );
      }
      else
      {
        sub_123C0(0, 0, v8, 4, 0, (int)v9, 8, (long long)"mode_t");
      }
      goto LABEL_13;
    }
  }
  *((char *)v6 + 107) = 0;
  v42 = (char *)v6 + 106;
  do
  {
    v43 = v36;
    --v42;
    v36 >>= 3;
    v42[1] = (v43 & 7) + 48;
  }
  while ( (char *)v6 + 99 != v42 );
LABEL_13:
  v13 = *(int *)(a1 + 116);
  v14 = (char *)v6 + 108;
  if ( dword_82BA4 == 4 && v13 > 0x1FFFFF )
  {
    sub_1DA70("uid");
    v15 = 0;
LABEL_89:
    *((char *)v6 + 115) = 0;
    v38 = (char *)v6 + 114;
    do
    {
      v39 = v15;
      --v38;
      v15 >>= 3;
      v38[1] = (v39 & 7) + 48;
    }
    while ( (char *)v6 + 107 != v38 );
    goto LABEL_19;
  }
  v15 = v13;
  if ( v13 <= 0x1FFFFF )
    goto LABEL_89;
  if ( (dword_82BA4 & 0xFFFFFFFB) == 2 )
  {
    *((char *)v6 + 108) = 0x80;
    v16 = (char *)v6 + 115;
    do
    {
      *v16-- = v15;
      v15 >>= 8;
    }
    while ( v14 != v16 );
  }
  else if ( !(unsigned char)sub_123C0(0, 0, v13, 4, 0, (int)v14, 8, (long long)"uid_t") )
  {
    return 0;
  }
LABEL_19:
  v17 = *(int *)(a1 + 120);
  v18 = (char *)v6 + 116;
  if ( v17 > 0x1FFFFF && dword_82BA4 == 4 )
  {
    sub_1DA70("gid");
    v19 = 0;
LABEL_93:
    *((char *)v6 + 123) = 0;
    v40 = (char *)v6 + 122;
    do
    {
      v41 = v19;
      --v40;
      v19 >>= 3;
      v40[1] = (v41 & 7) + 48;
    }
    while ( (char *)v6 + 115 != v40 );
    goto LABEL_25;
  }
  v19 = v17;
  if ( v17 <= 0x1FFFFF )
    goto LABEL_93;
  if ( (dword_82BA4 & 0xFFFFFFFB) != 2 )
  {
    if ( !(unsigned char)sub_123C0(0, 0, v17, 4, 0, (int)v18, 8, (long long)"gid_t") )
      return 0;
    v21 = *(long long *)(a1 + 136);
    if ( dword_82BA4 != 4 )
      goto LABEL_26;
    goto LABEL_86;
  }
  *((char *)v6 + 116) = 0x80;
  v20 = (char *)v6 + 123;
  do
  {
    *v20-- = v19;
    v19 >>= 8;
  }
  while ( v18 != v20 );
LABEL_25:
  v21 = *(long long *)(a1 + 136);
  if ( dword_82BA4 != 4 )
  {
LABEL_26:
    v22 = v21 >> 63;
    goto LABEL_27;
  }
LABEL_86:
  LODWORD(v22) = 0;
  if ( v21 >> 33 )
  {
    sub_1DA70("size");
    LODWORD(v21) = 0;
  }
LABEL_27:
  if ( !(unsigned char)sub_12680(v22, v21, 8, 0, (int)v6 + 124, 12, (long long)"off_t") )
    return 0;
  if ( dword_82AF0 != 1 )
  {
    if ( dword_82AF0 != 2 )
    {
      if ( dword_82AF0 )
        goto LABEL_32;
      goto LABEL_31;
    }
    if ( (*(long long *)(a1 + 256) > *((long long *)&xmmword_82AE0 + 1))
       - (*(long long *)(a1 + 256) < *((long long *)&xmmword_82AE0 + 1))
       + 2 * ((*(long long *)(a1 + 248) > (long long)xmmword_82AE0) - (*(long long *)(a1 + 248) < (long long)xmmword_82AE0)) <= 0 )
    {
LABEL_31:
      v68 = _mm_loadu_si128((const __m128i *)(a1 + 248));
      goto LABEL_32;
    }
  }
  v68 = _mm_loadu_si128((const __m128i *)&xmmword_82AE0);
LABEL_32:
  v23 = v68.m128i_i32[0];
  if ( dword_82BA4 == 4 )
  {
    if ( (unsigned long long)v68.m128i_i64[0] >> 33 || (LODWORD(v24) = 0, v68.m128i_i64[1]) )
    {
      sub_1DA70("mtime");
      v23 = v68.m128i_i32[0];
      LODWORD(v24) = 0;
      if ( (unsigned long long)v68.m128i_i64[0] >> 33 )
      {
        v68.m128i_i64[0] = 0;
        v23 = 0;
      }
    }
  }
  else
  {
    v24 = (unsigned long long)v68.m128i_i64[0] >> 63;
  }
  if ( !(unsigned char)sub_12680(v24, v23, 8, 0, (int)v6 + 136, 12, (long long)"time_t") )
    return 0;
  v25 = dword_82BA4;
  if ( (*(int *)(a1 + 112) & 0xB000) == 0x2000 )
  {
    v44 = *(long long *)(a1 + 128);
    v45 = (v44 >> 8) & 0xFFF | HIDWORD(v44) & 0xFFFFF000;
    v46 = v44 >> 12;
    LOBYTE(v46) = 0;
    v47 = (unsigned char)v44 | v46;
    if ( v45 > 0x1FFFFF && dword_82BA4 == 4 )
    {
      sub_1DA70("devmajor");
      v49 = 0;
      v48 = 0;
    }
    else
    {
      v48 = (v44 >> 8) & 0xFFF | HIDWORD(*(long long *)(a1 + 128)) & 0xFFFFF000;
      v49 = v45 >> 31;
    }
    if ( !(unsigned char)sub_12680(v49, v48, 4, 0, (int)v6 + 329, 8, (long long)"major_t") )
      return 0;
    if ( dword_82BA4 == 4 && v47 > 0x1FFFFF )
    {
      sub_1DA70("devminor");
      v51 = 0;
      v50 = 0;
    }
    else
    {
      v50 = v47;
      v51 = v47 >> 31;
    }
    if ( !(unsigned char)sub_12680(v51, v50, 4, 0, (int)v6 + 337, 8, (long long)"minor_t") )
      return 0;
  }
  else
  {
    if ( (dword_82BA4 & 0xFFFFFFFB) == 2 )
      goto LABEL_37;
    if ( !(unsigned char)sub_12680(0, 0, 4, 0, (int)v6 + 329, 8, (long long)"major_t")
      || !(unsigned char)sub_12680(0, 0, 4, 0, (int)v6 + 337, 8, (long long)"minor_t") )
    {
      return 0;
    }
  }
  v25 = dword_82BA4;
LABEL_37:
  if ( v25 == 4 )
  {
    sub_1DA70("atime");
    sub_1DA70("ctime");
    v25 = dword_82BA4;
  }
  else if ( byte_82B50 && (v25 & 0xFFFFFFFB) == 2 )
  {
    sub_12680(*(long long *)(a1 + 232) >> 63, *(long long *)(a1 + 232), 8, 0, (int)v6 + 345, 12, (long long)"time_t");
    sub_12680(*(long long *)(a1 + 264) >> 63, *(long long *)(a1 + 264), 8, 0, (int)v6 + 357, 12, (long long)"time_t");
    v25 = dword_82BA4;
  }
  if ( v25 == 1 )
  {
    *((char *)v6 + 156) = 0;
    return v6;
  }
  *((char *)v6 + 156) = 48;
  if ( v25 > 4 )
  {
    if ( v25 == 6 )
      goto LABEL_124;
    goto LABEL_126;
  }
  if ( v25 <= 2 )
  {
    if ( v25 == 2 )
    {
LABEL_124:
      *(long long *)((char *)v6 + 257) = 0x20207261747375LL;
      goto LABEL_45;
    }
LABEL_126:
    abort();
  }
  strcpy((char *)v6 + 257, "ustar");
  *(short *)((char *)v6 + 263) = 12336;
LABEL_45:
  if ( !byte_82AD2 )
  {
    if ( src )
      *(long long *)(a1 + 32) = sub_4EE30(src);
    else
      sub_27EB0(*(int *)(a1 + 116));
    if ( v67 )
      *(long long *)(a1 + 40) = sub_4EE30(v67);
    else
      sub_27F40(*(int *)(a1 + 120));
    v26 = *(const char **)(a1 + 32);
    if ( dword_82BA4 == 4 )
    {
      if ( strlen(v26) <= 0x20 && (unsigned char)sub_31A80(v26) )
      {
        v26 = *(const char **)(a1 + 32);
      }
      else
      {
        sub_1DA70("uname");
        v26 = *(const char **)(a1 + 32);
      }
    }
    for ( i = 0; i != 32; ++i )
    {
      v28 = v26[i];
      *((char *)v6 + i + 265) = v28;
      if ( !v28 )
        break;
    }
    *((char *)v6 + 296) = 0;
    if ( dword_82BA4 == 4 )
    {
      v62 = *(const char **)(a1 + 40);
      if ( strlen(v62) > 0x20 || !(unsigned char)sub_31A80(v62) )
        sub_1DA70("gname");
    }
    v29 = *(long long *)(a1 + 40);
    for ( j = 0; j != 32; ++j )
    {
      v31 = *(char *)(v29 + j);
      *((char *)v6 + j + 297) = v31;
      if ( !v31 )
        break;
    }
    *((char *)v6 + 328) = 0;
    v25 = dword_82BA4;
  }
  if ( v25 == 4 )
  {
    if ( dword_82A9C > 0 )
    {
      if ( *(long long *)(a1 + 56) )
        sub_1DA70("SCHILY.acl.access");
      if ( *(long long *)(a1 + 72) )
        sub_1DA70("SCHILY.acl.default");
    }
    if ( dword_82AA0 > 0 && *(long long *)(a1 + 48) )
      sub_1DA70("RHT.security.selinux");
    if ( dword_82A98 > 0 )
    {
      v32 = 0;
      v33 = *(long long *)(a1 + 344) == 0;
      v68.m128i_i64[0] = 0;
      v34 = *(long long *)(a1 + 352);
      if ( !v33 )
      {
        do
        {
          sub_1DA70(*(char **)(v34 + 24 * v32));
          v32 = v68.m128i_i64[0] + 1;
          v68.m128i_i64[0] = v32;
        }
        while ( v32 < *(long long *)(a1 + 344) );
      }
    }
  }
  return v6;
}



// Function: write_long_link @ 0x14390
void write_long_link(long long a1)
{
  long long v1; // r12
  char *v2; // rax

  if ( (unsigned int)dword_82BA4 > 6 )
    goto LABEL_10;
  if ( ((1LL << dword_82BA4) & 0x2A) == 0 )
  {
    if ( ((1LL << dword_82BA4) & 0x44) != 0 )
    {
      sub_133D0(a1, *(const char **)(a1 + 24), 75);
      return;
    }
    if ( dword_82BA4 == 4 )
    {
      sub_1DA70("linkpath");
      return;
    }
LABEL_10:
    abort();
  }
  if ( qword_82D50 )
    qword_82D50(a1);
  v1 = sub_4CE40(*(long long *)(a1 + 24));
  v2 = dcgettext(0, "%s: link name is too long; not dumped", 5);
  error(0, 0, v2, v1);
  dword_82D58 = 2;
}



// Function: dump_regular_file @ 0x14450
long long dump_regular_file(int fd, long long *a2)
{
  long long v2; // rbx
  long long v3; // r14
  unsigned char *v4; // rax
  long long v5; // rbp
  char *v6; // r15
  unsigned long long v7; // rax
  size_t v8; // r14
  long long v10; // rax
  long long v11; // rbx
  long long v12; // r12
  char *v13; // rax
  long long v14; // rbp
  unsigned long long v15; // rbx
  long long *v16; // rax
  unsigned int v17; // eax
  char *v18; // rdx
  long long *v19; // rax
  long long v20; // [rsp+0h] [rbp-68h]
  unsigned long long n; // [rsp+8h] [rbp-60h]
  char v22[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v23; // [rsp+28h] [rbp-40h]

  v2 = a2[17];
  v23 = __readfsqword(0x28u);
  v3 = sub_CC60();
  v4 = (unsigned char *)sub_13570((long long)a2);
  if ( !v4 )
    return 2;
  sub_13320((long long)a2, v4, v3);
  sub_C640(a2[1], a2[17], a2[17]);
  do
  {
    while ( 1 )
    {
      if ( v2 <= 0 )
        return 0;
      v5 = v2;
      v6 = (char *)sub_D360();
      v7 = sub_CD10(v6);
      v8 = v7;
      if ( v2 < v7 )
        break;
      if ( fd > 0 )
        goto LABEL_12;
      v2 -= v7;
      sub_CCD0(&v6[(v7 - 1) & 0xFFFFFFFFFFFFFE00LL]);
    }
    if ( (v2 & 0x1FF) != 0 )
    {
      v17 = 512 - (v2 & 0x1FF);
      v18 = &v6[v2];
      if ( v17 >= 8 )
      {
        *(long long *)v18 = 0;
        *(long long *)&v18[v17 - 8] = 0;
        memset(
          (void *)((unsigned long long)(v18 + 8) & 0xFFFFFFFFFFFFFFF8LL),
          0,
          8LL * ((v17 + (int)v18 - (((int)v18 + 8) & 0xFFFFFFF8)) >> 3));
      }
      else if ( (-(char)v2 & 4) != 0 )
      {
        *(int *)v18 = 0;
        *(int *)&v18[v17 - 4] = 0;
      }
      else if ( v17 )
      {
        *v18 = 0;
        if ( (-(char)v2 & 2) != 0 )
          *(short *)&v18[v17 - 2] = 0;
      }
    }
    if ( fd <= 0 )
    {
      sub_CCD0(&v6[(v2 - 1) & 0xFFFFFFFFFFFFFE00LL]);
      return 0;
    }
    v8 = v2;
LABEL_12:
    v10 = sub_25FE0(fd, v6, v8);
    if ( v10 == -1 )
    {
      sub_263A0(*a2, a2[17] - v2, v8);
      do
      {
        v19 = (long long *)sub_D360();
        v2 -= 512;
        *v19 = 0;
        v19[63] = 0;
        memset(
          (void *)((unsigned long long)(v19 + 1) & 0xFFFFFFFFFFFFFFF8LL),
          0,
          8LL * (((unsigned int)v19 - (((int)v19 + 8) & 0xFFFFFFF8) + 512) >> 3));
        sub_CCD0(v19);
      }
      while ( v2 > 0 );
      return 1;
    }
    v2 -= v10;
    v20 = v10;
    n = v2;
    sub_CCD0(&v6[(v8 - 1) & 0xFFFFFFFFFFFFFE00LL]);
  }
  while ( v20 == v8 );
  memset(&v6[v20], 0, v8 - v20);
  if ( (dword_81B80 & 0x80u) != 0 )
  {
    if ( qword_82D50 )
      qword_82D50(&v6[v20]);
    v11 = sub_45370(v2, v22);
    v12 = sub_4CE40(*a2);
    v13 = dcngettext(
            0,
            "%s: File shrank by %s byte; padding with zeros",
            "%s: File shrank by %s bytes; padding with zeros",
            n,
            5);
    error(0, 0, v13, v12, v11);
  }
  if ( !byte_82B52 )
    sub_2FD00(1);
  v14 = v5 - v8;
  if ( v14 > 0 )
  {
    v15 = v14 - 512 - ((v14 - 1) & 0xFFFFFFFFFFFFFE00LL);
    do
    {
      v16 = (long long *)sub_D360();
      v14 -= 512;
      *v16 = 0;
      v16[63] = 0;
      memset(
        (void *)((unsigned long long)(v16 + 1) & 0xFFFFFFFFFFFFFFF8LL),
        0,
        8LL * (((unsigned int)v16 - (((int)v16 + 8) & 0xFFFFFFF8) + 512) >> 3));
      sub_CCD0(v16);
    }
    while ( v14 != v15 );
  }
  return 1;
}



// Function: check_links @ 0x148b0
void check_links()
{
  long long v0; // rdi
  long long i; // rbp
  long long v2; // r12
  char *v3; // rax

  v0 = qword_822C0;
  if ( qword_822C0 )
  {
    for ( i = sub_439D0(); i; i = sub_43A20(qword_822C0, i) )
    {
      if ( *(long long *)(i + 16) )
      {
        if ( qword_82D50 )
          qword_82D50(v0);
        v2 = sub_4D2D0(i + 24);
        v3 = dcgettext(0, "Missing links to %s.", 5);
        error(0, 0, v3, v2);
      }
      v0 = qword_822C0;
    }
  }
}



// Function: subfile_open @ 0x14950
long long subfile_open(long long a1, long long a2, unsigned int a3)
{
  long long v5; // rdi
  int v6; // r12d

  if ( !byte_82298 )
  {
    byte_82298 = 1;
    strerror(2);
    dcgettext(0, "", 5);
  }
  do
  {
    v5 = (unsigned int)oldfd;
    if ( a1 )
      v5 = *(unsigned int *)(a1 + 424);
    v6 = __openat_2(v5, a2, a3);
  }
  while ( v6 < 0 && (unsigned char)sub_12990(a1) );
  return (unsigned int)v6;
}



// Function: check_exclusion_tags @ 0x149e0
long long check_exclusion_tags(long long a1, long long *a2)
{
  long long v2; // rbx
  int v3; // ebp
  unsigned char (*v4)(long long); // rax

  v2 = qword_822D0;
  if ( !qword_822D0 )
    return 0;
  while ( 1 )
  {
    v3 = sub_14950(a1, *(long long *)v2, dword_82808);
    if ( v3 < 0 )
      goto LABEL_4;
    v4 = *(unsigned char (**)(long long))(v2 + 24);
    if ( !v4 || v4((unsigned int)v3) )
      break;
    close(v3);
LABEL_4:
    v2 = *(long long *)(v2 + 32);
    if ( !v2 )
      return 0;
  }
  close(v3);
  if ( a2 )
    *a2 = *(long long *)v2;
  return *(unsigned int *)(v2 + 16);
}



// Function: restore_parent_fd @ 0x14a80
unsigned long long restore_parent_fd(long long a1)
{
  long long v1; // rbx
  int v2; // eax
  int v3; // ebp
  int v4; // eax
  int v6; // r12d
  struct stat v7; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v8; // [rsp+98h] [rbp-20h]

  v1 = *(long long *)(a1 + 408);
  v8 = __readfsqword(0x28u);
  if ( v1 && !*(int *)(v1 + 424) )
  {
    v2 = __openat_2(*(unsigned int *)(a1 + 424), "..", (unsigned int)dword_82804);
    v3 = v2;
    if ( v2 >= 0 )
    {
      if ( fstat(v2, &v7) || *(long long *)(v1 + 96) != v7.st_ino || *(long long *)(v1 + 88) != v7.st_dev )
      {
        close(v3);
        *(int *)(v1 + 424) = 2;
        return v8 - __readfsqword(0x28u);
      }
    }
    else
    {
      v4 = *__errno_location();
      v3 = -v4;
      if ( v4 > 0 )
      {
        v6 = __openat_2((unsigned int)oldfd, *(long long *)v1, (unsigned int)dword_82804);
        if ( v6 >= 0 )
        {
          if ( !fstat(v3, &v7) && *(long long *)(v1 + 96) == v7.st_ino && *(long long *)(v1 + 88) == v7.st_dev )
            v3 = v6;
          else
            close(v6);
        }
      }
    }
    *(int *)(v1 + 424) = v3;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: dump_file0 @ 0x14c50
void dump_file0(long long *a1, const char *a2, void *a3)
{
  char *v4; // r12
  void *v5; // rbp
  long long v6; // rbx
  unsigned int v7; // r13d
  const char **v8; // r14
  long long v9; // rax
  int *v10; // rax
  long long (*v11)(); // rdx
  long long v12; // rsi
  long long v13; // rax
  int v14; // eax
  long long *v15; // r8
  int v16; // r9d
  int v17; // edx
  int v18; // eax
  int v19; // eax
  long long v20; // rdx
  __syscall_slong_t v21; // rdi
  long long v22; // rcx
  long long v23; // rsi
  long long v24; // rax
  long long v25; // rdx
  long long v26; // rax
  bool v27; // zf
  char v28; // al
  unsigned int v29; // r9d
  unsigned long long v30; // rsi
  DIR *v31; // rax
  DIR *v32; // rdi
  long long v33; // rax
  char v34; // r11
  long long v35; // r13
  long long v36; // rax
  long long v37; // r14
  long long *v38; // rdi
  long long v39; // r13
  char *v40; // rax
  char v41; // r11
  int v42; // r9d
  long long v43; // rax
  long long v44; // rdi
  long long v45; // rax
  long long v46; // r14
  long long v47; // rdi
  const char *v48; // rbx
  long long v49; // rax
  long long v50; // r14
  size_t v51; // rax
  unsigned int v52; // r9d
  unsigned char *v53; // rax
  unsigned char *v54; // rsi
  long long i; // rdx
  unsigned char v56; // al
  const char *v57; // rsi
  long long v58; // r12
  char *v59; // rdx
  int v60; // eax
  char *v61; // rdx
  long long v62; // rdx
  long long v63; // rax
  unsigned int v64; // eax
  unsigned int v65; // r9d
  unsigned int v66; // ebx
  long long v67; // rdx
  long long v68; // rax
  long long v69; // rax
  size_t v70; // rbx
  long long v71; // rax
  long long v72; // r12
  unsigned char *v73; // rsi
  long long v74; // rax
  long long j; // rdx
  unsigned char v76; // cl
  char v77; // bl
  char *v78; // rsi
  long long v79; // rdi
  long long v80; // rax
  long long v81; // r13
  unsigned char *v82; // rax
  unsigned char *v83; // r12
  unsigned int v84; // edi
  unsigned long long v85; // rax
  unsigned int v86; // edi
  int v87; // eax
  unsigned int v88; // r9d
  long long v89; // r12
  char *v90; // rax
  unsigned int v91; // eax
  int v92; // eax
  long long v93; // r14
  signed long long v94; // r13
  char *v95; // rsi
  long long v96; // rdx
  long long v97; // rcx
  char *v98; // rdi
  unsigned long long v99; // r8
  void *v100; // r14
  char *v101; // rbx
  char *v102; // rbp
  size_t v103; // rax
  size_t v104; // r12
  void *v105; // rax
  char *v106; // r14
  unsigned int v107; // eax
  char *v108; // rdx
  int v109; // eax
  char *v110; // r13
  const char *v111; // rbx
  void *v112; // r14
  long long v113; // rcx
  size_t v114; // rbp
  char *v115; // rax
  char *v116; // rax
  char *v117; // r14
  size_t v118; // r13
  size_t v119; // rax
  char *v120; // r13
  char *v121; // [rsp+0h] [rbp-148h]
  unsigned long long v122; // [rsp+8h] [rbp-140h]
  char v123; // [rsp+18h] [rbp-130h]
  int v124; // [rsp+18h] [rbp-130h]
  int v125; // [rsp+18h] [rbp-130h]
  bool v126; // [rsp+1Fh] [rbp-129h]
  int bufa; // [rsp+20h] [rbp-128h]
  unsigned int bufb; // [rsp+20h] [rbp-128h]
  unsigned int buf; // [rsp+20h] [rbp-128h]
  __time_t v130; // [rsp+28h] [rbp-120h]
  __syscall_slong_t v131; // [rsp+30h] [rbp-118h]
  __off_t v132; // [rsp+38h] [rbp-110h]
  unsigned int ptrc; // [rsp+40h] [rbp-108h]
  char *ptr; // [rsp+40h] [rbp-108h]
  unsigned int ptra; // [rsp+40h] [rbp-108h]
  void *ptrd; // [rsp+40h] [rbp-108h]
  unsigned int ptre; // [rsp+40h] [rbp-108h]
  unsigned int ptrb; // [rsp+40h] [rbp-108h]
  int ptrf; // [rsp+40h] [rbp-108h]
  char ptrg; // [rsp+40h] [rbp-108h]
  char src; // [rsp+48h] [rbp-100h]
  unsigned char *srca; // [rsp+48h] [rbp-100h]
  unsigned int srcb; // [rsp+48h] [rbp-100h]
  char *srcc; // [rsp+48h] [rbp-100h]
  void *srcd; // [rsp+48h] [rbp-100h]
  char *srce; // [rsp+48h] [rbp-100h]
  unsigned char *v147; // [rsp+50h] [rbp-F8h]
  long long v148; // [rsp+50h] [rbp-F8h]
  char *v149; // [rsp+50h] [rbp-F8h]
  char *v150; // [rsp+58h] [rbp-F0h]
  long long v151; // [rsp+58h] [rbp-F0h]
  long long v152; // [rsp+60h] [rbp-E8h] BYREF
  char *s; // [rsp+68h] [rbp-E0h] BYREF
  struct stat v154; // [rsp+70h] [rbp-D8h] BYREF
  unsigned long long v155; // [rsp+108h] [rbp-40h]

  v4 = (char *)a2;
  v5 = a3;
  v6 = a1[51];
  v155 = __readfsqword(0x28u);
  v123 = byte_82B40;
  v126 = v6 == 0;
  if ( v6 )
  {
    v7 = *(int *)(v6 + 424);
    if ( byte_82B40 && !(unsigned int)sub_2DF50("add", a3) )
      return;
    v8 = (const char **)(a1 + 1);
    sub_25120(a1, v5);
    v9 = sub_33640(v5);
    sub_25120(a1 + 1, v9);
    sub_30E70(a1 + 1, 1);
    if ( (v7 & 0x80000000) != 0 )
    {
      v10 = __errno_location();
      v11 = sub_26370;
      *v10 = -v7;
LABEL_6:
      v12 = v126;
LABEL_7:
      sub_26490(v5, v12, v11);
      return;
    }
    v123 = 1;
  }
  else
  {
    v7 = oldfd;
    if ( byte_82B40 )
    {
      if ( !(unsigned int)sub_2DF50("add", a3) )
        return;
      v8 = (const char **)(a1 + 1);
      sub_25120(a1, v5);
      v13 = sub_33640(v5);
      sub_25120(a1 + 1, v13);
      sub_30E70(a1 + 1, 1);
      v123 = 0;
    }
    else
    {
      v8 = (const char **)(a1 + 1);
      sub_25120(a1, a3);
      v43 = sub_33640(v5);
      sub_25120(a1 + 1, v43);
      sub_30E70(a1 + 1, 1);
    }
  }
  v14 = fstatat(v7, a2, (struct stat *)(a1 + 11), flag);
  v15 = a1 + 11;
  v16 = v14;
  if ( v14 )
    goto LABEL_53;
  v17 = *((int *)a1 + 28);
  if ( (v17 & 0xF000) != 0x4000 )
  {
    if ( (v17 & 0xF000) != 0x8000 )
      goto LABEL_21;
    if ( byte_82A30 )
    {
      if ( !byte_82A69 || !byte_82A88 )
        goto LABEL_21;
      v67 = a1[17];
      v68 = v67 / 512;
      if ( (v67 & 0x1FF) != 0 && (unsigned long long)(v67 + 511) > 0x3FE )
        ++v68;
      if ( a1[19] >= v68 )
        goto LABEL_21;
    }
    else if ( !a1[17] && (v17 & 0x124) == 0x124 )
    {
      goto LABEL_21;
    }
  }
  v18 = sub_14950(v6, (long long)a2, dword_82808);
  if ( v18 < 0 )
  {
    v11 = sub_26370;
    goto LABEL_6;
  }
  *((int *)a1 + 106) = v18;
  bufa = v18;
  v19 = fstat(v18, (struct stat *)(a1 + 11));
  v16 = bufa;
  if ( v19 )
  {
LABEL_53:
    v11 = (long long (*)())sub_26460;
    goto LABEL_6;
  }
LABEL_21:
  v20 = a1[20];
  v21 = a1[25];
  v22 = a1[22];
  v132 = a1[17];
  v23 = (unsigned char)byte_82B50;
  a1[35] = v132;
  v24 = a1[21];
  a1[29] = v20;
  v25 = a1[23];
  a1[30] = v24;
  v26 = a1[24];
  a1[31] = v22;
  a1[32] = v25;
  v130 = v26;
  v131 = v21;
  a1[33] = v26;
  a1[34] = v21;
  if ( ((char)v23 != 1 || !v6) && (a1[14] & 0xF000) != 0x4000 )
  {
    v15 = (long long *)xmmword_82B00;
    v22 = ((long long)xmmword_82B00 < v22) - (unsigned int)((long long)xmmword_82B00 > v22);
    v27 = v25 == *((long long *)&xmmword_82B00 + 1);
    v25 = v25 < *((long long *)&xmmword_82B00 + 1);
    if ( !((unsigned char)v25 | v27) - (unsigned char)v25 + 2 * (int)v22 < 0 )
    {
      if ( !dword_82B8C
        || (v25 = ((long long)xmmword_82B00 < v26) - (unsigned int)((long long)xmmword_82B00 > v26),
            v22 = *((long long *)&xmmword_82B00 + 1) > v21,
            (*((long long *)&xmmword_82B00 + 1) < v21) - (int)v22 + 2 * (int)v25 < 0) )
      {
        if ( !(char)v23 && dword_82A54 && (dword_81B80 & 0x100) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(*((long long *)&xmmword_82B00 + 1));
          v57 = "%s: file is unchanged; not dumped";
          v58 = sub_4CE40(v5);
LABEL_73:
          v59 = dcgettext(0, v57, 5);
          error(0, 0, v59, v58);
          return;
        }
        return;
      }
    }
  }
  bufb = v16;
  v28 = sub_2BD80(a1, v23, v25, v22, v15);
  v29 = bufb;
  if ( v28 )
  {
    if ( (dword_81B80 & 0x400) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(a1);
      v57 = "%s: file is the archive; not dumped";
      v58 = sub_4CE40(v5);
      goto LABEL_73;
    }
    return;
  }
  buf = a1[14] & 0xF000;
  if ( buf == 0x4000 )
  {
    src = v28;
    ptrc = v29;
    sub_31ED0(v7, v4, a1, 0, 0);
    sub_32430(v7, v4, a1, ptrc);
    sub_32100(v7, v4);
    sub_12AC0((const char **)a1);
    sub_12AC0(v8);
    v30 = (unsigned long long)&v152;
    if ( (unsigned int)sub_149E0((long long)a1, &v152) == 3 )
    {
      v61 = dcgettext(0, "directory not dumped", 5);
      sub_12E70(*a1, v152, v61);
      return;
    }
    while ( 1 )
    {
      v31 = fdopendir(*((int *)a1 + 106));
      a1[52] = v31;
      if ( v31 )
        break;
      if ( !(unsigned char)sub_12990((long long)a1) )
      {
        v34 = src;
        goto LABEL_162;
      }
    }
    v30 = (unsigned int)dword_827E4;
    v32 = v31;
    v33 = sub_4D730(v31);
    v34 = src;
    ptr = (char *)v33;
    if ( v33 )
    {
      v35 = a1[51];
      v36 = sub_CC60(v32, v30);
      a1[17] = 0;
      v37 = v36;
      srca = (unsigned char *)sub_13570((long long)a1);
      if ( srca )
      {
        sub_17440(a1);
        if ( !byte_82B50 )
        {
          srca[156] = 53;
          v38 = a1;
          sub_13320((long long)a1, srca, v37);
          goto LABEL_35;
        }
        v38 = *(long long **)(qword_82620 + 56);
        if ( dword_82BA4 != 4 )
        {
          srca[156] = 68;
          if ( v38 )
          {
            v147 = srca;
            v93 = sub_CC60(v38, v30);
            srcc = (char *)sub_1FF00(*(long long *)(qword_82620 + 56));
            v94 = sub_1F690(srcc);
            v122 = v30;
            sub_12680((unsigned long long)v94 >> 63, v94, 8, 0, (int)v147 + 124, 12, (long long)"off_t");
            sub_13320((long long)a1, v147, v93);
            v95 = (char *)v94;
            sub_C640(a1[1], v94, v94);
            v98 = v121;
            v99 = v122;
            if ( v94 > 0 )
            {
              v150 = v4;
              v100 = v5;
              v148 = v6;
              v101 = srcc;
              while ( 1 )
              {
                v102 = (char *)sub_D360(v98, v95, v96, v97, v99);
                v103 = sub_CD10(v102);
                v104 = v103;
                if ( v103 > v94 )
                  break;
                v95 = v101;
                v94 -= v103;
                memcpy(v102, v101, v103);
                v101 += v104;
                v98 = &v102[(v104 - 1) & 0xFFFFFFFFFFFFFE00LL];
                sub_CCD0(v98);
                if ( v94 <= 0 )
                {
                  v6 = v148;
                  v4 = v150;
                  v5 = v100;
                  goto LABEL_44;
                }
              }
              v105 = v100;
              srcd = v101;
              v106 = v102;
              v6 = v148;
              v4 = v150;
              v5 = v105;
              if ( (v94 & 0x1FF) != 0 )
              {
                v107 = 512 - (v94 & 0x1FF);
                v108 = &v106[v94];
                if ( v107 >= 8 )
                {
                  *(long long *)v108 = 0;
                  *(long long *)&v108[v107 - 8] = 0;
                  memset(
                    (void *)((unsigned long long)(v108 + 8) & 0xFFFFFFFFFFFFFFF8LL),
                    0,
                    8LL * ((v107 + (int)v108 - (((int)v108 + 8) & 0xFFFFFFF8)) >> 3));
                }
                else if ( (-(char)v94 & 4) != 0 )
                {
                  *(int *)v108 = 0;
                  *(int *)&v108[v107 - 4] = 0;
                }
                else if ( v107 )
                {
                  *v108 = 0;
                  if ( (-(char)v94 & 2) != 0 )
                    *(short *)&v108[v107 - 2] = 0;
                }
              }
              memcpy(v106, srcd, v94);
              sub_CCD0(&v106[(v94 - 1) & 0xFFFFFFFFFFFFFE00LL]);
            }
            goto LABEL_44;
          }
          goto LABEL_35;
        }
        srca[156] = 53;
        if ( !v38 )
        {
LABEL_35:
          if ( dword_82AD4 )
          {
            if ( byte_82AD1 && v35 && *(long long *)(a1[51] + 88LL) != a1[11] )
            {
              if ( dword_82A54 && (dword_81B80 & 0x40000) != 0 )
              {
                if ( qword_82D50 )
                  qword_82D50(v38);
                v39 = sub_4CE40(*a1);
                v40 = dcgettext(0, "%s: file is on a different filesystem; not dumped", 5);
                error(0, 0, v40, v39);
              }
            }
            else
            {
              v109 = sub_149E0((long long)a1, &s);
              switch ( v109 )
              {
                case 1:
                  v116 = dcgettext(0, "contents not dumped", 5);
                  v117 = s;
                  sub_12E70(*a1, s, v116);
                  v118 = strlen((const char *)*a1);
                  v119 = strlen(v117);
                  v120 = (char *)sub_4ECC0(v118 + v119 + 1);
                  strcpy(v120, (const char *)*a1);
                  strcat(v120, v117);
                  sub_14BB0(a1, v117, v120);
                  free(v120);
                  break;
                case 2:
                  v115 = dcgettext(0, "contents not dumped", 5);
                  sub_12E70(*a1, s, v115);
                  break;
                case 0:
                  v110 = (char *)sub_4EE30((void *)*a1);
                  v151 = v6;
                  srce = (char *)strlen(v110);
                  v111 = ptr;
                  v149 = srce;
                  v112 = v5;
                  while ( 1 )
                  {
                    v114 = strlen(v111);
                    if ( !v114 )
                      break;
                    if ( &srce[v114] > v149 )
                    {
                      v149 = &srce[v114];
                      v110 = (char *)sub_4ECE0(v110, &srce[v114 + 1], &srce[v114], v113);
                    }
                    strcpy(&srce[(long long)v110], v111);
                    if ( !(unsigned char)sub_176D0(v110, a1) )
                      sub_14BB0(a1, v111, v110);
                    v111 += v114 + 1;
                  }
                  v6 = v151;
                  v5 = v112;
                  free(v110);
                  break;
              }
            }
          }
          goto LABEL_44;
        }
        ((void (*)(void))sub_1FF00)();
        sub_1DA70("GNU.dumpdir");
        sub_13320((long long)a1, srca, v37);
      }
LABEL_44:
      sub_14A80((long long)a1);
      free(ptr);
      v41 = 1;
      goto LABEL_45;
    }
LABEL_162:
    ptrg = v34;
    sub_26400(*a1, v30);
    v41 = ptrg;
LABEL_45:
    v42 = *((int *)a1 + 106);
    v7 = oldfd;
    if ( v6 )
      v7 = *(int *)(v6 + 424);
    if ( v41 )
    {
      if ( v42 < 0 )
      {
        v124 = *((int *)a1 + 106);
        *__errno_location() = -v124;
LABEL_50:
        sub_26490(v5, v126, sub_26460);
        goto LABEL_51;
      }
LABEL_94:
      if ( v42 )
      {
        v125 = v42;
        v87 = fstat(v42, &v154);
        v88 = v125;
        if ( v87 )
          goto LABEL_50;
      }
      else
      {
        if ( (v7 & 0x80000000) != 0 && v123 )
        {
          *__errno_location() = -v7;
          goto LABEL_50;
        }
        v92 = fstatat(v7, v4, &v154, flag);
        v88 = 0;
        if ( v92 )
          goto LABEL_50;
      }
      if ( (!((v131 < v154.st_ctim.tv_nsec)
            - (v131 > v154.st_ctim.tv_nsec)
            + 2 * ((v130 < v154.st_ctim.tv_sec) - (v130 > v154.st_ctim.tv_sec)))
         || byte_82AB8 == 1 && buf == 0x4000)
        && v154.st_size <= v132 )
      {
        if ( dword_82B88 == 1 && v88 && (v132 || buf == 0x4000) && (unsigned int)sub_26100(v88, v7, v4, a1[29], a1[30]) )
          sub_334B0(v5);
      }
      else
      {
        if ( (dword_81B80 & 0x10) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(buf);
          v89 = sub_4CE40(v5);
          v90 = dcgettext(0, "%s: file changed as we read it", 5);
          error(0, 0, v90, v89);
        }
        sub_2FD00(1);
      }
      if ( (unsigned char)sub_2FB80(a1) )
      {
        if ( byte_82AB8 )
          sub_31150(v5);
      }
      return;
    }
LABEL_51:
    sub_2FB80(a1);
    return;
  }
  v44 = qword_822C0;
  if ( qword_822C0 && (a1[13] > (unsigned long long)qword_822C8 || byte_82AB8) )
  {
    ptra = v29;
    v154.st_ino = a1[12];
    v154.st_dev = a1[11];
    v45 = sub_43960(qword_822C0, &v154);
    v29 = ptra;
    v46 = v45;
    if ( v45 )
    {
      v47 = v45 + 24;
      v48 = (const char *)sub_33640((void *)(v45 + 24));
      v49 = *(long long *)(v46 + 16);
      if ( v49 )
        *(long long *)(v46 + 16) = v49 - 1;
      srcb = ptra;
      v50 = sub_CC60(v47, 1);
      sub_25120(a1 + 3, v48);
      ptrd = (void *)((dword_82BA4 != 2) + 99LL);
      v51 = strlen(v48);
      v52 = srcb;
      if ( v51 > (unsigned long long)ptrd )
      {
        sub_14390((long long)a1);
        v52 = srcb;
      }
      a1[17] = 0;
      v44 = (long long)a1;
      ptre = v52;
      v53 = (unsigned char *)sub_13570((long long)a1);
      v29 = ptre;
      v54 = v53;
      if ( v53 )
      {
        for ( i = 0; i != 100; ++i )
        {
          v56 = v48[i];
          v54[i + 157] = v56;
          if ( !v56 )
            break;
        }
        v54[156] = 49;
        sub_13320((long long)a1, v54, v50);
        if ( byte_82AB8 )
          sub_31150((void *)*a1);
        return;
      }
    }
    v60 = a1[14] & 0xF000;
  }
  else
  {
    v60 = a1[14] & 0xF000;
  }
  switch ( v60 )
  {
    case 32768:
      ptrb = v29;
      sub_31ED0(v7, v4, a1, 0, 1);
      sub_32430(v7, v4, a1, ptrb);
      sub_32100(v7, v4);
      if ( !ptrb || !byte_82A88 )
        goto LABEL_92;
      v62 = a1[17];
      v63 = v62 / 512;
      if ( (v62 & 0x1FF) != 0 && (unsigned long long)(v62 + 511) > 0x3FE )
        ++v63;
      if ( a1[19] < v63 && (v91 = sub_2AF80(ptrb), v65 = ptrb, v66 = v91, v91 != 3) )
      {
        if ( v91 > 1 )
          goto LABEL_51;
      }
      else
      {
LABEL_92:
        v64 = sub_14450(ptrb, a1);
        v65 = ptrb;
        v66 = v64;
        if ( v64 > 1 )
        {
          if ( v64 == 3 )
            abort();
          goto LABEL_51;
        }
      }
      ptrf = v65;
      sub_12B40(a1);
      v42 = ptrf;
      if ( !v66 )
        goto LABEL_94;
      goto LABEL_51;
    case 40960:
      v69 = sub_3A040(v7, v4);
      a1[3] = v69;
      if ( !v69 )
      {
        if ( *__errno_location() == 12 )
          sub_17020();
        v12 = v126;
        v11 = sub_263D0;
        goto LABEL_7;
      }
      sub_30E70(a1 + 3, 4);
      v70 = (dword_82BA4 != 2) + 99LL;
      if ( strlen((const char *)a1[3]) > v70 )
        sub_14390((long long)a1);
      sub_32430(v7, v4, a1, 0);
      sub_32100(v7, v4);
      v71 = sub_CC60(v7, v4);
      a1[17] = 0;
      v72 = v71;
      v73 = (unsigned char *)sub_13570((long long)a1);
      if ( v73 )
      {
        v74 = a1[3];
        for ( j = 0; j != 100; ++j )
        {
          v76 = *(char *)(v74 + j);
          v73[j + 157] = v76;
          if ( !v76 )
            break;
        }
        v73[156] = 50;
        sub_13320((long long)a1, v73, v72);
        if ( byte_82AB8 )
          sub_31150(v5);
        sub_12B40(a1);
      }
      return;
    case 8192:
      v77 = 51;
      sub_31ED0(v7, v4, a1, 0, 1);
      sub_32430(v7, v4, a1, 0);
      v78 = v4;
      v79 = v7;
      sub_32100(v7, v4);
      goto LABEL_129;
    case 24576:
      v77 = 52;
      sub_31ED0(v7, v4, a1, 0, 1);
      sub_32430(v7, v4, a1, 0);
      v78 = v4;
      v79 = v7;
      sub_32100(v7, v4);
      goto LABEL_129;
    case 4096:
      v77 = 54;
      sub_31ED0(v7, v4, a1, 0, 1);
      sub_32430(v7, v4, a1, 0);
      v78 = v4;
      v79 = v7;
      sub_32100(v7, v4);
LABEL_129:
      if ( dword_82BA4 != 1 )
      {
        v80 = sub_CC60(v79, v78);
        a1[17] = 0;
        v81 = v80;
        v82 = (unsigned char *)sub_13570((long long)a1);
        v83 = v82;
        if ( v82 )
        {
          v82[156] = v77;
          if ( v77 != 54 )
          {
            v84 = (a1[16] >> 8) & 0xFFF | HIDWORD(a1[16]) & 0xFFFFF000;
            sub_12680(v84 >> 31, v84, 4, 0, (int)v82 + 329, 8, (long long)"major_t");
            v85 = a1[16];
            v86 = v85 >> 12;
            LOBYTE(v86) = 0;
            sub_12680(
              ((unsigned char)v85 | v86) >> 31,
              (unsigned char)v85 | v86,
              4,
              0,
              (int)v83 + 337,
              8,
              (long long)"minor_t");
          }
          sub_13320((long long)a1, v83, v81);
          if ( byte_82AB8 )
            sub_31150(v5);
        }
        return;
      }
LABEL_117:
      sub_12A20((long long)v5);
      return;
  }
  if ( v60 != 49152 )
    goto LABEL_117;
  if ( (dword_81B80 & 0x20) != 0 )
  {
    if ( qword_82D50 )
      qword_82D50(v44);
    v57 = "%s: socket ignored";
    v58 = sub_4CE40(v5);
    goto LABEL_73;
  }
}



// Function: create_archive @ 0x16020
long long create_archive(double a1, long long a2, long long a3)
{
  long long v3; // rax
  long long v4; // rdi
  long long v5; // rax
  char *v6; // rbp
  long long v7; // rdx
  long long v9; // rax
  long long *v10; // r15
  long long v11; // rax
  long long v12; // rbx
  const char *v13; // rbp
  const char *v14; // rbp
  char v15; // r14
  size_t v16; // r12
  int v17; // eax
  char *v18; // rbp
  size_t n; // [rsp+8h] [rbp-3E0h]
  size_t v20; // [rsp+28h] [rbp-3C0h] BYREF
  long long v21[11]; // [rsp+30h] [rbp-3B8h] BYREF
  struct stat buf; // [rsp+88h] [rbp-360h] BYREF
  int v23; // [rsp+1D8h] [rbp-210h]
  long long v24[51]; // [rsp+1F0h] [rbp-1F8h] BYREF
  long long *v25; // [rsp+388h] [rbp-60h]
  unsigned long long v26; // [rsp+3A8h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  v3 = 0;
  if ( dword_82630 != 2 )
    v3 = (unsigned char)byte_82B69 ^ 1u;
  qword_822C8 = v3;
  sub_F710(1, a3, a1);
  sub_C660();
  if ( byte_82B50 )
  {
    v20 = 0;
    sub_28CF0();
    while ( 1 )
    {
      v9 = sub_29260();
      v10 = (long long *)v9;
      if ( !v9 )
        break;
      a3 = 0;
      if ( !(unsigned char)sub_176D0(*(long long *)(v9 + 16), 0) )
      {
        v18 = (char *)v10[2];
        sub_2FB40(v24);
        v25 = 0;
        a3 = (long long)v18;
        sub_14C50(v24, v18, v18);
        sub_2FC00(v24, v18);
      }
    }
    sub_292E0();
    while ( 1 )
    {
      v11 = sub_29260();
      v12 = v11;
      if ( !v11 )
        break;
      a3 = 0;
      if ( !(unsigned char)sub_176D0(*(long long *)(v11 + 16), 0) )
      {
        v13 = *(const char **)(v12 + 16);
        n = strlen(v13);
        if ( n >= v20 )
        {
          do
            v10 = (long long *)sub_4ED20(v10, &v20);
          while ( v20 <= n );
          v13 = *(const char **)(v12 + 16);
        }
        a3 = (long long)v13;
        memcpy(v10, v13, n);
        if ( *((char *)v10 + n - 1) != 47 )
          *((char *)v10 + n++) = 47;
        sub_2FB40(v21);
        v14 = (const char *)sub_1FED0(*(long long *)(v12 + 56));
        if ( v14 )
        {
          while ( 1 )
          {
            v15 = *v14;
            if ( !*v14 )
              break;
            v16 = strlen(v14);
            if ( v15 == 89 )
            {
              if ( !v21[0] )
              {
                v17 = __openat_2((unsigned int)oldfd, *(long long *)(v12 + 16), (unsigned int)dword_82804);
                if ( v17 < 0 )
                {
                  a3 = *(long long *)(v12 + 64) == 0;
                  sub_26490(*(long long *)(v12 + 16), a3, sub_26370);
                  break;
                }
                v23 = v17;
                if ( fstat(v17, &buf) )
                {
                  a3 = *(long long *)(v12 + 64) == 0;
                  sub_26490(*(long long *)(v12 + 16), a3, sub_26460);
                  break;
                }
                v21[0] = sub_4EE30(*(void **)(v12 + 16));
              }
              for ( ; v20 < n + v16; v10 = (long long *)sub_4ED20(v10, &v20) )
                ;
              strcpy((char *)v10 + n, v14 + 1);
              sub_2FB40(v24);
              a3 = (long long)(v14 + 1);
              v25 = v21;
              sub_14C50(v24, v14 + 1, v10);
              if ( qword_82B28 )
                sub_1F780(v21);
              sub_2FC00(v24, a3);
            }
            v14 += v16 + 1;
          }
        }
        sub_2FC00(v21, a3);
      }
    }
    v4 = (long long)v10;
    free(v10);
  }
  else
  {
    while ( 1 )
    {
      v4 = 1;
      v5 = sub_281F0(1);
      v6 = (char *)v5;
      if ( !v5 )
        break;
      a3 = 0;
      if ( !(unsigned char)sub_176D0(v5, 0) )
      {
        sub_2FB40(v24);
        a3 = (long long)v6;
        v25 = 0;
        sub_14C50(v24, v6, v6);
        sub_2FC00(v24, v6);
      }
    }
  }
  sub_12FE0();
  sub_D8D0(v4, a3, v7);
  sub_31110();
  if ( qword_82B28 )
    return sub_20E40();
  else
    return v26 - __readfsqword(0x28u);
}



// Function: write_record @ 0x16620
long long write_record(int a1)
{
  void *v2; // rbp
  long long v3; // rdi
  long long result; // rax

  v2 = s1;
  s1 = (void *)qword_822E8;
  if ( byte_822E0 )
  {
    fd = 1;
    result = ((long long (*)(void))sub_F6F0)();
    fd = 0;
  }
  else
  {
    v3 = qword_81F30 + qword_822D8 - qword_81F38;
    if ( qword_81F30 + qword_822D8 != qword_81F38 )
      sub_16410(v3);
    result = sub_F6F0(v3);
  }
  s1 = v2;
  if ( a1 )
  {
    if ( !byte_822E0 )
    {
      result = qword_81F30 + qword_822D8;
      if ( qword_81F38 != qword_81F30 + qword_822D8 )
        result = sub_16410(qword_81F38 - (qword_81F30 + qword_822D8));
    }
  }
  dword_822E4 = 0;
  return result;
}



// Function: write_recent_blocks @ 0x166d0
long long write_recent_blocks(long long a1, long long a2)
{
  long long i; // rbx
  char *v4; // rsi
  long long result; // rax
  long long *v6; // rcx
  long long v7; // rdx
  bool v8; // zf

  if ( a2 )
  {
    for ( i = 0; i != a2; ++i )
    {
      while ( 1 )
      {
        v4 = (char *)(a1 + (i << 9));
        v6 = (long long *)(qword_822E8 + ((long long)dword_822E4 << 9));
        v7 = *(long long *)v4;
        result = (unsigned int)++dword_822E4;
        *v6 = v7;
        v6[63] = *((long long *)v4 + 63);
        v8 = (int)result == dword_82BA0;
        qmemcpy(
          (void *)((unsigned long long)(v6 + 1) & 0xFFFFFFFFFFFFFFF8LL),
          (const void *)(v4 - ((char *)v6 - ((unsigned long long)(v6 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
          8LL * (((unsigned int)v6 - (((int)v6 + 8) & 0xFFFFFFF8) + 512) >> 3));
        if ( v8 )
          break;
        if ( a2 == ++i )
          return result;
      }
      result = sub_16620(1);
    }
  }
  return result;
}



// Function: fatal_exit @ 0x16fd0
void fatal_exit()
{
  char *v0; // rax

  if ( qword_822F0 )
    qword_822F0();
  v0 = dcgettext(0, "Error is not recoverable: exiting now", 5);
  error(2, 0, v0);
  abort();
}



// Function: xalloc_die @ 0x17020
void xalloc_die()
{
  char *v0; // rax

  v0 = dcgettext(0, "memory exhausted", 5);
  error(0, 0, "%s", v0);
  sub_16FD0();
}



// Function: hg_initfn @ 0x17060
int *hg_initfn(int *a1)
{
  int *result; // rax

  result = a1;
  if ( !a1 )
    result = &unk_822F8;
  *result = 0x8000000;
  return result;
}



// Function: bzr_addfn @ 0x17080
const unsigned short **bzr_addfn(long long a1, const char *a2, unsigned int a3)
{
  const unsigned short **result; // rax
  long long v6; // rdx

  result = __ctype_b_loc();
  while ( 1 )
  {
    v6 = *a2;
    if ( ((*result)[v6] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v6 && (char)v6 != 35 )
  {
    if ( (char)v6 == 33 )
    {
      if ( a2[1] == 33 )
      {
        a2 += 2;
      }
      else
      {
        ++a2;
        a3 |= 0x20000000u;
      }
    }
    if ( !strncmp(a2, "RE:", 3u) )
    {
      a2 += 3;
      a3 = a3 & 0xE7FFFFFF | 0x8000000;
    }
    return (const unsigned short **)sub_40C90(a1, a2, a3);
  }
  return result;
}



// Function: git_addfn @ 0x17130
const unsigned short **git_addfn(long long a1, char *a2, unsigned int a3)
{
  const unsigned short **result; // rax
  long long v6; // rcx

  result = __ctype_b_loc();
  while ( 1 )
  {
    v6 = *a2;
    if ( ((*result)[v6] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v6 && (char)v6 != 35 )
  {
    if ( (char)v6 == 92 )
      a2 += a2[1] == 35;
    return (const unsigned short **)sub_40C90(a1, a2, a3);
  }
  return result;
}



// Function: hg_addfn @ 0x171b0
int hg_addfn(long long a1, const char *a2, int a3, int *a4)
{
  const unsigned short **v7; // rax
  const unsigned short *v8; // r14
  long long v9; // rdx
  size_t v10; // rax
  size_t v11; // r14
  int v12; // eax
  unsigned int v13; // ebx
  unsigned int v14; // eax
  const char *v15; // r13
  long long v16; // rax
  char *v17; // r15
  size_t n; // [rsp+8h] [rbp-40h]

  v7 = __ctype_b_loc();
  v8 = *v7;
  while ( 1 )
  {
    v9 = *a2;
    if ( (v8[v9] & 0x2000) == 0 )
      break;
    ++a2;
  }
  if ( (char)v9 && (char)v9 != 35 )
  {
    if ( !strncmp(a2, "syntax:", 7u) )
    {
      v15 = a2 + 7;
      if ( (v8[a2[7]] & 0x2000) != 0 )
      {
        do
          v16 = *++v15;
        while ( (v8[v16] & 0x2000) != 0 );
      }
      LODWORD(v7) = strcmp(v15, "regexp");
      if ( (int)v7 )
      {
        LODWORD(v7) = strcmp(v15, "glob");
        if ( !(int)v7 )
          *a4 = 0x10000000;
      }
      else
      {
        *a4 = 0x8000000;
      }
    }
    else
    {
      v10 = strlen(a2);
      v11 = v10;
      if ( a2[v10 - 1] == 47 )
      {
        n = v10 - 1;
        a3 |= 0x4000008u;
        v17 = (char *)sub_4ECC0(v10);
        memcpy(v17, a2, n);
        a2 = v17;
        v17[v11 - 1] = 0;
        sub_40820(a1, v17);
      }
      v12 = a3;
      v13 = a3 & 0xF7FFFFFF;
      v14 = v12 & 0xEFFFFFFF;
      if ( *a4 == 0x8000000 )
        v13 = v14;
      LODWORD(v7) = sub_40C90(a1, a2, v13 | *a4);
    }
  }
  return (int)v7;
}



// Function: cvs_addfn @ 0x17330
unsigned long long cvs_addfn(long long a1, long long a2, unsigned int a3)
{
  unsigned long long v4; // rbx
  long long v5; // rsi
  long long v7[29]; // [rsp+0h] [rbp-118h] BYREF
  unsigned long long v8; // [rsp+E8h] [rbp-30h]

  v8 = __readfsqword(0x28u);
  if ( !(unsigned int)sub_375A0(a2, v7, 2116) )
  {
    if ( v7[0] )
    {
      v4 = 0;
      do
      {
        v5 = *(long long *)(v7[1] + 8 * v4++);
        sub_40C90(a1, v5, a3);
      }
      while ( v7[0] > v4 );
    }
    sub_35800(v7);
  }
  return v8 - __readfsqword(0x28u);
}



// Function: excfile_add @ 0x173d0
long long excfile_add(char *src, int a2)
{
  size_t v2; // rax
  long long v3; // rax
  long long v4; // rbx
  long long result; // rax

  v2 = strlen(src);
  v3 = sub_4ECC0(v2 + 16);
  *(long long *)v3 = 0;
  v4 = v3;
  *(int *)(v3 + 8) = a2;
  strcpy((char *)(v3 + 12), src);
  result = qword_82300;
  if ( qword_82300 )
    *(long long *)qword_82300 = v4;
  else
    qword_82308 = v4;
  qword_82300 = v4;
  return result;
}



// Function: info_attach_exclist @ 0x17440
long long info_attach_exclist(long long a1)
{
  long long result; // rax
  long long v2; // rbx
  long long *v4; // r15
  unsigned int v5; // eax
  int v6; // r14d
  long long v7; // rdi
  FILE *v8; // r13
  long long v9; // rax
  const char *v10; // rdi
  char **v11; // r14
  char *v12; // rax
  char *v13; // r9
  long long v14; // rax
  char *v15; // rdi
  long long v16; // rax
  int v17; // edx
  char *v18; // r13
  int *v19; // rax
  int v20; // r12d
  const char *v21; // rax
  long long i; // [rsp+0h] [rbp-48h]
  long long v23; // [rsp+8h] [rbp-40h]

  result = *(long long *)(a1 + 432);
  v23 = result;
  if ( !result )
  {
    v2 = qword_82308;
    if ( qword_82308 )
    {
      v4 = 0;
      do
      {
        while ( faccessat(*(int *)(a1 + 424), (const char *)(v2 + 12), 0, 0) )
        {
LABEL_4:
          v2 = *(long long *)v2;
          if ( !v2 )
            goto LABEL_19;
        }
        v5 = sub_14950(a1, v2 + 12, 0);
        v6 = v5;
        if ( v5 == -1 )
        {
          sub_32EA0(v2 + 12);
          goto LABEL_4;
        }
        v7 = v5;
        v8 = fdopen(v5, "r");
        if ( !v8 )
        {
          if ( qword_82D50 )
            qword_82D50(v7);
          v18 = dcgettext(0, "%s: fdopen failed", 5);
          v19 = __errno_location();
          error(0, *v19, v18, v2 + 12);
          dword_82D58 = 2;
          close(v6);
          goto LABEL_4;
        }
        v9 = sub_40920();
        v10 = off_7F160;
        v11 = &off_7F160;
        for ( i = v9; v10; v11 += 5 )
        {
          if ( !strcmp(v10, (const char *)(v2 + 12)) )
            break;
          v10 = v11[5];
        }
        v12 = v11[3];
        v13 = v11[4];
        if ( v12 )
        {
          v14 = ((long long (*)(char *))v12)(v11[4]);
          v11[4] = (char *)v14;
          v13 = (char *)v14;
        }
        v15 = v11[2];
        if ( (unsigned int)sub_41000(v15, i, v8, 1342177281, 10, v13) )
        {
          v20 = *__errno_location();
          if ( qword_82D50 )
            qword_82D50(v15);
          v21 = (const char *)sub_4CE40(v2 + 12);
          error(0, v20, "%s", v21);
          sub_16FD0();
        }
        fclose(v8);
        v16 = sub_4ECC0(32);
        v17 = *(int *)(v2 + 8);
        *(long long *)(v16 + 24) = i;
        if ( v17 )
          v17 = *((int *)v11 + 2);
        *(int *)(v16 + 16) = v17;
        *(long long *)(v16 + 8) = v4;
        *(long long *)v16 = 0;
        if ( !v4 )
        {
          v23 = v16;
          v4 = (long long *)v16;
          goto LABEL_4;
        }
        v2 = *(long long *)v2;
        *v4 = v16;
        v4 = (long long *)v16;
      }
      while ( v2 );
    }
LABEL_19:
    *(long long *)(a1 + 432) = v23;
    return v23;
  }
  return result;
}



// Function: info_free_exclist @ 0x17680
void info_free_exclist(long long a1)
{
  void **v1; // rbx
  void **v2; // rbp

  v1 = *(void ***)(a1 + 432);
  while ( v1 )
  {
    v2 = v1;
    v1 = (void **)*v1;
    sub_40930(v2[3]);
    free(v2);
  }
  *(long long *)(a1 + 432) = 0;
}



// Function: excluded_name @ 0x176d0
long long excluded_name(char *a1, long long a2)
{
  long long v2; // r14
  int v3; // r12d
  void *v4; // rbp
  char *j; // r15
  long long **i; // rbx
  unsigned char v8; // [rsp+Fh] [rbp-39h]

  v2 = a2;
  v8 = sub_40AE0(qword_82B60, a1);
  if ( !v8 && a2 )
  {
    v3 = 0;
    v4 = 0;
    j = 0;
    do
    {
      for ( i = *(long long ***)(v2 + 432); i; i = (long long **)*i )
      {
        if ( ((int)i[2] & v3) == 0 )
        {
          if ( (unsigned char)sub_40AE0(i[3], a1) )
            goto LABEL_14;
          if ( !j )
          {
            for ( j = a1; *j == 46; j += 2 )
            {
              if ( j[1] != 47 )
                break;
            }
          }
          if ( (unsigned char)sub_40AE0(i[3], j) )
          {
LABEL_14:
            v8 = 1;
            goto LABEL_15;
          }
          if ( v4 )
          {
            if ( (unsigned char)sub_40AE0(i[3], v4) )
              goto LABEL_14;
          }
          else
          {
            v4 = (void *)sub_40250(a1);
            if ( (unsigned char)sub_40AE0(i[3], v4) )
              goto LABEL_14;
          }
        }
      }
      v2 = *(long long *)(v2 + 408);
      v3 = 2;
    }
    while ( v2 );
LABEL_15:
    free(v4);
  }
  return v8;
}



// Function: exclude_vcs_ignores @ 0x17810
long long exclude_vcs_ignores()
{
  char *v0; // rdi
  char **v1; // rbx
  long long result; // rax

  v0 = off_7F160;
  if ( off_7F160 )
  {
    v1 = &off_7F160;
    do
    {
      v1 += 5;
      result = sub_173D0(v0, 0);
      v0 = *v1;
    }
    while ( *v1 );
  }
  return result;
}



// Function: find_delayed_link_source @ 0x17850
long long *find_delayed_link_source(char *file)
{
  long long *v1; // r12
  struct stat v3; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-20h]

  v1 = qword_82318;
  v4 = __readfsqword(0x28u);
  if ( !qword_82318 )
    return 0;
  if ( fstatat(oldfd, file, &v3, 256) )
  {
    if ( *__errno_location() != 2 )
    {
      v1 = 0;
      sub_33410(file);
      return v1;
    }
    return 0;
  }
  while ( v1[1] != v3.st_dev || v1[2] != v3.st_ino )
  {
    v1 = (long long *)*v1;
    if ( !v1 )
      return 0;
  }
  return v1;
}



// Function: check_time @ 0x17910
void check_time(long long a1, long long a2, __syscall_slong_t a3)
{
  long long v5; // rbp
  char *v6; // rdx
  long long v7; // rdi
  long long v8; // r15
  __syscall_slong_t v9; // r14
  long long v10; // r14
  long long v11; // rbp
  char *v12; // rax
  struct timespec v13; // [rsp+0h] [rbp-68h] BYREF
  char v14[40]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v15; // [rsp+38h] [rbp-30h]

  v15 = __readfsqword(0x28u);
  if ( a2 < 0 )
  {
    if ( (dword_81B80 & 0x8000) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(a1);
      v5 = sub_23240(a2, a3, 1);
      v6 = dcgettext(0, "%s: implausibly old time stamp %s", 5);
      error(0, 0, v6, a1, v5);
    }
  }
  else if ( (a3 < stru_82A10.tv_nsec)
          - (a3 > stru_82A10.tv_nsec)
          + 2 * ((a2 < stru_82A10.tv_sec) - (a2 > stru_82A10.tv_sec)) < 0 )
  {
    sub_43220(&v13);
    v7 = a3 > v13.tv_nsec;
    if ( (int)((a3 < v13.tv_nsec) - v7 + 2 * ((a2 < v13.tv_sec) - (a2 > v13.tv_sec))) < 0 )
    {
      v8 = a2 - v13.tv_sec;
      v9 = a3 - v13.tv_nsec;
      if ( a3 - v13.tv_nsec < 0 )
      {
        v9 += 1000000000LL;
        --v8;
      }
      if ( (dword_81B80 & 0x8000) != 0 )
      {
        if ( qword_82D50 )
          qword_82D50(v7);
        v10 = sub_257A0(v8, v9, v14);
        v11 = sub_23240(a2, a3, 1);
        v12 = dcgettext(0, "%s: time stamp %s is %s s in the future", 5);
        error(0, 0, v12, a1, v11, v10);
      }
    }
  }
}



// Function: mark_after_links @ 0x17ae0
unsigned long long mark_after_links(long long *a1)
{
  struct stat vars0; // [rsp+0h] [rbp+0h] BYREF
  unsigned long long vars98; // [rsp+98h] [rbp+98h]

  vars98 = __readfsqword(0x28u);
  do
  {
    *((char *)a1 + 88) = 1;
    if ( !(unsigned int)sub_25FC0((char *)a1[20], &vars0) )
    {
      a1[1] = vars0.st_dev;
      a1[2] = vars0.st_ino;
      a1 = (long long *)*a1;
      if ( !a1 )
        return vars98 - __readfsqword(0x28u);
      continue;
    }
    sub_33410(a1[20]);
    a1 = (long long *)*a1;
    if ( !a1 )
      break;
  }
  while ( !*((char *)a1 + 88) );
  return vars98 - __readfsqword(0x28u);
}



// Function: delay_set_stat @ 0x17b80
unsigned long long delay_set_stat(char *src, long long a2, int a3, int a4, int a5, int a6)
{
  size_t v10; // rax
  long long *v11; // r15
  const char *v12; // r13
  long long v13; // rsi
  __m128i v14; // xmm0
  __m128i v15; // xmm1
  int v16; // eax
  int v17; // eax
  void *v18; // rdi
  long long v19; // rax
  void *v20; // rdi
  long long v21; // rax
  int v23; // eax
  void *v24; // rax
  long long v25; // rax
  size_t v28; // [rsp+8h] [rbp-E0h]
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v30; // [rsp+A8h] [rbp-40h]

  v30 = __readfsqword(0x28u);
  v10 = strlen(src);
  v11 = qword_82320;
  v28 = v10;
  if ( !qword_82320 )
  {
LABEL_20:
    v11 = (long long *)sub_4ECC0(168);
    v24 = qword_82320;
    qword_82320 = v11;
    *v11 = v24;
    v11[19] = v28;
    v25 = sub_4EE30(src);
    *((char *)v11 + 88) = 0;
    v11[20] = v25;
    if ( a2 )
    {
      v11[1] = *(long long *)(a2 + 88);
      v11[2] = *(long long *)(a2 + 96);
      *((int *)v11 + 6) = a5;
      goto LABEL_7;
    }
    *((int *)v11 + 6) = a5;
LABEL_15:
    *((int *)v11 + 18) = a3;
    *((char *)v11 + 80) = 1;
    *((int *)v11 + 19) = a4;
    v23 = dword_82580;
    *((int *)v11 + 21) = a6;
    *((int *)v11 + 23) = v23;
    v11[12] = 0;
    v11[13] = 0;
    v11[14] = 0;
    v11[15] = 0;
    v11[16] = 0;
    v11[18] = 0;
    v11[17] = 0;
    if ( !(unsigned char)sub_25B20(src) )
      return v30 - __readfsqword(0x28u);
    return sub_17AE0(v11);
  }
  while ( 1 )
  {
    v12 = (const char *)v11[20];
    if ( !strcmp(v12, src) )
      break;
    v11 = (long long *)*v11;
    if ( !v11 )
      goto LABEL_20;
  }
  if ( *((char *)v11 + 80) )
  {
    if ( fstatat(oldfd, v12, &buf, *((int *)v11 + 21)) )
    {
      sub_33410(v11[20]);
    }
    else
    {
      v11[1] = buf.st_dev;
      v11[2] = buf.st_ino;
    }
  }
  *((int *)v11 + 6) = a5;
  if ( !a2 )
    goto LABEL_15;
LABEL_7:
  v13 = *(long long *)(a2 + 48);
  v14 = _mm_loadu_si128((const __m128i *)(a2 + 232));
  v15 = _mm_loadu_si128((const __m128i *)(a2 + 248));
  *((int *)v11 + 7) = *(int *)(a2 + 116);
  v16 = *(int *)(a2 + 120);
  *((int *)v11 + 18) = a3;
  *((int *)v11 + 8) = v16;
  *((char *)v11 + 80) = 0;
  *((int *)v11 + 19) = a4;
  v17 = dword_82580;
  *((int *)v11 + 21) = a6;
  *((int *)v11 + 23) = v17;
  v11[12] = 0;
  *(__m128i *)(v11 + 5) = v14;
  *(__m128i *)(v11 + 7) = v15;
  sub_25120(v11 + 12, v13);
  v18 = *(void **)(a2 + 56);
  if ( v18 )
  {
    v19 = sub_4EDF0(v18, *(long long *)(a2 + 64) + 1LL);
    v20 = *(void **)(a2 + 72);
    v11[13] = v19;
    v11[14] = *(long long *)(a2 + 64);
    if ( v20 )
    {
LABEL_9:
      v11[15] = sub_4EDF0(v20, *(long long *)(a2 + 80) + 1LL);
      v21 = *(long long *)(a2 + 80);
      goto LABEL_10;
    }
  }
  else
  {
    v20 = *(void **)(a2 + 72);
    v11[13] = 0;
    v11[14] = 0;
    if ( v20 )
      goto LABEL_9;
  }
  v11[15] = 0;
  v21 = 0;
LABEL_10:
  v11[16] = v21;
  sub_1D820(a2, v11 + 18, v11 + 17);
  if ( !(unsigned char)sub_25B20(src) )
    return v30 - __readfsqword(0x28u);
  return sub_17AE0(v11);
}



// Function: make_directories @ 0x17ec0
long long make_directories(char *src, char *a2)
{
  char *v2; // rbx
  char v3; // al
  char v4; // al
  short v5; // r12
  unsigned int v6; // r12d
  __mode_t v7; // r15d
  int *v8; // rax
  int v9; // r13d
  int *v10; // r12
  unsigned int v11; // r15d
  char v13; // al
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v15; // [rsp+A8h] [rbp-40h]

  v2 = src;
  v15 = __readfsqword(0x28u);
  if ( *src )
  {
LABEL_2:
    while ( 1 )
    {
      v3 = *++v2;
      if ( !v3 )
        break;
      while ( src != v2 )
      {
        if ( v3 != 47 )
          break;
        v4 = *(v2 - 1);
        if ( v4 == 47 )
          break;
        if ( v4 == 46 )
        {
          if ( v2 == src + 1 )
            break;
          v13 = *(v2 - 2);
          if ( v13 == 47 || v13 == 46 && (v2 == src + 2 || *(v2 - 3) == 47) )
            break;
        }
        v5 = dword_8232C;
        *v2 = 0;
        v6 = ~v5 & 0x1FF;
        v7 = v6;
        if ( !byte_82330 )
          LOBYTE(v7) = v6 | 0xC0;
        if ( !mkdirat(oldfd, src, v7) )
        {
          sub_17B80(src, 0, v7 & ~dword_82328, 511, v6, 256);
          sub_23C30(src, (unsigned int)((int)v2 - (int)src), v6);
          *a2 = 1;
          *v2 = 47;
          goto LABEL_2;
        }
        v8 = __errno_location();
        v9 = *v8;
        v10 = v8;
        if ( *v8 != 17 )
        {
          v11 = fstatat(oldfd, src, &buf, 0);
          if ( v11 )
          {
            *v10 = v9;
            sub_32E40(src);
            *v2 = 47;
            return v11;
          }
        }
        *v2 = 47;
        v3 = *++v2;
        if ( !v3 )
          return 0;
      }
    }
  }
  return 0;
}



// Function: fd_chmod @ 0x18100
long long fd_chmod(unsigned int a1, long long a2, unsigned int a3, int a4, int a5)
{
  int *v8; // rax
  int v9; // edx
  int *v10; // r15

  if ( !(unsigned int)((long long (*)(void))sub_18090)() )
    return 0;
  v8 = __errno_location();
  v9 = *v8;
  v10 = v8;
  if ( *v8 != 1 )
  {
    if ( !a4 || a5 == 50 )
    {
      if ( !v9 || a5 == 50 && (v9 == 38 || v9 == 95) )
        return 0;
    }
    else
    {
      if ( v9 == 95 || v9 == 38 )
      {
        if ( !(unsigned int)sub_18090(a1, a2, a3, 0) )
          return 0;
        v9 = *v10;
      }
      if ( !v9 )
        return 0;
    }
  }
  *v10 = v9;
  return 0xFFFFFFFFLL;
}



// Function: set_stat @ 0x181b0
long long set_stat(char *file, long long a2, int a3, int a4, int a5, char a6, char a7, unsigned int a8)
{
  unsigned int v9; // r14d
  int v11; // ebx
  char v12; // r11
  int v13; // r8d
  int v14; // r9d
  int v15; // r13d
  int v16; // r9d
  int v18; // eax
  int *v19; // rax
  int v20; // eax
  int v21; // eax
  int v22; // eax
  unsigned int v23; // r13d
  int v24; // eax
  int *v25; // rax
  int v26; // eax
  int v27; // [rsp+Ch] [rbp-ECh]
  int v28; // [rsp+Ch] [rbp-ECh]
  char owner; // [rsp+14h] [rbp-E4h]
  char ownerc; // [rsp+14h] [rbp-E4h]
  __uid_t ownera; // [rsp+14h] [rbp-E4h]
  __uid_t ownerb; // [rsp+14h] [rbp-E4h]
  char ownerd; // [rsp+14h] [rbp-E4h]
  char groupa; // [rsp+18h] [rbp-E0h]
  __gid_t group; // [rsp+18h] [rbp-E0h]
  char groupb; // [rsp+18h] [rbp-E0h]
  char v37; // [rsp+1Ch] [rbp-DCh]
  char v38; // [rsp+1Ch] [rbp-DCh]
  struct stat buf; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v40; // [rsp+B8h] [rbp-40h]

  v9 = a6;
  v11 = a5;
  v12 = a7;
  v40 = __readfsqword(0x28u);
  v27 = ~a5;
  if ( byte_82A68 )
  {
    if ( dword_82AA8 > 0 )
    {
      v13 = 511;
      if ( a7 )
        goto LABEL_3;
LABEL_17:
      ownera = *(int *)(a2 + 116);
      group = *(int *)(a2 + 120);
      if ( a3 >= 0 )
      {
        v37 = a6;
        if ( !fchown(a3, ownera, group) )
          goto LABEL_39;
        v19 = __errno_location();
        a6 = v37;
        v20 = *v19;
        if ( v20 != 38 && v20 != 95 )
        {
LABEL_21:
          if ( a6 != 50 || (v21 = *__errno_location(), v21 != 38) && v21 != 95 )
            sub_32CE0(file, ownera, group);
LABEL_25:
          v13 = 4095;
          if ( dword_82AA4 <= 0 )
            v13 = 511;
          goto LABEL_3;
        }
      }
      v38 = a6;
      v24 = fchownat(oldfd, file, ownera, group, a8);
      a6 = v38;
      if ( v24 )
        goto LABEL_21;
LABEL_39:
      if ( (((unsigned char)a4 | (unsigned char)v27) & 0x49) != 0 )
      {
        v11 &= ~(a4 & 0xC00);
        v27 = ~v11;
      }
      goto LABEL_25;
    }
  }
  else
  {
    v13 = 511;
    if ( a7 )
      goto LABEL_3;
    if ( byte_82B50 )
      *(__m128i *)&buf.st_dev = _mm_loadu_si128((const __m128i *)(a2 + 232));
    else
      buf.st_ino = 1073741822;
    owner = a6;
    *(__m128i *)&buf.st_nlink = _mm_loadu_si128((const __m128i *)(a2 + 248));
    v18 = sub_412D0((unsigned int)a3, (unsigned int)oldfd, file, &buf, a8);
    a6 = owner;
    v12 = a7;
    if ( v18 )
    {
      if ( owner != 50 || (v25 = __errno_location(), a6 = 50, v12 = a7, v26 = *v25, v26 != 95) && v26 != 38 )
      {
        groupb = v12;
        ownerd = a6;
        sub_334B0(file);
        v12 = groupb;
        a6 = ownerd;
      }
    }
    else
    {
      if ( byte_82B50 )
      {
        sub_17910((long long)file, buf.st_dev, buf.st_ino);
        v12 = a7;
        a6 = owner;
      }
      groupa = v12;
      ownerc = a6;
      sub_17910((long long)file, buf.st_nlink, *(__syscall_slong_t *)&buf.st_mode);
      a6 = ownerc;
      v12 = groupa;
    }
    if ( dword_82AA8 > 0 )
      goto LABEL_17;
  }
  if ( dword_82AA4 <= 0 || (v13 = 4095, v12) )
    v13 = 511;
LABEL_3:
  v14 = *(int *)(a2 + 112) & ~dword_82328;
  if ( (v13 & (v27 | a4 ^ v14)) != 0 )
  {
    if ( (~((unsigned short)v13 | (unsigned short)v11) & 0xFFF) == 0 )
      goto LABEL_5;
    ownerb = v13;
    v28 = *(int *)(a2 + 112) & ~dword_82328;
    if ( a3 < 0 )
    {
      v22 = fstatat(oldfd, file, &buf, a8);
      v13 = ownerb;
      v14 = v28;
    }
    else
    {
      v22 = fstat(a3, &buf);
      v14 = v28;
      v13 = ownerb;
    }
    LOWORD(a4) = buf.st_mode;
    if ( v22 )
    {
      sub_33410(file);
    }
    else
    {
LABEL_5:
      v15 = a4 & 0xFFF;
      v16 = v13 & (v15 ^ v14);
      if ( v16 )
      {
        v23 = v16 ^ v15;
        if ( (unsigned int)sub_18100(a3, (long long)file, v23, a8, v9) )
          sub_32C30(file, v23);
      }
    }
  }
  sub_32570(a2, file, v9, 1);
  sub_31F50(a2, file, v9);
  return sub_324C0(a2, file, v9);
}



// Function: apply_delayed_links @ 0x185e0
unsigned long long apply_delayed_links()
{
  unsigned int *v0; // rbp
  const char *v1; // r14
  const char *v2; // rbx
  int v3; // edi
  int v4; // eax
  long long *v5; // rbx
  void *v6; // rdi
  unsigned int *v7; // rbx
  __m128i v9; // xmm0
  __m128i v10; // xmm1
  unsigned int v11; // eax
  long long v12; // rax
  struct stat v13; // [rsp+0h] [rbp-288h] BYREF
  char v14[48]; // [rsp+90h] [rbp-1F8h] BYREF
  long long v15; // [rsp+C0h] [rbp-1C8h]
  long long v16; // [rsp+C8h] [rbp-1C0h]
  long long v17; // [rsp+D0h] [rbp-1B8h]
  long long v18; // [rsp+D8h] [rbp-1B0h]
  long long v19; // [rsp+E0h] [rbp-1A8h]
  unsigned int v20; // [rsp+100h] [rbp-188h]
  unsigned int v21; // [rsp+104h] [rbp-184h]
  unsigned int v22; // [rsp+108h] [rbp-180h]
  __m128i v23; // [rsp+178h] [rbp-110h]
  __m128i v24; // [rsp+188h] [rbp-100h]
  long long v25; // [rsp+1E8h] [rbp-A0h]
  long long v26; // [rsp+1F0h] [rbp-98h]
  unsigned long long v27; // [rsp+248h] [rbp-40h]

  v0 = (unsigned int *)qword_82318;
  v27 = __readfsqword(0x28u);
  if ( !qword_82318 )
    goto LABEL_19;
  while ( 2 )
  {
    v1 = 0;
    sub_26300(v0[22]);
    v2 = (const char *)*((long long *)v0 + 12);
    if ( !v2 )
      goto LABEL_17;
    do
    {
      while ( 1 )
      {
        if ( fstatat(oldfd, v2 + 8, &v13, 256)
          || v13.st_dev != *((long long *)v0 + 1)
          || v13.st_ino != *((long long *)v0 + 2)
          || (*((long long *)v0 + 4) < -1LL)
           - (*((long long *)v0 + 4) >= 0LL)
           + 2 * ((*((long long *)v0 + 3) < -1LL) - (*((long long *)v0 + 3) >= 0LL)) )
        {
          goto LABEL_4;
        }
        if ( unlinkat(oldfd, v2 + 8, 0) )
        {
          sub_33490(v2 + 8);
          goto LABEL_4;
        }
        v3 = oldfd;
        if ( v1 )
        {
          v4 = linkat(oldfd, v1, oldfd, v2 + 8, 0);
          v3 = oldfd;
          if ( !v4 )
            goto LABEL_4;
        }
        if ( *((char *)v0 + 40) )
        {
          if ( symlinkat((const char *)v0 + 160, v3, v2 + 8) )
          {
            sub_33370(v0 + 40, v2 + 8);
          }
          else
          {
            v9 = _mm_loadu_si128((const __m128i *)(v0 + 14));
            v10 = _mm_loadu_si128((const __m128i *)(v0 + 18));
            v20 = v0[11];
            v1 = v2 + 8;
            v21 = v0[12];
            v11 = v0[13];
            v23 = v9;
            v22 = v11;
            v12 = *((long long *)v0 + 13);
            v24 = v10;
            v15 = v12;
            v16 = *((long long *)v0 + 14);
            v17 = *((long long *)v0 + 15);
            v18 = *((long long *)v0 + 16);
            v19 = *((long long *)v0 + 17);
            v26 = *((long long *)v0 + 19);
            v25 = *((long long *)v0 + 18);
            sub_181B0((char *)v2 + 8, (long long)v14, -1, 0, 0, 50, 0, 0x100u);
          }
          goto LABEL_4;
        }
        if ( linkat(v3, (const char *)v0 + 160, v3, v2 + 8, 0) )
          break;
LABEL_4:
        v2 = *(const char **)v2;
        if ( !v2 )
          goto LABEL_15;
      }
      sub_32DC0(v0 + 40, v2 + 8);
      v2 = *(const char **)v2;
    }
    while ( v2 );
LABEL_15:
    v5 = (long long *)*((long long *)v0 + 12);
    while ( v5 )
    {
      v6 = v5;
      v5 = (long long *)*v5;
      free(v6);
    }
LABEL_17:
    sub_1D580(*((void **)v0 + 19));
    free(*((void **)v0 + 13));
    v7 = *(unsigned int **)v0;
    free(v0);
    if ( v7 )
    {
      v0 = v7;
      continue;
    }
    break;
  }
LABEL_19:
  qword_82318 = 0;
  return v27 - __readfsqword(0x28u);
}



// Function: maybe_recoverable @ 0x18980
long long maybe_recoverable(char *src, char a2, char *a3)
{
  long long result; // rax
  int *v5; // rax
  int v6; // ebp
  int *v7; // r12
  struct stat *v8; // rsi
  char *v9; // rax
  struct stat v10; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v11; // [rsp+98h] [rbp-30h]

  v11 = __readfsqword(0x28u);
  result = 0;
  if ( *a3 )
    return result;
  v5 = __errno_location();
  v6 = *v5;
  v7 = v5;
  if ( *v5 == 17 )
  {
    v8 = 0;
  }
  else
  {
    if ( v6 <= 17 )
    {
      if ( v6 != 2 )
      {
LABEL_8:
        *v7 = v6;
        return 0;
      }
      goto LABEL_11;
    }
    if ( v6 != 40 && (v6 & 0xFFFFFFBF) != 0x1F || !a2 || dword_82B34 != 2 || byte_82B69 )
      goto LABEL_8;
    if ( !strchr(src, 47) )
      goto LABEL_22;
    v8 = &v10;
    if ( (unsigned int)sub_25FC0(src, &v10) )
      goto LABEL_8;
  }
  if ( dword_82B34 == 5 )
  {
    result = 2;
    if ( (dword_81B80 & 0x100000) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(src);
      v9 = dcgettext(0, "%s: skipping existing file", 5);
      error(0, 0, v9, src);
      return 2;
    }
    return result;
  }
  if ( (unsigned int)dword_82B34 > 5 )
  {
    if ( dword_82B34 != 6 || (unsigned char)sub_188C0(src, v8) )
    {
LABEL_11:
      if ( (unsigned int)sub_17EC0(src, a3) || !*a3 )
        goto LABEL_8;
      return 1;
    }
LABEL_22:
    if ( (int)sub_26A40(src, 0) > 0 )
      return 1;
    goto LABEL_11;
  }
  if ( (unsigned int)dword_82B34 <= 2 )
    goto LABEL_22;
  result = 0;
  if ( dword_82B34 != 4 )
    goto LABEL_11;
  return result;
}



// Function: extract_fifo @ 0x18b30
long long extract_fifo(char *file, char a2)
{
  short v2; // bx
  __mode_t v3; // ebx
  int v4; // eax
  char v6; // [rsp+17h] [rbp-31h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-30h]

  v2 = dword_828B0;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( dword_82AA8 > 0 )
    v2 = dword_828B0 & 0xFFC0;
  v3 = v2 & 0x1FF;
  while ( mkfifoat(oldfd, file, v3) )
  {
    v4 = sub_18980(file, 0, &v6);
    if ( !v4 )
    {
      sub_32E60(file);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
  }
  sub_181B0(file, (long long)&qword_82840, -1, v3 & ~dword_82328, 511, a2, 0, 0x100u);
  return 0;
}



// Function: extract_node @ 0x18c10
long long extract_node(char *file, char a2)
{
  short v2; // bx
  __mode_t v3; // ebx
  int v4; // eax
  char v6; // [rsp+17h] [rbp-31h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-30h]

  v2 = dword_828B0;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( dword_82AA8 > 0 )
    v2 = dword_828B0 & 0xFFC0;
  v3 = v2 & 0x61FF;
  while ( mknodat(oldfd, file, v3, dev) )
  {
    v4 = sub_18980(file, 0, &v6);
    if ( !v4 )
    {
      sub_32E80(file);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
  }
  sub_181B0(file, (long long)&qword_82840, -1, v3 & ~dword_82328, 511, a2, 0, 0x100u);
  return 0;
}



// Function: create_placeholder_file @ 0x18d00
long long create_placeholder_file(char *src, char a2, char *a3, void **a4)
{
  int v6; // eax
  int v7; // r14d
  int v8; // eax
  unsigned int v9; // r13d
  size_t v11; // rax
  long long *v12; // rbx
  void *v13; // rax
  __dev_t st_dev; // rax
  __ino_t st_ino; // rax
  __m128i v16; // xmm0
  __m128i v17; // xmm1
  long long v18; // rax
  size_t v19; // rax
  long long v20; // rax
  long long v21; // rsi
  void *v22; // r12
  size_t v23; // rbx
  struct stat v24; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v25; // [rsp+98h] [rbp-30h]

  v25 = __readfsqword(0x28u);
  while ( 1 )
  {
    v6 = openat(oldfd, src, 193, 0);
    v7 = v6;
    if ( v6 >= 0 )
      break;
    if ( *__errno_location() == 17 && sub_17850(src) )
      return 0;
    v8 = sub_18980(src, 0, a3);
    if ( !v8 )
    {
      v9 = -1;
      sub_32EA0(src);
      return v9;
    }
    if ( v8 == 2 )
      return 0;
  }
  if ( fstat(v6, &v24) )
  {
    v9 = -1;
    sub_33410(src);
    close(v7);
  }
  else if ( close(v7) )
  {
    v9 = -1;
    sub_32D60(src);
  }
  else
  {
    v11 = strlen(from);
    v12 = (long long *)sub_4ECC0(v11 + 161);
    if ( a4 )
    {
      v13 = *a4;
      *a4 = v12;
    }
    else
    {
      v13 = qword_82318;
      qword_82318 = v12;
    }
    *v12 = v13;
    st_dev = v24.st_dev;
    v12[3] = -1;
    v12[4] = -1;
    v12[1] = st_dev;
    st_ino = v24.st_ino;
    *((char *)v12 + 40) = a2;
    v12[2] = st_ino;
    if ( a2 )
    {
      v16 = _mm_loadu_si128((const __m128i *)&xmmword_82928);
      v17 = _mm_loadu_si128((const __m128i *)&xmmword_82938);
      *((int *)v12 + 11) = dword_828B0;
      v18 = qword_828B4;
      *(__m128i *)(v12 + 7) = v16;
      v12[6] = v18;
      *(__m128i *)(v12 + 9) = v17;
    }
    *((int *)v12 + 22) = dword_82580;
    v19 = strlen(src);
    v20 = sub_4ECC0(v19 + 9);
    v12[12] = v20;
    *(long long *)v20 = 0;
    strcpy((char *)(v20 + 8), src);
    v21 = qword_82870;
    v12[13] = 0;
    sub_25120(v12 + 13, v21);
    v12[14] = 0;
    v12[15] = 0;
    v12[16] = 0;
    v12[17] = 0;
    sub_1D820(&qword_82840, v12 + 19, v12 + 18);
    strcpy((char *)v12 + 160, from);
    v22 = qword_82320;
    if ( !qword_82320 )
      return 0;
    while ( 1 )
    {
      if ( !*((char *)v22 + 88) )
      {
        v23 = *((long long *)v22 + 19);
        v9 = strncmp(src, *((const char **)v22 + 20), v23);
        if ( !v9 && src[v23] == 47 && (char *)sub_400D0(src) == &src[v23 + 1] )
          break;
      }
      v22 = *(void **)v22;
      if ( !v22 )
        return 0;
    }
    sub_17AE0((long long *)v22);
  }
  return v9;
}



// Function: extract_link @ 0x18fd0
long long extract_link(char *src)
{
  char *v1; // r12
  void **v2; // rax
  int *v3; // rbx
  int v4; // eax
  int v5; // eax
  int v6; // r14d
  unsigned int v7; // r8d
  int *v9; // rbx
  size_t v10; // rax
  long long v11; // r12
  char v12; // [rsp+Fh] [rbp-159h] BYREF
  struct stat buf; // [rsp+10h] [rbp-158h] BYREF
  struct stat v14; // [rsp+A0h] [rbp-C8h] BYREF
  unsigned long long v15; // [rsp+138h] [rbp-30h]

  v1 = from;
  v15 = __readfsqword(0x28u);
  v12 = 0;
  if ( !byte_82B92 && (unsigned char)sub_29420(from) )
  {
    return (unsigned int)sub_18D00(src, 0, &v12, 0);
  }
  else
  {
    v2 = (void **)sub_17850(v1);
    if ( v2 )
    {
      return (unsigned int)sub_18D00(src, 0, &v12, v2);
    }
    else
    {
      v3 = __errno_location();
      do
      {
        v5 = linkat(oldfd, v1, oldfd, src, 0);
        v6 = *v3;
        if ( !v5 )
        {
          v9 = qword_82318;
          if ( qword_82318 && !fstatat(oldfd, v1, &buf, 256) )
          {
            while ( v9[22] != dword_82580
                 || *((long long *)v9 + 1) != buf.st_dev
                 || *((long long *)v9 + 2) != buf.st_ino
                 || (*((long long *)v9 + 4) >= 0LL)
                  - (*((long long *)v9 + 4) < -1LL)
                  + 2 * ((*((long long *)v9 + 3) >= 0LL) - (*((long long *)v9 + 3) < -1LL)) )
            {
              v9 = *(int **)v9;
              if ( !v9 )
                return 0;
            }
            v10 = strlen(src);
            v11 = sub_4ECC0(v10 + 9);
            strcpy((char *)(v11 + 8), src);
            *(long long *)v11 = *((long long *)v9 + 12);
            *((long long *)v9 + 12) = v11;
          }
          return 0;
        }
        if ( v6 == 17 && !strcmp(v1, src)
          || !fstatat(oldfd, v1, &buf, 256)
          && !fstatat(oldfd, src, &v14, 256)
          && buf.st_dev == v14.st_dev
          && buf.st_ino == v14.st_ino )
        {
          return 0;
        }
        *v3 = v6;
        v4 = sub_18980(src, 0, &v12);
      }
      while ( v4 == 1 );
      v7 = 0;
      if ( v4 != 2 && (!byte_82B50 || *v3 != 17) )
      {
        sub_32DC0(v1, src);
        return 1;
      }
    }
  }
  return v7;
}



// Function: extract_symlink @ 0x19270
long long extract_symlink(char *file)
{
  const char *v2; // rdi
  char v3; // al
  int v4; // eax
  char v6; // [rsp+17h] [rbp-21h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-20h]

  v2 = from;
  v7 = __readfsqword(0x28u);
  v6 = 0;
  if ( !byte_82B92 )
  {
    if ( *from == 47 )
      return sub_18D00(file, 1, &v6, 0);
    v3 = sub_29420(from);
    v2 = from;
    if ( v3 )
      return sub_18D00(file, 1, &v6, 0);
  }
  while ( symlinkat(v2, oldfd, file) )
  {
    v4 = sub_18980(file, 0, &v6);
    if ( !v4 )
    {
      sub_33370(from, file);
      return 0xFFFFFFFFLL;
    }
    if ( v4 == 2 )
      return 0;
    v2 = from;
  }
  sub_181B0(file, (long long)&qword_82840, -1, 0, 0, 50, 0, 0x100u);
  return 0;
}



// Function: extract_file @ 0x19380
long long extract_file(char *file, struct stat *p_buf)
{
  unsigned long long v3; // rdi
  unsigned int v4; // ebp
  int v5; // r13d
  long long v6; // rdi
  long long v7; // rdx
  long long v8; // rcx
  long long v9; // r8
  long long v10; // rax
  struct stat *v11; // rbx
  unsigned long long v12; // r12
  long long v13; // rax
  struct stat *v14; // rbp
  int v15; // r12d
  __mode_t v17; // r12d
  int v18; // eax
  int v19; // r12d
  int v20; // ebx
  void *v21; // r13
  int v22; // eax
  int v23; // eax
  int v24; // eax
  char *v25; // rax
  char *v26; // rax
  int *v27; // rbx
  int v28; // [rsp+4h] [rbp-F4h]
  int v29; // [rsp+4h] [rbp-F4h]
  int st_mode; // [rsp+8h] [rbp-F0h]
  int v31; // [rsp+8h] [rbp-F0h]
  int v32; // [rsp+Ch] [rbp-ECh]
  char v33; // [rsp+17h] [rbp-E1h] BYREF
  unsigned long long v34; // [rsp+18h] [rbp-E0h] BYREF
  struct stat buf; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v36; // [rsp+B8h] [rbp-40h]

  v3 = (unsigned int)dword_82AA8;
  v28 = (int)p_buf;
  v36 = __readfsqword(0x28u);
  v33 = 0;
  v4 = dword_828B0 & 0x1C0;
  if ( dword_82AA8 <= 0 )
    v4 = dword_828B0 & 0x1FF;
  if ( byte_82A6A )
  {
    v32 = 0;
    v5 = 1;
    st_mode = 0;
    goto LABEL_6;
  }
  if ( qword_82A60 )
  {
    p_buf = (struct stat *)(&qword_60 + 6);
    v32 = 0;
    st_mode = 0;
    v5 = sub_2C8B0(file);
    if ( v5 < 0 )
    {
      sub_24660(file, 102);
      return 0;
    }
LABEL_6:
    sub_C670((void **)&qword_82840);
    if ( byte_82960 )
    {
      p_buf = (struct stat *)&qword_82840;
      sub_2B460((unsigned int)v5, &qword_82840, &v34);
      v6 = v34;
    }
    else
    {
      v6 = qword_828C8;
      v34 = qword_828C8;
      while ( v6 > 0 )
      {
        sub_C710(v6);
        v10 = sub_D360(v6, p_buf, v7, v8, v9);
        v11 = (struct stat *)v10;
        if ( !v10 )
        {
          if ( qword_82D50 )
            qword_82D50(v6);
          v26 = dcgettext(0, "Unexpected EOF in archive", 5);
          p_buf = 0;
          error(0, 0, v26);
          v6 = v34;
          dword_82D58 = 2;
          break;
        }
        v12 = sub_CD10(v10);
        if ( v12 > v34 )
          v12 = v34;
        p_buf = v11;
        *__errno_location() = 0;
        v13 = sub_26070(v5);
        v34 -= v12;
        v14 = (struct stat *)v13;
        sub_CCD0((char *)v11 + v12 - 1);
        if ( (struct stat *)v12 != v14 )
        {
          if ( !qword_82A60 )
          {
            p_buf = v14;
            sub_33510(file, v14, v12);
          }
          v6 = v34;
          break;
        }
        v6 = v34;
      }
    }
    sub_23D40(v6, p_buf);
    sub_C6A0();
    if ( byte_82A6A )
      return 0;
    if ( qword_82A60 )
    {
      v15 = close(v5);
      if ( v15 >= 0 )
        goto LABEL_17;
    }
    else
    {
      sub_181B0(file, (long long)&qword_82840, v5, st_mode, v32, v28, 0, (dword_82B34 != 2) << 8);
      v15 = close(v5);
      if ( v15 >= 0 )
        goto LABEL_17;
    }
    sub_32D60(file);
LABEL_17:
    if ( qword_82A60 )
      sub_2CE00();
    return (unsigned int)v15;
  }
  LOBYTE(v34) = 0;
  if ( dword_82A98 <= 0 || !qword_82998 )
  {
LABEL_26:
    v19 = 0;
    goto LABEL_27;
  }
  v17 = (unsigned short)dword_828B0 & (unsigned short)~(short)dword_82328 & 0x1FF;
  while ( mknodat(oldfd, file, v17, 0) )
  {
    v3 = (unsigned long long)file;
    v18 = sub_18980(file, 0, &v34);
    if ( !v18 )
    {
      ((void (*)(void))sub_24660)();
      v15 = 1;
      sub_32EA0(file);
      return (unsigned int)v15;
    }
    if ( v18 == 2 )
      goto LABEL_26;
  }
  v19 = 1;
  v3 = (unsigned long long)&qword_82840;
  sub_32570(&qword_82840, file, (unsigned int)(char)p_buf, 0);
  do
  {
LABEL_27:
    v20 = dword_82B34;
    v21 = &unk_809C1;
    if ( dword_82B34 == 2 )
      LODWORD(v21) = byte_82B69 == 0 ? 658241 : 527169;
    v22 = (int)v21;
    if ( v19 )
    {
      LOBYTE(v22) = (unsigned char)v21 & 0x7F;
      LODWORD(v21) = v22;
    }
    if ( v28 == 55 && !dword_82310 )
    {
      dword_82310 = 1;
      if ( (dword_81B80 & 8) != 0 )
      {
        if ( qword_82D50 )
          qword_82D50(v3);
        v25 = dcgettext(0, "Extracting contiguous files as regular files", 5);
        error(0, 0, v25);
      }
    }
    p_buf = (struct stat *)file;
    v23 = openat(oldfd, file, (int)v21, v4);
    v5 = v23;
    if ( v23 >= 0 )
    {
      if ( v20 != 2 )
      {
        v32 = 511;
        st_mode = v4 & ~dword_82328;
        goto LABEL_6;
      }
      p_buf = &buf;
      if ( fstat(v23, &buf) )
      {
        v27 = __errno_location();
        v31 = *v27;
        close(v5);
        *v27 = v31;
      }
      else
      {
        st_mode = buf.st_mode;
        if ( (buf.st_mode & 0xF000) == 0x8000 )
        {
          v32 = -1;
          goto LABEL_6;
        }
        close(v5);
        *__errno_location() = 17;
      }
    }
    v3 = (unsigned long long)file;
    v24 = sub_18980(file, 1, &v33);
  }
  while ( v24 == 1 );
  v29 = v24;
  ((void (*)(void))sub_24660)();
  if ( v29 == 2 )
    return 0;
  v15 = 1;
  sub_32EA0(file);
  return (unsigned int)v15;
}



// Function: extract_dir @ 0x198d0
long long extract_dir(unsigned long long src, char *a2)
{
  char *v2; // rbp
  int v3; // ebx
  int v4; // eax
  __mode_t v5; // r13d
  int v6; // r15d
  int st_mode; // r14d
  unsigned int v8; // r12d
  int *v9; // rbx
  char *v10; // rdi
  int v11; // eax
  int v12; // eax
  int v13; // r9d
  int v15; // eax
  struct stat *v16; // rsi
  long long v17; // rcx
  long long *v18; // rdi
  const char *v19; // rbx
  struct stat *p_buf; // rdx
  int v21; // eax
  __m128i v22; // xmm0
  __m128i v23; // xmm1
  int v24; // eax
  __mode_t v25; // eax
  long long v26; // r12
  char *v27; // rax
  struct stat *v28; // [rsp+8h] [rbp-190h]
  char v29; // [rsp+17h] [rbp-181h]
  char v30; // [rsp+2Fh] [rbp-169h] BYREF
  struct stat v31; // [rsp+30h] [rbp-168h] BYREF
  struct stat buf; // [rsp+C0h] [rbp-D8h] BYREF
  unsigned long long v33; // [rsp+158h] [rbp-40h]

  v2 = (char *)src;
  v3 = (int)a2;
  v33 = __readfsqword(0x28u);
  v30 = 0;
  if ( byte_82AD1 && !qword_827F0 )
  {
    src = (unsigned int)oldfd;
    a2 = ".";
    if ( fstatat(oldfd, ".", &buf, 0) )
    {
      src = (unsigned long long)".";
      sub_26460(".");
    }
    else
    {
      qword_827F0 = buf.st_dev;
    }
  }
  if ( byte_82B50 )
  {
    sub_21100(v2);
  }
  else if ( v3 == 68 )
  {
    sub_24660(src, a2);
  }
  v4 = 448;
  if ( dword_82AA8 <= 0 && dword_82AA4 <= 0 )
    v4 = 511;
  v5 = v4 & dword_828B0;
  if ( !byte_82330 )
    LOBYTE(v5) = v5 | 0xC0;
  v6 = 0;
  st_mode = 0;
  while ( 1 )
  {
    v8 = mkdirat(oldfd, v2, v5);
    if ( !v8 )
      goto LABEL_50;
    v9 = __errno_location();
    if ( *v9 != 17 )
      goto LABEL_20;
    v29 = v30;
    if ( v30 )
      break;
    if ( byte_82B30 )
      goto LABEL_28;
    if ( (unsigned int)dword_82B34 <= 2 )
      goto LABEL_16;
LABEL_20:
    v11 = sub_18980(v2, 0, &v30);
    if ( !v11 )
    {
      if ( *v9 != 17 )
      {
        v8 = 1;
        sub_32E40(v2);
        return v8;
      }
LABEL_22:
      v12 = dword_82B34;
LABEL_23:
      if ( (v12 & 0xFFFFFFFD) != 0 )
        return v8;
      v13 = 0;
LABEL_25:
      sub_17B80(v2, (long long)&qword_82840, st_mode, v6, dword_828B0, v13);
      return v8;
    }
    if ( v11 != 1 )
      goto LABEL_22;
  }
  if ( !byte_82B30 )
    goto LABEL_16;
LABEL_28:
  if ( !fstatat(oldfd, v2, &buf, 256)
    && (buf.st_mode & 0xF000) == 0xA000
    && !fstatat(oldfd, v2, &buf, 0)
    && (buf.st_mode & 0xF000) == 0x4000 )
  {
    *v9 = 17;
    return 0;
  }
  *v9 = 17;
LABEL_16:
  v10 = v2;
  if ( (unsigned int)sub_25FC0(v2, &v31) )
  {
LABEL_19:
    *v9 = 17;
    goto LABEL_20;
  }
  st_mode = v31.st_mode;
  if ( (v31.st_mode & 0xF000) != 0x4000 )
  {
    v6 = -1;
    goto LABEL_19;
  }
  if ( !v29 )
  {
    v12 = dword_82B34;
    if ( dword_82B34 == 1 )
    {
      v15 = 448;
      if ( dword_82AA8 <= 0 && dword_82AA4 <= 0 )
        v15 = 511;
      v5 = (byte_82330 == 0 ? 0xC0 : 0) | v31.st_mode & v15;
      v8 = sub_18100(0xFFFFFFFF, (long long)v2, v5, 256, 53);
      if ( !v8 )
      {
        v16 = &v31;
        v17 = 36;
        v18 = &qword_82898;
        while ( v17 )
        {
          *(int *)v18 = v16->st_dev;
          v16 = (struct stat *)((char *)v16 + 4);
          v18 = (long long *)((char *)v18 + 4);
          --v17;
        }
LABEL_50:
        v13 = 256;
        v6 = 511;
        st_mode = v5 & ~dword_82328;
        goto LABEL_25;
      }
      sub_32C30(v2, v5);
      v12 = dword_82B34;
    }
    v6 = -1;
    goto LABEL_23;
  }
  v19 = (const char *)qword_82320;
  if ( qword_82320 )
  {
    p_buf = &buf;
    while ( 1 )
    {
      v10 = (char *)(unsigned int)oldfd;
      v28 = p_buf;
      v21 = fstatat(oldfd, *((const char **)v19 + 20), p_buf, *((int *)v19 + 21));
      p_buf = v28;
      if ( v21 )
        break;
      if ( buf.st_dev == v31.st_dev && buf.st_ino == v31.st_ino )
      {
        v22 = _mm_loadu_si128((const __m128i *)&xmmword_82928);
        v23 = _mm_loadu_si128((const __m128i *)&xmmword_82938);
        *((long long *)v19 + 1) = qword_82898;
        *((long long *)v19 + 2) = qword_828A0;
        *((int *)v19 + 6) = dword_828B0;
        *((int *)v19 + 7) = qword_828B4;
        v24 = HIDWORD(qword_828B4);
        *(__m128i *)(v19 + 40) = v22;
        *((int *)v19 + 8) = v24;
        v25 = buf.st_mode;
        *(__m128i *)(v19 + 56) = v23;
        *((int *)v19 + 18) = v25;
        *((int *)v19 + 19) = -1;
        *((char *)v19 + 80) = 0;
        return 0;
      }
      v19 = *(const char **)v19;
      if ( !v19 )
        goto LABEL_61;
    }
    sub_33410(*((long long *)v19 + 20));
  }
  else
  {
LABEL_61:
    if ( qword_82D50 )
      qword_82D50(v10);
    v26 = sub_4CE40(v2);
    v27 = dcgettext(0, "%s: Unexpected inconsistency when making directory", 5);
    error(0, 0, v27, v26);
    dword_82D58 = 2;
  }
  return 0;
}



// Function: apply_nonancestor_delayed_set_stat @ 0x19da0
unsigned long long apply_nonancestor_delayed_set_stat(const char *s1, unsigned char a2)
{
  size_t v3; // rax
  unsigned int *v4; // r15
  char v5; // bl
  __m128i v6; // xmm0
  __m128i v7; // xmm1
  char *v8; // rdi
  unsigned int v9; // eax
  long long v10; // rax
  void *v11; // rdi
  unsigned char v12; // al
  __mode_t st_mode; // r12d
  int v14; // r13d
  size_t v15; // rdx
  char v16; // al
  long long v17; // rdi
  long long v18; // r12
  char *v19; // rax
  size_t v21; // [rsp+8h] [rbp-290h]
  struct stat buf; // [rsp+10h] [rbp-288h] BYREF
  char v23[48]; // [rsp+A0h] [rbp-1F8h] BYREF
  long long v24; // [rsp+D0h] [rbp-1C8h]
  long long v25; // [rsp+D8h] [rbp-1C0h]
  long long v26; // [rsp+E0h] [rbp-1B8h]
  long long v27; // [rsp+E8h] [rbp-1B0h]
  long long v28; // [rsp+F0h] [rbp-1A8h]
  unsigned int v29; // [rsp+110h] [rbp-188h]
  unsigned int v30; // [rsp+114h] [rbp-184h]
  unsigned int v31; // [rsp+118h] [rbp-180h]
  __m128i v32; // [rsp+188h] [rbp-110h]
  __m128i v33; // [rsp+198h] [rbp-100h]
  long long v34; // [rsp+1F8h] [rbp-A0h]
  long long v35; // [rsp+200h] [rbp-98h]
  unsigned long long v36; // [rsp+258h] [rbp-40h]

  v36 = __readfsqword(0x28u);
  v3 = strlen(s1);
  v4 = (unsigned int *)qword_82320;
  v21 = v3;
  if ( qword_82320 )
  {
    v5 = 0;
    do
    {
      v12 = *((char *)v4 + 88);
      st_mode = v4[18];
      v14 = v4[19];
      v5 |= v12;
      if ( v12 > a2 )
        return v36 - __readfsqword(0x28u);
      v15 = *((long long *)v4 + 19);
      if ( v15 < v21 )
      {
        v16 = s1[v15];
        if ( v16 )
        {
          if ( (v16 == 47 || s1[v15 - 1] == 47) && !memcmp(s1, *((const void **)v4 + 20), v15) )
            return v36 - __readfsqword(0x28u);
        }
      }
      sub_26300(v4[23]);
      if ( v5 )
      {
        v17 = (unsigned int)oldfd;
        if ( fstatat(oldfd, *((const char **)v4 + 20), &buf, v4[21]) )
        {
          sub_33410(*((long long *)v4 + 20));
          goto LABEL_4;
        }
        st_mode = buf.st_mode;
        if ( buf.st_dev != *((long long *)v4 + 1) || buf.st_ino != *((long long *)v4 + 2) )
        {
          if ( qword_82D50 )
            qword_82D50(v17);
          v18 = sub_4CE40(*((long long *)v4 + 20));
          v19 = dcgettext(0, "%s: Directory renamed before its status could be extracted", 5);
          error(0, 0, v19, v18);
          dword_82D58 = 2;
          goto LABEL_4;
        }
        v14 = -1;
      }
      v6 = _mm_loadu_si128((const __m128i *)(v4 + 10));
      v7 = _mm_loadu_si128((const __m128i *)(v4 + 14));
      v8 = (char *)*((long long *)v4 + 20);
      v29 = v4[6];
      v30 = v4[7];
      v9 = v4[8];
      v32 = v6;
      v31 = v9;
      v10 = *((long long *)v4 + 12);
      v33 = v7;
      v24 = v10;
      v25 = *((long long *)v4 + 13);
      v26 = *((long long *)v4 + 14);
      v27 = *((long long *)v4 + 15);
      v28 = *((long long *)v4 + 16);
      v35 = *((long long *)v4 + 18);
      v34 = *((long long *)v4 + 17);
      sub_181B0(v8, (long long)v23, -1, st_mode, v14, 53, *((char *)v4 + 80), v4[21]);
LABEL_4:
      v11 = (void *)*((long long *)v4 + 20);
      qword_82320 = *(void **)v4;
      free(v11);
      sub_1D580(*((void **)v4 + 18));
      free(*((void **)v4 + 12));
      free(*((void **)v4 + 13));
      free(*((void **)v4 + 15));
      free(v4);
      v4 = (unsigned int *)qword_82320;
    }
    while ( qword_82320 );
  }
  return v36 - __readfsqword(0x28u);
}



// Function: extract_finish @ 0x1a080
unsigned long long extract_finish()
{
  sub_19DA0("", 0);
  sub_185E0();
  return sub_19DA0("", 1u);
}



// Function: extr_init @ 0x1a0b0
int extr_init()
{
  int result; // eax
  int v1; // ebx

  byte_82330 = geteuid() == 0;
  dword_82AA4 += (unsigned char)byte_82330;
  dword_82AA8 += (unsigned char)byte_82330;
  result = umask(0);
  dword_8232C = result;
  if ( dword_82AA4 <= 0 )
  {
    v1 = result;
    result = umask(result);
    dword_82328 = v1;
  }
  else
  {
    dword_82328 = 0;
  }
  return result;
}



// Function: remove_delayed_set_stat @ 0x1a110
void remove_delayed_set_stat(char *s2)
{
  void *v1; // rbp
  int v2; // r13d
  long long *v3; // r12
  void *v4; // rbx
  char *v5; // r14

  v1 = qword_82320;
  if ( qword_82320 )
  {
    v2 = dword_82580;
    v3 = 0;
    while ( 1 )
    {
      v4 = *(void **)v1;
      if ( *((int *)v1 + 23) == v2 )
      {
        v5 = (char *)*((long long *)v1 + 20);
        if ( !strcmp(v5, s2) )
          break;
      }
      v3 = v1;
      if ( !v4 )
        return;
      v1 = v4;
    }
    free(v5);
    sub_1D580(*((void **)v1 + 18));
    free(*((void **)v1 + 12));
    free(*((void **)v1 + 13));
    free(*((void **)v1 + 15));
    free(v1);
    if ( v3 )
      *v3 = v4;
    else
      qword_82320 = v4;
  }
}



// Function: extract_archive @ 0x1a1e0
long long extract_archive()
{
  const char *v0; // rdi
  long long v1; // rsi
  char *v2; // rdi
  char *v3; // r12
  unsigned int v4; // ebp
  long long (*v5)(char *, struct stat *); // rbx
  char *v6; // rdi
  long long v7; // r12
  char *v8; // rax
  long long result; // rax
  long long v10; // r12
  char *v11; // rax
  unsigned int v12; // ebp
  unsigned int v13; // ebp
  long long v14; // r12
  char *v15; // rax
  long long v16; // r13
  char *v17; // rax
  char *v18; // rax
  long long v19; // r12
  char *v20; // rax

  v0 = (const char *)qword_82408;
  qword_822F0 = (long long (*)(void))sub_1A080;
  sub_CCD0(qword_82408);
  if ( !byte_82B92 )
  {
    v0 = (const char *)qword_82840;
    if ( (unsigned char)sub_29420(qword_82840) )
    {
      if ( qword_82D50 )
        qword_82D50(v0);
      v10 = sub_4CE40(qword_82840);
      v11 = dcgettext(0, "%s: Member name contains '..'", 5);
      v1 = 0;
      v0 = 0;
      error(0, 0, v11, v10);
      dword_82D58 = 2;
      return sub_24660(v0, v1);
    }
  }
  v1 = (long long)file;
  if ( !*file )
    return sub_24660(v0, v1);
  if ( !byte_82B40 )
  {
    if ( !dword_82A54 )
      goto LABEL_5;
LABEL_31:
    v1 = qword_82408;
    sub_23BC0(&qword_82840, qword_82408, -1);
    if ( byte_827E0 )
      goto LABEL_6;
LABEL_32:
    v12 = dword_82580;
    v1 = 0;
    sub_19DA0(file, 0);
    sub_26300(v12);
    goto LABEL_6;
  }
  v0 = "extract";
  if ( !(unsigned int)sub_2DF50("extract", file) )
    return sub_24660(v0, v1);
  if ( dword_82A54 )
    goto LABEL_31;
LABEL_5:
  if ( !byte_827E0 )
    goto LABEL_32;
LABEL_6:
  if ( byte_82B84 )
  {
    v2 = file;
    v1 = 0;
    if ( !(unsigned char)sub_25C10(file) )
    {
      v13 = *__errno_location();
      if ( qword_82D50 )
        qword_82D50(v2);
      v14 = sub_4CE40(file);
      v15 = dcgettext(0, "%s: Was unable to backup this file", 5);
      v1 = v13;
      v0 = 0;
      error(0, v13, v15, v14);
      dword_82D58 = 2;
      return sub_24660(v0, v1);
    }
  }
  if ( (unsigned char)sub_2B3A0(&qword_82840) )
  {
    v3 = file;
    v4 = 83;
LABEL_10:
    if ( byte_82A6A || qword_82A60 )
    {
      v5 = sub_19380;
      goto LABEL_37;
    }
    v5 = sub_19380;
LABEL_13:
    if ( dword_82B34 != 3 )
    {
      if ( dword_82B34 == 6 )
      {
        v1 = 0;
        v6 = v3;
        if ( (unsigned char)sub_188C0(v3, 0) )
        {
          if ( (dword_81B80 & 0x800) != 0 )
          {
            if ( qword_82D50 )
              qword_82D50(v3);
            v7 = sub_4D2D0(v3);
            v8 = dcgettext(0, "Current %s is newer or same age", 5);
            v1 = 0;
            v6 = 0;
            error(0, 0, v8, v7);
          }
          goto LABEL_20;
        }
      }
      goto LABEL_37;
    }
    v1 = (unsigned char)byte_82ABA;
    if ( !(unsigned int)sub_26A40(v3, (unsigned char)byte_82ABA) && (*__errno_location() & 0xFFFFFFFD) != 0 )
    {
      v6 = v3;
      sub_33490(v3);
      goto LABEL_20;
    }
LABEL_37:
    v3 = file;
    goto LABEL_38;
  }
  v3 = file;
  v4 = *(char *)(qword_82408 + 156);
  v6 = file;
  switch ( *(char *)(qword_82408 + 156) )
  {
    case 0:
    case 0x30:
    case 0x37:
      if ( !byte_82850 )
        goto LABEL_10;
      v5 = (long long (*)(char *, struct stat *))sub_198D0;
      break;
    case 0x31:
      v4 = 49;
      v5 = (long long (*)(char *, struct stat *))sub_18FD0;
      break;
    case 0x32:
      v4 = 50;
      v5 = (long long (*)(char *, struct stat *))sub_19270;
      break;
    case 0x33:
      dword_828B0 |= 0x2000u;
      v4 = 51;
      v5 = (long long (*)(char *, struct stat *))sub_18C10;
      break;
    case 0x34:
      dword_828B0 |= 0x6000u;
      v4 = 52;
      v5 = (long long (*)(char *, struct stat *))sub_18C10;
      break;
    case 0x35:
    case 0x44:
      v5 = (long long (*)(char *, struct stat *))sub_198D0;
      if ( byte_829C8 )
        byte_827E0 = 1;
      break;
    case 0x36:
      v4 = 54;
      v5 = (long long (*)(char *, struct stat *))sub_18B30;
      break;
    case 0x4B:
    case 0x4C:
      if ( qword_82D50 )
        qword_82D50(file);
      v18 = dcgettext(0, "Unexpected long name header", 5);
      v1 = 0;
      v6 = 0;
      error(0, 0, v18);
      dword_82D58 = 2;
      goto LABEL_20;
    case 0x4D:
      if ( qword_82D50 )
      {
        qword_82D50(file);
        v6 = file;
      }
      v19 = sub_4CE40(v6);
      v20 = dcgettext(0, "%s: Cannot extract -- file is continued from another volume", 5);
      v1 = 0;
      v6 = 0;
      error(0, 0, v20, v19);
      dword_82D58 = 2;
      goto LABEL_20;
    case 0x53:
      v4 = 83;
      goto LABEL_10;
    case 0x56:
      goto LABEL_20;
    default:
      if ( (dword_81B80 & 0x10000) != 0 )
      {
        if ( qword_82D50 )
          qword_82D50(file);
        v16 = sub_4CE40(v3);
        v17 = dcgettext(0, "%s: Unknown file type '%c', extracted as normal file", 5);
        error(0, 0, v17, v16, v4);
      }
      goto LABEL_10;
  }
  if ( byte_82A6A )
    goto LABEL_20;
  if ( !qword_82A60 )
    goto LABEL_13;
  if ( v5 == sub_19380 )
  {
LABEL_38:
    result = v5(v3, (struct stat *)v4);
    if ( !(int)result )
      return result;
    goto LABEL_21;
  }
LABEL_20:
  result = sub_24660(v6, v1);
LABEL_21:
  if ( byte_82B84 )
    return sub_25EA0();
  return result;
}



// Function: rename_directory @ 0x1a700
long long rename_directory(char *s2, char *src)
{
  char *v6; // rdi
  int *v7; // rax
  int v8; // r13d
  int *v9; // rbx
  long long v10; // rbx
  long long v11; // r12
  char *v12; // rax
  long long result; // rax
  int v14; // r8d
  void *v15; // rbx
  int v16; // r13d
  char *v17; // r14
  char v18; // [rsp+7h] [rbp-31h] BYREF
  unsigned long long v19; // [rsp+8h] [rbp-30h]

  v6 = (char *)(unsigned int)oldfd;
  v19 = __readfsqword(0x28u);
  if ( renameat(oldfd, s2, oldfd, src) )
  {
    v7 = __errno_location();
    v8 = *v7;
    v9 = v7;
    if ( *v7 == 2 )
    {
      v6 = src;
      if ( !(unsigned int)sub_17EC0(src, &v18) )
      {
        v6 = (char *)(unsigned int)oldfd;
        v14 = renameat(oldfd, s2, oldfd, src);
        result = 1;
        if ( !v14 )
          return result;
        v8 = *v9;
      }
    }
    if ( qword_82D50 )
      qword_82D50(v6);
    v10 = sub_4D2B0(1, src);
    v11 = sub_4D2B0(0, s2);
    v12 = dcgettext(0, "Cannot rename %s to %s", 5);
    error(0, v8, v12, v11, v10);
    result = 0;
    dword_82D58 = 2;
  }
  else
  {
    v15 = qword_82320;
    if ( qword_82320 )
    {
      v16 = dword_82580;
      while ( 1 )
      {
        if ( *((int *)v15 + 23) == v16 )
        {
          v17 = (char *)*((long long *)v15 + 20);
          if ( !strcmp(v17, s2) )
            break;
        }
        v15 = *(void **)v15;
        if ( !v15 )
          return 1;
      }
      free(v17);
      *((long long *)v15 + 20) = sub_4EE30(src);
      *((long long *)v15 + 19) = strlen(src);
      return 1;
    }
    else
    {
      return 1;
    }
  }
  return result;
}



// Function: dummy_coder @ 0x1a8a0
void dummy_coder()
{
  ;
}



// Function: dummy_decoder @ 0x1a8b0
void dummy_decoder()
{
  ;
}



// Function: assign_time_option @ 0x1a8c0
unsigned long long assign_time_option(long long a1, long long *a2, char *a3)
{
  long long v6; // rax
  long long v7; // rdx
  char *v8; // rax
  char *v10; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-20h]

  v11 = __readfsqword(0x28u);
  v6 = sub_25930(a3, &v10);
  if ( v7 < 0 || *v10 )
  {
    if ( qword_82D50 )
      qword_82D50(a3);
    v8 = dcgettext(0, "Time stamp is out of allowed range", 5);
    error(0, 0, v8);
    dword_82D58 = 2;
  }
  else
  {
    *a2 = v6;
    sub_25120(a1, a3);
  }
  return v11 - __readfsqword(0x28u);
}



// Function: dumpdir_decoder @ 0x1a970
void *dumpdir_decoder(long long a1, long long a2, const void *a3, size_t a4)
{
  void *v6; // rax

  v6 = (void *)sub_4ECC0(a4);
  *(long long *)(a1 + 400) = v6;
  return memcpy(v6, a3, a4);
}



// Function: x_obstack_grow @ 0x1a9b0
void *x_obstack_grow(long long *a1, const void *a2, size_t a3)
{
  long long v5; // r12
  void *v6; // rdi
  void *result; // rax

  v5 = *a1;
  v6 = *(void **)(*a1 + 24LL);
  if ( a3 > *(long long *)(v5 + 32) - (long long)v6 )
  {
    obstack_newchunk(v5, a3);
    v6 = *(void **)(v5 + 24);
  }
  result = memcpy(v6, a2, a3);
  *(long long *)(v5 + 24) += a3;
  a1[1] += a3;
  return result;
}



// Function: xheader_xattr__add @ 0x1aa20
long long xheader_xattr__add(long long *a1, long long *a2, void *a3, void *a4, long long a5)
{
  long long v9; // rdi
  long long v10; // rax
  long long v11; // rbx
  long long v12; // rax
  long long v13; // r12
  long long result; // rax

  v9 = *a1;
  v10 = *a2 + 1;
  *a2 = v10;
  v11 = 24 * v10 - 24;
  v12 = sub_4ECE0(v9, 24 * v10, a3, a4);
  *a1 = v12;
  *(long long *)(v12 + v11) = sub_4EE30(a3);
  v13 = v11 + *a1;
  *(long long *)(v13 + 8) = sub_4EDF0(a4, a5 + 1);
  result = *a1;
  *(long long *)(*a1 + v11 + 16) = a5;
  return result;
}



// Function: xattr_acls_d_decoder @ 0x1aaa0
long long xattr_acls_d_decoder(long long a1, long long a2, void *a3, long long a4)
{
  long long result; // rax

  result = sub_4EDF0(a3, a4 + 1);
  *(long long *)(a1 + 80) = a4;
  *(long long *)(a1 + 72) = result;
  return result;
}



// Function: xattr_acls_a_decoder @ 0x1aad0
long long xattr_acls_a_decoder(long long a1, long long a2, void *a3, long long a4)
{
  long long result; // rax

  result = sub_4EDF0(a3, a4 + 1);
  *(long long *)(a1 + 64) = a4;
  *(long long *)(a1 + 56) = result;
  return result;
}



// Function: locate_handler @ 0x1ab00
char **locate_handler(char *s2)
{
  char **v1; // r12
  const char *v2; // rbp
  size_t v3; // rax

  v1 = &off_7D260;
  v2 = "atime";
  do
  {
    if ( *((char *)v1 + 28) )
    {
      v3 = strlen(v2);
      if ( !strncmp(v2, s2, v3) )
        return v1;
    }
    else if ( !strcmp(v2, s2) )
    {
      return v1;
    }
    v2 = v1[4];
    v1 += 4;
  }
  while ( v2 );
  return 0;
}



// Function: run_override_list @ 0x1ab80
void run_override_list(long long **a1, long long a2)
{
  long long **v2; // rbp
  char **v3; // rbx
  const char *v4; // r13
  size_t v5; // rax

  if ( a1 )
  {
    v2 = a1;
    do
    {
      v3 = sub_1AB00((char *)v2[1]);
      if ( v3 )
      {
        v4 = (const char *)v2[2];
        v5 = strlen(v4);
        ((void (*)(long long, char *, const char *, size_t))v3[2])(a2, *v3, v4, v5);
      }
      v2 = (long long **)*v2;
    }
    while ( v2 );
  }
}



// Function: out_of_range_header @ 0x1abf0
unsigned long long out_of_range_header(long long a1, long long a2, long long a3, long long a4)
{
  long long v6; // rbp
  long long v7; // r13
  char *v8; // rax
  long long v10; // [rsp+0h] [rbp-68h] BYREF
  char v11[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v12; // [rsp+38h] [rbp-30h]

  v12 = __readfsqword(0x28u);
  v6 = sub_45230(a3, &v10);
  v7 = sub_45370(a4, v11);
  if ( qword_82D50 )
    qword_82D50(a4);
  v8 = dcgettext(0, "Extended header %s=%s is out of range %s..%s", 5);
  error(0, 0, v8, a1, a2, v6, v7);
  dword_82D58 = 2;
  return v12 - __readfsqword(0x28u);
}



// Function: decode_time @ 0x1aca0
long long decode_time(long long *a1, char *a2, long long a3)
{
  long long v4; // rax
  long long v5; // rdx
  char *v7; // rax
  char *v8[5]; // [rsp+0h] [rbp-28h] BYREF

  v8[1] = (char *)__readfsqword(0x28u);
  v4 = sub_25930(a2, v8);
  if ( v5 < 0 )
  {
    if ( v8[0] <= a2 || *v8[0] )
    {
      if ( qword_82D50 )
        qword_82D50(a2);
      v7 = dcgettext(0, "Malformed extended header: invalid %s=%s", 5);
      error(0, 0, v7, a3, a2);
      dword_82D58 = 2;
      return 0;
    }
    else
    {
      sub_1ABF0(a3, (long long)a2, 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL);
      return 0;
    }
  }
  else
  {
    a1[1] = v5;
    *a1 = v4;
    return 1;
  }
}



// Function: mtime_decoder @ 0x1ad90
unsigned long long mtime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1ACA0(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 248) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: ctime_decoder @ 0x1adf0
unsigned long long ctime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1ACA0(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 264) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: atime_decoder @ 0x1ae50
unsigned long long atime_decoder(long long a1, long long a2, char *a3)
{
  __m128i v4; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1ACA0(v4.m128i_i64, a3, a2) )
    *(__m128i *)(a1 + 232) = _mm_load_si128(&v4);
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_map_decoder @ 0x1aeb0
unsigned long long sparse_map_decoder(const char *a1, long long a2, const char *a3)
{
  intmax_t v3; // r12
  const char *v4; // r13
  const char *v5; // rbx
  int v6; // r14d
  int *v7; // r15
  char v8; // r8
  intmax_t v9; // rax
  unsigned long long v10; // rdx
  unsigned long long v11; // rcx
  intmax_t *v12; // rdx
  const char *v13; // rsi
  char *v14; // rax
  char *v16; // rax
  char *v17; // rax
  char v19; // [rsp+Fh] [rbp-49h]
  char *endptr; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v21; // [rsp+18h] [rbp-40h]

  v4 = a3;
  v21 = __readfsqword(0x28u);
  *((long long *)a1 + 38) = 0;
  if ( (unsigned int)(*a3 - 48) <= 9 )
  {
    v5 = a1;
    v6 = 1;
    v7 = __errno_location();
    while ( 1 )
    {
      *v7 = 0;
      a1 = v4;
      v9 = strtoimax(v4, &endptr, 10);
      if ( v6 )
      {
        if ( *v7 == 34 )
          goto LABEL_18;
        v3 = v9;
      }
      else
      {
        if ( *v7 == 34 )
        {
LABEL_18:
          sub_1ABF0(a2, (long long)v4, 0, 0x7FFFFFFFFFFFFFFFLL);
          return v21 - __readfsqword(0x28u);
        }
        v10 = *((long long *)v5 + 38);
        if ( v10 >= *((long long *)v5 + 39) )
        {
          if ( qword_82D50 )
            qword_82D50(v4);
          v13 = "Malformed extended header: excess %s=%s";
          goto LABEL_16;
        }
        v11 = v10 + 1;
        v12 = (intmax_t *)(*((long long *)v5 + 40) + 16 * v10);
        *((long long *)v5 + 38) = v11;
        *v12 = v3;
        v12[1] = v9;
      }
      v8 = *endptr;
      if ( !*endptr )
      {
        if ( v6 )
        {
          if ( qword_82D50 )
            qword_82D50(v4);
          v16 = dcgettext(0, "Malformed extended header: invalid %s: odd number of values", 5);
          error(0, 0, v16, a2);
          dword_82D58 = 2;
        }
        return v21 - __readfsqword(0x28u);
      }
      if ( v8 != 44 )
      {
        if ( qword_82D50 )
        {
          qword_82D50(v4);
          v8 = *endptr;
        }
        v19 = v8;
        v17 = dcgettext(0, "Malformed extended header: invalid %s: unexpected delimiter %c", 5);
        error(0, 0, v17, a2, (unsigned int)v19);
        dword_82D58 = 2;
        return v21 - __readfsqword(0x28u);
      }
      v4 = endptr + 1;
      if ( (unsigned int)(endptr[1] - 48) > 9 )
        break;
      v6 ^= 1u;
    }
  }
  if ( qword_82D50 )
    qword_82D50(a1);
  v13 = "Malformed extended header: invalid %s=%s";
LABEL_16:
  v14 = dcgettext(0, v13, 5);
  error(0, 0, v14, a2, v4);
  dword_82D58 = 2;
  return v21 - __readfsqword(0x28u);
}



// Function: gid_decoder @ 0x1b1f0
unsigned long long gid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 120) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: uid_decoder @ 0x1b250
unsigned long long uid_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 116) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: size_decoder @ 0x1b610
unsigned long long size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
    *(long long *)(a1 + 136) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_size_decoder @ 0x1b670
unsigned long long volume_size_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, -1, a2) )
    qword_81F08 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_minor_decoder @ 0x1b6d0
unsigned long long sparse_minor_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 296) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_major_decoder @ 0x1b730
unsigned long long sparse_major_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, 0xFFFFFFFFLL, a2) )
    *(int *)(a1 + 292) = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: volume_offset_decoder @ 0x1b790
unsigned long long volume_offset_decoder(long long a1, long long a2, long long a3)
{
  long long v4; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v4, a3, -1, a2) )
    qword_81F00 = v4;
  return v5 - __readfsqword(0x28u);
}



// Function: sparse_size_decoder @ 0x1b7f0
unsigned long long sparse_size_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // rax
  long long v5; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v6; // [rsp+8h] [rbp-10h]

  v6 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v5, a3, 0x7FFFFFFFFFFFFFFFLL, a2) )
  {
    v3 = v5;
    *(char *)(a1 + 336) = 1;
    *(long long *)(a1 + 328) = v3;
  }
  return v6 - __readfsqword(0x28u);
}



// Function: sparse_numblocks_decoder @ 0x1b860
unsigned long long sparse_numblocks_decoder(long long *a1, long long a2, long long a3)
{
  long long v4; // rdi
  long long v5; // rax
  long long v7; // [rsp+0h] [rbp-18h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-10h]

  v8 = __readfsqword(0x28u);
  if ( (unsigned char)sub_1B100(&v7, a3, -1, a2) )
  {
    v4 = v7;
    a1[39] = v7;
    v5 = sub_4EDB0(v4, 16);
    a1[38] = 0;
    a1[40] = v5;
  }
  return v8 - __readfsqword(0x28u);
}



// Function: decg @ 0x1bab0
long long decg(long long *a1, char *a2, void *a3, long long a4)
{
  char **v6; // rax
  long long result; // rax
  long long *v8; // rbx

  v6 = sub_1AB00(a2);
  if ( v6 && ((char)v6[3] & 2) != 0 )
    return ((long long (*)(long long *, char *, void *, long long))v6[2])(a1, a2, a3, a4);
  v8 = (long long *)sub_4ECC0(24);
  v8[1] = sub_4EE30(a2);
  if ( a3 )
    a3 = (void *)sub_4EE30(a3);
  result = *a1;
  v8[2] = (long long)a3;
  *a1 = (long long)v8;
  *v8 = result;
  return result;
}



// Function: xheader_print_n @ 0x1bb40
unsigned long long xheader_print_n(long long *a1, char *a2, const void *a3, size_t a4)
{
  char *v4; // r15
  char *v5; // r14
  char v6; // r12
  long long v7; // rbp
  char *v8; // rax
  char *v9; // rbp
  char *v10; // r15
  long long v11; // rsi
  char *v12; // rax
  long long v13; // rbp
  char *v14; // rax
  long long v15; // rbp
  char *v16; // rax
  long long v17; // rbp
  char *v18; // rax
  char *v20; // rax
  size_t v22; // [rsp+10h] [rbp-68h]
  char v24[20]; // [rsp+20h] [rbp-58h] BYREF
  char v25[4]; // [rsp+34h] [rbp-44h] BYREF
  unsigned long long v26; // [rsp+38h] [rbp-40h]

  v4 = a2;
  v5 = qword_82340;
  v26 = __readfsqword(0x28u);
  if ( !qword_82340 )
  {
    qword_82338 = 256;
    v20 = (char *)sub_4ECC0(256);
    v6 = *a2;
    qword_82340 = v20;
    v5 = v20;
    if ( v6 )
      goto LABEL_3;
LABEL_23:
    v9 = v5;
    goto LABEL_13;
  }
  *qword_82340 = 0;
  v6 = *a2;
  if ( !*a2 )
    goto LABEL_23;
LABEL_3:
  v7 = 0;
  do
  {
    if ( v7 + 2 < (unsigned long long)qword_82338 )
    {
      v8 = &v5[v7];
      if ( v6 == 37 )
        goto LABEL_10;
    }
    else
    {
      v5 = (char *)sub_4ED20(v5, &qword_82338);
      qword_82340 = v5;
      v8 = &v5[v7];
      if ( v6 == 37 )
      {
LABEL_10:
        *(int *)v8 = 3486245;
        v7 += 2;
        goto LABEL_7;
      }
    }
    if ( v6 == 61 )
    {
      *(int *)v8 = 4469541;
      v7 += 2;
    }
    else
    {
      *v8 = v6;
    }
LABEL_7:
    v6 = *++v4;
    ++v7;
  }
  while ( v6 );
  v9 = &v5[v7];
LABEL_13:
  *v9 = 0;
  v10 = 0;
  v22 = strlen(v5);
  do
  {
    v11 = sub_45370(&v10[v22 + 3 + a4], v24);
    v12 = v10;
    v10 = &v25[-v11];
  }
  while ( v12 != &v25[-v11] );
  sub_1A9B0(a1, (const void *)v11, (size_t)&v25[-v11]);
  v13 = *a1;
  v14 = *(char **)(*a1 + 24LL);
  if ( *(char **)(*a1 + 32LL) == v14 )
  {
    obstack_newchunk(*a1, 1);
    v14 = *(char **)(v13 + 24);
  }
  *(long long *)(v13 + 24) = v14 + 1;
  *v14 = 32;
  ++a1[1];
  sub_1A9B0(a1, v5, v22);
  v15 = *a1;
  v16 = *(char **)(*a1 + 24LL);
  if ( *(char **)(*a1 + 32LL) == v16 )
  {
    obstack_newchunk(*a1, 1);
    v16 = *(char **)(v15 + 24);
  }
  *(long long *)(v15 + 24) = v16 + 1;
  *v16 = 61;
  ++a1[1];
  sub_1A9B0(a1, a3, a4);
  v17 = *a1;
  v18 = *(char **)(*a1 + 24LL);
  if ( *(char **)(*a1 + 32LL) == v18 )
  {
    obstack_newchunk(*a1, 1);
    v18 = *(char **)(v17 + 24);
  }
  *(long long *)(v17 + 24) = v18 + 1;
  *v18 = 10;
  ++a1[1];
  return v26 - __readfsqword(0x28u);
}



// Function: code_string @ 0x1bdb0
unsigned long long code_string(void *src, char *a2, long long *a3)
{
  char v4; // al
  char *v5; // r14
  size_t v6; // rax
  char *s; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+8h] [rbp-30h]

  v9 = __readfsqword(0x28u);
  v4 = sub_31920(1, src, &s);
  v5 = s;
  if ( !v4 )
  {
    s = (char *)sub_4EE30(src);
    v5 = s;
  }
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  free(s);
  return v9 - __readfsqword(0x28u);
}



// Function: xattr_selinux_coder @ 0x1be50
unsigned long long xattr_selinux_coder(long long a1, char *a2, long long *a3)
{
  return sub_1BDB0(*(void **)(a1 + 48), a2, a3);
}



// Function: dumpdir_coder @ 0x1bf20
unsigned long long dumpdir_coder(long long a1, char *a2, long long *a3, char *a4)
{
  size_t v6; // rax

  v6 = sub_1F690(a4);
  return sub_1BB40(a3, a2, a4, v6);
}



// Function: mtime_coder @ 0x1bf50
unsigned long long mtime_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  long long v6; // r8
  long long v7; // rsi
  const char *v8; // r12
  size_t v9; // rax
  long long v11; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v12; // [rsp+28h] [rbp-20h]

  v12 = __readfsqword(0x28u);
  if ( a4 )
  {
    v6 = *a4;
    v7 = a4[1];
  }
  else
  {
    v6 = *(long long *)(a1 + 248);
    v7 = *(long long *)(a1 + 256);
  }
  v8 = (const char *)sub_257A0(v6, v7, &v11);
  v9 = strlen(v8);
  sub_1BB40(a3, a2, v8, v9);
  return v12 - __readfsqword(0x28u);
}



// Function: ctime_coder @ 0x1bfe0
unsigned long long ctime_coder(long long a1, char *a2, long long *a3)
{
  long long v5; // rsi
  long long v6; // rdi
  const char *v7; // r12
  size_t v8; // rax
  long long v10; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v11; // [rsp+28h] [rbp-20h]

  v5 = *(long long *)(a1 + 272);
  v6 = *(long long *)(a1 + 264);
  v11 = __readfsqword(0x28u);
  v7 = (const char *)sub_257A0(v6, v5, &v10);
  v8 = strlen(v7);
  sub_1BB40(a3, a2, v7, v8);
  return v11 - __readfsqword(0x28u);
}



// Function: atime_coder @ 0x1c060
unsigned long long atime_coder(long long a1, char *a2, long long *a3)
{
  long long v5; // rsi
  long long v6; // rdi
  const char *v7; // r12
  size_t v8; // rax
  long long v10; // [rsp+0h] [rbp-48h] BYREF
  unsigned long long v11; // [rsp+28h] [rbp-20h]

  v5 = *(long long *)(a1 + 240);
  v6 = *(long long *)(a1 + 232);
  v11 = __readfsqword(0x28u);
  v7 = (const char *)sub_257A0(v6, v5, &v10);
  v8 = strlen(v7);
  sub_1BB40(a3, a2, v7, v8);
  return v11 - __readfsqword(0x28u);
}



// Function: sparse_offset_coder @ 0x1c0e0
unsigned long long sparse_offset_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(*(long long *)(*(long long *)(a1 + 320) + 16LL * *a4), 0, -1, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: sparse_major_coder @ 0x1c170
unsigned long long sparse_major_coder(long long a1, char *a2, long long *a3)
{
  long long v4; // rdi
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v4 = *(unsigned int *)(a1 + 292);
  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(v4, 0, -1, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: sparse_minor_coder @ 0x1c1f0
unsigned long long sparse_minor_coder(long long a1, char *a2, long long *a3)
{
  long long v4; // rdi
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v4 = *(unsigned int *)(a1 + 296);
  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(v4, 0, -1, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: volume_size_coder @ 0x1c370
unsigned long long volume_size_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  long long v5; // rdi
  const char *v6; // r12
  size_t v7; // rax
  long long v9; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v10; // [rsp+18h] [rbp-20h]

  v5 = *a4;
  v10 = __readfsqword(0x28u);
  v6 = (const char *)sub_255C0(v5, 0, -1, &v9);
  v7 = strlen(v6);
  sub_1BB40(a3, a2, v6, v7);
  return v10 - __readfsqword(0x28u);
}



// Function: uid_coder @ 0x1c3f0
unsigned long long uid_coder(long long a1, char *a2, long long *a3)
{
  long long v4; // rdi
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v4 = *(unsigned int *)(a1 + 116);
  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(v4, 0, 0xFFFFFFFFLL, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: sparse_numblocks_coder @ 0x1c4e0
unsigned long long sparse_numblocks_coder(long long a1, char *a2, long long *a3)
{
  long long v4; // rdi
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v4 = *(long long *)(a1 + 304);
  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(v4, 0, -1, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: gid_coder @ 0x1c560
unsigned long long gid_coder(long long a1, char *a2, long long *a3)
{
  long long v4; // rdi
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v4 = *(unsigned int *)(a1 + 120);
  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(v4, 0, 0xFFFFFFFFLL, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: sparse_numbytes_coder @ 0x1c5d0
unsigned long long sparse_numbytes_coder(long long a1, char *a2, long long *a3, long long *a4)
{
  const char *v5; // r12
  size_t v6; // rax
  long long v8; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v9 = __readfsqword(0x28u);
  v5 = (const char *)sub_255C0(*(long long *)(*(long long *)(a1 + 320) + 16LL * *a4 + 8), 0, -1, &v8);
  v6 = strlen(v5);
  sub_1BB40(a3, a2, v5, v6);
  return v9 - __readfsqword(0x28u);
}



// Function: volume_filename_decoder @ 0x1c660
long long volume_filename_decoder(long long a1, long long a2, long long a3)
{
  long long result; // rax

  if ( qword_81F10 )
  {
    free(qword_81F10);
    qword_81F10 = 0;
  }
  result = sub_31920(0, a3, &qword_81F10);
  if ( !(char)result )
    return sub_25120(&qword_81F10, a3);
  return result;
}



// Function: volume_label_decoder @ 0x1c6d0
long long volume_label_decoder(long long a1, long long a2, long long a3)
{
  long long result; // rax

  if ( qword_81F18 )
  {
    free(qword_81F18);
    qword_81F18 = 0;
  }
  result = sub_31920(0, a3, &qword_81F18);
  if ( !(char)result )
    return sub_25120(&qword_81F18, a3);
  return result;
}



// Function: linkpath_decoder @ 0x1c740
long long linkpath_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  void *v6; // rdi
  long long result; // rax

  v3 = a1 + 24;
  v6 = *(void **)(a1 + 24);
  if ( v6 )
  {
    free(v6);
    *(long long *)(a1 + 24) = 0;
  }
  result = sub_31920(0, a3, v3);
  if ( !(char)result )
    return sub_25120(v3, a3);
  return result;
}



// Function: xattr_selinux_decoder @ 0x1c790
long long xattr_selinux_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  void *v6; // rdi
  long long result; // rax

  v3 = a1 + 48;
  v6 = *(void **)(a1 + 48);
  if ( v6 )
  {
    free(v6);
    *(long long *)(a1 + 48) = 0;
  }
  result = sub_31920(0, a3, v3);
  if ( !(char)result )
    return sub_25120(v3, a3);
  return result;
}



// Function: uname_decoder @ 0x1c7e0
long long uname_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  void *v6; // rdi
  long long result; // rax

  v3 = a1 + 32;
  v6 = *(void **)(a1 + 32);
  if ( v6 )
  {
    free(v6);
    *(long long *)(a1 + 32) = 0;
  }
  result = sub_31920(0, a3, v3);
  if ( !(char)result )
    return sub_25120(v3, a3);
  return result;
}



// Function: gname_decoder @ 0x1c830
long long gname_decoder(long long a1, long long a2, long long a3)
{
  long long v3; // r12
  void *v6; // rdi
  long long result; // rax

  v3 = a1 + 40;
  v6 = *(void **)(a1 + 40);
  if ( v6 )
  {
    free(v6);
    *(long long *)(a1 + 40) = 0;
  }
  result = sub_31920(0, a3, v3);
  if ( !(char)result )
    return sub_25120(v3, a3);
  return result;
}



// Function: sparse_path_decoder @ 0x1c880
long long sparse_path_decoder(long long a1, long long a2, long long a3)
{
  void *v5; // rdi
  void *v6; // rdi
  long long result; // rax

  *(char *)(a1 + 337) = 1;
  v5 = *(void **)a1;
  if ( v5 )
  {
    free(v5);
    *(long long *)a1 = 0;
  }
  if ( !(unsigned char)sub_31920(0, a3, a1) )
    sub_25120(a1, a3);
  v6 = *(void **)(a1 + 8);
  if ( v6 )
  {
    free(v6);
    *(long long *)(a1 + 8) = 0;
  }
  if ( !(unsigned char)sub_31920(0, a3, a1 + 8) )
    sub_25120(a1 + 8, a3);
  result = sub_403C0(*(long long *)(a1 + 8));
  *(char *)(a1 + 16) = result;
  return result;
}



// Function: path_decoder @ 0x1c920
void path_decoder(long long a1, long long a2, long long a3)
{
  void *v5; // rdi
  void *v6; // rdi

  if ( !*(char *)(a1 + 337) )
  {
    v5 = *(void **)a1;
    if ( v5 )
    {
      free(v5);
      *(long long *)a1 = 0;
    }
    if ( !(unsigned char)sub_31920(0, a3, a1) )
      sub_25120(a1, a3);
    v6 = *(void **)(a1 + 8);
    if ( v6 )
    {
      free(v6);
      *(long long *)(a1 + 8) = 0;
    }
    if ( !(unsigned char)sub_31920(0, a3, a1 + 8) )
      sub_25120(a1 + 8, a3);
    *(char *)(a1 + 16) = sub_403C0(*(long long *)(a1 + 8));
  }
}



// Function: xheader_set_option @ 0x1ca20
char *xheader_set_option(char *a1)
{
  char *result; // rax
  char *v2; // r15
  char *v3; // rax
  char *v4; // r13
  char *v5; // r14
  char v6; // bl
  const unsigned short *v7; // rcx
  char *v8; // r14
  unsigned char v9; // r13
  const unsigned short *v10; // rax
  const char *v11; // rsi
  char **v12; // rbx
  const char *v13; // rsi
  long long v14; // r12
  char *v15; // rax
  long long *v16; // rbx
  long long v17; // rax
  long long v18; // rax
  const char *v19; // rdi
  char **v20; // rdx
  int v21; // eax
  long long *v22; // rbx
  long long v23; // rax
  char *v24; // rax
  char *v25; // rax
  char **v26; // [rsp+8h] [rbp-40h]

  result = strtok(a1, ",");
  if ( !result )
    return result;
  v2 = result;
  do
  {
    v3 = strchr(v2, 61);
    v4 = v3;
    if ( !v3 )
    {
      if ( qword_82D50 )
        qword_82D50(v2);
      v24 = dcgettext(0, "Keyword %s is unknown or not yet implemented", 5);
      error(0, 0, v24, v2);
LABEL_32:
      sub_2DEA0(2);
    }
    if ( v3 == v2 )
    {
      if ( qword_82D50 )
        qword_82D50(v2);
      v13 = "Malformed pax option: %s";
      v14 = sub_4D2D0(v2);
LABEL_31:
      v15 = dcgettext(0, v13, 5);
      error(0, 0, v15, v14);
      goto LABEL_32;
    }
    v5 = v3;
    v6 = 1;
    if ( *(v3 - 1) == 58 )
    {
      v5 = v3 - 1;
      v6 = 0;
    }
    if ( v5 > v2 )
    {
      v7 = *__ctype_b_loc();
      do
      {
        if ( (v7[(unsigned char)*v5] & 0x2000) == 0 )
          break;
        --v5;
      }
      while ( v5 != v2 );
    }
    *v5 = 0;
    v8 = v4 + 1;
    v9 = v4[1];
    if ( v9 )
    {
      v10 = *__ctype_b_loc();
      do
      {
        if ( (v10[v9] & 0x2000) == 0 )
          break;
        v9 = *++v8;
      }
      while ( v9 );
    }
    if ( !strcmp(v2, "delete") )
    {
      v11 = "atime";
      v12 = &off_7D260;
      while ( *((char *)v12 + 28) || ((char)v12[3] & 1) == 0 || fnmatch(v8, v11, 0) )
      {
        v11 = v12[4];
        v12 += 4;
        if ( !v11 )
        {
          v16 = (long long *)sub_4ECC0(24);
          v17 = sub_4EE30(v8);
          v16[2] = 0;
          v16[1] = v17;
          v18 = qword_82390;
          qword_82390 = (long long)v16;
          *v16 = v18;
          goto LABEL_21;
        }
      }
      if ( qword_82D50 )
        qword_82D50(v8);
      v13 = "Pattern %s cannot be used";
      v14 = sub_4D2D0(v8);
      goto LABEL_31;
    }
    if ( !strcmp(v2, "exthdr.name") )
    {
      sub_25120(&qword_82370, v8);
      goto LABEL_21;
    }
    if ( !strcmp(v2, "globexthdr.name") )
    {
      sub_25120(&qword_82358, v8);
      goto LABEL_21;
    }
    if ( !strcmp(v2, "exthdr.mtime") )
    {
      sub_1A8C0((long long)&qword_82368, &qword_82360, v8);
      goto LABEL_21;
    }
    if ( !strcmp(v2, "globexthdr.mtime") )
    {
      sub_1A8C0((long long)&qword_82350, &qword_82348, v8);
      goto LABEL_21;
    }
    v19 = "atime";
    v20 = &off_7D260;
    do
    {
      if ( !*((char *)v20 + 28) && ((char)v20[3] & 1) != 0 )
      {
        v26 = v20;
        v21 = strcmp(v19, v2);
        v20 = v26;
        if ( !v21 )
        {
          if ( qword_82D50 )
            qword_82D50(v19);
          v25 = dcgettext(0, "Keyword %s cannot be overridden", 5);
          error(0, 0, v25, v2);
          goto LABEL_32;
        }
      }
      v19 = v20[4];
      v20 += 4;
    }
    while ( v19 );
    if ( v6 )
    {
      v22 = (long long *)sub_4ECC0(24);
      v22[1] = sub_4EE30(v2);
      v22[2] = sub_4EE30(v8);
      v23 = qword_82388;
      qword_82388 = (long long)v22;
    }
    else
    {
      v22 = (long long *)sub_4ECC0(24);
      v22[1] = sub_4EE30(v2);
      v22[2] = sub_4EE30(v8);
      v23 = qword_82380;
      qword_82380 = (long long)v22;
    }
    *v22 = v23;
LABEL_21:
    result = strtok(0, ",");
    v2 = result;
  }
  while ( result );
  return result;
}



// Function: xheader_format_name @ 0x1cdf0
unsigned long long xheader_format_name(long long *a1, char *a2, long long a3)
{
  char *v3; // r13
  char v4; // al
  char *v6; // rbx
  const char *v7; // r15
  const char *v8; // r14
  void *v9; // rbp
  long long v10; // r12
  char v11; // dl
  char *v12; // r10
  size_t v13; // rax
  long long v14; // rdi
  unsigned long long v15; // r12
  char v16; // al
  char *v17; // rbx
  char v18; // dl
  __pid_t v20; // eax
  char *v21; // rcx
  char *v22; // rax
  size_t v23; // rax
  long long v24; // rax
  char *src; // [rsp+0h] [rbp-A8h]
  char *v26; // [rsp+8h] [rbp-A0h]
  long long *v28; // [rsp+20h] [rbp-88h]
  long long *v29; // [rsp+20h] [rbp-88h]
  long long *v30; // [rsp+20h] [rbp-88h]
  long long *v31; // [rsp+20h] [rbp-88h]
  char *v32; // [rsp+28h] [rbp-80h]
  char v33[20]; // [rsp+30h] [rbp-78h] BYREF
  char v34; // [rsp+44h] [rbp-64h] BYREF
  char v35[20]; // [rsp+50h] [rbp-58h] BYREF
  char v36; // [rsp+64h] [rbp-44h] BYREF
  unsigned long long v37; // [rsp+68h] [rbp-40h]

  v3 = a2;
  v37 = __readfsqword(0x28u);
  v4 = *a2;
  if ( *a2 )
  {
    v6 = a2;
    v7 = 0;
    src = 0;
    v8 = 0;
    v9 = 0;
    v10 = 0;
    v26 = 0;
    while ( 1 )
    {
      v11 = v6[1];
      v12 = v6 + 1;
      if ( v4 != 37 || !v11 )
      {
        ++v10;
        v4 = v6[1];
        goto LABEL_4;
      }
      if ( v11 == 102 )
        break;
      if ( v11 > 102 )
      {
        if ( v11 == 110 )
        {
          v29 = a1;
          v26 = (char *)sub_45370(a3, v35);
          v21 = v26;
          v22 = &v36;
        }
        else
        {
          if ( v11 != 112 )
            goto LABEL_48;
          v29 = a1;
          v20 = getpid();
          src = (char *)sub_45370(v20, v33);
          v21 = src;
          v22 = &v34;
        }
        a1 = v29;
        v12 = v6 + 2;
        v10 += v22 - v21;
        v4 = v6[2];
LABEL_4:
        v6 = v12;
        if ( !v4 )
          goto LABEL_15;
      }
      else
      {
        if ( v11 == 37 )
        {
          v4 = v6[2];
          ++v10;
          v12 = v6 + 2;
          goto LABEL_4;
        }
        if ( v11 != 100 )
        {
LABEL_48:
          v4 = v6[2];
          v10 += 2;
          v12 = v6 + 2;
          goto LABEL_4;
        }
        if ( !a1 )
          goto LABEL_55;
        if ( !v9 )
        {
          v31 = a1;
          v24 = sub_40230(*a1);
          v12 = v6 + 1;
          a1 = v31;
          v9 = (void *)v24;
        }
        v32 = v12;
        v28 = a1;
        v8 = (const char *)sub_33640(v9);
        v13 = strlen(v8);
        a1 = v28;
        v10 += v13;
        v4 = v6[2];
        v6 = v32 + 1;
        if ( !v4 )
        {
LABEL_15:
          v14 = v10 + 1;
          goto LABEL_16;
        }
      }
    }
    if ( !a1 )
    {
LABEL_55:
      v4 = v6[2];
      v12 = v6 + 2;
      goto LABEL_4;
    }
    v30 = a1;
    v7 = (const char *)sub_400D0(*a1);
    v23 = strlen(v7);
    a1 = v30;
    v12 = v6 + 2;
    v10 += v23;
    v4 = v6[2];
    goto LABEL_4;
  }
  src = 0;
  v7 = 0;
  v8 = 0;
  v9 = 0;
  v26 = 0;
  v14 = 1;
LABEL_16:
  v15 = sub_4ECC0(v14);
  v16 = *a2;
  if ( !*a2 )
  {
    v17 = (char *)v15;
    free(v9);
    goto LABEL_33;
  }
  v17 = (char *)v15;
  while ( 1 )
  {
    while ( 1 )
    {
      v18 = v3[1];
      if ( v16 == 37 )
        break;
      *v17 = v16;
      ++v3;
      v16 = v18;
      ++v17;
LABEL_19:
      if ( !v16 )
        goto LABEL_29;
    }
    if ( v18 != 102 )
      break;
    if ( v7 )
    {
      v17 = stpcpy(v17, v7);
      goto LABEL_41;
    }
LABEL_27:
    v16 = v3[2];
    v3 += 2;
LABEL_28:
    if ( !v16 )
      goto LABEL_29;
  }
  if ( v18 > 102 )
  {
    if ( v18 == 110 )
    {
      v17 = stpcpy(v17, v26);
      goto LABEL_41;
    }
    if ( v18 == 112 )
    {
      v17 = stpcpy(v17, src);
LABEL_41:
      v16 = v3[2];
      v3 += 2;
      goto LABEL_28;
    }
    *v17 = 37;
LABEL_50:
    v17[1] = v18;
    v16 = v3[2];
    v17 += 2;
    v3 += 2;
    goto LABEL_19;
  }
  if ( v18 == 37 )
  {
    *v17 = 37;
    v16 = v3[2];
    ++v17;
    v3 += 2;
    goto LABEL_19;
  }
  if ( v18 == 100 )
  {
    if ( v8 )
      v17 = stpcpy(v17, v8);
    goto LABEL_27;
  }
  *v17 = 37;
  if ( v18 )
    goto LABEL_50;
  ++v17;
LABEL_29:
  free(v9);
  if ( v15 < (unsigned long long)v17 )
  {
    do
    {
      if ( *(v17 - 1) != 47 )
        break;
      --v17;
    }
    while ( (char *)v15 != v17 );
  }
LABEL_33:
  *v17 = 0;
  return v15;
}



// Function: xheader_xhdr_name @ 0x1d1d0
unsigned long long xheader_xhdr_name(long long *a1)
{
  if ( !qword_82370 )
    sub_25120(&qword_82370, off_7D640[(unsigned char)byte_82A38]);
  return sub_1CDF0(a1, (char *)qword_82370, 0);
}



// Function: xheader_ghdr_name @ 0x1d220
unsigned long long xheader_ghdr_name()
{
  char *v0; // r8
  const char *v2; // r12
  char *v3; // rbp
  size_t v4; // rbx
  size_t v5; // rax
  char *v6; // rax

  v0 = (char *)qword_82358;
  if ( !qword_82358 )
  {
    v2 = off_7D640[(unsigned char)byte_82A38 + 2];
    v3 = getenv("TMPDIR");
    if ( !v3 )
      v3 = "/tmp";
    v4 = strlen(v3);
    v5 = strlen(v2);
    qword_82358 = sub_4ECC0(v4 + v5 + 1);
    v6 = strcpy((char *)qword_82358, v3);
    v0 = strcat(v6, v2);
  }
  return sub_1CDF0(0, v0, qword_82398 + 1);
}



// Function: xheader_write @ 0x1d2d0
void xheader_write(unsigned char a1, long long a2, long long a3, long long a4)
{
  size_t v7; // r14
  unsigned long long v8; // rsi
  unsigned char *v9; // rax
  int v10; // ebp
  unsigned char *v11; // rdi
  long long v12; // rdx
  long long v13; // rcx
  long long v14; // r8
  size_t v15; // rbx
  char *v16; // rax
  long long *v17; // r15
  char *v18; // r8
  unsigned int v19; // eax
  char *v20; // r14
  char *v21; // rdi
  char *v22; // rsi
  unsigned int v23; // ecx

  v7 = *(long long *)(a4 + 8);
  if ( a1 == 103 )
  {
    if ( qword_82350 )
      a3 = qword_82348;
  }
  else if ( a1 == 120 && qword_82368 )
  {
    a3 = qword_82360;
  }
  v8 = *(long long *)(a4 + 8);
  v9 = (unsigned char *)sub_13050(a2, v8, a3);
  v10 = v7 & 0x1FF;
  v9[156] = a1;
  v11 = v9;
  sub_13230(v9);
  v15 = v7 + *(long long *)(a4 + 16);
  while ( 1 )
  {
    v16 = (char *)sub_D360(v11, v8, v12, v13, v14);
    v17 = (long long *)(v15 - v7);
    if ( v7 == v10 )
      break;
    v21 = (char *)((unsigned long long)(v16 + 8) & 0xFFFFFFFFFFFFFFF8LL);
    v7 -= 512LL;
    *(long long *)v16 = *v17;
    v22 = (char *)v17 - (v16 - v21);
    v23 = (unsigned int)((int)v16 - (int)v21 + 512) >> 3;
    *((long long *)v16 + 63) = v17[63];
    qmemcpy(v21, v22, 8LL * v23);
    v8 = (unsigned long long)&v22[8 * v23];
    v11 = (unsigned char *)v16;
    sub_CCD0(v16);
    if ( !v7 )
      goto LABEL_13;
  }
  v18 = (char *)memcpy(v16, (const void *)(v15 - v7), v7);
  v19 = 512 - v7;
  v20 = &v18[v7];
  if ( v19 >= 8 )
  {
    *(long long *)v20 = 0;
    *(long long *)&v20[v19 - 8] = 0;
    memset(
      (void *)((unsigned long long)(v20 + 8) & 0xFFFFFFFFFFFFFFF8LL),
      0,
      8LL * ((v19 + (int)v20 - (((int)v20 + 8) & 0xFFFFFFF8)) >> 3));
  }
  else if ( (v19 & 4) != 0 )
  {
    *(int *)v20 = 0;
    *(int *)&v20[v19 - 4] = 0;
  }
  else if ( v19 )
  {
    *v20 = 0;
    if ( (v19 & 2) != 0 )
      *(short *)&v20[v19 - 2] = 0;
  }
  sub_CCD0(v18);
LABEL_13:
  if ( *(long long *)a4 )
  {
    obstack_free(*(struct obstack **)a4, 0);
    free(*(void **)a4);
    *(long long *)a4 = 0;
  }
  else
  {
    free(*(void **)(a4 + 16));
  }
  *(long long *)(a4 + 16) = 0;
  *(long long *)(a4 + 8) = 0;
  if ( a1 == 103 )
    ++qword_82398;
}



// Function: xheader_forbid_global @ 0x1d4e0
void xheader_forbid_global(long long a1)
{
  char *v1; // rax

  if ( qword_82388 )
  {
    if ( qword_82D50 )
      qword_82D50(a1);
    v1 = dcgettext(0, "can't update global extended header record", 5);
    error(0, 0, v1);
    sub_2DEA0(2);
  }
}



// Function: xheader_xattr_init @ 0x1d530
void xheader_xattr_init(long long *a1)
{
  a1[44] = 0;
  a1[43] = 0;
  a1[7] = 0;
  a1[8] = 0;
  a1[9] = 0;
  a1[10] = 0;
  a1[6] = 0;
}



// Function: xheader_xattr_free @ 0x1d580
void xheader_xattr_free(void **ptr, long long a2)
{
  void **v3; // rbx
  long long v4; // rbp
  void *v5; // rdi

  if ( a2 )
  {
    v3 = ptr;
    v4 = 0;
    do
    {
      v5 = *v3;
      ++v4;
      v3 += 3;
      free(v5);
      free(*(v3 - 2));
    }
    while ( a2 != v4 );
  }
  free(ptr);
}



// Function: xheader_xattr_add @ 0x1d5d0
void xheader_xattr_add(long long a1, const char *a2, void *a3, long long a4)
{
  size_t v6; // rbp
  size_t v7; // rax
  char *v8; // rbp

  v6 = strlen(a2);
  v7 = strlen("SCHILY.xattr.");
  v8 = (char *)sub_4ECC0(v6 + v7 + 1);
  qmemcpy(v8, "SCHILY.xattr.", 13);
  strcpy(v8 + 13, a2);
  sub_1AA20((long long *)(a1 + 352), (long long *)(a1 + 344), v8, a3, a4);
  free(v8);
}



// Function: xattr_decoder @ 0x1d660
unsigned long long xattr_decoder(long long a1, const char *a2, const void *a3, long long a4)
{
  size_t v6; // rax
  long long *v7; // rcx
  signed long long v8; // rdx
  void *v9; // rsp
  long long *v10; // rcx
  signed long long v11; // rdx
  void *v12; // rsp
  char *v13; // rcx
  char *v14; // rax
  char v15; // dl
  char v16; // dl
  size_t v17; // rax
  long long v19; // [rsp+0h] [rbp-2048h]
  long long v20; // [rsp+8h] [rbp-2040h] BYREF
  char v21[4088]; // [rsp+10h] [rbp-2038h] BYREF
  long long v22; // [rsp+1008h] [rbp-1040h] BYREF
  char v23; // [rsp+1010h] [rbp-1038h] BYREF
  char v24[7]; // [rsp+1011h] [rbp-1037h] BYREF
  long long v25; // [rsp+2008h] [rbp-40h] BYREF
  unsigned long long v26; // [rsp+2010h] [rbp-38h]

  v26 = __readfsqword(0x28u);
  v6 = strlen(a2);
  v7 = (long long *)((char *)&v25 - ((v6 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v25 != v7 )
  {
    while ( &v22 != v7 )
      ;
  }
  v8 = ((short)v6 + 24) & 0xFF0;
  v9 = alloca(v8);
  if ( (((short)v6 + 24) & 0xFF0) != 0 )
    *(long long *)&v21[v8 + 4080] = *(long long *)&v21[v8 + 4080];
  memcpy(&v23, a2, v6 + 1);
  v10 = (long long *)((char *)&v22 - ((a4 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v22 != v10 )
  {
    while ( &v20 != v10 )
      ;
  }
  v11 = ((short)a4 + 24) & 0xFF0;
  v12 = alloca(v11);
  if ( (((short)a4 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v19 + v11) = *(long long *)((char *)&v19 + v11);
  memcpy(v21, a3, a4 + 1);
  v13 = v24;
  v14 = &v23;
  while ( 1 )
  {
    v15 = *v14;
    if ( *v14 != 37 )
      break;
    v16 = v14[1];
    if ( v16 == 51 )
    {
      if ( v14[2] != 68 )
        goto LABEL_17;
      *(v13 - 1) = 61;
      v14 += 3;
    }
    else
    {
      if ( v16 != 50 || v14[2] != 53 )
      {
LABEL_17:
        *(v13 - 1) = 37;
        goto LABEL_11;
      }
      *(v13 - 1) = 37;
      v14 += 3;
    }
LABEL_12:
    ++v13;
  }
  *(v13 - 1) = v15;
  if ( v15 )
  {
LABEL_11:
    ++v14;
    goto LABEL_12;
  }
  v17 = strlen("SCHILY.xattr.");
  sub_1D5D0(a1, &v24[v17 - 1], v21, a4);
  return v26 - __readfsqword(0x28u);
}



// Function: xheader_xattr_copy @ 0x1d820
long long xheader_xattr_copy(long long a1, long long *a2, long long *a3)
{
  unsigned long long v4; // rbx
  long long v5; // rdx
  long long result; // rax

  *a2 = 0;
  *a3 = 0;
  if ( *(long long *)(a1 + 344) )
  {
    v4 = 0;
    do
    {
      v5 = 3 * v4++;
      result = sub_1AA20(
                 a2,
                 a3,
                 *(void **)(*(long long *)(a1 + 352) + 8 * v5),
                 *(void **)(*(long long *)(a1 + 352) + 8 * v5 + 8),
                 *(long long *)(*(long long *)(a1 + 352) + 8 * v5 + 16));
    }
    while ( *(long long *)(a1 + 344) > v4 );
  }
  return result;
}



// Function: xheader_decode @ 0x1d8a0
unsigned long long xheader_decode(long long a1)
{
  long long v2; // rdi
  long long v3; // rsi
  bool v4; // zf
  long long v6; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v7; // [rsp+8h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  sub_1AB80((long long **)qword_82388, a1);
  sub_1AB80((long long **)qword_82378, a1);
  v2 = *(long long *)(a1 + 368);
  if ( v2 )
  {
    v3 = *(long long *)(a1 + 376);
    v6 = v3 + 512;
    while ( (unsigned char)sub_1B390(v2, v3, &v6, &sub_1B9B0, a1) )
    {
      v2 = *(long long *)(a1 + 368);
      v3 = *(long long *)(a1 + 376);
    }
  }
  sub_1AB80((long long **)qword_82380, a1);
  v4 = *(char *)(a1 + 336) == 0;
  *(long long *)(a1 + 280) = *(long long *)(a1 + 136);
  if ( !v4 )
    *(long long *)(a1 + 136) = *(long long *)(a1 + 328);
  return v7 - __readfsqword(0x28u);
}



// Function: xheader_decode_global @ 0x1d980
unsigned long long xheader_decode_global(long long a1)
{
  long long v2; // rdi
  long long v4; // rsi
  void **v5; // rbx
  void **v6; // rbp
  long long v7; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v8; // [rsp+8h] [rbp-30h]

  v2 = *(long long *)(a1 + 8);
  v8 = __readfsqword(0x28u);
  if ( v2 )
  {
    v4 = *(long long *)(a1 + 16);
    v5 = (void **)qword_82378;
    v7 = v4 + 512;
    if ( qword_82378 )
    {
      do
      {
        v6 = v5;
        v5 = (void **)*v5;
        free(v6[1]);
        free(v6[2]);
        free(v6);
      }
      while ( v5 );
      v2 = *(long long *)(a1 + 8);
      v4 = *(long long *)(a1 + 16);
    }
    qword_82378 = 0;
    while ( (unsigned char)sub_1B390(v2, v4, &v7, sub_1BAB0, &qword_82378) )
    {
      v2 = *(long long *)(a1 + 8);
      v4 = *(long long *)(a1 + 16);
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: xheader_store @ 0x1da70
void xheader_store(char *s2, long long a2, long long a3)
{
  char **v4; // rax
  char **v5; // r14
  long long v6; // rbx
  long long v7; // rbx
  long long v8; // rax

  if ( !*(long long *)(a2 + 376) )
  {
    v4 = sub_1AB00(s2);
    v5 = v4;
    if ( v4 )
    {
      if ( v4[1] )
      {
        v6 = qword_82390;
        if ( qword_82390 )
        {
          while ( fnmatch(*(const char **)(v6 + 8), s2, 0) )
          {
            v6 = *(long long *)v6;
            if ( !v6 )
              goto LABEL_9;
          }
        }
        else
        {
LABEL_9:
          if ( !*(long long *)(a2 + 360) )
          {
            v8 = sub_4ECC0(88);
            *(long long *)(a2 + 360) = v8;
            obstack_begin(v8, 0, 0, sub_4ECC0, &free);
          }
          v7 = qword_82380;
          if ( qword_82380 )
          {
            while ( strcmp(*(const char **)(v7 + 8), s2) )
            {
              v7 = *(long long *)v7;
              if ( !v7 )
                goto LABEL_16;
            }
          }
          else
          {
LABEL_16:
            ((void (*)(long long, char *, long long, long long))v5[1])(a2, s2, a2 + 360, a3);
          }
        }
      }
    }
  }
}



// Function: xheader_read @ 0x1db80
long long xheader_read(long long a1, char *a2, long long a3)
{
  long long v3; // rax
  long long v4; // r14
  int *v5; // r12
  long long v6; // r13
  long long v7; // rbp
  int *v9; // rdi
  long long v10; // rax
  long long v11; // rdx
  long long v12; // rcx
  long long v13; // r8
  long long result; // rax
  long long v15; // rbp
  char *v16; // rcx
  char *v17; // rdi
  signed long long v18; // rcx
  char *v19; // rsi
  char *v20; // rax

  v3 = 0;
  if ( a3 >= 0 )
    v3 = a3;
  v4 = 0;
  v5 = a2;
  v6 = v3 + 512;
  v7 = v3;
  *(long long *)(a1 + 8) = v3 + 512;
  v9 = (int *)(v3 + 513);
  v10 = sub_4ECC0(v3 + 513);
  *(long long *)(a1 + 16) = v10;
  *(char *)(v10 + v7 + 512) = 0;
  do
  {
    v15 = 512;
    if ( (unsigned long long)v6 <= 0x200 )
      v15 = v6;
    if ( !v5 )
    {
      if ( qword_82D50 )
        qword_82D50(v9);
      v20 = dcgettext(0, "Unexpected EOF in archive", 5);
      error(0, 0, v20);
      sub_16FD0();
    }
    v16 = (char *)(v4 + *(long long *)(a1 + 16));
    if ( (unsigned int)v15 < 8 )
    {
      if ( (v15 & 4) != 0 )
      {
        *(int *)v16 = *v5;
        *(int *)&v16[(unsigned int)v15 - 4] = *(int *)((char *)v5 + (unsigned int)v15 - 4);
      }
      else if ( (int)v15 )
      {
        *v16 = *(char *)v5;
        if ( (v15 & 2) != 0 )
          *(short *)&v16[(unsigned int)v15 - 2] = *(short *)((char *)v5 + (unsigned int)v15 - 2);
      }
    }
    else
    {
      v17 = (char *)((unsigned long long)(v16 + 8) & 0xFFFFFFFFFFFFFFF8LL);
      *(long long *)v16 = *(long long *)v5;
      *(long long *)&v16[(unsigned int)v15 - 8] = *(long long *)((char *)v5 + (unsigned int)v15 - 8);
      v18 = v16 - v17;
      v19 = (char *)v5 - v18;
      LODWORD(v18) = (unsigned int)(v15 + v18) >> 3;
      qmemcpy(v17, v19, 8LL * (unsigned int)v18);
      a2 = &v19[8 * (unsigned int)v18];
    }
    v9 = v5;
    v6 -= v15;
    v4 += v15;
    sub_CCD0(v5);
    result = sub_D360(v5, a2, v11, v12, v13);
    v5 = (int *)result;
  }
  while ( v6 > 0 );
  return result;
}



// Function: xheader_finish @ 0x1dcd0
long long xheader_finish(long long *a1)
{
  long long i; // rbx
  long long result; // rax
  long long v3; // rsi
  long long v4; // rdx
  long long v5; // rdx
  long long v6; // rcx

  for ( i = qword_82380; i; i = *(long long *)i )
    sub_1BDB0(*(void **)(i + 16), *(char **)(i + 8), a1);
  result = *a1;
  v3 = *(long long *)(*a1 + 16LL);
  v4 = *(long long *)(*a1 + 24LL);
  if ( v4 == v3 )
    *(char *)(result + 80) |= 2u;
  v5 = ~*(long long *)(result + 48) & (*(long long *)(result + 48) + v4);
  v6 = *(long long *)(result + 32);
  if ( v5 - *(long long *)(result + 8) <= (unsigned long long)(v6 - *(long long *)(result + 8)) )
    v6 = v5;
  *(long long *)(result + 24) = v6;
  *(long long *)(result + 16) = v6;
  a1[2] = v3;
  return result;
}



// Function: xheader_write_global @ 0x1dd60
void xheader_write_global(long long *a1)
{
  long long v1; // rbx
  void *v2; // rbp
  long long v3; // rax

  v1 = qword_82388;
  if ( qword_82388 )
  {
    if ( *a1 || (v3 = sub_4ECC0(88), *a1 = v3, obstack_begin(v3, 0, 0, sub_4ECC0, &free), (v1 = qword_82388) != 0) )
    {
      do
      {
        sub_1BDB0(*(void **)(v1 + 16), *(char **)(v1 + 8), a1);
        v1 = *(long long *)v1;
      }
      while ( v1 );
    }
  }
  if ( *a1 )
  {
    sub_1DCD0(a1);
    v2 = (void *)sub_1D220();
    sub_1D2D0(0x67u, (long long)v2, tp.tv_sec, (long long)a1);
    free(v2);
  }
}



// Function: xheader_string_begin @ 0x1de70
void xheader_string_begin(long long a1)
{
  *(long long *)(a1 + 24) = 0;
}



// Function: xheader_string_add @ 0x1de80
void xheader_string_add(long long *a1, const char *a2)
{
  size_t v2; // rax
  long long v3; // rax

  if ( !a1[2] )
  {
    if ( !*a1 )
    {
      v3 = sub_4ECC0(88);
      *a1 = v3;
      obstack_begin(v3, 0, 0, sub_4ECC0, &free);
    }
    a1[3] += strlen(a2);
    v2 = strlen(a2);
    sub_1A9B0(a1, a2, v2);
  }
}



// Function: xheader_string_end @ 0x1df00
long long xheader_string_end(long long *a1, const char *a2)
{
  long long result; // rax
  char *v3; // rbx
  size_t v4; // r14
  long long v5; // r8
  char *v6; // rax
  size_t v7; // rax
  char *v8; // r8
  unsigned long long v9; // r12
  long long v10; // rbx
  char *v11; // rdx
  long long v12; // rax
  long long v13; // r13
  char *v14; // rax
  size_t v15; // rdx
  long long v16; // rax
  long long v17; // rdi
  void *v18; // rbp
  char *v19; // rax
  long long v20; // rax
  char *src; // [rsp+8h] [rbp-60h]
  char *srcb; // [rsp+8h] [rbp-60h]
  char *srca; // [rsp+8h] [rbp-60h]
  char v24[20]; // [rsp+10h] [rbp-58h] BYREF
  char v25[4]; // [rsp+24h] [rbp-44h] BYREF
  unsigned long long v26; // [rsp+28h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  result = 0;
  if ( !a1[2] )
  {
    if ( !*a1 )
    {
      v20 = sub_4ECC0(88);
      *a1 = v20;
      obstack_begin(v20, 0, 0, sub_4ECC0, &free);
    }
    v3 = 0;
    v4 = a1[3] + strlen(a2) + 3;
    do
    {
      v5 = sub_45370(&v3[v4], v24);
      v6 = v3;
      v3 = &v25[-v5];
    }
    while ( v6 != &v25[-v5] );
    src = (char *)v5;
    v7 = strlen(a2);
    v8 = src;
    v9 = (unsigned long long)&v3[v7 + 2];
    v10 = *a1;
    v11 = *(char **)(*a1 + 32);
    v12 = *(long long *)(*a1 + 24);
    v13 = *a1;
    if ( v9 > (unsigned long long)&v11[-v12] )
    {
      obstack_newchunk(*a1, v9);
      v13 = *a1;
      v12 = *(long long *)(v10 + 24);
      v8 = src;
      v11 = *(char **)(*a1 + 32);
    }
    *(long long *)(v10 + 24) = v9 + v12;
    v14 = *(char **)(v13 + 24);
    a1[1] += v9;
    if ( v14 == v11 )
    {
      srca = v8;
      obstack_newchunk(v13, 1);
      v14 = *(char **)(v13 + 24);
      v8 = srca;
    }
    srcb = v8;
    *(long long *)(v13 + 24) = v14 + 1;
    *v14 = 10;
    v15 = a1[3];
    v16 = *a1;
    ++a1[1];
    v17 = *(long long *)(v16 + 24);
    v18 = (void *)(v17 + ~v15 - v9);
    memmove((void *)(~v15 + v17), v18, v15);
    v19 = stpcpy((char *)v18, srcb);
    *v19 = 32;
    *stpcpy(v19 + 1, a2) = 61;
    return 1;
  }
  return result;
}



// Function: hash_directory_meta @ 0x1e0d0
unsigned long long hash_directory_meta(long long a1, unsigned long long a2)
{
  return (*(long long *)(a1 + 32) + *(long long *)(a1 + 24)) % a2;
}



// Function: compare_directory_meta @ 0x1e0f0
bool compare_directory_meta(long long a1, long long a2)
{
  bool result; // al

  result = 0;
  if ( *(long long *)(a1 + 24) == *(long long *)(a2 + 24) )
    return *(long long *)(a1 + 32) == *(long long *)(a2 + 32);
  return result;
}



// Function: make_directory @ 0x1e190
long long *make_directory(const char *src, long long a2)
{
  size_t v3; // rbx
  long long v4; // rax
  long long *v5; // r12
  size_t v6; // rdi
  void *v7; // rax
  char *v8; // rax

  v3 = strlen(src);
  v4 = sub_4ECC0(96);
  *(long long *)v4 = 0;
  v5 = (long long *)v4;
  *(long long *)(v4 + 48) = 0;
  *(long long *)(v4 + 40) = 0;
  *(long long *)(v4 + 64) = 0;
  *(int *)(v4 + 60) = 0;
  if ( v3 > 1 && src[v3 - 1] == 47 )
    v6 = v3--;
  else
    v6 = v3 + 1;
  v7 = (void *)sub_4ECC0(v6);
  v5[11] = v7;
  v8 = memcpy(v7, src, v3);
  v5[10] = a2;
  v8[v3] = 0;
  v5[9] = 0;
  return v5;
}



// Function: find_directory @ 0x1e240
long long find_directory(const char *src)
{
  long long v1; // r12
  long long v2; // rax
  void **v3; // rbp

  v1 = qword_823B0;
  if ( qword_823B0 )
  {
    v2 = sub_26700((unsigned int)dword_82580, src);
    v3 = (void **)sub_1E190(src, v2);
    v1 = sub_43960(qword_823B0, v3);
    free(v3[10]);
    free(v3[11]);
    free(v3);
  }
  return v1;
}



// Function: compare_directory_canonical_names @ 0x1e2c0
bool compare_directory_canonical_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 80), *(const char **)(a2 + 80)) == 0;
}



// Function: dumpdir_create0 @ 0x1e2e0
const char **dumpdir_create0(char *s, char *a2)
{
  char v3; // r13
  const char *v4; // r15
  size_t v5; // r12
  const char *v6; // rbx
  size_t v7; // r14
  long long v8; // rdi
  long long v9; // r8
  const char **v10; // r14
  long long v11; // rax
  const char *v12; // rbx
  long long *v13; // r13
  int v14; // esi
  long long v15; // r12
  const char *v17; // [rsp+8h] [rbp-50h]
  long long v19; // [rsp+18h] [rbp-40h]

  v3 = *s;
  if ( *s )
  {
    v17 = 0;
    v4 = s;
    v5 = 1;
    v6 = 0;
    do
    {
      v7 = strlen(v4) + 1;
      v5 += v7;
      if ( !a2 || strchr(a2, v3) )
        ++v17;
      v4 += v7;
      ++v6;
      v3 = *v4;
    }
    while ( *v4 );
    v8 = v5 + 32;
    v9 = (long long)(v17 + 1);
  }
  else
  {
    v17 = 0;
    v9 = 1;
    v8 = 33;
    v6 = 0;
    v5 = 1;
  }
  v19 = v9;
  v10 = (const char **)sub_4ECC0(v8);
  *v10 = (const char *)(v10 + 4);
  memcpy(v10 + 4, s, v5);
  v10[1] = v6;
  v10[2] = v17;
  v11 = sub_4EDB0(v19, 8);
  v12 = *v10;
  v10[3] = (const char *)v11;
  v13 = (long long *)v11;
  v14 = *v12;
  if ( *v12 )
  {
    v15 = 0;
    do
    {
      if ( !a2 || strchr(a2, v14) )
        v13[v15++] = v12 + 1;
      v12 += strlen(v12) + 1;
      v14 = *v12;
    }
    while ( *v12 );
    v13 += v15;
  }
  *v13 = 0;
  return v10;
}



// Function: note_directory @ 0x1e440
long long *note_directory(
        const char *src,
        long long a2,
        long long a3,
        long long a4,
        long long a5,
        char a6,
        char a7,
        char *s)
{
  const char **v12; // rbp
  long long v13; // rax
  long long *v14; // rax
  long long *v15; // r12
  long long v16; // rdi
  long long v17; // rdi

  v12 = (const char **)s;
  v13 = sub_26700((unsigned int)dword_82580, src);
  v14 = sub_1E190(src, v13);
  v15 = v14;
  if ( qword_823B8 )
    *(long long *)qword_823B8 = v14;
  else
    qword_823C0 = v14;
  qword_823B8 = (long long)v14;
  v14[1] = a2;
  v14[2] = a3;
  v14[3] = a4;
  v14[4] = a5;
  *((int *)v14 + 14) = 1;
  if ( a6 )
    *((int *)v14 + 15) |= 2u;
  if ( a7 )
    *((int *)v14 + 15) |= 4u;
  if ( s )
    v12 = sub_1E2E0(s, "YND");
  v16 = qword_823B0;
  v15[5] = v12;
  if ( !v16 && (qword_823B0 = sub_43BF0(0, 0, sub_1E5B0, sub_1E2C0, 0), (v16 = qword_823B0) == 0)
    || !sub_44240(v16, v15)
    || (v17 = qword_823A8) == 0 && (qword_823A8 = sub_43BF0(0, 0, sub_1E0D0, sub_1E0F0, 0), (v17 = qword_823A8) == 0)
    || !sub_44240(v17, v15) )
  {
    sub_17020();
  }
  return v15;
}



// Function: hash_directory_canonical_name @ 0x1e5b0
long long hash_directory_canonical_name(long long a1)
{
  return sub_43B80(*(long long *)(a1 + 80));
}



// Function: obstack_code_rename @ 0x1e5c0
void *obstack_code_rename(long long a1, char *a2, char *a3)
{
  const char *v3; // r13
  char *v6; // rax
  size_t v7; // rax
  void *v8; // rdi
  size_t v9; // rbp
  char *v10; // rax
  size_t v11; // rax
  void *v12; // rdi
  size_t v13; // rbp
  void *result; // rax

  v3 = a2;
  if ( *a2 )
  {
    v3 = (const char *)sub_33640(a2);
    v6 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v6 )
      goto LABEL_3;
  }
  else
  {
    v6 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v6 )
      goto LABEL_3;
  }
  obstack_newchunk(a1, 1);
  v6 = *(char **)(a1 + 24);
LABEL_3:
  *(long long *)(a1 + 24) = v6 + 1;
  *v6 = 82;
  v7 = strlen(v3);
  v8 = *(void **)(a1 + 24);
  v9 = v7 + 1;
  if ( v7 + 1 > *(long long *)(a1 + 32) - (long long)v8 )
  {
    obstack_newchunk(a1, v9);
    v8 = *(void **)(a1 + 24);
  }
  memcpy(v8, v3, v9);
  v10 = (char *)(v9 + *(long long *)(a1 + 24));
  *(long long *)(a1 + 24) = v10;
  if ( *a3 )
  {
    a3 = (char *)sub_33640(a3);
    v10 = *(char **)(a1 + 24);
    if ( *(char **)(a1 + 32) != v10 )
      goto LABEL_7;
  }
  else if ( *(char **)(a1 + 32) != v10 )
  {
    goto LABEL_7;
  }
  obstack_newchunk(a1, 1);
  v10 = *(char **)(a1 + 24);
LABEL_7:
  *(long long *)(a1 + 24) = v10 + 1;
  *v10 = 84;
  v11 = strlen(a3);
  v12 = *(void **)(a1 + 24);
  v13 = v11 + 1;
  if ( v11 + 1 > *(long long *)(a1 + 32) - (long long)v12 )
  {
    obstack_newchunk(a1, v13);
    v12 = *(void **)(a1 + 24);
  }
  result = memcpy(v12, a3, v13);
  *(long long *)(a1 + 24) += v13;
  return result;
}



// Function: read_obstack @ 0x1e730
long long read_obstack(_IO_FILE *a1, _IO_FILE *a2, long long *a3)
{
  char *IO_read_ptr; // rax
  unsigned int v5; // r12d
  long long i; // r14
  char *v7; // rax
  char *IO_read_base; // rdx
  char *v9; // rax

  IO_read_ptr = a1->_IO_read_ptr;
  if ( IO_read_ptr >= a1->_IO_read_end )
  {
    v5 = __uflow(a1);
  }
  else
  {
    a1->_IO_read_ptr = IO_read_ptr + 1;
    v5 = (unsigned char)*IO_read_ptr;
  }
  for ( i = 0; v5 + 1 > 1; ++i )
  {
    IO_read_base = a2->_IO_read_base;
    if ( a2->_IO_write_base == IO_read_base )
    {
      obstack_newchunk(a2, 1);
      IO_read_base = a2->_IO_read_base;
    }
    a2->_IO_read_base = IO_read_base + 1;
    *IO_read_base = v5;
    v7 = a1->_IO_read_ptr;
    if ( v7 >= a1->_IO_read_end )
    {
      v5 = __uflow(a1);
    }
    else
    {
      a1->_IO_read_ptr = v7 + 1;
      v5 = (unsigned char)*v7;
    }
  }
  v9 = a2->_IO_read_base;
  if ( a2->_IO_write_base == v9 )
  {
    obstack_newchunk(a2, 1);
    v9 = a2->_IO_read_base;
  }
  a2->_IO_read_base = v9 + 1;
  *v9 = 0;
  *a3 = i;
  return v5;
}



// Function: read_num @ 0x1e820
char read_num(FILE *stream, long long a2, long long a3, long long a4, long long *a5)
{
  char *IO_read_ptr; // rax
  int v8; // ecx
  bool v9; // r14
  long long i; // rbx
  long long v11; // rdi
  char *v12; // rax
  char result; // al
  __off_t v14; // rax
  long long v15; // rbx
  long long v16; // r12
  char *v17; // rdx
  int v18; // r14d
  __off_t v19; // rax
  long long v20; // rbx
  long long v21; // r12
  char *v22; // rax
  char *v23; // rbx
  const char *v24; // rax
  long long v25; // r14
  __off_t v26; // rax
  long long v27; // rbx
  long long v28; // r12
  char *v29; // rax
  long long v30; // rcx
  __off_t v31; // rax
  char *v32; // [rsp-10h] [rbp-D8h]
  char *v33; // [rsp-8h] [rbp-D0h]
  long long v35; // [rsp+0h] [rbp-C8h]
  int v36; // [rsp+0h] [rbp-C8h]
  char nptr[32]; // [rsp+10h] [rbp-B8h] BYREF
  char v39[32]; // [rsp+30h] [rbp-98h] BYREF
  char v40[32]; // [rsp+50h] [rbp-78h] BYREF
  char v41[24]; // [rsp+70h] [rbp-58h] BYREF
  unsigned long long v42; // [rsp+88h] [rbp-40h]

  v42 = __readfsqword(0x28u);
  IO_read_ptr = stream->_IO_read_ptr;
  if ( IO_read_ptr >= stream->_IO_read_end )
  {
    v8 = __uflow(stream);
  }
  else
  {
    stream->_IO_read_ptr = IO_read_ptr + 1;
    v8 = (unsigned char)*IO_read_ptr;
  }
  v9 = v8 == 45;
  for ( i = 0; ; ++i )
  {
    result = v9 && (int)i == 0;
    if ( result )
    {
      nptr[0] = v8;
      v12 = stream->_IO_read_ptr;
      if ( v12 < stream->_IO_read_end )
        goto LABEL_7;
      goto LABEL_11;
    }
    v11 = (unsigned int)(v8 - 48);
    if ( (unsigned int)v11 > 9 )
      break;
    nptr[i] = v8;
    if ( i == 20 )
    {
      if ( qword_82D50 )
        qword_82D50(v11);
      v14 = ftello(stream);
      v15 = sub_452D0(v14, v39);
      v16 = sub_4CE40(qword_82B28);
      v33 = nptr;
      v32 = (char *)(&dword_14 + 1);
      v17 = dcgettext(0, "%s: byte %s: %s %.*s... too long", 5);
      goto LABEL_15;
    }
    v12 = stream->_IO_read_ptr;
    if ( v12 < stream->_IO_read_end )
    {
LABEL_7:
      stream->_IO_read_ptr = v12 + 1;
      v8 = (unsigned char)*v12;
      continue;
    }
LABEL_11:
    v8 = __uflow(stream);
  }
  nptr[(int)i] = 0;
  if ( v8 < 0 )
  {
    if ( (stream->_flags & 0x20) != 0 )
      sub_33090(qword_82B28);
    if ( (int)i )
    {
      if ( qword_82D50 )
        qword_82D50(v11);
      v23 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
      v24 = (const char *)sub_4CE40(qword_82B28);
      error(0, 0, "%s: %s", v24, v23);
      sub_16FD0();
    }
  }
  else
  {
    if ( v8 )
    {
      if ( qword_82D50 )
      {
        v36 = v8;
        qword_82D50(v11);
        v8 = v36;
      }
      LODWORD(a4) = v8;
      v31 = ftello(stream);
      v15 = sub_452D0(v31, v39);
      v16 = sub_4CE40(qword_82B28);
      v17 = dcgettext(0, "%s: byte %s: %s %s followed by invalid byte 0x%02x", 5);
      v33 = (char *)(unsigned int)a4;
      v32 = nptr;
LABEL_15:
      error(0, 0, v17, v16, v15, a2, v32, v33, a4);
      sub_16FD0();
    }
    *a5 = sub_255E0(nptr, 0);
    v18 = *__errno_location();
    if ( v18 )
    {
      if ( v18 != 34 )
      {
        if ( qword_82D50 )
          qword_82D50(nptr);
        v19 = ftello(stream);
        v20 = sub_452D0(v19, v39);
        v21 = sub_4CE40(qword_82B28);
        v22 = dcgettext(0, "%s: byte %s: %s %s", 5);
        error(0, v18, v22, v21, v20, a2, nptr, v22, a4);
        sub_16FD0();
      }
      if ( qword_82D50 )
        qword_82D50(nptr);
      v35 = sub_45370(a4, v41);
      v25 = sub_45230(a3, v40);
      v26 = ftello(stream);
      v27 = sub_452D0(v26, v39);
      v28 = sub_4CE40(qword_82B28);
      v29 = dcgettext(0, "%s: byte %s: (valid range %s..%s)\n\t%s %s", 5);
      error(0, 34, v29, v28, v27, v25, v35, a2, nptr, v30);
      sub_16FD0();
    }
    return 1;
  }
  return result;
}



// Function: read_timespec @ 0x1ebb0
unsigned long long read_timespec(FILE *stream, long long *a2)
{
  char *v3; // rbx
  const char *v4; // rax
  long long v5; // [rsp+8h] [rbp-30h] BYREF
  long long v6; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-20h]

  v7 = __readfsqword(0x28u);
  if ( !sub_1E820(stream, (long long)"sec", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, &v5)
    || !sub_1E820(stream, (long long)"nsec", 0, 999999999, &v6) )
  {
    if ( qword_82D50 )
      qword_82D50(stream);
    v3 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
    v4 = (const char *)sub_4CE40(qword_82B28);
    error(0, 0, "%s: %s", v4, v3);
    sub_16FD0();
  }
  *a2 = v5;
  a2[1] = v6;
  return v7 - __readfsqword(0x28u);
}



// Function: dumpdir_locate @ 0x1ec90
const char *dumpdir_locate(long long a1, const char *a2)
{
  unsigned long long v2; // rbp
  unsigned long long v3; // r12
  long long v4; // r14
  unsigned long long v5; // rbx
  const char *v6; // r15
  int v7; // eax

  if ( !a1 )
    return 0;
  v2 = 0;
  v3 = *(long long *)(a1 + 16);
  v4 = *(long long *)(a1 + 24);
LABEL_3:
  if ( v3 <= v2 )
    return 0;
  while ( 1 )
  {
    v5 = (v3 + v2) >> 1;
    v6 = *(const char **)(v4 + 8 * v5);
    v7 = strcmp(a2, v6);
    if ( v7 < 0 )
    {
      v3 = (v3 + v2) >> 1;
      goto LABEL_3;
    }
    if ( !v7 )
      return v6 - 1;
    v2 = v5 + 1;
    if ( v3 <= v5 + 1 )
      return 0;
  }
}



// Function: find_directory_meta @ 0x1ed10
long long find_directory_meta(void *a1, void *a2)
{
  long long v2; // r12
  void **v4; // rax
  long long v5; // rdi
  void **v6; // rbp

  v2 = qword_823A8;
  if ( qword_823A8 )
  {
    v4 = (void **)sub_1E190("", 0);
    v5 = qword_823A8;
    v4[3] = a1;
    v6 = v4;
    v4[4] = a2;
    v2 = sub_43960(v5, v4);
    free(v6[10]);
    free(v6[11]);
    free(v6);
  }
  return v2;
}



// Function: procdir @ 0x1ed80
long long *procdir(char *src, long long a2, char a3, char *a4)
{
  char *v5; // rbp
  long long v7; // rax
  long long *v8; // r13
  void *v9; // rsi
  long long v10; // rax
  long long v11; // r15
  char *v12; // r14
  long long v13; // r14
  char *v14; // rax
  size_t v15; // rax
  long long *v16; // r15
  size_t v17; // r8
  char v18; // dl
  int v19; // eax
  long long v21; // rax
  long long v22; // r14
  char *v23; // rax
  void **v24; // r14
  int v25; // eax
  int v26; // eax
  char *v27; // rax
  long long v28; // rax
  int v29; // ebx
  long long v30; // r14
  char *v31; // rax
  long long v32; // r15
  long long *v33; // rax
  char *v34; // r14
  long long v35; // r14
  char *v36; // rax
  size_t v37; // rax
  long long *v38; // r15
  size_t v39; // r8
  long long v40; // rax
  long long v41; // r14
  char *v42; // rax
  unsigned int v43; // eax
  char *v44; // rax
  char *v45; // rax
  long long v46; // [rsp+0h] [rbp-68h]
  size_t v47; // [rsp+0h] [rbp-68h]
  long long v48; // [rsp+0h] [rbp-68h]
  size_t v49; // [rsp+0h] [rbp-68h]
  long long v50; // [rsp+8h] [rbp-60h]
  size_t v51; // [rsp+8h] [rbp-60h]
  size_t v53; // [rsp+18h] [rbp-50h]
  long long v54[9]; // [rsp+20h] [rbp-48h] BYREF

  v5 = src;
  v54[1] = __readfsqword(0x28u);
  v50 = *(long long *)(a2 + 88);
  v7 = sub_1E240(src);
  if ( !v7 )
  {
    v32 = sub_1ED10(*(void **)(a2 + 88), *(void **)(a2 + 96));
    v33 = sub_1E440(
            src,
            *(long long *)(a2 + 176),
            *(long long *)(a2 + 184),
            *(long long *)(a2 + 88),
            *(long long *)(a2 + 96),
            v50 < 0,
            1,
            0);
    v8 = v33;
    if ( v32 )
    {
      v34 = *(char **)(v32 + 88);
      src = v34;
      if ( strcmp(v34, v5) )
      {
        if ( (dword_81B80 & 0x2000) != 0 )
        {
          if ( qword_82D50 )
          {
            qword_82D50(v34);
            v34 = *(char **)(v32 + 88);
          }
          v48 = sub_4D2B0(1, v34);
          v35 = sub_4CE40(v5);
          v36 = dcgettext(0, "%s: Directory has been renamed from %s", 5);
          error(0, 0, v36, v35, v48);
          v34 = *(char **)(v32 + 88);
        }
        v8[8] = v32;
        *((int *)v8 + 15) |= 0x10u;
        *(int *)(v32 + 60) &= ~0x10u;
        src = v5;
        v51 = strlen(v34);
        v37 = strlen(v5);
        v38 = qword_823C0;
        v39 = v37;
        if ( qword_823C0 )
        {
          do
          {
            src = (char *)(v38 + 11);
            v49 = v39;
            sub_25500(v38 + 11, v34, v51, v5);
            v38 = (long long *)*v38;
            v39 = v49;
          }
          while ( v38 );
        }
      }
      *((int *)v8 + 14) = 1;
      v18 = 0;
    }
    else
    {
      *((int *)v33 + 15) |= 8u;
      if ( (dword_81B80 & 0x1000) != 0 )
      {
        if ( qword_82D50 )
          qword_82D50(src);
        v41 = sub_4CE40(src);
        v42 = dcgettext(0, "%s: Directory is new", 5);
        src = 0;
        error(0, 0, v42, v41);
      }
      v43 = 2;
      if ( !qword_82B28 )
      {
        src = (char *)xmmword_82B00;
        if ( (*((long long *)&xmmword_82B00 + 1) < *(long long *)(a2 + 184))
           - (*((long long *)&xmmword_82B00 + 1) > *(long long *)(a2 + 184))
           + 2 * ((*(long long *)(a2 + 176) > (long long)xmmword_82B00) - (*(long long *)(a2 + 176) < (long long)xmmword_82B00)) >= 0 )
        {
          v43 = 1;
          if ( dword_82B8C )
            v43 = (((*((long long *)&xmmword_82B00 + 1) < *(long long *)(a2 + 200))
                  - (*((long long *)&xmmword_82B00 + 1) > *(long long *)(a2 + 200))
                  + 2
                  * ((*(long long *)(a2 + 192) > (long long)xmmword_82B00)
                   - (unsigned int)(*(long long *)(a2 + 192) < (long long)xmmword_82B00))) >> 31)
                + 1;
        }
      }
      *((int *)v8 + 14) = v43;
      v18 = 0;
    }
    goto LABEL_28;
  }
  v8 = (long long *)v7;
  if ( (*(char *)(v7 + 60) & 1) != 0 )
  {
    if ( (a3 & 0x20) == 0 )
    {
LABEL_24:
      *a4 = 78;
      return v8;
    }
    sub_25120(v7 + 88, src);
  }
  src = (char *)v8[11];
  if ( strcmp(src, v5) )
    goto LABEL_24;
  v9 = *(void **)(a2 + 96);
  if ( byte_82B20 && ((*((char *)v8 + 60) & 2) == 0 || v50 >= 0) )
  {
    src = *(char **)(a2 + 88);
    if ( (char *)v8[3] != src )
      goto LABEL_12;
  }
  if ( (void *)v8[4] != v9 )
  {
    src = *(char **)(a2 + 88);
LABEL_12:
    v10 = sub_1ED10(src, v9);
    v11 = v10;
    if ( v10 )
    {
      v12 = *(char **)(v10 + 88);
      src = v12;
      if ( strcmp(v12, v5) )
      {
        if ( (dword_81B80 & 0x2000) != 0 )
        {
          if ( qword_82D50 )
          {
            qword_82D50(v12);
            v12 = *(char **)(v11 + 88);
          }
          v46 = sub_4D2B0(1, v12);
          v13 = sub_4CE40(v5);
          v14 = dcgettext(0, "%s: Directory has been renamed from %s", 5);
          error(0, 0, v14, v13, v46);
          v12 = *(char **)(v11 + 88);
        }
        v8[8] = v11;
        *((int *)v8 + 15) |= 0x10u;
        *(int *)(v11 + 60) &= ~0x10u;
        src = v5;
        v53 = strlen(v12);
        v15 = strlen(v5);
        v16 = qword_823C0;
        v17 = v15;
        if ( qword_823C0 )
        {
          do
          {
            src = (char *)(v16 + 11);
            v47 = v17;
            sub_25500(v16 + 11, v12, v53, v5);
            v16 = (long long *)*v16;
            v17 = v47;
          }
          while ( v16 );
        }
      }
      *((int *)v8 + 14) = 1;
      v18 = 0;
    }
    else
    {
      v40 = *(long long *)(a2 + 88);
      *((int *)v8 + 14) = 2;
      v18 = 1;
      v8[3] = v40;
      v8[4] = *(long long *)(a2 + 96);
    }
    v19 = *((int *)v8 + 15);
    if ( v50 < 0 )
      v19 = *((int *)v8 + 15) | 2;
    goto LABEL_27;
  }
  *((int *)v8 + 14) = 1;
  v19 = *((int *)v8 + 15);
  v18 = 0;
LABEL_27:
  *((int *)v8 + 15) = v19 | 4;
LABEL_28:
  if ( byte_82AD1 && (v21 = *(long long *)(a2 + 408)) != 0 && *(long long *)(a2 + 88) != *(long long *)(v21 + 88) )
  {
    if ( (dword_81B80 & 0x40000) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(src);
      v22 = sub_4CE40(v8[11]);
      v23 = dcgettext(0, "%s: directory is on a different filesystem; not dumped", 5);
      error(0, 0, v23, v22);
    }
    v24 = (void **)v8[5];
    *((int *)v8 + 14) = 0;
    if ( !v24 )
    {
      *((int *)v8 + 15) |= 1u;
      return v8;
    }
    free(v24[3]);
    free(v24);
    v8[5] = 0;
    v25 = *((int *)v8 + 14);
  }
  else
  {
    if ( (a3 & 0x10) != 0 )
    {
      v29 = a3 & 3;
      *((int *)v8 + 14) = v29;
      if ( !v29 )
        *a4 = 78;
    }
    if ( v18 && (dword_81B80 & 0x2000) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(src);
      v30 = sub_4CE40(v5);
      v31 = dcgettext(0, "%s: Directory has been renamed", 5);
      error(0, 0, v31, v30);
    }
    v25 = *((int *)v8 + 14);
  }
  *((int *)v8 + 15) |= 1u;
  if ( v25 )
  {
    v26 = sub_149E0(a2, v54);
    switch ( v26 )
    {
      case 2:
        v44 = dcgettext(0, "contents not dumped", 5);
        sub_12E70(v5, v54[0], v44);
        v8[9] = v54[0];
        break;
      case 3:
        v45 = dcgettext(0, "directory not dumped", 5);
        sub_12E70(v5, v54[0], v45);
        *a4 = 78;
        *((int *)v8 + 14) = 0;
        break;
      case 1:
        v27 = dcgettext(0, "contents not dumped", 5);
        sub_12E70(v5, v54[0], v27);
        v28 = v54[0];
        *((int *)v8 + 14) = 0;
        v8[9] = v28;
        break;
    }
  }
  return v8;
}



// Function: write_directory_file_entry @ 0x1f3f0
long long write_directory_file_entry(long long a1, _IO_FILE *a2)
{
  int v3; // eax
  const char *v4; // r12
  unsigned char v5; // si
  const char *v6; // r13
  char *IO_write_ptr; // rax
  const char *v8; // r13
  size_t v9; // rax
  const char *v10; // r13
  size_t v11; // rax
  const char *v12; // r13
  size_t v13; // rax
  const char *v14; // r12
  size_t v15; // rax
  const char *v16; // r12
  size_t v17; // rax
  long long v18; // rbp
  long long v19; // rax
  bool v20; // zf
  void *v21; // r12
  long long v22; // rbp
  const char *v23; // rbp
  size_t v24; // rsi
  char *v25; // rax
  char *v26; // rax
  long long v28[9]; // [rsp+0h] [rbp-48h] BYREF

  v28[3] = __readfsqword(0x28u);
  if ( (*(int *)(a1 + 60) & 4) == 0 )
    return ((unsigned char)((unsigned int)a2->_flags >> 5) ^ 1) & 1;
  v3 = *(int *)(a1 + 60) & 2;
  v4 = "1";
  v5 = 49 - (v3 == 0);
  if ( !v3 )
    v4 = "0";
  v6 = v4 + 1;
  while ( 1 )
  {
    IO_write_ptr = a2->_IO_write_ptr;
    if ( IO_write_ptr < a2->_IO_write_end )
    {
      a2->_IO_write_ptr = IO_write_ptr + 1;
      *IO_write_ptr = v5;
      goto LABEL_7;
    }
    if ( __overflow(a2, v5) == -1 )
      break;
LABEL_7:
    if ( v6 == v4 )
      break;
    v5 = *++v4;
  }
  v8 = (const char *)sub_255C0(*(long long *)(a1 + 8), 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFLL, v28);
  v9 = strlen(v8);
  fwrite_unlocked(v8, v9 + 1, 1u, a2);
  v10 = (const char *)sub_45230(*(long long *)(a1 + 16), v28);
  v11 = strlen(v10);
  fwrite_unlocked(v10, v11 + 1, 1u, a2);
  v12 = (const char *)sub_255C0(*(long long *)(a1 + 24), 0, -1, v28);
  v13 = strlen(v12);
  fwrite_unlocked(v12, v13 + 1, 1u, a2);
  v14 = (const char *)sub_255C0(*(long long *)(a1 + 32), 0, -1, v28);
  v15 = strlen(v14);
  fwrite_unlocked(v14, v15 + 1, 1u, a2);
  v16 = *(const char **)(a1 + 88);
  v17 = strlen(v16);
  fwrite_unlocked(v16, v17 + 1, 1u, a2);
  v18 = *(long long *)(a1 + 40);
  if ( v18 )
  {
    v19 = sub_4ECC0(24);
    v20 = *(long long *)(v18 + 16) == 0;
    *(long long *)v19 = v18;
    v21 = (void *)v19;
    *(int *)(v19 + 8) = 0;
    *(long long *)(v19 + 16) = 0;
    if ( !v20 )
    {
      v22 = **(long long **)(v18 + 24);
      *(long long *)(v19 + 16) = 1;
      v23 = (const char *)(v22 - 1);
      do
      {
        v24 = strlen(v23) + 1;
        fwrite_unlocked(v23, v24, 1u, a2);
        v23 = (const char *)sub_1E120(v21);
      }
      while ( v23 );
    }
    free(v21);
  }
  v25 = a2->_IO_write_ptr;
  if ( a2->_IO_write_end <= v25 )
  {
    if ( __overflow(a2, 0) != -1 )
      goto LABEL_15;
  }
  else
  {
    a2->_IO_write_ptr = v25 + 1;
    *v25 = 0;
LABEL_15:
    v26 = a2->_IO_write_ptr;
    if ( v26 >= a2->_IO_write_end )
    {
      __overflow(a2, 0);
    }
    else
    {
      a2->_IO_write_ptr = v26 + 1;
      *v26 = 0;
    }
  }
  return ((unsigned char)((unsigned int)a2->_flags >> 5) ^ 1) & 1;
}



// Function: clear_directory_table @ 0x1f6e0
void clear_directory_table()
{
  void **v0; // rbx
  void **v1; // rbp

  if ( qword_823B0 )
    sub_43CF0();
  if ( qword_823A8 )
    sub_43CF0();
  v0 = (void **)qword_823C0;
  if ( qword_823C0 )
  {
    do
    {
      v1 = v0;
      v0 = (void **)*v0;
      free(v1[10]);
      free(v1[11]);
      free(v1);
    }
    while ( v0 );
  }
  qword_823B8 = 0;
  qword_823C0 = 0;
}



// Function: rebase_directory @ 0x1f770
long long rebase_directory(long long a1, long long a2, long long a3, long long a4)
{
  return sub_25500(a1 + 88, a2, a3, a4);
}



// Function: update_parent_directory @ 0x1f780
unsigned long long update_parent_directory(long long a1)
{
  const char *v2; // rdi
  long long v3; // rax
  long long *v4; // rbx
  __time_t tv_sec; // rdx
  struct stat v7; // [rsp+0h] [rbp-B8h] BYREF
  unsigned long long v8; // [rsp+98h] [rbp-20h]

  v2 = *(const char **)a1;
  v8 = __readfsqword(0x28u);
  v3 = sub_1E240(v2);
  if ( v3 )
  {
    v4 = (long long *)v3;
    if ( fstat(*(int *)(a1 + 424), &v7) )
    {
      sub_26460(v4[11]);
    }
    else
    {
      tv_sec = v7.st_mtim.tv_sec;
      v4[2] = v7.st_mtim.tv_nsec;
      v4[1] = tv_sec;
    }
  }
  return v8 - __readfsqword(0x28u);
}



// Function: scan_directory @ 0x1f810
long long *scan_directory(long long a1)
{
  void *v1; // r15
  long long v2; // rax
  long long v3; // rbx
  char *v4; // r14
  char v5; // dl
  long long *v6; // r12
  void *v7; // r14
  int v8; // eax
  const char *v9; // rdi
  size_t v10; // rbx
  char *v11; // rax
  char *v12; // rbp
  long long v14; // rax
  const char *v15; // r15
  size_t v16; // rbx
  long long v17; // r13
  size_t v18; // rax
  const char **v19; // r15
  const char *v20; // r13
  long long v21; // r13
  char *v22; // r15
  const char *v23; // rax
  const char *v24; // rcx
  int v25; // eax
  char v26; // al
  char **v27; // r15
  long long v28; // rax
  char *v29; // r13
  void *v30; // rbx
  char *v31; // rax
  char *v32; // r9
  int *v33; // rax
  long long (*v34)(); // rdx
  char *v35; // r9
  struct stat *p_buf; // rsi
  char *v37; // r14
  char v38; // dl
  int v39; // eax
  int v40; // eax
  int v41; // eax
  char v42; // dl
  int fd[2]; // [rsp+0h] [rbp-238h]
  const char *fdc; // [rsp+0h] [rbp-238h]
  int fda[2]; // [rsp+0h] [rbp-238h]
  int fdb; // [rsp+0h] [rbp-238h]
  int fdd[2]; // [rsp+0h] [rbp-238h]
  void *base; // [rsp+8h] [rbp-230h]
  char *ptr; // [rsp+10h] [rbp-228h]
  struct stat *v50; // [rsp+18h] [rbp-220h]
  char *v51; // [rsp+18h] [rbp-220h]
  int v52; // [rsp+18h] [rbp-220h]
  long long v53; // [rsp+20h] [rbp-218h]
  char *s; // [rsp+28h] [rbp-210h]
  char v55; // [rsp+3Fh] [rbp-1F9h] BYREF
  char v56[88]; // [rsp+40h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+98h] [rbp-1A0h] BYREF
  long long v58; // [rsp+1D8h] [rbp-60h]
  int v59; // [rsp+1E8h] [rbp-50h]
  unsigned long long v60; // [rsp+1F8h] [rbp-40h]

  v1 = *(void **)a1;
  v60 = __readfsqword(0x28u);
  v2 = sub_14860();
  v3 = *(long long *)(a1 + 408);
  ptr = (char *)v2;
  v53 = *(long long *)(a1 + 88);
  if ( v2 )
  {
    sub_17440(a1);
    v4 = (char *)sub_4EE30(v1);
    sub_253A0(v4);
    v5 = 0;
    if ( !v3 )
      v5 = 32;
    v6 = sub_1ED80(v4, a1, v5, &v55);
    free(v4);
    v7 = (void *)sub_265F0((char *)v1);
    v8 = *((int *)v6 + 14);
    if ( !v8 )
    {
      v9 = (const char *)v6[9];
      if ( v9 )
      {
        v10 = strlen(v9);
        v11 = (char *)sub_4ECC0(v10 + 3);
        *v11 = 89;
        v12 = v11;
        memcpy(v11 + 1, (const void *)v6[9], v10 + 1);
        v12[v10 + 2] = 0;
        v6[6] = v6[5];
        v6[5] = sub_1E2E0(v12, 0);
        free(v12);
      }
      goto LABEL_7;
    }
    v50 = 0;
    if ( v8 != 2 )
    {
      v14 = v6[8];
      if ( v14 )
      {
        v50 = *(struct stat **)(v14 + 48);
        if ( !v50 )
          v50 = *(struct stat **)(v14 + 40);
      }
      else
      {
        v50 = (struct stat *)v6[5];
      }
    }
    if ( *ptr )
    {
      v15 = ptr;
      v16 = 0;
      v17 = 0;
      do
      {
        ++v16;
        v18 = strlen(v15);
        v15 += v18 + 1;
        v17 += v18 + 2;
      }
      while ( *v15 );
      *(long long *)fd = v17 + 1;
      base = (void *)sub_4EDB0(v16, 8);
      if ( !*ptr )
      {
LABEL_18:
        v21 = 0;
        qsort(base, v16, 8u, compar);
        s = (char *)sub_4ECC0(*(long long *)fd);
        v22 = s;
        if ( v16 )
        {
          do
          {
            fdc = (const char *)*((long long *)base + v21);
            v23 = sub_1EC90((long long)v50, fdc);
            v24 = fdc;
            if ( v23 )
              v25 = v6[9] == 0 ? 0xFFFFFFD7 : 0;
            else
              v25 = v6[9] == 0 ? 0x10 : 0;
            *v22++ = v25 + 73;
            do
            {
              v26 = *v24++;
              *v22++ = v26;
            }
            while ( v26 );
            ++v21;
          }
          while ( v16 != v21 );
        }
        goto LABEL_24;
      }
    }
    else
    {
      v16 = 0;
      *(long long *)fd = 1;
      base = (void *)sub_4EDB0(0, 8);
      if ( !*ptr )
      {
        qsort(base, 0, 8u, compar);
        s = (char *)sub_4ECC0(1);
        v22 = s;
LABEL_24:
        *v22 = 0;
        v6[6] = v6[5];
        v6[5] = sub_1E2E0(s, 0);
        free(s);
        free(base);
        v27 = (char **)v6[5];
        v28 = sub_4ECC0(24);
        v29 = *v27;
        *(long long *)v28 = v27;
        v30 = (void *)v28;
        *(int *)(v28 + 8) = 1;
        *(long long *)(v28 + 16) = 0;
        if ( !*v29 )
        {
LABEL_33:
          free(v30);
          goto LABEL_7;
        }
        *(long long *)(v28 + 16) = strlen(v29) + 1;
        while ( 1 )
        {
          while ( 1 )
          {
            v31 = (char *)sub_26680(v7, v29 + 1);
            if ( *v29 != 73 )
            {
              *(long long *)fda = v31;
              if ( !(unsigned char)sub_176D0(v31, a1) )
                break;
            }
            *v29 = 78;
            v29 = (char *)sub_1E120(v30);
            if ( !v29 )
              goto LABEL_33;
          }
          v51 = *(char **)fda;
          fdb = *(int *)(a1 + 424);
          sub_2FB40(v56);
          v32 = v51;
          if ( fdb < 0 )
            break;
          p_buf = (struct stat *)(v29 + 1);
          v39 = fstatat(fdb, v29 + 1, &buf, flag);
          v35 = v51;
          if ( v39 )
            goto LABEL_39;
          if ( (buf.st_mode & 0xF000) == 0x4000 )
          {
            v40 = sub_14950(a1, (long long)(v29 + 1), dword_82808);
            v35 = v51;
            if ( v40 < 0 )
            {
              v34 = sub_26370;
              goto LABEL_31;
            }
            p_buf = &buf;
            v59 = v40;
            v41 = fstat(v40, &buf);
            v35 = v51;
            v42 = v41;
            if ( v41 )
            {
LABEL_39:
              v34 = (long long (*)())sub_26460;
              goto LABEL_31;
            }
            if ( (buf.st_mode & 0xF000) == 0x4000 )
            {
              if ( dword_82AD4 )
              {
                if ( *((int *)v6 + 14) == 2 )
                  v42 = 18;
              }
              else
              {
                v42 = 16;
              }
              *v29 = 68;
              v58 = a1;
              p_buf = (struct stat *)v56;
              sub_1ED80(v51, (long long)v56, v42, v29);
              sub_14A80((long long)v56);
              goto LABEL_32;
            }
          }
          if ( !byte_82AD1 || buf.st_dev == v53 )
          {
            if ( *v29 == 89 )
              goto LABEL_32;
            p_buf = (struct stat *)xmmword_82B00;
            if ( (buf.st_mtim.tv_nsec > *((long long *)&xmmword_82B00 + 1))
               - (buf.st_mtim.tv_nsec < *((long long *)&xmmword_82B00 + 1))
               + 2 * (((long long)xmmword_82B00 < buf.st_mtim.tv_sec) - ((long long)xmmword_82B00 > buf.st_mtim.tv_sec)) >= 0
              || dword_82B8C
              && (*((long long *)&xmmword_82B00 + 1) < buf.st_ctim.tv_nsec)
               - (*((long long *)&xmmword_82B00 + 1) > buf.st_ctim.tv_nsec)
               + 2 * (((long long)xmmword_82B00 < buf.st_ctim.tv_sec) - ((long long)xmmword_82B00 > buf.st_ctim.tv_sec)) >= 0 )
            {
              *v29 = 89;
              goto LABEL_32;
            }
          }
          *v29 = 78;
LABEL_32:
          sub_2FC00(v56, p_buf);
          v29 = (char *)sub_1E120(v30);
          if ( !v29 )
            goto LABEL_33;
        }
        v52 = fdb;
        *(long long *)fdd = v32;
        v33 = __errno_location();
        v34 = sub_26370;
        v35 = *(char **)fdd;
        *v33 = -v52;
LABEL_31:
        p_buf = 0;
        sub_26490(v35, 0, v34);
        *v29 = 78;
        goto LABEL_32;
      }
    }
    v19 = (const char **)base;
    v20 = ptr;
    do
    {
      *v19++ = v20;
      v20 += strlen(v20) + 1;
    }
    while ( *v20 );
    goto LABEL_18;
  }
  sub_331A0(v1);
  sub_17440(a1);
  v37 = (char *)sub_4EE30(v1);
  sub_253A0(v37);
  v38 = 0;
  if ( !v3 )
    v38 = 32;
  v6 = sub_1ED80(v37, a1, v38, &v55);
  free(v37);
  v7 = (void *)sub_265F0((char *)v1);
LABEL_7:
  sub_26660(v7);
  free(ptr);
  return v6;
}



// Function: safe_directory_contents @ 0x1ff00
void *safe_directory_contents(long long a1)
{
  void *result; // rax
  void **v2; // rdx

  result = &unk_64EEC;
  if ( a1 )
  {
    v2 = *(void ***)(a1 + 40);
    if ( v2 )
    {
      result = *v2;
      if ( !*v2 )
        return &unk_64EEC;
    }
  }
  return result;
}



// Function: append_incremental_renames @ 0x1ff30
unsigned long long append_incremental_renames(long long a1)
{
  const char **v1; // rax
  const char *v2; // r14
  size_t v3; // r12
  char *next_free; // r15
  const char *v5; // rbx
  size_t v6; // rax
  long long *v7; // rbx
  long long *v8; // rax
  char **v9; // r13
  long long *v10; // r15
  long long v11; // rax
  char *v12; // rax
  void **v13; // r12
  char *object_base; // rdi
  char *chunk_limit; // rdx
  char *v17; // r15
  char *v18; // rax
  size_t v19; // rax
  char *v20; // rdi
  size_t v21; // r14
  char *v22; // rsi
  long long *v23; // r14
  struct obstack obstack; // [rsp+10h] [rbp-98h] BYREF
  unsigned long long v26; // [rsp+68h] [rbp-40h]

  v26 = __readfsqword(0x28u);
  if ( qword_823C0 )
  {
    obstack_begin(&obstack, 0, 0, sub_4ECC0, &free);
    if ( a1 )
    {
      v1 = *(const char ***)(a1 + 40);
      if ( v1 )
      {
        v2 = *v1;
        v3 = 0;
        if ( *v1 )
        {
          next_free = obstack.next_free;
          if ( *v2 )
          {
            v5 = *v1;
            do
            {
              v6 = strlen(v5) + 1;
              v5 += v6;
              v3 += v6;
            }
            while ( *v5 );
            if ( obstack.chunk_limit - next_free < v3 )
            {
              obstack_newchunk(&obstack, v3);
              next_free = obstack.next_free;
            }
          }
          memcpy(next_free, v2, v3);
          obstack.next_free += v3;
        }
      }
      else
      {
        v3 = 0;
      }
      v7 = qword_823C0;
      if ( qword_823C0 )
      {
        do
        {
LABEL_14:
          while ( (*((char *)v7 + 60) & 0x10) == 0 )
          {
LABEL_13:
            v7 = (long long *)*v7;
            if ( !v7 )
              goto LABEL_22;
          }
          v8 = v7;
          do
          {
            v9 = (char **)v8;
            v8 = (long long *)v8[8];
            if ( v8 == v7 )
            {
              v17 = (char *)sub_40230(v7[11]);
              v18 = obstack.next_free;
              if ( obstack.chunk_limit == obstack.next_free )
              {
                obstack_newchunk(&obstack, 1);
                v18 = obstack.next_free;
              }
              obstack.next_free = v18 + 1;
              *v18 = 88;
              v19 = strlen(v17);
              v20 = obstack.next_free;
              v21 = v19 + 1;
              if ( v19 + 1 > obstack.chunk_limit - obstack.next_free )
              {
                obstack_newchunk(&obstack, v21);
                v20 = obstack.next_free;
              }
              memcpy(v20, v17, v21);
              v22 = (char *)v7[11];
              obstack.next_free += v21;
              v23 = v7;
              sub_1E5C0((long long)&obstack, v22, "");
              if ( v7 != v9 )
              {
                do
                {
                  sub_1E5C0((long long)&obstack, *(char **)(v23[8] + 88LL), (char *)v23[11]);
                  v23 = (long long *)v23[8];
                }
                while ( v23 != v9 );
              }
              sub_1E5C0((long long)&obstack, "", v9[11]);
              free(v17);
              goto LABEL_13;
            }
          }
          while ( v8 );
          v10 = v7;
          while ( 1 )
          {
            v11 = v10[8];
            if ( !v11 )
              break;
            sub_1E5C0((long long)&obstack, *(char **)(v11 + 88), (char *)v10[11]);
            v10 = (long long *)v10[8];
            if ( !v10 )
              goto LABEL_13;
          }
          v7 = (long long *)*v7;
        }
        while ( v7 );
LABEL_22:
        if ( !a1 )
          goto LABEL_31;
      }
      v12 = obstack.next_free;
      if ( obstack.next_free - obstack.object_base != v3 )
      {
        if ( obstack.next_free == obstack.chunk_limit )
        {
          obstack_newchunk(&obstack, 1);
          v12 = obstack.next_free;
        }
        obstack.next_free = v12 + 1;
        *v12 = 0;
        v13 = *(void ***)(a1 + 40);
        free(v13[3]);
        free(v13);
        object_base = obstack.object_base;
        if ( obstack.next_free == obstack.object_base )
          *((char *)&obstack + 80) |= 2u;
        chunk_limit = obstack.chunk_limit;
        if ( (~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask])
           - (unsigned long long)obstack.chunk <= obstack.chunk_limit - (char *)obstack.chunk )
          chunk_limit = (char *)(~*(long long *)&obstack.alignment_mask
                               & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask]);
        obstack.next_free = chunk_limit;
        obstack.object_base = chunk_limit;
        *(long long *)(a1 + 40) = sub_1E2E0(object_base, "YND");
      }
    }
    else
    {
      v7 = qword_823C0;
      v3 = 0;
      if ( qword_823C0 )
        goto LABEL_14;
    }
LABEL_31:
    obstack_free(&obstack, 0);
  }
  return v26 - __readfsqword(0x28u);
}



// Function: show_snapshot_field_ranges @ 0x202a0
unsigned long long show_snapshot_field_ranges()
{
  char **v0; // rbx
  char *v1; // rsi
  long long v2; // rdi
  const char *v3; // r14
  const char *v4; // rax
  long long v6; // [rsp+0h] [rbp-68h] BYREF
  char v7[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v8; // [rsp+38h] [rbp-30h]

  v0 = &off_7D660;
  v8 = __readfsqword(0x28u);
  __printf_chk(1, "This tar's snapshot file field ranges are\n");
  __printf_chk(1, "   (%-15s => [ %s, %s ]):\n\n", "field name", "min", "max");
  v1 = 0;
  v2 = 1;
  while ( 1 )
  {
    v0 += 3;
    v3 = (const char *)sub_255C0(v2, v1, v2, v7);
    v4 = (const char *)sub_255C0(*(v0 - 2), *(v0 - 2), *(v0 - 1), &v6);
    __printf_chk(1, "    %-15s => [ %s, %s ],\n", *(v0 - 3), v4, v3);
    if ( !*v0 )
      break;
    v2 = (long long)v0[2];
    v1 = v0[1];
  }
  __printf_chk(1, "\n");
  return v8 - __readfsqword(0x28u);
}



// Function: read_directory_file @ 0x203b0
unsigned long long read_directory_file()
{
  int v0; // eax
  int v1; // ebp
  char *v2; // rbp
  char *v3; // rdi
  char *v4; // rbx
  char *v5; // rax
  char i; // al
  char *v7; // rax
  uintmax_t v8; // rax
  FILE *v9; // rdi
  long long v10; // r14
  long long v11; // r15
  char *object_base; // r13
  char *chunk_limit; // rdx
  FILE *v14; // rdi
  char *IO_read_ptr; // rax
  int v16; // eax
  char *v17; // r10
  char *v18; // rdx
  char *v20; // rdi
  __int128 v21; // rax
  long long chunk_size; // rbp
  long long v23; // r13
  long long v24; // rax
  const char *v25; // rbp
  int v26; // eax
  char *v27; // rax
  char *v28; // r13
  uintmax_t v29; // r12
  long long v30; // r14
  int *v31; // rbp
  long long v32; // rdx
  const char *v33; // rdi
  int v34; // eax
  const char *v35; // rsi
  char *v36; // rbx
  const char *v37; // rax
  char *v38; // rax
  char *v39; // rbp
  size_t v40; // rax
  long long v41; // rbp
  __int128 v42; // rax
  int *v43; // rax
  const char *v44; // rbp
  int *v45; // r12
  uintmax_t v46; // rax
  char *v47; // rbp
  const char *v48; // rax
  char *v49; // rbx
  const char *v50; // rax
  char *v51; // rbp
  __off_t v52; // rax
  long long v53; // rbx
  long long v54; // r12
  char *v55; // rax
  char *v56; // rbx
  const char *v57; // r12
  int *v58; // rax
  bool v59; // [rsp+Ch] [rbp-10Ch]
  bool v60; // [rsp+Ch] [rbp-10Ch]
  struct _obstack_chunk *block; // [rsp+10h] [rbp-108h]
  void *blockb; // [rsp+10h] [rbp-108h]
  void *blocka; // [rsp+10h] [rbp-108h]
  uintmax_t v64; // [rsp+18h] [rbp-100h]
  long long v65; // [rsp+20h] [rbp-F8h]
  char *lineptr; // [rsp+30h] [rbp-E8h] BYREF
  size_t n; // [rsp+38h] [rbp-E0h] BYREF
  long long v68; // [rsp+40h] [rbp-D8h] BYREF
  void *ptr; // [rsp+48h] [rbp-D0h] BYREF
  size_t v70[2]; // [rsp+50h] [rbp-C8h] BYREF
  struct obstack endptr; // [rsp+60h] [rbp-B8h] BYREF
  char v72[24]; // [rsp+C0h] [rbp-58h] BYREF
  unsigned long long v73; // [rsp+D8h] [rbp-40h]

  v73 = __readfsqword(0x28u);
  lineptr = 0;
  n = 0;
  v0 = open(qword_82B28, dword_82B24 == 0 ? 578 : 66, 438);
  if ( v0 < 0 )
  {
    sub_32EA0(qword_82B28);
    return v73 - __readfsqword(0x28u);
  }
  v1 = v0;
  qword_823A0 = fdopen(v0, "r+");
  if ( !qword_823A0 )
  {
    sub_32EA0(qword_82B28);
    close(v1);
    return v73 - __readfsqword(0x28u);
  }
  sub_29260();
  sub_292E0();
  if ( __getdelim(&lineptr, &n, 10, qword_823A0) <= 0 )
    goto LABEL_40;
  v2 = lineptr;
  v3 = lineptr;
  if ( strncmp(lineptr, "GNU tar", 7u) )
  {
    ptr = 0;
    v70[0] = 0;
LABEL_79:
    v39 = strdup(v2);
    ptr = v39;
    v40 = strlen(v39);
    v20 = v39;
    v41 = 1;
    v70[0] = v40 + 1;
    *(long long *)&v42 = sub_25930(v20, (char **)&endptr);
    xmmword_82B00 = v42;
    if ( v42 >= 0 )
    {
      v64 = 0;
      v65 = 1;
      v26 = __getdelim((char **)&ptr, v70, 10, qword_823A0);
      goto LABEL_56;
    }
    goto LABEL_106;
  }
  v4 = v2 + 8;
  if ( v2[7] != 45 )
  {
    if ( qword_82D50 )
      qword_82D50(v3);
    v5 = dcgettext(0, "Bad incremental file format", 5);
    v3 = (char *)(&dword_0 + 1);
    error(1, 0, v5);
    dword_82D58 = 2;
  }
  for ( i = v2[8]; i != 45; ++v4 )
  {
    if ( !i )
    {
      if ( qword_82D50 )
        qword_82D50(v3);
      v7 = dcgettext(0, "Bad incremental file format", 5);
      v3 = (char *)(&dword_0 + 1);
      error(1, 0, v7);
      dword_82D58 = 2;
    }
    i = v4[1];
  }
  v8 = strtoumax(v4 + 1, 0, 10);
  v64 = v8;
  if ( v8 > 1 )
  {
    if ( v8 == 2 )
    {
      obstack_begin(&endptr, 0, 0, sub_4ECC0, &free);
      sub_1EBB0(qword_823A0, &xmmword_82B00);
      while ( sub_1E820(qword_823A0, (long long)"nfs", 0, 1, &v68) )
      {
        v59 = v68 != 0;
        sub_1EBB0(qword_823A0, v70);
        v9 = qword_823A0;
        if ( !sub_1E820(qword_823A0, (long long)"dev", 0, -1, &v68)
          || (v9 = qword_823A0, v10 = v68, !sub_1E820(qword_823A0, (long long)"ino", 0, -1, &v68))
          || (v9 = qword_823A0, v11 = v68, (unsigned int)sub_1E730(qword_823A0, (_IO_FILE *)&endptr, &ptr)) )
        {
          if ( qword_82D50 )
            qword_82D50(v9);
          v49 = dcgettext(0, "Unexpected EOF in snapshot file", 5);
          v50 = (const char *)sub_4CE40(qword_82B28);
          error(0, 0, "%s: %s", v50, v49);
          sub_16FD0();
        }
        object_base = endptr.object_base;
        if ( endptr.object_base == endptr.next_free )
          *((char *)&endptr + 80) |= 2u;
        chunk_limit = endptr.chunk_limit;
        if ( (~*(long long *)&endptr.alignment_mask & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask])
           - (unsigned long long)endptr.chunk <= endptr.chunk_limit - (char *)endptr.chunk )
          chunk_limit = (char *)(~*(long long *)&endptr.alignment_mask
                               & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask]);
        endptr.next_free = chunk_limit;
        endptr.object_base = chunk_limit;
        while ( !(unsigned int)sub_1E730(qword_823A0, (_IO_FILE *)&endptr, &ptr) && (unsigned long long)ptr > 1 )
          ;
        v14 = qword_823A0;
        IO_read_ptr = qword_823A0->_IO_read_ptr;
        if ( IO_read_ptr >= qword_823A0->_IO_read_end )
        {
          v16 = __uflow(qword_823A0);
        }
        else
        {
          qword_823A0->_IO_read_ptr = IO_read_ptr + 1;
          v16 = (unsigned char)*IO_read_ptr;
        }
        if ( v16 )
        {
          if ( qword_82D50 )
            qword_82D50(v14);
          v51 = dcgettext(0, "Missing record terminator", 5);
          v52 = ftello(qword_823A0);
          v53 = sub_452D0(v52, v72);
          v54 = sub_4CE40(qword_82B28);
          v55 = dcgettext(0, "%s: byte %s: %s", 5);
          error(0, 0, v55, v54, v53, v51);
          sub_16FD0();
        }
        v17 = endptr.object_base;
        if ( endptr.object_base == endptr.next_free )
          *((char *)&endptr + 80) |= 2u;
        v18 = endptr.chunk_limit;
        if ( (~*(long long *)&endptr.alignment_mask & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask])
           - (unsigned long long)endptr.chunk <= endptr.chunk_limit - (char *)endptr.chunk )
          v18 = (char *)(~*(long long *)&endptr.alignment_mask
                       & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask]);
        endptr.next_free = v18;
        endptr.object_base = v18;
        block = (struct _obstack_chunk *)v17;
        sub_1E440(object_base, v70[0], v70[1], v10, v11, v59, 0, v17);
        if ( block <= endptr.chunk || (char *)block >= endptr.chunk_limit )
        {
          obstack_free(&endptr, block);
        }
        else
        {
          endptr.object_base = (char *)block;
          endptr.next_free = (char *)block;
        }
      }
    }
    else
    {
      if ( qword_82D50 )
        qword_82D50(v4 + 1);
      v38 = dcgettext(0, "Unsupported incremental format version: %lu", 5);
      error(1, 0, v38, v64);
      dword_82D58 = 2;
    }
    goto LABEL_40;
  }
  ptr = 0;
  v2 = lineptr;
  v70[0] = 0;
  if ( v8 != 1 )
    goto LABEL_79;
  if ( __getdelim((char **)&ptr, v70, 10, qword_823A0) > 0 )
  {
    v20 = (char *)ptr;
    *(long long *)&v21 = sub_25930((char *)ptr, (char **)&endptr);
    xmmword_82B00 = v21;
    if ( v21 >= 0 )
    {
      v65 = 2;
      chunk_size = endptr.chunk_size;
      if ( *(char *)endptr.chunk_size )
      {
        v43 = __errno_location();
        v44 = (const char *)(chunk_size + 1);
        *v43 = 0;
        v45 = v43;
        v46 = strtoumax(v44, (char **)&endptr, 10);
        if ( !*v45 )
        {
          if ( v46 > 0x3B9AC9FF )
          {
            *v45 = 34;
          }
          else if ( v44 != (const char *)endptr.chunk_size )
          {
            *((long long *)&xmmword_82B00 + 1) = v46;
            v65 = 2;
            goto LABEL_55;
          }
        }
        if ( qword_82D50 )
          qword_82D50(v44);
        v47 = dcgettext(0, "Invalid time stamp", 5);
        v48 = (const char *)sub_4CE40(qword_82B28);
        error(0, *v45, "%s:%ld: %s", v48, 2, v47);
        dword_82D58 = 2;
        *(long long *)&xmmword_82B00 = 0x8000000000000000LL;
        *((long long *)&xmmword_82B00 + 1) = -1;
        v65 = 2;
      }
LABEL_55:
      while ( 1 )
      {
        v26 = __getdelim((char **)&ptr, v70, 10, qword_823A0);
LABEL_56:
        if ( v26 <= 0 )
          break;
        v27 = (char *)ptr + v26 - 1;
        v60 = *(char *)ptr == 43;
        ++v65;
        v28 = (char *)ptr + v60;
        if ( *v27 == 10 )
          *v27 = 0;
        v29 = 0;
        v30 = 0;
        v31 = __errno_location();
        if ( v64 == 1 )
        {
          v30 = sub_25930(v28, (char **)&endptr);
          if ( v32 < 0 || *(char *)endptr.chunk_size != 32 )
          {
            if ( qword_82D50 )
              qword_82D50(v28);
            v35 = "Invalid modification time";
            goto LABEL_68;
          }
          v33 = (const char *)endptr.chunk_size;
          blocka = (void *)endptr.chunk_size;
          *v31 = 0;
          v29 = strtoumax(v33, (char **)&endptr, 10);
          v34 = *v31;
          if ( v29 > 0x3B9AC9FF )
          {
            if ( !v34 )
              *v31 = 34;
LABEL_65:
            if ( qword_82D50 )
              qword_82D50(v33);
            v35 = "Invalid modification time (nanoseconds)";
LABEL_68:
            v36 = dcgettext(0, v35, 5);
            v37 = (const char *)sub_4CE40(qword_82B28);
            error(0, *v31, "%s:%ld: %s", v37, v65, v36);
            sub_16FD0();
          }
          if ( v34 )
            goto LABEL_65;
          v28 = (char *)endptr.chunk_size;
          if ( blocka == (void *)endptr.chunk_size || *(char *)endptr.chunk_size != 32 )
            goto LABEL_65;
        }
        v23 = sub_255E0(v28, (char **)&endptr);
        if ( *v31 || *(char *)endptr.chunk_size != 32 )
        {
          if ( qword_82D50 )
            qword_82D50(endptr.chunk_size);
          v35 = "Invalid device number";
          goto LABEL_68;
        }
        v24 = sub_255E0((char *)endptr.chunk_size, (char **)&endptr);
        if ( *v31 || *(char *)endptr.chunk_size != 32 )
        {
          if ( qword_82D50 )
            qword_82D50(endptr.chunk_size);
          v35 = "Invalid inode number";
          goto LABEL_68;
        }
        v25 = (const char *)(endptr.chunk_size + 1);
        blockb = (void *)v24;
        sub_251B0(endptr.chunk_size + 1);
        sub_1E440(v25, v30, v29, v23, (long long)blockb, v60, 0, 0);
      }
      free(ptr);
      goto LABEL_40;
    }
    v41 = 2;
LABEL_106:
    if ( qword_82D50 )
      qword_82D50(v20);
    v56 = dcgettext(0, "Invalid time stamp", 5);
    v57 = (const char *)sub_4CE40(qword_82B28);
    v58 = __errno_location();
    error(0, *v58, "%s:%ld: %s", v57, v41, v56);
    sub_16FD0();
  }
  sub_32F00(qword_82B28);
  free(ptr);
LABEL_40:
  if ( (qword_823A0->_flags & 0x20) != 0 )
    sub_32F00(qword_82B28);
  free(lineptr);
  return v73 - __readfsqword(0x28u);
}



// Function: write_directory_file @ 0x20e40
unsigned long long write_directory_file()
{
  FILE *v0; // rbp
  int v1; // eax
  double v2; // xmm0_8
  const char *v3; // r12
  size_t v4; // rax
  const char *v5; // r12
  size_t v6; // rax
  char v8[24]; // [rsp+0h] [rbp-38h] BYREF
  unsigned long long v9; // [rsp+18h] [rbp-20h]

  v0 = qword_823A0;
  v9 = __readfsqword(0x28u);
  if ( qword_823A0 )
  {
    if ( fseeko(qword_823A0, 0, 0) )
      sub_331E0(qword_82B28);
    v1 = fileno(v0);
    if ( (unsigned int)sub_2C110(v1) )
      sub_33450(qword_82B28);
    v2 = __fprintf_chk(v0, 1, "%s-%s-%d\n", "GNU tar", "1.34", 2);
    v3 = (const char *)sub_45230(tp.tv_sec, v8);
    v4 = strlen(v3);
    fwrite_unlocked(v3, v4 + 1, 1u, v0);
    v5 = (const char *)sub_45370(tp.tv_nsec, v8);
    v6 = strlen(v5);
    fwrite_unlocked(v5, v6 + 1, 1u, v0);
    if ( (v0->_flags & 0x20) != 0
      || qword_823B0 && (sub_43B00(qword_823B0, sub_1F3F0, v0, v2), (v0->_flags & 0x20) != 0) )
    {
      sub_334F0(qword_82B28);
    }
    if ( fclose(v0) )
      sub_32D60(qword_82B28);
  }
  return v9 - __readfsqword(0x28u);
}



// Function: is_dumpdir @ 0x20fc0
long long is_dumpdir(long long a1, char *a2)
{
  long long result; // rax
  unsigned long long v3; // r12
  char *v4; // r13
  size_t v5; // rax
  size_t v6; // rbx
  long long v7; // rdx
  long long v8; // rcx
  long long v9; // r8
  char *v10; // r14
  char *v11; // rax
  long long v12; // [rsp+0h] [rbp-40h]

  result = *(unsigned char *)(a1 + 392);
  if ( (char)result && !*(long long *)(a1 + 400) )
  {
    v3 = *(long long *)(a1 + 136);
    v12 = sub_4ECC0(v3);
    v4 = (char *)v12;
    sub_CCD0(qword_82408);
    sub_C670((void **)a1);
    for ( ; v3; v3 -= v6 )
    {
      sub_C710(v3);
      v10 = (char *)sub_D360(v3, a2, v7, v8, v9);
      if ( !v10 )
      {
        if ( qword_82D50 )
          qword_82D50(v3);
        v11 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(1, 0, v11);
        dword_82D58 = 2;
      }
      v5 = sub_CD10(v10);
      a2 = v10;
      if ( v5 > v3 )
        v5 = v3;
      v6 = v5;
      memcpy(v4, v10, v5);
      v4 += v6;
      sub_CCD0(&v10[v6 - 1]);
    }
    sub_C6A0();
    *(char *)(a1 + 393) = 1;
    *(long long *)(a1 + 400) = v12;
    return *(unsigned char *)(a1 + 392);
  }
  return result;
}



// Function: purge_directory @ 0x21100
void purge_directory(long long *a1, char *a2)
{
  long long *v3; // rdi
  char *v4; // rbp
  char v5; // r14
  char *v6; // r12
  char v7; // bl
  int v8; // r15d
  unsigned int v9; // r13d
  char *v10; // rax
  void *v11; // r15
  void *v12; // rbp
  char *v13; // rbx
  void **v14; // r14
  long long *v15; // r12
  const char *v16; // rsi
  long long v17; // r13
  char *v18; // rax
  const char *v19; // rax
  char v20; // al
  size_t v21; // rax
  long long v22; // r14
  char *v23; // r12
  char *v24; // r13
  size_t v25; // r12
  long long v26; // rdx
  long long v27; // rcx
  long long v28; // r14
  char *v29; // r12
  int v30; // r13d
  char *v31; // rax
  char *v32; // rax
  long long v33; // r13
  char *v34; // rax
  char *v35; // rax
  const char *v36; // rsi
  char *v37; // rax
  void *srcb; // [rsp+0h] [rbp-E8h]
  void *srca; // [rsp+0h] [rbp-E8h]
  char *s; // [rsp+8h] [rbp-E0h]
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v43; // [rsp+A8h] [rbp-40h]

  v3 = &qword_82840;
  v43 = __readfsqword(0x28u);
  if ( !(unsigned char)sub_20FC0((long long)&qword_82840, a2) )
    goto LABEL_2;
  a2 = 0;
  v3 = a1;
  s = (char *)sub_26970(a1, 0);
  if ( !s )
    goto LABEL_2;
  v4 = qword_829D0;
  v5 = *qword_829D0;
  if ( !*qword_829D0 )
    goto LABEL_28;
  v6 = qword_829D0;
  v7 = *qword_829D0;
  v8 = 0;
  v9 = 0;
  do
  {
    if ( v9 )
    {
      if ( v7 != v9 )
      {
        if ( qword_82D50 )
          qword_82D50(v3);
        v32 = dcgettext(0, "Malformed dumpdir: expected '%c' but found %#3o", 5);
        a2 = 0;
        v3 = 0;
        error(0, 0, v32, v9, (unsigned char)v7);
        dword_82D58 = 2;
        goto LABEL_2;
      }
      if ( v7 == 84 )
      {
        if ( !v6[1] && !v8 )
        {
          if ( qword_82D50 )
            qword_82D50(v3);
          v36 = "Malformed dumpdir: empty name in 'T'";
          goto LABEL_87;
        }
        v9 = 0;
        goto LABEL_11;
      }
    }
    else if ( v7 == 84 )
    {
      if ( qword_82D50 )
        qword_82D50(v3);
      v36 = "Malformed dumpdir: 'T' not preceded by 'R'";
      goto LABEL_87;
    }
    if ( v7 == 88 )
    {
      if ( v8 )
      {
        if ( qword_82D50 )
          qword_82D50(v3);
        v36 = "Malformed dumpdir: 'X' duplicated";
        goto LABEL_87;
      }
      v8 = 1;
    }
    else if ( v7 == 82 )
    {
      if ( v6[1] )
      {
        v9 = 84;
      }
      else
      {
        if ( !v8 )
        {
          if ( qword_82D50 )
            qword_82D50(v3);
          v36 = "Malformed dumpdir: empty name in 'R'";
LABEL_87:
          v37 = dcgettext(0, v36, 5);
          a2 = 0;
          v3 = 0;
          error(0, 0, v37);
          dword_82D58 = 2;
          goto LABEL_2;
        }
        v9 = 84;
        v8 = 0;
      }
    }
LABEL_11:
    v3 = (long long *)v6;
    v6 += strlen(v6) + 1;
    v7 = *v6;
  }
  while ( *v6 );
  if ( v9 )
  {
    if ( qword_82D50 )
      qword_82D50(v3);
    v35 = dcgettext(0, "Malformed dumpdir: expected '%c' but found end of data", 5);
    a2 = 0;
    v3 = 0;
    error(0, 0, v35, v9);
    dword_82D58 = 2;
LABEL_2:
    sub_24660(v3, a2);
    return;
  }
  if ( !v8 || (dword_81B80 & 2) == 0 )
    goto LABEL_47;
  if ( qword_82D50 )
    qword_82D50(v3);
  v10 = dcgettext(0, "Malformed dumpdir: 'X' never used", 5);
  error(0, 0, v10);
  v4 = qword_829D0;
  v5 = *qword_829D0;
  if ( *qword_829D0 )
  {
LABEL_47:
    v11 = 0;
    while ( 1 )
    {
      if ( v5 == 88 )
      {
        v25 = strlen(v4 + 1);
        v11 = (void *)sub_4ECE0(v11, v25 + 12, v26, v27);
        memcpy(v11, v4 + 1, v25);
        strcpy((char *)v11 + v25, "/tar.XXXXXX");
        if ( !mkdtemp((char *)v11) )
        {
          if ( qword_82D50 )
            qword_82D50(v11);
          v28 = sub_4D2D0(v11);
          v29 = dcgettext(0, "Cannot create temporary directory using template %s", 5);
          a2 = (char *)(unsigned int)*__errno_location();
          error(0, (int)a2, v29, v28);
          dword_82D58 = 2;
          free(v11);
          v3 = (long long *)s;
          free(s);
          goto LABEL_2;
        }
      }
      else
      {
        v21 = strlen(v4);
        if ( v5 == 82 )
        {
          v22 = (long long)&v4[v21 + 1];
          v23 = v4 + 1;
          v24 = &v4[v21 + 2];
          if ( v4[1] )
            v23 = (char *)sub_33640(v4 + 1);
          if ( *(char *)(v22 + 1) )
            v24 = (char *)sub_33640(v24);
          if ( *v23 )
          {
            if ( !*v24 )
              v24 = (char *)v11;
          }
          else
          {
            v23 = (char *)v11;
          }
          a2 = v24;
          if ( !(unsigned char)sub_1A700(v23, v24) )
          {
            free(v11);
            v3 = (long long *)s;
            free(s);
            goto LABEL_2;
          }
          v4 = (char *)v22;
        }
      }
      v4 += strlen(v4) + 1;
      v5 = *v4;
      if ( !*v4 )
        goto LABEL_29;
    }
  }
LABEL_28:
  v11 = 0;
LABEL_29:
  v12 = 0;
  free(v11);
  v13 = s;
  v14 = (void **)sub_1E2E0(qword_829D0, "YND");
  if ( *s )
  {
    v15 = a1;
    while ( 1 )
    {
      free(v12);
      v12 = (void *)sub_29320(v15, v13);
      if ( !(unsigned int)sub_25FC0((char *)v12, &buf) )
        break;
      if ( *__errno_location() != 2 )
      {
        sub_26460(v12);
        if ( qword_82D50 )
          qword_82D50(v12);
        v16 = "%s: Not purging directory: unable to stat";
        v17 = sub_4CE40(v12);
LABEL_35:
        v18 = dcgettext(0, v16, 5);
        error(0, 0, v18, v17);
      }
LABEL_36:
      v13 += strlen(v13) + 1;
      if ( !*v13 )
        goto LABEL_72;
    }
    v19 = sub_1EC90((long long)v14, v13);
    if ( v19 )
    {
      v20 = *v19;
      if ( v20 == 68 )
      {
        if ( (buf.st_mode & 0xF000) == 0x4000 )
          goto LABEL_36;
      }
      else if ( v20 != 89 || (buf.st_mode & 0xF000) != 0x4000 )
      {
        goto LABEL_36;
      }
    }
    if ( !byte_82AD1 || buf.st_dev == qword_827F0 )
    {
      if ( !byte_82B40 || (unsigned int)sub_2DF50("delete", v12) )
      {
        if ( dword_82A54 )
        {
          v33 = sub_4D2D0(v12);
          srca = (void *)qword_82DD0;
          v34 = dcgettext(0, "%s: Deleting %s\n", 5);
          __fprintf_chk(qword_81F28, 1, v34, srca, v33);
        }
        if ( !(unsigned int)sub_26A40(v12, 1) )
        {
          v30 = *__errno_location();
          if ( qword_82D50 )
            qword_82D50(v12);
          srcb = (void *)sub_4CE40(v12);
          v31 = dcgettext(0, "%s: Cannot remove", 5);
          error(0, v30, v31, srcb);
          dword_82D58 = 2;
        }
      }
      goto LABEL_36;
    }
    if ( qword_82D50 )
      qword_82D50(v14);
    v16 = "%s: directory is on a different device: not purging";
    v17 = sub_4CE40(v12);
    goto LABEL_35;
  }
LABEL_72:
  free(v12);
  free(v14[3]);
  free(v14);
  free(s);
}



// Function: list_dumpdir @ 0x21850
void list_dumpdir(long long a1, long long a2)
{
  int v2; // r15d
  long long v3; // r14
  long long v4; // r13
  long long v5; // rbx
  char *IO_write_ptr; // rax
  long long v7; // rax
  long long v8; // rcx
  unsigned char v9; // si
  char *v10; // rax
  double v11; // xmm0_8

  if ( a2 )
  {
    v2 = 0;
    v3 = 3228673;
    v4 = a2 - 1;
    v5 = a1 + 1;
    while ( 1 )
    {
      v8 = (unsigned int)*(char *)(v5 - 1);
      v9 = *(char *)(v5 - 1);
      if ( v9 )
      {
        if ( (unsigned char)(*(char *)(v5 - 1) - 68) <= 0x15u && _bittest64(&v3, (unsigned int)(v8 - 68)) )
        {
          v11 = __fprintf_chk(qword_81F28, 1, "%c", v8);
          if ( !v2 )
            __fprintf_chk(qword_81F28, 1, " ", v11);
          v2 = 1;
        }
        else
        {
          IO_write_ptr = qword_81F28->_IO_write_ptr;
          if ( IO_write_ptr >= qword_81F28->_IO_write_end )
          {
            __overflow(qword_81F28, v9);
          }
          else
          {
            qword_81F28->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = v9;
          }
        }
        ++v5;
        v7 = v4 - 1;
        if ( !v4 )
          return;
      }
      else
      {
        v10 = qword_81F28->_IO_write_ptr;
        if ( v10 >= qword_81F28->_IO_write_end )
        {
          __overflow(qword_81F28, 10);
        }
        else
        {
          qword_81F28->_IO_write_ptr = v10 + 1;
          *v10 = 10;
        }
        v2 = 0;
        ++v5;
        v7 = v4 - 1;
        if ( !v4 )
          return;
      }
      v4 = v7;
    }
  }
}



// Function: decode_xform @ 0x21980
const char *decode_xform(char *src, int *a2)
{
  int v2; // eax
  const char *v3; // r12
  size_t v5; // rax

  v2 = *a2;
  v3 = src;
  if ( *a2 == 2 )
  {
    v3 = (const char *)sub_33640(src);
    if ( !qword_82A90 )
      return v3;
  }
  else
  {
    if ( v2 == 4 )
      return v3;
    if ( v2 == 1 )
      v3 = (const char *)sub_33640(src);
    if ( !qword_82A90 )
      return v3;
  }
  v5 = sub_293B0(v3);
  if ( v5 == -1 )
    v5 = strlen(v3);
  return &v3[v5];
}



// Function: from_header @ 0x21a20
long long from_header(
        long long src,
        size_t n,
        long long a3,
        unsigned long long a4,
        unsigned long long a5,
        int a6,
        int a7)
{
  unsigned char *v9; // rbx
  long long v11; // r12
  const unsigned short **v12; // rax
  unsigned char *v13; // rcx
  __int128 *v14; // rsi
  const unsigned short **v15; // r11
  const unsigned short *v16; // rax
  const unsigned short *v17; // rdx
  unsigned char *v18; // r9
  unsigned char *v19; // r15
  char v20; // r8
  bool v21; // zf
  int v22; // eax
  long long result; // rax
  const char *v24; // rsi
  char *v25; // rax
  char *v26; // rax
  unsigned char *v27; // rcx
  bool v28; // r8
  long long v29; // rax
  unsigned long long v30; // rdi
  long long v31; // rbx
  long long v32; // rax
  long long v33; // r12
  long long v34; // r14
  char *v35; // rax
  long long v36; // r10
  int v37; // edx
  unsigned int v38; // eax
  char *v39; // rax
  long long v40; // rax
  unsigned char *v41; // rcx
  int v42; // r8d
  long long v43; // rdi
  long long v44; // rax
  char *v45; // rax
  long long *v46; // rcx
  signed long long v47; // rax
  void *v48; // rsp
  long long v49; // r12
  char *v50; // rax
  char *v51; // rax
  long long v52; // [rsp+0h] [rbp-1498h]
  long long v53; // [rsp+8h] [rbp-1490h] BYREF
  char v54[7]; // [rsp+10h] [rbp-1488h] BYREF
  long long v55; // [rsp+1008h] [rbp-490h] BYREF
  unsigned char *v56; // [rsp+1010h] [rbp-488h]
  const unsigned short **v57; // [rsp+1018h] [rbp-480h]
  unsigned long long v58; // [rsp+1020h] [rbp-478h]
  int v59; // [rsp+102Ch] [rbp-46Ch]
  unsigned long long v60; // [rsp+1030h] [rbp-468h]
  char v61[33]; // [rsp+1038h] [rbp-460h] BYREF
  char v62[31]; // [rsp+1059h] [rbp-43Fh] BYREF
  char v63; // [rsp+1078h] [rbp-420h] BYREF
  char v64[999]; // [rsp+1079h] [rbp-41Fh] BYREF
  unsigned long long v65; // [rsp+1460h] [rbp-38h]

  v9 = (unsigned char *)(src + n);
  v58 = a5;
  LODWORD(v57) = a6;
  v59 = a7;
  v65 = __readfsqword(0x28u);
  if ( src + n == (*(char *)src == 0) + src )
  {
LABEL_20:
    if ( a3 && (char)v59 != 1 )
    {
      if ( qword_82D50 )
        qword_82D50(src);
      v24 = "Blanks in header where numeric %s value expected";
LABEL_25:
      v25 = dcgettext(0, v24, 5);
      error(0, 0, v25, a3);
      dword_82D58 = 2;
    }
    return -1;
  }
  v60 = (*(char *)src == 0) + src;
  v11 = src;
  v12 = __ctype_b_loc();
  v13 = (unsigned char *)v60;
  v14 = (__int128 *)*v12;
  v15 = v12;
  while ( 1 )
  {
    v16 = (const unsigned short *)*v13;
    v17 = v16;
    if ( (*((char *)v14 + 2 * (long long)v16 + 1) & 0x20) == 0 )
      break;
    if ( v9 == ++v13 )
      goto LABEL_20;
  }
  v60 = -(long long)a4;
  LODWORD(a4) = (char)v16 - 48;
  if ( (unsigned int)a4 > 7 )
  {
    if ( (char)v57 )
    {
      result = 0;
      if ( !(char)v17 )
        return result;
    }
    else
    {
      if ( (((char)v16 - 43) & 0xFD) == 0 )
      {
        if ( !(char)v59 && !byte_82450 )
        {
          byte_82450 = 1;
          if ( qword_82D50 )
          {
            v56 = v13;
            v57 = v15;
            qword_82D50(src);
            v13 = v56;
            v15 = v57;
          }
          v56 = v13;
          v57 = v15;
          v26 = dcgettext(0, "Archive contains obsolescent base-64 headers", 5);
          error(0, 0, v26);
          v13 = v56;
          v15 = v57;
          LOBYTE(v17) = *v56;
        }
        v27 = v13 + 1;
        v14 = &xmmword_82460;
        v28 = (char)v17 == 45;
        a4 = 0;
        if ( v9 != v27 )
        {
          while ( 1 )
          {
            v17 = (const unsigned short *)*v27;
            v29 = *((char *)&xmmword_82460 + (long long)v17);
            if ( (char)v29 > 63 )
            {
              if ( v27 != v9 && (char)v17 && ((*v15)[(long long)v17] & 0x2000) == 0 )
                goto LABEL_28;
              goto LABEL_45;
            }
            if ( (a4 & 0x3FFFFFFFFFFFFFFLL) != a4 )
              break;
            ++v27;
            a4 = v29 | (a4 << 6);
            if ( v9 == v27 )
              goto LABEL_45;
          }
          v46 = (long long *)((char *)&v55 - ((n + 24) & 0xFFFFFFFFFFFFF000LL));
          if ( &v55 != v46 )
          {
            while ( &v53 != v46 )
              ;
          }
          v47 = ((short)n + 24) & 0xFF0;
          v48 = alloca(v47);
          if ( (((short)n + 24) & 0xFF0) != 0 )
            *(long long *)((char *)&v52 + v47) = *(long long *)((char *)&v52 + v47);
          memcpy(v54, (const void *)src, n);
          v21 = (char)v59 == 1;
          v54[n] = 0;
          if ( !v21 && a3 )
          {
            if ( qword_82D50 )
              qword_82D50(v54);
            v49 = sub_4D2D0(v54);
            v50 = dcgettext(0, "Archive signed base-64 string %s is out of %s range", 5);
            error(0, 0, v50, v49, a3);
            dword_82D58 = 2;
            return -1;
          }
          return -1;
        }
        result = 0;
        if ( (char)v17 == 45 )
          return result;
        return a4;
      }
      if ( (char)v16 == 0x80 || (char)v16 == 0xFF )
      {
        v41 = v13 + 1;
        v42 = (unsigned char)v16 & 0x40;
        v43 = (unsigned char)v16 & 0x40;
        a4 = ((unsigned char)v16 & 0x3F) - v42;
        while ( 1 )
        {
          v44 = *v41++;
          a4 = v44 + (a4 << 8);
          if ( v9 == v41 )
            break;
          if ( (((unsigned long long)(unsigned int)-v42 << 50) | a4 & 0xFFFFFFFFFFFFFFLL) != a4 )
          {
            if ( !a3 || (char)v59 == 1 )
              return -1;
            if ( qword_82D50 )
              qword_82D50(v43);
            v24 = "Archive base-256 value is out of %s range";
            goto LABEL_25;
          }
        }
        v28 = v42 != 0;
        if ( (char)v43 )
          a4 = -(long long)a4;
LABEL_45:
        if ( !v28 )
          goto LABEL_15;
        if ( v60 >= a4 )
          return -(long long)a4;
        if ( (char)v59 != 1 && a3 )
        {
          v30 = a4;
          v31 = sub_45370(v60, v62);
          v32 = sub_45370(a4, v64);
          *(char *)(v32 - 1) = 45;
          v33 = v32 - 1;
          goto LABEL_50;
        }
        return -1;
      }
      result = 0;
      if ( !(char)v17 )
        return result;
    }
    goto LABEL_28;
  }
  v18 = v13 + 1;
  a4 = (int)a4;
  if ( v9 == v13 + 1 )
  {
    v19 = v9;
    v20 = 0;
LABEL_56:
    if ( v58 >= a4 && !v20 )
      return a4;
  }
  else
  {
    v19 = v13 + 1;
    v20 = 0;
    while ( 1 )
    {
      src = (unsigned int)(char)*v19;
      v22 = src - 48;
      if ( (unsigned int)(src - 48) > 7 )
        break;
      src = a4 & 0x1FFFFFFFFFFFFFFFLL;
      v21 = (a4 & 0x1FFFFFFFFFFFFFFFLL) == a4;
      a4 = v22 + 8 * a4;
      LOBYTE(src) = !v21;
      ++v19;
      v20 |= !v21;
      if ( v9 == v19 )
        goto LABEL_56;
    }
    if ( v58 >= a4 && !v20 )
    {
      if ( v9 == v19 || !*v19 )
        return a4;
LABEL_14:
      if ( (*((char *)v14 + 2 * (unsigned char)src + 1) & 0x20) != 0 )
        goto LABEL_15;
LABEL_28:
      if ( !a3 )
        return -1;
      goto LABEL_29;
    }
  }
  if ( (char)v17 <= 49 )
    goto LABEL_75;
  if ( a3 )
  {
    v36 = 7 - (char)(((char)v17 - 48) | 4);
    if ( v9 == v18 )
    {
      v19 = v9;
      src = 0;
    }
    else
    {
      src = 0;
      do
      {
        v38 = (char)*v18 - 48;
        if ( v38 > 7 )
        {
          v19 = v18;
          goto LABEL_65;
        }
        v37 = v36;
        LOBYTE(v37) = (v36 & 0x1FFFFFFFFFFFFFFFLL) != v36;
        ++v18;
        src = v37 | (unsigned int)src;
        v17 = (const unsigned short *)(7 - v38);
        v36 = (int)v17 + 8 * v36;
      }
      while ( v9 != v18 );
      v19 = v9;
    }
LABEL_65:
    a4 = v36 + 1;
    v20 = src | (v36 == -1);
    if ( v20 == 1 )
      goto LABEL_117;
    if ( v60 >= a4 )
    {
      if ( !(char)v59 )
      {
        if ( qword_82D50 )
        {
          v57 = (const unsigned short **)v13;
          v58 = v36;
          v60 = (unsigned long long)v15;
          qword_82D50(src);
          v13 = (unsigned char *)v57;
          v36 = v58;
          v15 = (const unsigned short **)v60;
        }
        v58 = v36;
        v60 = (unsigned long long)v15;
        v57 = (const unsigned short **)v13;
        v39 = dcgettext(0, "Archive octal value %.*s is out of %s range; assuming two's complement", 5);
        v14 = 0;
        error(0, 0, v39, (unsigned int)((int)v19 - (int)v57), v57, a3);
        v15 = (const unsigned short **)v60;
        v36 = v58;
      }
      if ( v9 == v19 )
        return ~v36;
      v40 = *v19;
      if ( !(char)v40 )
        return ~v36;
      v17 = *v15;
      if ( ((*v15)[v40] & 0x2000) != 0 )
        return -(long long)a4;
LABEL_29:
      if ( !qword_82448 )
      {
        qword_82448 = sub_4C700(0, v14, v17);
        sub_4C760(qword_82448, 8);
      }
      while ( (unsigned char *)v11 != v9 && !*(v9 - 1) )
        --v9;
      sub_4C820(&v63, 1000, v11, &v9[-v11], qword_82448);
      if ( (char)v59 )
        return -1;
      if ( qword_82D50 )
        qword_82D50(&v63);
      v45 = dcgettext(0, "Archive contains %.*s where numeric %s value expected", 5);
      error(0, 0, v45, 1000, &v63, a3);
LABEL_99:
      dword_82D58 = 2;
      return -1;
    }
LABEL_75:
    if ( !v20 )
      goto LABEL_76;
LABEL_117:
    if ( !a3 || (char)v59 == 1 )
      return -1;
    if ( qword_82D50 )
    {
      v60 = (unsigned long long)v13;
      qword_82D50(src);
      v13 = (unsigned char *)v60;
    }
    v60 = (unsigned long long)v13;
    v51 = dcgettext(0, "Archive octal value %.*s is out of %s range", 5);
    error(0, 0, v51, (unsigned int)((int)v19 - v60), v60, a3);
    goto LABEL_99;
  }
  if ( v20 )
    return -1;
LABEL_76:
  if ( v9 != v19 )
  {
    LOBYTE(src) = *v19;
    if ( *v19 )
      goto LABEL_14;
  }
LABEL_15:
  if ( v58 >= a4 )
    return a4;
  if ( !a3 || (char)v59 == 1 )
    return -1;
  v30 = a4;
  v31 = sub_45370(v60, v62);
  v33 = sub_45370(a4, v64);
LABEL_50:
  if ( v60 )
    *(char *)--v31 = 45;
  if ( qword_82D50 )
    qword_82D50(v30);
  v34 = sub_45370(v58, v61);
  v35 = dcgettext(0, "Archive value %s is out of %s range %s..%s", 5);
  error(0, 0, v35, v33, a3, v31, v34);
  dword_82D58 = 2;
  return -1;
}



// Function: tar_checksum @ 0x22480
long long tar_checksum(unsigned char *a1, unsigned char a2)
{
  unsigned char *v4; // rdx
  int v5; // esi
  unsigned char *v6; // r8
  int v7; // ecx
  int v8; // edi
  unsigned char *v9; // rdx
  int v10; // edi
  int v11; // ebx
  int v12; // eax
  unsigned int v13; // r8d

  v4 = a1;
  v5 = 0;
  v6 = a1 + 512;
  v7 = 0;
  do
  {
    v8 = *v4++;
    v7 += v8;
    v5 += (char)v8;
  }
  while ( v6 != v4 );
  if ( !v7 )
    return 3;
  v9 = a1 + 155;
  do
  {
    v10 = *v9--;
    v7 -= v10;
    v5 -= (char)v10;
  }
  while ( a1 + 147 != v9 );
  v11 = v7 + 256;
  v12 = sub_21A20((long long)(a1 + 148), 8u, 0, 0, 0x7FFFFFFFu, 1, a2);
  if ( v12 < 0 )
    return 5;
  if ( v11 == v12 )
    return 1;
  v13 = 5;
  if ( v5 + 256 == v12 )
    return 1;
  return v13;
}



// Function: read_header @ 0x22570
long long read_header(char *a1, char *a2, char *a3, long long a4, long long a5)
{
  long long v5; // r12
  unsigned int v6; // ebx
  long long v7; // r13
  char *v8; // rbp
  char v9; // al
  void *v10; // rdi
  char *v11; // rsi
  long long v13; // rax
  char v14; // dl
  long long v15; // rax
  long long *v16; // rax
  long long v17; // rax
  unsigned long long v18; // rbx
  long long v19; // rbp
  long long v20; // rcx
  char *v21; // rdi
  char *v22; // rbp
  long long v23; // rcx
  char *v24; // rsi
  unsigned long long v25; // rbx
  size_t v26; // rax
  size_t v27; // r12
  long long v28; // rax
  char *v29; // r13
  char *v30; // rax
  int v31; // r8d
  __m128i *v32; // rax
  int v33; // edx
  __m128i v34; // xmm2
  int v35; // eax
  __m128i v36; // xmm3
  __m128i v37; // xmm4
  __m128i v38; // xmm5
  __m128i v39; // xmm6
  __m128i v40; // xmm7
  __m128i v41; // xmm1
  long long v42; // rdx
  __m128i v43; // xmm2
  __m128i v44; // xmm3
  __m128i v45; // xmm4
  __m128i v46; // xmm5
  __m128i v47; // xmm6
  char v48; // al
  __m128i v49; // xmm7
  __m128i v50; // xmm1
  __m128i v51; // xmm2
  size_t v52; // rax
  long long v53; // [rsp-10h] [rbp-1B8h]
  char *v54; // [rsp-8h] [rbp-1B0h]
  int v55; // [rsp+4h] [rbp-1A4h]
  long long *v56; // [rsp+8h] [rbp-1A0h]
  char *v57; // [rsp+10h] [rbp-198h]
  char *v58; // [rsp+18h] [rbp-190h]
  long long v59; // [rsp+20h] [rbp-188h]
  char *ptr; // [rsp+28h] [rbp-180h]
  long long v61; // [rsp+30h] [rbp-178h]
  long long v62; // [rsp+38h] [rbp-170h]
  char v63[32]; // [rsp+40h] [rbp-168h] BYREF
  char s[16]; // [rsp+60h] [rbp-148h] BYREF
  __m128i v65; // [rsp+70h] [rbp-138h]
  __m128i v66; // [rsp+80h] [rbp-128h]
  __m128i v67; // [rsp+90h] [rbp-118h]
  __m128i v68; // [rsp+A0h] [rbp-108h]
  __m128i v69; // [rsp+B0h] [rbp-F8h]
  __m128i v70; // [rsp+C0h] [rbp-E8h]
  __m128i v71; // [rsp+D0h] [rbp-D8h]
  __m128i v72; // [rsp+E0h] [rbp-C8h]
  long long v73; // [rsp+F0h] [rbp-B8h]
  short v74; // [rsp+F8h] [rbp-B0h]
  char v75; // [rsp+FAh] [rbp-AEh]
  char v76; // [rsp+FBh] [rbp-ADh]
  unsigned long long v77; // [rsp+168h] [rbp-40h]

  v56 = a1;
  v57 = a2;
  v55 = (int)a3;
  v77 = __readfsqword(0x28u);
  v61 = 0;
  v62 = 0;
  v58 = 0;
  ptr = 0;
  v59 = (long long)(a2 + 360);
  while ( 1 )
  {
    while ( 1 )
    {
      v5 = sub_D360(a1, a2, a3, a4, a5);
      *v56 = v5;
      if ( !v5 )
      {
        v6 = 4;
        goto LABEL_10;
      }
      v6 = sub_22480((unsigned char *)v5, 0);
      if ( v6 != 1 )
        goto LABEL_10;
      if ( *(char *)(v5 + 156) == 49 )
      {
        v7 = v5;
        *((long long *)v57 + 17) = 0;
        goto LABEL_6;
      }
      v13 = sub_21A20(v5 + 124, 0xCu, (long long)"off_t", 0, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
      *((long long *)v57 + 17) = v13;
      if ( v13 < 0 )
      {
        v6 = 5;
        goto LABEL_10;
      }
      v14 = *(char *)(v5 + 156);
      if ( (unsigned char)(v14 - 75) > 1u && v14 != 120 )
        break;
      if ( v55 == 1 )
        goto LABEL_24;
      if ( (unsigned char)(v14 - 75) > 1u )
        goto LABEL_16;
      v18 = v13 + 512;
      if ( (v13 & 0x1FF) != 0 )
      {
        v18 = v13 + 1024 - (v13 & 0x1FF);
        if ( v13 > v18 )
          sub_17020();
      }
      v19 = sub_4ECC0(v18 + 1);
      if ( *(char *)(v5 + 156) == 76 )
      {
        free(ptr);
        v62 = v18 >> 9;
        ptr = (char *)v19;
      }
      else
      {
        free(v58);
        v61 = v18 >> 9;
        v58 = (char *)v19;
      }
      sub_CCD0(v5);
      v20 = v19;
      v21 = (char *)((v19 + 8) & 0xFFFFFFFFFFFFFFF8LL);
      v22 = (char *)(v19 + 512);
      v23 = v20 - (long long)v21;
      *((long long *)v22 - 64) = *(long long *)v5;
      a3 = *(char **)(v5 + 504);
      v24 = (char *)(v5 - v23);
      *((long long *)v22 - 1) = a3;
      LODWORD(v23) = (unsigned int)(v23 + 512) >> 3;
      qmemcpy(v21, v24, 8LL * (unsigned int)v23);
      a2 = &v24[8 * (unsigned int)v23];
      a1 = &v21[8 * (unsigned int)v23];
      a4 = 0;
      v25 = v18 - 512;
      if ( v25 )
      {
        while ( 1 )
        {
          v28 = sub_D360(a1, a2, a3, a4, a5);
          v29 = (char *)v28;
          if ( !v28 )
            break;
          v26 = sub_CD10(v28);
          a2 = v29;
          if ( v25 <= v26 )
            v26 = v25;
          v27 = v26;
          memcpy(v22, v29, v26);
          a1 = &v29[v27 - 1];
          v22 += v27;
          sub_CCD0(a1);
          v25 -= v27;
          if ( !v25 )
            goto LABEL_38;
        }
        if ( qword_82D50 )
          qword_82D50(a1);
        v30 = dcgettext(0, "Unexpected EOF in archive", 5);
        a2 = 0;
        a1 = 0;
        error(0, 0, v30);
        dword_82D58 = 2;
      }
LABEL_38:
      *v22 = 0;
    }
    if ( v14 != 103 && v14 != 88 )
      break;
    if ( v55 == 1 )
      goto LABEL_24;
    if ( v14 == 88 )
    {
LABEL_16:
      v15 = sub_21A20(v5 + 124, 0xCu, (long long)"off_t", 0, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
      a1 = (char *)v59;
      sub_1DB80(v59, (char *)v5, v15);
      a4 = v53;
      a2 = v54;
    }
    else
    {
      v16 = (long long *)qword_82560;
      if ( !qword_82560 )
      {
        v16 = (long long *)sub_4ECC0(512);
        qword_82560 = (long long)v16;
      }
      *v16 = *(long long *)v5;
      v16[63] = *(long long *)(v5 + 504);
      qmemcpy(
        (void *)((unsigned long long)(v16 + 1) & 0xFFFFFFFFFFFFFFF8LL),
        (const void *)(v5 - ((long long)v16 - ((unsigned long long)(v16 + 1) & 0xFFFFFFFFFFFFFFF8LL))),
        8LL * (((unsigned int)v16 - (((int)v16 + 8) & 0xFFFFFFF8) + 512) >> 3));
      memset(v63, 0, sizeof(v63));
      v17 = sub_21A20(v5 + 124, 0xCu, (long long)"off_t", 0, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
      a2 = (char *)v5;
      sub_1DB80((long long)v63, (char *)v5, v17);
      sub_1D980((long long)v63);
      a1 = v63;
      sub_1DE20(v63, 0.0);
      a3 = v54;
      if ( v55 == 2 )
      {
LABEL_24:
        v6 = 2;
        goto LABEL_10;
      }
    }
  }
  v7 = v5;
LABEL_6:
  free(qword_823F8);
  v8 = ptr + 512;
  if ( !ptr )
  {
    if ( *(char *)(v7 + 345) )
    {
      v8 = s;
      v31 = strcmp((const char *)(v7 + 257), "ustar");
      v32 = (__m128i *)s;
      if ( !v31 )
      {
        v41 = _mm_loadu_si128((const __m128i *)(v7 + 345));
        v42 = *(long long *)(v7 + 489);
        v76 = 0;
        v43 = _mm_loadu_si128((const __m128i *)(v7 + 361));
        v44 = _mm_loadu_si128((const __m128i *)(v7 + 377));
        v45 = _mm_loadu_si128((const __m128i *)(v7 + 393));
        *(__m128i *)s = v41;
        v46 = _mm_loadu_si128((const __m128i *)(v7 + 409));
        v47 = _mm_loadu_si128((const __m128i *)(v7 + 425));
        v48 = *(char *)(v7 + 499);
        v73 = v42;
        v49 = _mm_loadu_si128((const __m128i *)(v7 + 441));
        LOWORD(v42) = *(short *)(v7 + 497);
        v65 = v43;
        v50 = _mm_loadu_si128((const __m128i *)(v7 + 457));
        v75 = v48;
        v51 = _mm_loadu_si128((const __m128i *)(v7 + 473));
        v74 = v42;
        v66 = v44;
        v67 = v45;
        v68 = v46;
        v69 = v47;
        v70 = v49;
        v71 = v50;
        v72 = v51;
        v52 = strlen(s);
        s[v52] = 47;
        v32 = (__m128i *)&s[v52 + 1];
      }
    }
    else
    {
      v8 = s;
      v32 = (__m128i *)s;
    }
    v62 = 0;
    *v32 = _mm_loadu_si128((const __m128i *)v7);
    v32[1] = _mm_loadu_si128((const __m128i *)(v7 + 16));
    v32[2] = _mm_loadu_si128((const __m128i *)(v7 + 32));
    v32[3] = _mm_loadu_si128((const __m128i *)(v7 + 48));
    v32[4] = _mm_loadu_si128((const __m128i *)(v7 + 64));
    v32[5] = _mm_loadu_si128((const __m128i *)(v7 + 80));
    v33 = *(int *)(v7 + 96);
    v32[6].m128i_i8[4] = 0;
    v32[6].m128i_i32[0] = v33;
  }
  qword_823F8 = ptr;
  qword_823E8 = v62;
  sub_25120(v57, v8);
  sub_25120(v57 + 8, v8);
  v9 = sub_403C0(*((long long *)v57 + 1));
  v10 = qword_823F0;
  v57[16] = v9;
  free(v10);
  if ( v58 )
  {
    v11 = v58 + 512;
  }
  else
  {
    v34 = _mm_loadu_si128((const __m128i *)(v7 + 157));
    v35 = *(int *)(v7 + 253);
    v70.m128i_i8[4] = 0;
    v11 = s;
    v36 = _mm_loadu_si128((const __m128i *)(v7 + 173));
    v37 = _mm_loadu_si128((const __m128i *)(v7 + 189));
    v61 = 0;
    v38 = _mm_loadu_si128((const __m128i *)(v7 + 205));
    v70.m128i_i32[0] = v35;
    v39 = _mm_loadu_si128((const __m128i *)(v7 + 221));
    *(__m128i *)s = v34;
    v40 = _mm_loadu_si128((const __m128i *)(v7 + 237));
    v65 = v36;
    v66 = v37;
    v67 = v38;
    v68 = v39;
    v69 = v40;
  }
  qword_823F0 = v58;
  qword_823E0 = v61;
  sub_25120(v57 + 24, v11);
  v58 = 0;
  ptr = 0;
LABEL_10:
  free(ptr);
  free(v58);
  return v6;
}



// Function: decode_header @ 0x22c60
long long decode_header(long long a1, long long a2, int *a3, int a4)
{
  long long v7; // r13
  int v8; // r12d
  long long v9; // rax
  long long v10; // rsi
  bool v11; // zf
  long long v12; // rsi
  __m128i v13; // xmm0
  int v14; // eax
  unsigned long long v15; // rax
  int v16; // eax
  long long result; // rax
  __m128i v18; // xmm0
  long long v19; // r12
  unsigned int v20; // eax
  long long v21; // rdi
  long long v22; // rax
  long long v23; // [rsp+0h] [rbp-58h]

  v7 = sub_21A20(a1 + 100, 8u, (long long)"mode_t", 0x8000000000000000LL, 0xFFFFFFFFFFFFFFFFLL, 0, 0);
  if ( !strcmp((const char *)(a1 + 257), "ustar") )
  {
    if ( *(char *)(a1 + 475)
      || (unsigned char)(*(char *)(a1 + 476) - 48) > 7u
      || *(char *)(a1 + 487) != 32
      || (unsigned char)(*(char *)(a1 + 488) - 48) > 7u
      || (v8 = 5, *(char *)(a1 + 499) != 32) )
    {
      v8 = 3 - ((*(long long *)(a2 + 368) == 0) - 1);
    }
  }
  else
  {
    v8 = 1;
    if ( !strcmp((const char *)(a1 + 257), "ustar  ") )
      v8 = (v7 & 0xFFFFFFFFFFFFF000LL) == 0 ? 6 : 2;
  }
  *a3 = v8;
  *(int *)(a2 + 112) = v7 & 0xFFF;
  v9 = sub_21A20(a1 + 136, 0xCu, (long long)"time_t", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
  v10 = 0;
  v11 = *(char *)(a1 + 265) == 0;
  *(long long *)(a2 + 248) = v9;
  *(long long *)(a2 + 256) = 0;
  if ( !v11 )
    v10 = a1 + 265;
  sub_25160(a2 + 32, v10, 32);
  v12 = 0;
  if ( *(char *)(a1 + 297) )
    v12 = a1 + 297;
  sub_25160(a2 + 40, v12, 32);
  sub_1D530((long long *)a2);
  if ( v8 == 2 )
  {
    if ( !byte_82B50 )
    {
      v18 = _mm_loadu_si128((const __m128i *)&tp);
      *(__m128i *)(a2 + 264) = v18;
      *(__m128i *)(a2 + 232) = v18;
      goto LABEL_11;
    }
    v21 = a1 + 357;
    *(long long *)(a2 + 232) = sub_21A20(
                              a1 + 345,
                              0xCu,
                              (long long)"time_t",
                              0x8000000000000000LL,
                              0x7FFFFFFFFFFFFFFFuLL,
                              0,
                              0);
LABEL_42:
    v12 = 12;
    v22 = sub_21A20(v21, 0xCu, (long long)"time_t", 0x8000000000000000LL, 0x7FFFFFFFFFFFFFFFuLL, 0, 0);
    *(long long *)(a2 + 272) = 0;
    *(long long *)(a2 + 264) = v22;
    *(long long *)(a2 + 240) = 0;
    goto LABEL_11;
  }
  if ( v8 == 5 )
  {
    v21 = a1 + 488;
    *(long long *)(a2 + 232) = sub_21A20(
                              a1 + 476,
                              0xCu,
                              (long long)"time_t",
                              0x8000000000000000LL,
                              0x7FFFFFFFFFFFFFFFuLL,
                              0,
                              0);
    goto LABEL_42;
  }
  v13 = _mm_loadu_si128((const __m128i *)&tp);
  *(__m128i *)(a2 + 264) = v13;
  *(__m128i *)(a2 + 232) = v13;
  if ( v8 == 1 )
  {
    *(int *)(a2 + 116) = sub_21A20(a1 + 108, 8u, (long long)"uid_t", 0, 0xFFFFFFFF, 0, 0);
    v12 = 8;
    *(int *)(a2 + 120) = sub_21A20(a1 + 116, 8u, (long long)"gid_t", 0, 0xFFFFFFFF, 0, 0);
    v15 = 0;
    goto LABEL_20;
  }
LABEL_11:
  if ( a4 )
  {
    if ( byte_82AD2 || !*(char *)(a1 + 265) || !(unsigned int)sub_27FD0((char *)(a1 + 265)) )
      *(int *)(a2 + 116) = sub_21A20(a1 + 108, 8u, (long long)"uid_t", 0, 0xFFFFFFFF, 0, 0);
    if ( byte_82AD2 || !*(char *)(a1 + 297) || (v12 = a2 + 120, !(unsigned int)sub_28080((char *)(a1 + 297))) )
    {
      v14 = sub_21A20(a1 + 116, 8u, (long long)"gid_t", 0, 0xFFFFFFFF, 0, 0);
      v12 = v23;
      *(int *)(a2 + 120) = v14;
    }
  }
  v15 = 0;
  if ( (unsigned int)(*(char *)(a1 + 156) - 51) <= 1 )
  {
    v12 = 8;
    v19 = sub_21A20(a1 + 337, 8u, (long long)"minor_t", 0xFFFFFFFF80000000LL, 0x7FFFFFFFu, 0, 0);
    v20 = sub_21A20(a1 + 329, 8u, (long long)"major_t", 0xFFFFFFFF80000000LL, 0x7FFFFFFFu, 0, 0);
    v15 = (v20 << 8) & 0xFFF00
        | (unsigned char)v19
        | (v19 << 12) & 0xFFFFFF00000LL
        | ((unsigned long long)v20 << 32) & 0xFFFFF00000000000LL;
  }
LABEL_20:
  *(long long *)(a2 + 128) = v15;
  sub_1D8A0(a2);
  if ( (unsigned char)sub_2B3A0(a2) )
  {
    result = sub_2B400(a2, v12);
    *(char *)(a2 + 288) = 1;
  }
  else
  {
    v16 = dword_82400;
    *(char *)(a2 + 288) = 0;
    result = v16 & 0xFFFFFFFB;
    if ( (int)result == 2 && (result = qword_82408, *(char *)(qword_82408 + 156) == 68) || *(long long *)(a2 + 400) )
      *(char *)(a2 + 392) = 1;
  }
  return result;
}



// Function: tartime @ 0x23240
char *tartime(time_t a1, unsigned int a2, char a3)
{
  unsigned int v3; // ebp
  struct tm *v5; // rcx
  char *result; // rax
  char *v7; // rax
  char *v8; // rcx
  char *i; // rdx
  long long v10; // rdx
  size_t v11; // rax
  char *v12; // [rsp+8h] [rbp-30h]
  time_t timer; // [rsp+10h] [rbp-28h] BYREF
  unsigned long long v14; // [rsp+18h] [rbp-20h]

  v3 = a2;
  v14 = __readfsqword(0x28u);
  timer = a1;
  if ( a2 && a1 < 0 )
  {
    timer = a1 + 1;
    v3 = 1000000000 - a2;
  }
  if ( byte_82B91 )
  {
    v5 = gmtime(&timer);
    if ( v5 )
    {
LABEL_6:
      if ( a3 )
      {
        strftime(byte_82420, 0x25u, "%Y-%m-%d %H:%M:%S", v5);
        v11 = strlen(byte_82420);
        sub_256A0(v3, &byte_82420[v11]);
      }
      else
      {
        strftime(byte_82420, 0x25u, "%Y-%m-%d %H:%M", v5);
      }
      return byte_82420;
    }
  }
  else
  {
    v5 = localtime(&timer);
    if ( v5 )
      goto LABEL_6;
  }
  if ( a1 < 0 )
  {
    v7 = (char *)(sub_45370(-timer, &unk_82426) - 1);
    *v7 = 45;
  }
  else
  {
    v7 = (char *)sub_45370(timer, &unk_82426);
  }
  v8 = (char *)&unk_82441;
  if ( !a3 )
    v8 = (char *)&unk_82441 - 13;
  for ( i = v7; i > v8; --i )
    *(i - 1) = 32;
  v10 = v8 - v7;
  if ( v8 > v7 )
    v10 = 0;
  result = &v7[v10];
  if ( a3 )
  {
    v12 = result;
    sub_256A0(v3, &unk_8243A);
    return v12;
  }
  return result;
}



// Function: simple_print_header @ 0x233e0
unsigned long long simple_print_header(FILE *a1, long long a2, long long a3, double a4)
{
  FILE *v4; // rbp
  char *IO_read_ptr; // r15
  long long v7; // r12
  char *v8; // rax
  int v9; // eax
  const char *IO_write_base; // r12
  const char *v11; // r14
  long long v12; // rax
  int v13; // esi
  int v14; // eax
  const char *v15; // rax
  char *v16; // rax
  long long v17; // rax
  unsigned long long v18; // rdi
  int v19; // eax
  long long v20; // rax
  FILE *v21; // r12
  const char *v22; // rax
  char *v23; // rax
  const char *v25; // rsi
  long long v26; // r12
  char *v27; // rax
  char *IO_write_ptr; // rax
  char *v29; // rax
  const char *v30; // rsi
  char *v31; // rax
  long long v32; // rax
  long long v33; // rax
  char *v34; // rax
  const char *v35; // rax
  char *v36; // [rsp+8h] [rbp-110h]
  int v37; // [rsp+10h] [rbp-108h]
  int v38; // [rsp+20h] [rbp-F8h]
  char v39[2]; // [rsp+32h] [rbp-E6h] BYREF
  char v40; // [rsp+34h] [rbp-E4h] BYREF
  char v41[9]; // [rsp+35h] [rbp-E3h] BYREF
  char v42[2]; // [rsp+3Eh] [rbp-DAh] BYREF
  char v43[32]; // [rsp+40h] [rbp-D8h] BYREF
  char v44[32]; // [rsp+60h] [rbp-B8h] BYREF
  char v45[32]; // [rsp+80h] [rbp-98h] BYREF
  char s[56]; // [rsp+A0h] [rbp-78h] BYREF
  unsigned long long v47; // [rsp+D8h] [rbp-40h]

  v4 = a1;
  v47 = __readfsqword(0x28u);
  if ( byte_827E1 )
  {
    IO_read_ptr = a1->_IO_read_ptr;
    if ( !IO_read_ptr )
      IO_read_ptr = *(char **)&a1->_flags;
  }
  else
  {
    IO_read_ptr = *(char **)&a1->_flags;
    if ( !*(long long *)&a1->_flags )
      IO_read_ptr = a1->_IO_read_ptr;
  }
  if ( byte_82B7C )
  {
    if ( a3 < 0 )
      a3 = sub_CC60(a1, a2);
    v7 = sub_45370(a3 - qword_823E0 - qword_823E8, s);
    v8 = dcgettext(0, "block %s: ", 5);
    a1 = qword_81F28;
    a4 = __fprintf_chk(qword_81F28, 1, v8, v7);
  }
  if ( dword_82A54 <= 1 )
  {
    v21 = qword_81F28;
    v22 = (const char *)sub_4CA70(IO_read_ptr);
    fputs_unlocked(v22, v21);
    if ( byte_827E1 && LOBYTE(v4->_IO_read_end) )
    {
      IO_write_ptr = qword_81F28->_IO_write_ptr;
      if ( IO_write_ptr >= qword_81F28->_IO_write_end )
      {
        __overflow(qword_81F28, 47);
      }
      else
      {
        qword_81F28->_IO_write_ptr = IO_write_ptr + 1;
        *IO_write_ptr = 47;
      }
    }
LABEL_35:
    v23 = qword_81F28->_IO_write_ptr;
    if ( v23 >= qword_81F28->_IO_write_end )
    {
      __overflow(qword_81F28, 10);
    }
    else
    {
      qword_81F28->_IO_write_ptr = v23 + 1;
      *v23 = 10;
    }
  }
  else
  {
    v40 = 63;
    switch ( *(char *)(a2 + 156) )
    {
      case 0:
      case 0x30:
      case 0x53:
        v40 = LOBYTE(v4->_IO_read_end) == 0 ? 45 : 100;
        break;
      case 0x31:
        v40 = 104;
        break;
      case 0x32:
        v40 = 108;
        break;
      case 0x33:
        v40 = 99;
        break;
      case 0x34:
        v40 = 98;
        break;
      case 0x35:
      case 0x44:
        v40 = 100;
        break;
      case 0x36:
        v40 = 112;
        break;
      case 0x37:
        v40 = 67;
        break;
      case 0x4B:
      case 0x4C:
        v40 = 76;
        if ( qword_82D50 )
          qword_82D50(a1);
        v29 = dcgettext(0, "Unexpected long name header", 5);
        error(0, 0, v29);
        dword_82D58 = 2;
        break;
      case 0x4D:
        v40 = 77;
        break;
      case 0x56:
        byte_82451 = 1;
        v40 = 86;
        break;
      default:
        break;
    }
    sub_329F0((unsigned int)v4->_fileno, v41);
    sub_326F0(v4, v42);
    v36 = sub_23240((time_t)v4[1]._IO_write_base, (unsigned int)v4[1]._IO_write_ptr, byte_82B90);
    v9 = strlen(v36);
    if ( dword_7F228 < v9 )
      dword_7F228 = v9;
    IO_write_base = v4->_IO_write_base;
    if ( !IO_write_base || !*IO_write_base || dword_82400 == 1 || byte_82AD2 )
      IO_write_base = (const char *)sub_45370((unsigned int)v4->_flags2, v43);
    v11 = v4->_IO_write_ptr;
    if ( !v11 || !*v11 || dword_82400 == 1 || byte_82AD2 )
      v11 = (const char *)sub_45370(LODWORD(v4->_old_offset), v44);
    if ( (unsigned int)(*(char *)(a2 + 156) - 51) <= 1 )
    {
      v17 = sub_45370(
              (*(long long *)&v4->_cur_column >> 8) & 0xFFF
            | (unsigned int)HIDWORD(*(long long *)&v4->_cur_column) & 0xFFFFF000,
              v45);
      __strcpy_chk(s, v17, 42);
      __strcat_chk(s, ",", 42);
      v18 = *(long long *)&v4->_cur_column;
      v19 = (unsigned char)v18;
      v18 >>= 12;
      LOBYTE(v18) = 0;
      v20 = sub_45370(v19 | (unsigned int)v18, v45);
      __strcat_chk(s, v20, 42);
    }
    else
    {
      v12 = sub_45370(v4->_lock, v45);
      __strcpy_chk(s, v12, 42);
    }
    v38 = strlen(s);
    v37 = strlen(IO_write_base);
    v13 = v38;
    v14 = v38 + v37 + strlen(v11) + 2;
    if ( dword_7F22C < v14 )
      dword_7F22C = v14;
    else
      v13 = dword_7F22C - v14 + v38;
    __fprintf_chk(qword_81F28, 1, "%s %s/%s %*s %-*s", &v40, IO_write_base, v11, v13, s, dword_7F228, v36);
    v15 = (const char *)sub_4CA70(IO_read_ptr);
    a4 = __fprintf_chk(qword_81F28, 1, " %s", v15);
    if ( byte_827E1 && LOBYTE(v4->_IO_read_end) )
    {
      v16 = qword_81F28->_IO_write_ptr;
      if ( v16 >= qword_81F28->_IO_write_end )
      {
        __overflow(qword_81F28, 47);
      }
      else
      {
        qword_81F28->_IO_write_ptr = v16 + 1;
        *v16 = 47;
      }
    }
    switch ( *(char *)(a2 + 156) )
    {
      case 0:
      case 0x30:
      case 0x33:
      case 0x34:
      case 0x35:
      case 0x36:
      case 0x37:
      case 0x44:
      case 0x53:
        goto LABEL_35;
      case 0x31:
        v25 = " link to %s\n";
        v26 = sub_4CA70(v4->_IO_read_base);
        goto LABEL_42;
      case 0x32:
        v35 = (const char *)sub_4CA70(v4->_IO_read_base);
        a4 = __fprintf_chk(qword_81F28, 1, " -> %s\n", v35);
        break;
      case 0x4B:
        v30 = "--Long Link--\n";
        goto LABEL_51;
      case 0x4C:
        v30 = "--Long Name--\n";
        goto LABEL_51;
      case 0x4D:
        v32 = sub_21A20(a2 + 369, 0xCu, (long long)"uintmax_t", 0, 0xFFFFFFFFFFFFFFFFLL, 0, 0);
        v33 = sub_45370(v32, v45);
        __strcpy_chk(s, v33, 42);
        v34 = dcgettext(0, "--Continued at byte %s--\n", 5);
        a4 = __fprintf_chk(qword_81F28, 1, v34, s, a4);
        break;
      case 0x56:
        v30 = "--Volume Header--\n";
LABEL_51:
        v31 = dcgettext(0, v30, 5);
        a4 = __fprintf_chk(qword_81F28, 1, v31, a4);
        break;
      default:
        v39[0] = *(char *)(a2 + 156);
        v39[1] = 0;
        v25 = " unknown file type %s\n";
        v26 = sub_4D2D0(v39);
LABEL_42:
        v27 = dcgettext(0, v25, 5);
        a4 = __fprintf_chk(qword_81F28, 1, v27, v26);
        break;
    }
  }
  fflush_unlocked(qword_81F28);
  sub_32830(v4, a4);
  return v47 - __readfsqword(0x28u);
}



// Function: print_volume_label @ 0x23ae0
unsigned long long print_volume_label(double a1)
{
  int v2; // [rsp+Ch] [rbp-3ECh] BYREF
  FILE v3[2]; // [rsp+10h] [rbp-3E8h] BYREF
  long long v4[65]; // [rsp+1D0h] [rbp-228h] BYREF
  unsigned long long v5; // [rsp+3D8h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  memset(v4, 0, 0x200u);
  BYTE4(v4[19]) = 86;
  if ( qword_82560 )
  {
    v4[17] = *(long long *)(qword_82560 + 136);
    LODWORD(v4[18]) = *(int *)(qword_82560 + 144);
  }
  sub_2FB40(v3);
  sub_25120(&v3[0]._IO_read_ptr, ".");
  sub_22C60((long long)v4, (long long)v3, &v2, 0);
  sub_25120(&v3[0]._IO_read_ptr, qword_81F18);
  sub_233E0(v3, (long long)v4, 0, a1);
  sub_2FC00(v3, v4);
  return v5 - __readfsqword(0x28u);
}



// Function: print_for_mkdir @ 0x23c30
unsigned long long print_for_mkdir(long long a1, long long a2, unsigned int a3)
{
  long long v5; // rdi
  const char *v6; // rbp
  char *v7; // rax
  long long v8; // rax
  long long v9; // r12
  char *v10; // rax
  char v11; // [rsp+5h] [rbp-43h] BYREF
  char v12[10]; // [rsp+6h] [rbp-42h] BYREF
  char v13[24]; // [rsp+10h] [rbp-38h] BYREF
  unsigned long long v14; // [rsp+28h] [rbp-20h]

  v14 = __readfsqword(0x28u);
  if ( dword_82A54 > 1 )
  {
    v5 = a3;
    v11 = 100;
    sub_329F0(a3, v12);
    if ( byte_82B7C )
    {
      v8 = sub_CC60(v5, v12);
      v9 = sub_45370(v8, v13);
      v10 = dcgettext(0, "block %s: ", 5);
      __fprintf_chk(qword_81F28, 1, v10, v9);
    }
    v6 = (const char *)sub_4CA70(a1);
    v7 = dcgettext(0, "Creating directory:", 5);
    __fprintf_chk(qword_81F28, 1, "%s %*s %s\n", &v11, dword_7F22C + dword_7F228 + 1, v7, v6);
  }
  return v14 - __readfsqword(0x28u);
}



// Function: skip_file @ 0x23d40
void *skip_file(signed long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // rdi
  void *result; // rax
  long long v5; // rdx
  long long v6; // rcx
  long long v7; // r8
  unsigned long long v8; // rbp
  long long v9; // rdi
  __off_t v10; // rax
  char *v11; // rax

  v2 = a1;
  if ( byte_827F8 )
  {
    v10 = sub_D780(a1);
    if ( v10 < 0 )
      byte_827F8 = 0;
    else
      v2 = a1 - (v10 << 9);
  }
  v3 = v2;
  result = sub_C710(v2);
  if ( v2 > 0 )
  {
    v8 = v2 - 512 - ((v2 - 1) & 0xFFFFFFFFFFFFFE00LL);
    do
    {
      v9 = sub_D360(v3, a2, v5, v6, v7);
      if ( !v9 )
      {
        if ( qword_82D50 )
          qword_82D50(0);
        v11 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(0, 0, v11);
        sub_16FD0();
      }
      sub_CCD0(v9);
      v2 -= 512;
      v3 = v2;
      result = sub_C710(v2);
    }
    while ( v2 != v8 );
  }
  return result;
}



// Function: read_and @ 0x23e80
long long read_and(void (*a1)(void))
{
  long long v1; // rdx
  long long *v2; // rsi
  long long v3; // rax
  double v4; // xmm0_8
  int v5; // r13d
  long long *p_flags; // rdi
  long long v7; // rcx
  long long v8; // r8
  long long v9; // rdx
  long long v11; // rcx
  long long v12; // r8
  long long v13; // rax
  long long v14; // r12
  char *v15; // rax
  char *v16; // rdi
  char v17; // al
  long long v18; // rcx
  long long v19; // r8
  FILE *v20; // rdi
  char *v21; // rax
  char *v22; // rax
  long long v23; // rcx
  long long v24; // r8
  long long v25; // rcx
  long long v26; // r8
  long long v27; // rax
  long long v28; // rdi
  long long v29; // rcx
  long long v30; // r8
  long long v31; // rax
  long long v32; // r12
  char *v33; // rax
  long long v34; // rax
  long long v35; // r13
  char *v36; // rax
  long long v37; // rcx
  long long v38; // r8
  long long v39; // rax
  char *v40; // rax
  char *v41; // rax
  long long v42; // rcx
  long long v43; // r8
  long long *v44; // [rsp-8h] [rbp-70h]
  long long v46; // [rsp+8h] [rbp-60h]
  long long v47; // [rsp+8h] [rbp-60h]
  char v48[24]; // [rsp+10h] [rbp-58h] BYREF
  unsigned long long v49; // [rsp+28h] [rbp-40h]

  v1 = 65;
  v2 = (long long *)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s";
  v49 = __readfsqword(0x28u);
  v3 = 0;
  xmmword_82460 = (__int128)_mm_load_si128((const __m128i *)"@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s");
  xmmword_82470 = xmmword_82460;
  xmmword_82480 = xmmword_82460;
  xmmword_82490 = xmmword_82460;
  xmmword_824A0 = xmmword_82460;
  xmmword_824B0 = xmmword_82460;
  xmmword_824C0 = xmmword_82460;
  xmmword_824D0 = xmmword_82460;
  xmmword_824E0 = xmmword_82460;
  xmmword_824F0 = xmmword_82460;
  xmmword_82500 = xmmword_82460;
  xmmword_82510 = xmmword_82460;
  xmmword_82520 = xmmword_82460;
  xmmword_82530 = xmmword_82460;
  xmmword_82540 = xmmword_82460;
  xmmword_82550 = xmmword_82460;
  while ( 1 )
  {
    *((char *)&xmmword_82460 + v1) = v3++;
    if ( v3 == 64 )
      break;
    v1 = aAbcdefghijklmn[v3];
  }
  v4 = sub_282F0(
         a1,
         "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s",
         v1,
         &xmmword_82460);
  v5 = 0;
  sub_F710(
    0,
    (long long)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s",
    v4);
  while ( 2 )
  {
    sub_2FC00(&qword_82840, v2);
    v2 = &qword_82840;
    p_flags = &qword_82408;
    switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v7, v8) )
    {
      case 0u:
      case 2u:
LABEL_56:
        abort();
      case 1u:
LABEL_18:
        while ( 2 )
        {
          while ( 2 )
          {
            v2 = &qword_82840;
            sub_22C60(qword_82408, (long long)&qword_82840, &dword_82400, 1);
            v16 = file;
            if ( (unsigned char)sub_28880(file) )
            {
              if ( xmmword_82B00 < 0
                || (v16 = (char *)(qword_82408 + 136),
                    v27 = sub_21A20(
                            qword_82408 + 136,
                            0xCu,
                            (long long)"time_t",
                            0x8000000000000000LL,
                            0x7FFFFFFFFFFFFFFFuLL,
                            0,
                            0),
                    v2 = v44,
                    *(_OWORD *)&xmmword_82938 = (unsigned long long)v27,
                    (int)((*((long long *)&xmmword_82B00 + 1) >> 63)
                        - (*((long long *)&xmmword_82B00 + 1) > 0LL)
                        + 2 * (((long long)xmmword_82B00 < v27) - ((long long)xmmword_82B00 > v27))) >= 0) )
              {
                v2 = (long long *)qword_829D8;
                v16 = file;
                if ( !(unsigned char)sub_176D0(file, qword_829D8) )
                {
                  v28 = (unsigned int)*(char *)(qword_82408 + 156);
                  if ( (int)v28 == 86 )
                    goto LABEL_42;
                  goto LABEL_55;
                }
              }
            }
            v17 = *(char *)(qword_82408 + 156);
            if ( v17 == 77 )
            {
              v28 = 77;
LABEL_55:
              v2 = &qword_82840;
              sub_222E0(v28, &qword_82840);
LABEL_42:
              a1();
              p_flags = &qword_82840;
              if ( !(unsigned char)sub_289E0(&qword_82840) )
              {
                sub_2FC00(&qword_82840, v2);
                v2 = &qword_82840;
                p_flags = &qword_82408;
                switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v29, v30) )
                {
                  case 0u:
                  case 2u:
                    goto LABEL_56;
                  case 1u:
                    continue;
                  case 3u:
                    goto LABEL_48;
                  case 4u:
                    goto LABEL_6;
                  case 5u:
                    goto LABEL_49;
                  default:
                    goto LABEL_8;
                }
              }
              goto LABEL_8;
            }
            break;
          }
          if ( v17 == 86 )
            goto LABEL_42;
          if ( v17 == 53 && byte_82A89 )
          {
            if ( qword_82D50 )
              qword_82D50(v16);
            v47 = sub_4CE40(file);
            v41 = dcgettext(0, "%s: Omitting", 5);
            v2 = 0;
            error(0, 0, v41, v47);
          }
          if ( byte_829C9 )
          {
            p_flags = &qword_82840;
            if ( !(unsigned char)sub_289E0(&qword_82840) )
            {
              sub_2FC00(&qword_82840, v2);
              v2 = &qword_82840;
              p_flags = &qword_82408;
              switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v18, v19) )
              {
                case 0u:
                case 2u:
                  goto LABEL_56;
                case 1u:
                  continue;
                case 3u:
                  goto LABEL_48;
                case 4u:
                  goto LABEL_6;
                case 5u:
                  goto LABEL_49;
                default:
                  goto LABEL_8;
              }
            }
          }
          else
          {
            sub_23E00();
            p_flags = &qword_82840;
            if ( !(unsigned char)sub_289E0(&qword_82840) )
            {
              sub_2FC00(&qword_82840, v2);
              v2 = &qword_82840;
              p_flags = &qword_82408;
              switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v37, v38) )
              {
                case 0u:
                case 2u:
                  goto LABEL_56;
                case 1u:
                  continue;
                case 3u:
LABEL_48:
                  v5 = 1;
                  goto LABEL_10;
                case 4u:
                  goto LABEL_6;
                case 5u:
LABEL_49:
                  v20 = (FILE *)qword_82408;
                  sub_CCD0(qword_82408);
                  goto LABEL_29;
                default:
                  goto LABEL_8;
              }
            }
          }
          break;
        }
LABEL_8:
        sub_D8D0((long long)p_flags, (long long)v2, v9);
        return sub_28A70();
      case 3u:
LABEL_10:
        if ( byte_82B7C )
        {
          v39 = sub_CC60(&qword_82408, &qword_82840);
          v46 = sub_45370(v39, v48);
          v40 = dcgettext(0, "block %s: ** Block of NULs **\n", 5);
          v2 = (long long *)(&dword_0 + 1);
          __fprintf_chk(qword_81F28, 1, v40, v46);
        }
        sub_CCD0(qword_82408);
        if ( !byte_82B51 )
        {
          v2 = &qword_82840;
          p_flags = &qword_82408;
          if ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v11, v12) != 3
            && (dword_81B80 & 1) != 0 )
          {
            if ( qword_82D50 )
              qword_82D50(&qword_82408);
            v13 = sub_CC60(&qword_82408, &qword_82840);
            v14 = sub_45370(v13, v48);
            v15 = dcgettext(0, "A lone zero block at %s", 5);
            v2 = 0;
            p_flags = 0;
            error(0, 0, v15, v14);
          }
          goto LABEL_8;
        }
        p_flags = &qword_82840;
        if ( (unsigned char)sub_289E0(&qword_82840) )
          goto LABEL_8;
        continue;
      case 4u:
LABEL_6:
        if ( byte_82B7C )
        {
          v31 = sub_CC60(&qword_82408, &qword_82840);
          v32 = sub_45370(v31, v48);
          v33 = dcgettext(0, "block %s: ** End of File **\n", 5);
          p_flags = (long long *)&qword_81F28->_flags;
          v2 = (long long *)(&dword_0 + 1);
          __fprintf_chk(qword_81F28, 1, v33, v32);
        }
        goto LABEL_8;
      case 5u:
        v20 = (FILE *)qword_82408;
        sub_CCD0(qword_82408);
        if ( v5 )
        {
          if ( v5 != 1 )
          {
LABEL_36:
            p_flags = &qword_82840;
            if ( !(unsigned char)sub_289E0(&qword_82840) )
            {
              sub_2FC00(&qword_82840, &qword_82840);
              v2 = &qword_82840;
              p_flags = &qword_82408;
              switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v25, v26) )
              {
                case 0u:
                case 2u:
                  goto LABEL_56;
                case 1u:
                  goto LABEL_18;
                case 3u:
                  goto LABEL_9;
                case 4u:
                  goto LABEL_6;
                case 5u:
                  sub_CCD0(qword_82408);
                  p_flags = &qword_82840;
                  if ( !(unsigned char)sub_289E0(&qword_82840) )
                  {
                    sub_2FC00(&qword_82840, &qword_82840);
                    v2 = &qword_82840;
                    p_flags = &qword_82408;
                    switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v42, v43) )
                    {
                      case 0u:
                      case 2u:
                        goto LABEL_56;
                      case 1u:
                        goto LABEL_18;
                      case 3u:
                        goto LABEL_9;
                      case 4u:
                        goto LABEL_6;
                      case 5u:
                        goto LABEL_59;
                      default:
                        goto LABEL_8;
                    }
                  }
                  goto LABEL_8;
                default:
                  goto LABEL_8;
              }
            }
            goto LABEL_8;
          }
        }
        else
        {
          if ( qword_82D50 )
            qword_82D50(v20);
          v21 = dcgettext(0, "This does not look like a tar archive", 5);
          v2 = 0;
          v20 = 0;
          error(0, 0, v21);
          dword_82D58 = 2;
        }
LABEL_29:
        if ( byte_82B7C )
        {
          v34 = sub_CC60(v20, v2);
          v35 = sub_45370(v34 - qword_823E0 - qword_823E8, v48);
          v36 = dcgettext(0, "block %s: ", 5);
          v20 = qword_81F28;
          __fprintf_chk(qword_81F28, 1, v36, v35);
        }
        if ( qword_82D50 )
          qword_82D50(v20);
        v22 = dcgettext(0, "Skipping to next header", 5);
        v2 = 0;
        error(0, 0, v22);
        p_flags = &qword_82840;
        dword_82D58 = 2;
        if ( !(unsigned char)sub_289E0(&qword_82840) )
        {
          sub_2FC00(&qword_82840, 0);
          v2 = &qword_82840;
          p_flags = &qword_82408;
          switch ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v23, v24) )
          {
            case 0u:
            case 2u:
              goto LABEL_56;
            case 1u:
              goto LABEL_18;
            case 3u:
LABEL_9:
              v5 = 5;
              goto LABEL_10;
            case 4u:
              goto LABEL_6;
            case 5u:
LABEL_59:
              sub_CCD0(qword_82408);
              goto LABEL_36;
            default:
              goto LABEL_8;
          }
        }
        goto LABEL_8;
      default:
        goto LABEL_8;
    }
  }
}



// Function: list_archive @ 0x24580
void list_archive(long long a1, long long a2, double a3)
{
  long long v3; // rax
  char *v4; // rsi
  long long v5; // rax
  long long v6; // [rsp+0h] [rbp-18h]
  long long v7; // [rsp+8h] [rbp-10h]

  v3 = sub_CC60(a1, a2);
  if ( dword_82A54 )
  {
    v4 = (char *)qword_82408;
    if ( dword_82400 == 4 && !byte_82451 && qword_81F18 )
    {
      v7 = v3;
      v6 = qword_82408;
      sub_23AE0(a3);
      byte_82451 = 1;
      v3 = v7;
      v4 = (char *)v6;
    }
    sub_233E0((FILE *)&qword_82840, (long long)v4, v3, a3);
    if ( byte_82B50 && dword_82A54 > 2 && (unsigned char)sub_20FC0((long long)&qword_82840, v4) )
    {
      v5 = sub_1F690(qword_829D0);
      sub_21850((long long)qword_829D0, v5);
    }
  }
  if ( !byte_829C9 )
    sub_23E00();
}



// Function: test_archive_label @ 0x24680
long long test_archive_label(long long a1)
{
  long long v1; // rax
  long long v2; // rdx
  double v3; // xmm0_8
  long long *v4; // rsi
  long long *v5; // rdi
  long long v6; // rcx
  long long v7; // r8
  long long v8; // rdx
  void *v10; // rbp

  v1 = 0;
  v2 = 65;
  xmmword_82460 = (__int128)_mm_load_si128((const __m128i *)"@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s");
  xmmword_82470 = xmmword_82460;
  xmmword_82480 = xmmword_82460;
  xmmword_82490 = xmmword_82460;
  xmmword_824A0 = xmmword_82460;
  xmmword_824B0 = xmmword_82460;
  xmmword_824C0 = xmmword_82460;
  xmmword_824D0 = xmmword_82460;
  xmmword_824E0 = xmmword_82460;
  xmmword_824F0 = xmmword_82460;
  xmmword_82500 = xmmword_82460;
  xmmword_82510 = xmmword_82460;
  xmmword_82520 = xmmword_82460;
  xmmword_82530 = xmmword_82460;
  xmmword_82540 = xmmword_82460;
  xmmword_82550 = xmmword_82460;
  while ( 1 )
  {
    *((char *)&xmmword_82460 + v2) = v1++;
    if ( v1 == 64 )
      break;
    v2 = aAbcdefghijklmn[v1];
  }
  v3 = sub_282F0(
         a1,
         "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s",
         v2,
         &xmmword_82460);
  sub_F710(
    0,
    (long long)"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/@@@@@@@@@@@@@@@@%s:%u: invalid %s: %s",
    v3);
  v4 = &qword_82840;
  v5 = &qword_82408;
  if ( (unsigned int)sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v6, v7) == 1 )
  {
    sub_22C60(qword_82408, (long long)&qword_82840, &dword_82400, 0);
    v4 = (long long *)qword_82408;
    if ( *(char *)(qword_82408 + 156) == 86 )
      sub_25160(&qword_81F18, qword_82408, 100);
    v5 = (long long *)qword_81F18;
    if ( qword_81F18 )
    {
      if ( dword_82A54 )
      {
        sub_23AE0(v3);
        v5 = (long long *)qword_81F18;
      }
      if ( !(unsigned char)sub_28880(v5) && byte_82B10 )
      {
        v10 = sub_F3F0(qword_81F18);
        sub_28880(v10);
        v5 = (long long *)v10;
        free(v10);
      }
    }
  }
  sub_D8D0((long long)v5, (long long)v4, v8);
  return sub_28C00();
}



// Function: map_hash @ 0x247f0
unsigned long long map_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: map_compare @ 0x24800
bool map_compare(long long *a1, long long *a2)
{
  return *a1 == *a2;
}



// Function: name_to_uid @ 0x24810
long long name_to_uid(const char *a1)
{
  struct passwd *v1; // rax

  v1 = getpwnam(a1);
  if ( v1 )
    return v1->pw_uid;
  else
    return -1;
}



// Function: name_to_gid @ 0x24840
long long name_to_gid(const char *a1)
{
  struct group *v1; // rax

  v1 = getgrnam(a1);
  if ( v1 )
    return v1->gr_gid;
  else
    return -1;
}



// Function: owner_map_read @ 0x24de0
long long owner_map_read(long long a1)
{
  return sub_24950(&qword_82570, a1, sub_24810, "UID");
}



// Function: owner_map_translate @ 0x24e10
long long owner_map_translate(unsigned int a1, int *a2, long long *a3)
{
  long long v4; // rax
  long long v5; // rdx
  long long v6; // rax
  long long result; // rax
  long long v8[7]; // [rsp+0h] [rbp-38h] BYREF

  v8[3] = __readfsqword(0x28u);
  if ( qword_82570 && (v8[0] = a1, (v4 = sub_43960(qword_82570, v8)) != 0) )
  {
    v5 = *(long long *)(v4 + 8);
    v6 = *(long long *)(v4 + 16);
    *a2 = v5;
    *a3 = v6;
    return 0;
  }
  else
  {
    result = 1;
    if ( dword_82ABC != -1 )
    {
      *a2 = dword_82ABC;
      result = 0;
    }
    if ( qword_82AC0 )
    {
      *a3 = qword_82AC0;
      return 0;
    }
  }
  return result;
}



// Function: group_map_read @ 0x24eb0
long long group_map_read(long long a1)
{
  return sub_24950(&qword_82568, a1, sub_24840, "GID");
}



// Function: group_map_translate @ 0x24ee0
long long group_map_translate(unsigned int a1, int *a2, long long *a3)
{
  long long v4; // rax
  long long v5; // rdx
  long long v6; // rax
  long long result; // rax
  long long v8[7]; // [rsp+0h] [rbp-38h] BYREF

  v8[3] = __readfsqword(0x28u);
  if ( qword_82568 && (v8[0] = a1, (v4 = sub_43960(qword_82568, v8)) != 0) )
  {
    v5 = *(long long *)(v4 + 8);
    v6 = *(long long *)(v4 + 16);
    *a2 = v5;
    *a3 = v6;
    return 0;
  }
  else
  {
    result = 1;
    if ( dword_82B54 != -1 )
    {
      *a2 = dword_82B54;
      result = 0;
    }
    if ( qword_82B58 )
    {
      *a3 = qword_82B58;
      return 0;
    }
  }
  return result;
}



// Function: quote_n_colon @ 0x250f0
long long quote_n_colon(unsigned int a1, long long a2)
{
  unsigned int v2; // eax

  v2 = sub_4C740(0);
  return sub_4CF60(a1, v2, a2);
}



// Function: assign_string_n @ 0x25160
void assign_string_n(void **a1, const char *a2, size_t a3)
{
  size_t v4; // r12
  void *v5; // rax
  char *v6; // rax

  free(*a1);
  if ( a2 )
  {
    v4 = strnlen(a2, a3);
    v5 = (void *)sub_4ECC0(v4 + 1);
    v6 = memcpy(v5, a2, v4);
    v6[v4] = 0;
    *a1 = v6;
  }
}



// Function: unquote_string @ 0x251b0
long long unquote_string(char *a1)
{
  char v1; // dl
  char *v2; // rax
  unsigned int v3; // r8d
  char v4; // si
  char v6; // dl
  char v7; // si
  char v8; // si
  char v9; // dl

  v1 = *a1;
  v2 = a1;
  if ( !*a1 )
    return 1;
  v3 = 1;
  while ( v1 != 92 )
  {
    if ( v2 == a1 )
    {
      v1 = v2[1];
      ++a1;
      ++v2;
      if ( !v1 )
        goto LABEL_9;
    }
    else
    {
      *v2++ = v1;
      v1 = *++a1;
LABEL_5:
      if ( !v1 )
        goto LABEL_9;
    }
  }
  v4 = a1[1];
  switch ( v4 )
  {
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
      v7 = v4 - 48;
      if ( (unsigned char)(a1[2] - 48) > 7u )
      {
        *v2 = v7;
        a1 += 2;
        v1 = *a1;
        ++v2;
      }
      else
      {
        v8 = a1[2] + 8 * v7 - 48;
        v9 = a1[3];
        if ( (unsigned char)(v9 - 48) <= 7u )
        {
          a1 += 4;
          *v2++ = v9 + 8 * v8 - 48;
          v1 = *a1;
        }
        else
        {
          *v2 = v8;
          a1 += 3;
          v1 = *a1;
          ++v2;
        }
      }
      goto LABEL_5;
    case '?':
      *v2 = 127;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case '\\':
      *v2 = 92;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'a':
      *v2 = 7;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'b':
      *v2 = 8;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'f':
      *v2 = 12;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'n':
      *v2 = 10;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'r':
      *v2 = 13;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 't':
      *v2 = 9;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    case 'v':
      *v2 = 11;
      a1 += 2;
      v1 = *a1;
      ++v2;
      goto LABEL_5;
    default:
      *v2 = 92;
      v6 = a1[1];
      if ( v6 )
      {
        v2[1] = v6;
        a1 += 2;
        v1 = *a1;
        v2 += 2;
        v3 = 0;
        goto LABEL_5;
      }
      ++v2;
      ++a1;
      v3 = 0;
LABEL_9:
      if ( a1 != v2 )
        *v2 = 0;
      return v3;
  }
}



// Function: zap_slashes @ 0x253a0
const char *zap_slashes(const char *a1)
{
  const char *v1; // rax

  if ( a1 )
  {
    if ( *a1 )
    {
      v1 = &a1[strlen(a1) - 1];
      if ( a1 < v1 )
      {
        do
        {
          if ( *v1 != 47 )
            break;
          *v1-- = 0;
        }
        while ( a1 != v1 );
      }
    }
  }
  return a1;
}



// Function: normalize_filename_x @ 0x253e0
long long normalize_filename_x(char *a1)
{
  char v1; // dl
  char *i; // rax
  char *v3; // rsi
  char v4; // dl
  char *v5; // rcx
  char *v6; // rax
  char *j; // rcx
  long long result; // rax

  v1 = *a1;
  i = a1;
  v3 = a1;
  if ( *a1 == 46 )
  {
    do
    {
      if ( i[1] != 47 )
        break;
      v4 = i[2];
      v5 = i + 2;
      i += 2;
      if ( v4 == 47 )
      {
        v4 = v5[1];
        for ( i = v5 + 1; v4 == 47; ++i )
          v4 = i[1];
      }
      v3 += v4 == 0;
      *v3 = v4;
    }
    while ( v4 == 46 );
    v1 = *i;
  }
  *v3 = v1;
  v6 = i + 1;
  for ( j = v3 + 1; v1; *(j - 1) = v1 )
  {
    while ( v1 != 47 )
    {
      v1 = *v6;
LABEL_13:
      ++j;
      ++v6;
      *(j - 1) = v1;
      if ( !v1 )
        goto LABEL_18;
    }
    v1 = *v6;
    if ( v6[*v6 == 46] != 47 )
      goto LABEL_13;
    do
    {
      v6 += (v1 == 46) + 1;
      v1 = *v6;
    }
    while ( v6[*v6 == 46] == 47 );
    ++j;
    ++v6;
  }
LABEL_18:
  result = j - a1;
  if ( j - a1 <= 2 )
    return result;
  result = (unsigned char)*(j - 2);
  if ( (char)result == 46 )
  {
    if ( *(j - 3) != 47 )
      goto LABEL_22;
    result = --j - a1;
    if ( j - a1 <= 2 )
      goto LABEL_22;
    goto LABEL_21;
  }
  if ( (char)result == 47 )
LABEL_21:
    --j;
LABEL_22:
  *(j - 1) = 0;
  return result;
}



// Function: replace_prefix @ 0x25500
int replace_prefix(const char **a1, const void *a2, size_t a3, const void *a4, size_t a5)
{
  char *v7; // r13
  size_t v8; // rax
  size_t v9; // rbp
  long long v10; // rdx
  long long v11; // rcx
  char *v12; // rsi
  long long v13; // rax

  v7 = (char *)*a1;
  v8 = strlen(*a1);
  if ( v8 > a3 )
  {
    v9 = v8;
    LODWORD(v8) = memcmp(v7, a2, a3);
    if ( !(int)v8 )
    {
      v12 = &v7[a3];
      if ( v7[a3] == 47 )
      {
        if ( a3 < a5 )
        {
          v13 = sub_4ECE0(v7, v9 - a3 + a5 + 1, v10, v11);
          *a1 = (const char *)v13;
          v7 = (char *)v13;
          v12 = (char *)(v13 + a3);
        }
        memmove(&v7[a5], v12, v9 - a3 + 1);
        LODWORD(v8) = (unsigned int)memcpy(v7, a4, a5);
      }
    }
  }
  return v8;
}



// Function: sysinttostr @ 0x255c0
long long sysinttostr(unsigned long long a1, long long a2, unsigned long long a3, long long a4)
{
  if ( a1 > a3 )
    return sub_45230(a1, a4);
  else
    return sub_45370(a1, a4);
}



// Function: strtosysint @ 0x255e0
long long strtosysint(char *nptr, char **endptr, long long a3, long long a4)
{
  int *v6; // rax
  int *v7; // rbx
  long long result; // rax
  bool v9; // cc

  v6 = __errno_location();
  *v6 = 0;
  v7 = v6;
  if ( a4 < 0 )
  {
    if ( (unsigned int)(*nptr - 48) > 9 )
      goto LABEL_3;
    result = strtoumax(nptr, endptr, 10);
    if ( a4 < (unsigned long long)result )
    {
      *v7 = 34;
      return a4;
    }
  }
  else
  {
    if ( (unsigned int)(nptr[*nptr == 45] - 48) > 9 )
    {
LABEL_3:
      *v6 = 22;
      return 0;
    }
    result = strtoimax(nptr, endptr, 10);
    if ( result < a3 || a4 < result )
    {
      v9 = result < a3;
      result = a4;
      *v7 = 34;
      if ( v9 )
        return a3;
    }
  }
  return result;
}



// Function: code_timespec @ 0x257a0
long long code_timespec(long long a1, unsigned int a2, char *a3)
{
  long long v3; // rax
  int v5; // ebx
  long long v6; // rax
  long long v7; // r8
  long long v8; // rax
  int v9; // esi
  int v10; // edi
  int v11; // edi
  long long v12; // rcx

  v3 = a1;
  if ( a2 > 0x3B9AC9FF )
  {
    if ( a1 >= 0 )
    {
      v7 = sub_45370(a1, a3 + 1);
      goto LABEL_15;
    }
    v5 = 0;
    goto LABEL_5;
  }
  v5 = a2;
  if ( a2 && a1 < 0 )
  {
    v3 = a1 + 1;
    v5 = 1000000000 - a2;
LABEL_5:
    v6 = sub_45370(-v3, a3 + 1);
    *(char *)(v6 - 1) = 45;
    v7 = v6 - 1;
    goto LABEL_6;
  }
  if ( a1 < 0 )
    goto LABEL_5;
  v7 = sub_45370(a1, a3 + 1);
LABEL_6:
  if ( !v5 )
  {
LABEL_15:
    a3[21] = 0;
    return v7;
  }
  v8 = v5;
  a3[21] = 46;
  if ( v5 % 10 )
  {
    a3[31] = 0;
    v12 = 8;
    a3[30] = v5 % 10 + 48;
    goto LABEL_11;
  }
  v9 = 9;
  do
  {
    v10 = v9--;
    v5 = ((int)((unsigned long long)(1717986919 * v8) >> 32) >> 2) - (v5 >> 31);
    v8 = v5;
  }
  while ( !(v5 % 10) );
  v11 = v10 - 2;
  v12 = v11;
  a3[v9 + 22] = 0;
  a3[v11 + 22] = v5 % 10 + 48;
  if ( !v11 )
    return v7;
  do
  {
LABEL_11:
    v5 = ((int)((unsigned long long)(1717986919 * v8) >> 32) >> 2) - (v5 >> 31);
    v8 = v5;
    a3[v12-- + 21] = v5 % 10 + 48;
  }
  while ( (int)v12 );
  return v7;
}



// Function: decode_timespec @ 0x25930
long long decode_timespec(char *nptr, char **endptr, char a3)
{
  char v4; // r14
  int *v6; // r12
  signed long long v8; // r8
  char *v9; // rbp
  char *v10; // rax
  int v11; // esi
  char v12; // cl
  unsigned char v13; // r9
  int v14; // eax
  int v15; // edx
  intmax_t v16; // rax

  v4 = *nptr;
  v6 = __errno_location();
  if ( (unsigned int)(nptr[v4 == 45] - 48) > 9 )
  {
    *v6 = 22;
    *endptr = nptr;
    return 0x8000000000000000LL;
  }
  *v6 = 0;
  if ( v4 == 45 )
  {
    v16 = strtoimax(nptr, endptr, 10);
    v9 = *endptr;
    v8 = v16;
    if ( !a3 )
      goto LABEL_16;
LABEL_6:
    v10 = v9;
    if ( *v9 == 46 )
      goto LABEL_7;
LABEL_16:
    *endptr = v9;
    return v8;
  }
  v8 = strtoumax(nptr, endptr, 10);
  if ( v8 >= 0 )
  {
    v9 = *endptr;
    if ( !a3 )
      goto LABEL_16;
    goto LABEL_6;
  }
  *v6 = 34;
  v9 = *endptr;
  if ( a3 && *v9 == 46 )
  {
    v10 = *endptr;
    v8 = 0x8000000000000000LL;
LABEL_7:
    v11 = v10[1];
    ++v9;
    v12 = v10[1];
    if ( (unsigned int)(v11 - 48) > 9 )
    {
      v15 = 0;
      v13 = 0;
      v14 = 0;
    }
    else
    {
      v13 = 0;
      v14 = 0;
      v15 = 0;
      do
      {
        if ( v14 <= 8 )
        {
          ++v14;
          v15 = v11 + 10 * v15 - 48;
        }
        else
        {
          v13 |= v12 != 48;
        }
        v11 = *++v9;
        v12 = v11;
      }
      while ( (unsigned int)(v11 - 48) <= 9 );
      if ( v14 > 8 )
        goto LABEL_15;
    }
    do
    {
      ++v14;
      v15 *= 10;
    }
    while ( v14 != 9 );
LABEL_15:
    if ( v4 == 45 && v13 + v15 && v8 != 0x8000000000000000LL )
      --v8;
    goto LABEL_16;
  }
  *endptr = v9;
  return 0x8000000000000000LL;
}



// Function: maybe_backup_file @ 0x25c10
long long maybe_backup_file(char *src, char a2)
{
  char *v2; // rax
  char *v3; // rax
  char *v5; // rax
  char *v6; // rbp
  long long v7; // rdi
  int v8; // r13d
  long long v9; // rbx
  long long v10; // r12
  char *v11; // rax
  long long v12; // rbx
  long long v13; // r12
  char *v14; // rax
  struct stat v15; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v16; // [rsp+98h] [rbp-30h]

  v16 = __readfsqword(0x28u);
  free(new);
  v2 = 0;
  if ( src )
    v2 = (char *)sub_4EE30(src);
  new = v2;
  free(old);
  old = 0;
  if ( a2 )
  {
    if ( !byte_82D70 )
    {
      v3 = strchr(src, 58);
      qword_82D78 = (long long)v3;
      if ( v3 )
      {
        if ( v3 > src && !memchr(src, 47, v3 - src) )
          return 1;
      }
    }
  }
  if ( fstatat(oldfd, src, &v15, flag) )
  {
    if ( *__errno_location() != 2 )
    {
      sub_33410(src);
      return 0;
    }
    return 1;
  }
  if ( (v15.st_mode & 0xF000) == 0x4000 || a2 && (v15.st_mode & 0xB000) == 0x2000 )
    return 1;
  v5 = (char *)sub_3FFD0((unsigned int)oldfd, src, (unsigned int)dword_82B80);
  old = v5;
  v6 = v5;
  if ( !v5 )
    sub_17020();
  v7 = (unsigned int)oldfd;
  if ( !renameat(oldfd, new, oldfd, v5) )
  {
    if ( dword_82A54 )
    {
      v12 = sub_4D2B0(1, v6);
      v13 = sub_4D2B0(0, new);
      v14 = dcgettext(0, "Renaming %s to %s\n", 5);
      __fprintf_chk(qword_81F28, 1, v14, v13, v12);
    }
    return 1;
  }
  v8 = *__errno_location();
  if ( qword_82D50 )
  {
    qword_82D50(v7);
    v6 = old;
  }
  v9 = sub_4D2B0(1, v6);
  v10 = sub_4CE40(new);
  v11 = dcgettext(0, "%s: Cannot rename to %s", 5);
  error(0, v8, v11, v10, v9);
  dword_82D58 = 2;
  free(old);
  old = 0;
  return 0;
}



// Function: undo_last_backup @ 0x25ea0
void undo_last_backup()
{
  long long v0; // rdi
  int v1; // ebp
  long long v2; // rbx
  long long v3; // r12
  char *v4; // rax
  long long v5; // rbx
  long long v6; // r12
  char *v7; // rax

  if ( old )
  {
    v0 = (unsigned int)oldfd;
    if ( renameat(oldfd, old, oldfd, new) )
    {
      v1 = *__errno_location();
      if ( qword_82D50 )
        qword_82D50(v0);
      v2 = sub_4D2B0(1, new);
      v3 = sub_4CE40(old);
      v4 = dcgettext(0, "%s: Cannot rename to %s", 5);
      error(0, v1, v4, v3, v2);
      dword_82D58 = 2;
    }
    if ( dword_82A54 )
    {
      v5 = sub_4D2B0(1, new);
      v6 = sub_4D2B0(0, old);
      v7 = dcgettext(0, "Renaming %s back to %s\n", 5);
      __fprintf_chk(qword_81F28, 1, v7, v6, v5);
    }
    free(old);
    old = 0;
  }
}



// Function: blocking_read @ 0x25fe0
long long blocking_read(int fd, void *buf, size_t nbytes)
{
  long long v4; // r12
  int v6; // edx
  int v7; // ecx
  int v8; // r8d
  int v9; // r9d
  int v10; // eax
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d

  v4 = sub_4D570(fd, buf, nbytes);
  if ( v4 == -1
    && *__errno_location() == 11
    && (v10 = sub_4F850(fd, 3, v6, v7, v8, v9), v10 >= 0)
    && (v10 & 0x800) != 0
    && (BYTE1(v10) &= ~8u, (unsigned int)sub_4F850(fd, 4, v10, v11, v12, v13) != -1) )
  {
    return sub_4D570(fd, buf, nbytes);
  }
  else
  {
    return v4;
  }
}



// Function: blocking_write @ 0x26070
unsigned long long blocking_write(int fd, char *a2, size_t a3)
{
  unsigned long long v4; // r12
  int v5; // edx
  int v6; // ecx
  int v7; // r8d
  int v8; // r9d
  int v10; // eax
  int v11; // ecx
  int v12; // r8d
  int v13; // r9d

  v4 = sub_431A0(fd, a2, a3);
  if ( a3 > v4 && *__errno_location() == 11 )
  {
    v10 = sub_4F850(fd, 3, v5, v6, v7, v8);
    if ( v10 >= 0 && (v10 & 0x800) != 0 )
    {
      BYTE1(v10) &= ~8u;
      if ( (unsigned int)sub_4F850(fd, 4, v10, v11, v12, v13) != -1 )
        v4 += sub_431A0(fd, &a2[v4], a3 - v4);
    }
  }
  return v4;
}



// Function: set_file_atime @ 0x26100
long long set_file_atime(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v6[7]; // [rsp+0h] [rbp-38h] BYREF

  v6[5] = __readfsqword(0x28u);
  v6[1] = a5;
  v6[0] = a4;
  v6[3] = 1073741822;
  return sub_412D0(a1, a2, a3, v6, (unsigned int)flag);
}



// Function: chdir_count @ 0x26160
long long chdir_count()
{
  long long result; // rax

  result = (unsigned int)(qword_825E8 - 1);
  if ( !qword_825E8 )
    return 0;
  return result;
}



// Function: chdir_arg @ 0x26180
long long chdir_arg(char *a1, long long a2, long long a3)
{
  long long result; // rax
  unsigned long long v4; // rcx
  char *v5; // rbx
  long long v6; // rdx
  char *v7; // rcx
  char v8; // dl
  char v9; // dl
  long long v10; // rdx

  result = qword_825E8;
  v4 = qword_825E0;
  v5 = a1;
  if ( qword_825E8 != qword_825E0 )
    goto LABEL_2;
  if ( qword_825E0 )
  {
    if ( qword_825F0 )
    {
      if ( (unsigned long long)qword_825E0 <= 0x38E38E38E38E38DLL )
      {
LABEL_15:
        v4 += (v4 >> 1) + 1;
        goto LABEL_16;
      }
LABEL_24:
      sub_17020();
    }
    a3 = ((unsigned long long)qword_825E0 * (unsigned __int128)0x18uLL) >> 64 != 0;
    if ( 24 * qword_825E0 < 0 || !is_mul_ok(qword_825E0, 0x18u) )
      goto LABEL_24;
  }
  else
  {
    v4 = 2;
    if ( qword_825F0 )
      goto LABEL_15;
  }
LABEL_16:
  qword_825E0 = v4;
  qword_825F0 = sub_4ECE0(qword_825F0, 24 * v4, a3, v4);
  v10 = qword_825F0;
  result = qword_825E8;
  if ( !qword_825E8 )
  {
    *(long long *)(qword_825F0 + 8) = 0;
    *(long long *)v10 = ".";
    result = 1;
    *(int *)(v10 + 16) = -100;
    qword_825E8 = 1;
  }
LABEL_2:
  if ( *a1 == 46 )
  {
    while ( v5[1] == 47 )
    {
      v9 = v5[2];
      v7 = v5 + 2;
      v5 += 2;
      if ( v9 == 47 )
      {
        while ( 1 )
        {
          v8 = v7[1];
          v5 = v7 + 1;
          if ( v8 != 47 )
            break;
          ++v7;
        }
        if ( v8 != 46 )
        {
LABEL_10:
          if ( *v5 )
            goto LABEL_3;
          return (unsigned int)(result - 1);
        }
      }
      else if ( v9 != 46 )
      {
        goto LABEL_10;
      }
    }
    if ( !v5[1] )
      return (unsigned int)(result - 1);
  }
LABEL_3:
  v6 = qword_825F0 + 24 * result;
  *(long long *)v6 = v5;
  *(long long *)(v6 + 8) = 0;
  *(int *)(v6 + 16) = 0;
  qword_825E8 = result + 1;
  return result;
}



// Function: tar_dirname @ 0x26320
long long tar_dirname()
{
  return *(long long *)(qword_825F0 + 24LL * dword_82580);
}



// Function: open_diag @ 0x26370
long long open_diag(long long a1)
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_32EA0(a1);
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_32EE0();
  return result;
}



// Function: read_diag_details @ 0x263a0
long long read_diag_details()
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_32F20();
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_32FE0();
  return result;
}



// Function: readlink_diag @ 0x263d0
long long readlink_diag(long long a1)
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_33140(a1);
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_33160(a1);
  return result;
}



// Function: savedir_diag @ 0x26400
long long savedir_diag(long long a1)
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_331A0(a1);
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_32BD0("savedir", a1);
  return result;
}



// Function: seek_diag_details @ 0x26430
long long seek_diag_details(long long a1, long long a2)
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_33200(a1, a2);
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_332D0();
  return result;
}



// Function: stat_diag @ 0x26460
long long stat_diag(long long a1)
{
  long long result; // rax

  if ( !byte_82B52 )
    return sub_33410(a1);
  if ( (dword_81B80 & 0x800000) != 0 )
    return sub_33430(a1);
  return result;
}



// Function: file_removed_diag @ 0x26490
long long file_removed_diag(long long a1, char a2, long long (*a3)(long long))
{
  long long v5; // r12
  char *v6; // rax

  if ( a2 || *__errno_location() != 2 )
    return a3(a1);
  if ( (dword_81B80 & 0x40) != 0 )
  {
    if ( qword_82D50 )
      qword_82D50(a1);
    v5 = sub_4CE40(a1);
    v6 = dcgettext(0, "%s: File removed before we read it", 5);
    error(0, 0, v6, v5);
  }
  return sub_2FD00(1);
}



// Function: xfork @ 0x26520
__pid_t xfork()
{
  __pid_t result; // eax
  char *v1; // rax

  result = fork();
  if ( result == -1 )
  {
    v1 = dcgettext(0, "child process", 5);
    sub_32B70("fork", v1);
  }
  return result;
}



// Function: xpipe @ 0x26560
int xpipe(int *a1)
{
  int result; // eax
  char *v2; // rax

  result = pipe(a1);
  if ( result < 0 )
  {
    v2 = dcgettext(0, "interprocess channel", 5);
    sub_32B70("pipe", v2);
  }
  return result;
}



// Function: page_aligned_alloc @ 0x265a0
unsigned long long page_aligned_alloc(long long *a1, long long a2)
{
  unsigned long long v2; // rbx
  long long v3; // rax

  v2 = getpagesize();
  if ( __CFADD__(v2, a2) )
    sub_17020();
  v3 = sub_4ECC0(v2 + a2);
  *a1 = v3;
  return v3 + v2 - 1 - (v3 + v2 - 1) % v2;
}



// Function: namebuf_create @ 0x265f0
char **namebuf_create(char *src)
{
  char **v1; // r12
  size_t v2; // rax
  char *v3; // rbx
  size_t v4; // rax
  bool v5; // zf

  v1 = (char **)sub_4ECC0(24);
  v2 = strlen(src);
  v1[1] = (char *)(v2 + 2);
  v3 = (char *)sub_4ECC0(v2 + 2);
  *v1 = v3;
  strcpy(v3, src);
  v4 = strlen(v3);
  v5 = v3[v4 - 1] == 47;
  v1[2] = (char *)v4;
  if ( !v5 )
  {
    v3[v4] = 47;
    v1[2] = (char *)(v4 + 1);
  }
  return v1;
}



// Function: namebuf_free @ 0x26660
void namebuf_free(void **ptr)
{
  free(*ptr);
  free(ptr);
}



// Function: namebuf_name @ 0x26680
long long namebuf_name(long long *a1, const char *a2)
{
  size_t v3; // rbp
  long long v4; // rax
  long long v5; // rdi
  long long v6; // rax

  v3 = strlen(a2);
  v4 = a1[2];
  if ( a1[1] > v4 + v3 + 1 )
  {
    v5 = *a1;
  }
  else
  {
    v5 = *a1;
    do
    {
      v6 = sub_4ED20(v5, a1 + 1);
      *a1 = v6;
      v5 = v6;
      v4 = a1[2];
    }
    while ( v4 + v3 + 1 >= a1[1] );
  }
  strcpy((char *)(v4 + v5), a2);
  return *a1;
}



// Function: normalize_filename @ 0x26700
char *normalize_filename(long long *a1, char *a2)
{
  const char *v2; // r12
  int v3; // ebp
  long long v4; // rcx
  const char *v5; // r14
  size_t v6; // rax
  size_t v7; // rbx
  size_t v8; // rbp
  size_t v9; // rax
  char *v10; // r13
  long long v12; // rax
  long long v13; // rdx
  int v14; // ebx
  int v15; // eax
  long long v16; // rax
  long long v17; // r14
  long long v18; // r15
  long long **v19; // r8
  long long *v20; // r13
  const char *v21; // r14
  size_t v22; // rax
  long long v23; // r14
  size_t v24; // rax
  long long v25; // r15
  long long v26; // r13
  long long v27; // rax
  long long v28; // [rsp+8h] [rbp-50h]
  long long v29; // [rsp+10h] [rbp-48h]
  unsigned int v30; // [rsp+1Ch] [rbp-3Ch]

  v2 = a2;
  if ( *a2 == 47 )
  {
    v10 = (char *)sub_4EE30(a2);
    goto LABEL_5;
  }
  if ( qword_825F0 )
  {
    v3 = (int)a1;
    v4 = 24LL * (int)a1;
    v5 = *(const char **)(qword_825F0 + v4 + 8);
    v29 = v4;
    if ( !v5 )
    {
      v12 = qword_825F0 + v4 - 16;
      v30 = dword_82580;
      v13 = (unsigned int)a1;
      if ( (int)a1 < 0 )
      {
        v14 = (int)a1 + 1;
LABEL_13:
        if ( (int)a1 < v14 )
          goto LABEL_4;
      }
      else
      {
        while ( 1 )
        {
          v14 = v13;
          v13 = (unsigned int)(v13 - 1);
          if ( !v14 )
            break;
          v12 -= 24;
          if ( *(long long *)(v12 + 24) )
            goto LABEL_13;
        }
      }
      v15 = dword_82580;
      do
      {
        while ( 1 )
        {
          if ( v14 != v15 )
          {
            a1 = (long long *)(unsigned int)v14;
            sub_24F80((unsigned int)v14, a2, v13);
          }
          v26 = qword_825F0;
          if ( !v14 )
            break;
          v16 = v14++;
          v17 = 3 * v16;
          v18 = 24 * v16;
          v19 = (long long **)(qword_825F0 + 24 * v16);
          a1 = *v19;
          if ( *(char *)*v19 == 47 )
          {
            v28 = qword_825F0 + 24 * v16;
            *(long long *)(v28 + 8) = sub_4EE30(a1);
          }
          else
          {
            v20 = (long long *)sub_265F0(*(char **)(qword_825F0 + v18 - 16));
            v21 = *(const char **)(qword_825F0 + 8 * v17);
            if ( *(char *)(*v20 + v20[2] - 1) != 47 )
            {
              sub_26680(v20, "/");
              ++v20[2];
            }
            a2 = (char *)v21;
            sub_26680(v20, v21);
            v22 = strlen(v21);
            v23 = *v20;
            v24 = v20[2] + v22;
            v20[2] = v24;
            v25 = qword_825F0 + v18;
            if ( *(char *)(v23 + v24 - 1) == 47 )
              *(char *)(v23 + v24) = 0;
            a1 = v20;
            free(v20);
            *(long long *)(v25 + 8) = v23;
          }
          v15 = dword_82580;
          if ( v3 < v14 )
            goto LABEL_27;
        }
        v27 = sub_4EE70(a1, a2, v13);
        *(long long *)(v26 + 8) = v27;
        if ( !v27 )
          goto LABEL_9;
        v14 = 1;
        v15 = dword_82580;
      }
      while ( v3 >= 1 );
LABEL_27:
      if ( v30 != v15 )
        sub_24F80(v30, a2, v13);
      v5 = *(const char **)(qword_825F0 + v29 + 8);
    }
  }
  else
  {
    v5 = (const char *)qword_82588;
    if ( !qword_82588 )
    {
      qword_82588 = ((long long (*)(void))sub_4EE70)();
      v5 = (const char *)qword_82588;
      if ( !qword_82588 )
LABEL_9:
        sub_32B70("getcwd", ".");
    }
  }
LABEL_4:
  v6 = strlen(v5);
  v7 = v6 + 1;
  v8 = v6;
  v9 = strlen(v2);
  v10 = (char *)sub_4ECC0(v7 + v9 + 1);
  strcpy(v10, v5);
  v10[v8] = 47;
  strcpy(&v10[v7], v2);
LABEL_5:
  sub_253E0(v10);
  return v10;
}



// Function: tar_savedir @ 0x26970
DIR *tar_savedir(long long a1, int a2)
{
  int v3; // eax
  DIR *v4; // r13
  int v6; // r12d
  DIR *v7; // rax
  long long v8; // r12
  DIR *v9; // rdi

  v3 = __openat_2((unsigned int)oldfd, a1, dword_82808 | 0x10000u);
  if ( v3 < 0 )
  {
    if ( a2 || *__errno_location() != 2 )
      sub_32EA0(a1);
    return 0;
  }
  v6 = v3;
  v7 = fdopendir(v3);
  v4 = v7;
  if ( v7 )
  {
    v8 = sub_4D730(v7);
    if ( !v8 )
      sub_331A0(a1);
    v9 = v4;
    v4 = (DIR *)v8;
    if ( !closedir(v9) )
      return v4;
  }
  else
  {
    sub_331A0(a1);
    if ( !close(v6) )
      return 0;
  }
  sub_331A0(a1);
  return v4;
}



// Function: remove_any_file @ 0x26a40
long long remove_any_file(char *a1, int a2)
{
  int *v2; // rbx
  long long result; // rax
  unsigned long long v4; // rdx
  long long v5; // rcx
  DIR *v6; // rax
  DIR *v7; // r13
  DIR *v8; // r14
  size_t v9; // r12
  void *v10; // r15
  int v11; // [rsp+8h] [rbp-40h]
  int v12; // [rsp+Ch] [rbp-3Ch]

  if ( !unlinkat(oldfd, a1, 0) )
    return 1;
  v2 = __errno_location();
  if ( *v2 != 1 && *v2 != 21 )
    return 0;
  if ( !(unsigned int)sub_25BA0(a1) )
    return 1;
  v4 = (unsigned int)*v2;
  result = 0;
  if ( (unsigned int)v4 <= 0x27 )
  {
    v5 = 0x8000020001LL;
    if ( _bittest64(&v5, v4) )
    {
      if ( a2 != 1 )
        return (unsigned int)-(a2 == 2);
      v6 = sub_26970((long long)a1, 0);
      v7 = v6;
      if ( !v6 )
        return 0;
      v8 = v6;
      v9 = strlen((const char *)v6);
      if ( v9 )
      {
        while ( 1 )
        {
          v10 = (void *)sub_29320(a1, v8);
          v11 = remove_any_file(v10, 1);
          v12 = *v2;
          free(v10);
          if ( !v11 )
            break;
          v8 = (DIR *)((char *)v8 + v9 + 1);
          v9 = strlen((const char *)v8);
          if ( !v9 )
            goto LABEL_16;
        }
        free(v7);
        *v2 = v12;
        return 0;
      }
      else
      {
LABEL_16:
        free(v7);
        return (unsigned int)sub_25BA0(a1) == 0;
      }
    }
  }
  return result;
}



// Function: merge_sort_sll @ 0x26b90
long long *merge_sort_sll(long long *a1, int a2, int (*a3)(long long *, long long *))
{
  long long *v4; // rax
  int v5; // edx
  long long *v6; // r14
  long long *v7; // rbp
  long long v8; // rax
  long long *v9; // rbx
  long long *i; // r13
  long long *v11; // rdx
  long long *v12; // rax
  long long *v13; // rdx
  long long *result; // rax
  long long v15[7]; // [rsp+10h] [rbp-38h] BYREF

  v15[1] = __readfsqword(0x28u);
  if ( a2 == 1 )
    return a1;
  if ( a2 == 2 )
  {
    if ( a3(a1, (long long *)*a1) > 0 )
    {
      result = (long long *)*a1;
      *(long long *)*a1 = a1;
      *a1 = 0;
      return result;
    }
    return a1;
  }
  v4 = a1;
  v5 = (a2 + 1) / 2 - 1;
  if ( (a2 + 1) / 2 != 1 )
  {
    do
    {
      v4 = (long long *)*v4;
      --v5;
    }
    while ( v5 );
  }
  v6 = (long long *)*v4;
  *v4 = 0;
  v7 = (long long *)merge_sort_sll(a1, (unsigned int)((a2 + 1) / 2), a3);
  v8 = merge_sort_sll(v6, (unsigned int)(a2 / 2), a3);
  v9 = (long long *)v8;
  if ( v7 && v8 )
  {
    for ( i = v15; ; i = v12 )
    {
      if ( a3(v7, v9) >= 0 )
      {
        v11 = v9;
        v12 = v9;
        v9 = (long long *)*v9;
        *i = v11;
      }
      else
      {
        v13 = v7;
        v12 = v7;
        v7 = (long long *)*v7;
        *i = v13;
        if ( !v7 )
          goto LABEL_12;
      }
      if ( !v9 )
        goto LABEL_12;
    }
  }
  v12 = v15;
LABEL_12:
  if ( !v7 )
    v7 = v9;
  *v12 = v7;
  return (long long *)v15[0];
}



// Function: name_elt_alloc @ 0x26d00
long long *name_elt_alloc()
{
  long long *result; // rax
  long long *v1; // rdx
  long long *v2; // rcx

  result = (long long *)sub_4ECC0(56);
  v1 = qword_826B0;
  if ( !qword_826B0 )
  {
    *result = 0;
    result[1] = 0;
    *((int *)result + 4) = 3;
    qword_826B0 = result;
    result = (long long *)sub_4ECC0(56);
    v1 = qword_826B0;
  }
  v2 = (long long *)v1[1];
  result[1] = v2;
  if ( v2 )
    *v2 = result;
  *result = v1;
  v1[1] = result;
  return result;
}



// Function: name_compare @ 0x26d70
bool name_compare(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 88), *(const char **)(a2 + 88)) == 0;
}



// Function: compare_names @ 0x26d90
int compare_names(long long a1, long long a2)
{
  return strcmp(*(const char **)(a1 + 16), *(const char **)(a2 + 16));
}



// Function: handle_option @ 0x26db0
long long handle_option(char *a1, long long a2)
{
  char v2; // bl
  char *v3; // r12
  const unsigned short *v5; // rdx
  unsigned int v6; // r13d
  long long *v7; // rsi
  long long v8; // rdi
  long long v9; // rax
  long long *v10; // rax
  long long *v11; // rdx
  long long v13; // rbx
  char *v14; // rax
  int v15; // [rsp+0h] [rbp-138h] BYREF
  long long v16; // [rsp+8h] [rbp-130h]
  long long v17; // [rsp+10h] [rbp-128h]
  long long v18; // [rsp+20h] [rbp-118h] BYREF
  long long *v19; // [rsp+28h] [rbp-110h]
  long long v20; // [rsp+30h] [rbp-108h]
  unsigned long long v21; // [rsp+108h] [rbp-30h]

  v2 = *a1;
  v21 = __readfsqword(0x28u);
  if ( !v2 )
    return 1;
  v3 = a1;
  v5 = *__ctype_b_loc();
  while ( (v5[v2] & 0x2000) != 0 )
  {
    v2 = *++v3;
    if ( !v2 )
      return 1;
  }
  if ( v2 == 45 )
  {
    v20 = 1;
    v6 = sub_375A0(v3, &v18, 33558086);
    if ( v6 )
    {
      if ( qword_82D50 )
        qword_82D50(v3);
      v13 = sub_358F0(&v18);
      v14 = dcgettext(0, "cannot split string '%s': %s", 5);
      error(0, 0, v14, v3, v13);
      sub_16FD0();
    }
    v7 = v19;
    v8 = (unsigned int)(v18 + v20);
    *v19 = qword_82DD0;
    v9 = *(long long *)(a2 + 24);
    v15 = 2;
    v16 = v9;
    v17 = *(long long *)(a2 + 32);
    ((void (*)(long long, long long *, int *))sub_2FAC0)(v8, v7, &v15);
    if ( v20 + v18 )
    {
      v10 = v19;
      v11 = &v19[v20 + v18];
      do
        *v10++ = 0;
      while ( v11 != v10 );
    }
    sub_35800(&v18);
  }
  else
  {
    return 1;
  }
  return v6;
}



// Function: regex_usage_warning @ 0x26f30
long long regex_usage_warning(long long a1)
{
  char *v1; // rax
  char *v2; // rax

  if ( dword_826FC )
    return (unsigned int)dword_82634;
  if ( (unsigned char)sub_40850(a1, 0) )
  {
    dword_82634 = 1;
    if ( qword_82D50 )
      qword_82D50(a1);
    v1 = dcgettext(0, "Pattern matching characters used in file names", 5);
    error(0, 0, v1);
    if ( qword_82D50 )
      qword_82D50(0);
    v2 = dcgettext(0, "Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning", 5);
    error(0, 0, v2);
  }
  return (unsigned int)dword_82634;
}



// Function: name_hash @ 0x26fd0
long long name_hash(long long a1)
{
  return sub_43B80(*(long long *)(a1 + 88));
}



// Function: name_list_advance @ 0x270f0
void name_list_advance()
{
  int *v0; // rbp
  long long *v1; // rax
  int v2; // eax
  void *v3; // rdi
  long long *v4; // rax

  v0 = qword_826B0;
  v1 = *(long long **)qword_826B0;
  qword_826B0 = v1;
  if ( v1 )
    v1[1] = 0;
  v2 = v0[4];
  if ( v2 == 4 || v2 == 1 )
  {
    if ( dword_82BA8 == 3 || dword_82BA8 == 8 )
    {
      v4 = qword_82628;
      *(long long *)v0 = 0;
      *((long long *)v0 + 1) = v4;
      if ( v4 )
        *v4 = v0;
      qword_82628 = v0;
    }
  }
  else
  {
    if ( v2 != 3 )
    {
      while ( 1 )
      {
        v3 = qword_82628;
        if ( !qword_82628 )
          break;
        qword_82628 = (void *)*((long long *)qword_82628 + 1);
        free(v3);
      }
    }
    free(v0);
  }
}



// Function: names_parse_opt @ 0x271a0
long long names_parse_opt(int a1, long long a2, long long a3)
{
  long long *v3; // rax
  unsigned int v4; // r12d
  long long v6; // rax
  long long v7; // rbx
  long long v8; // r13
  char *v9; // rax
  int *i; // rax
  int v11; // edx
  long long *v12; // rax
  long long *v13; // rax
  long long *v14; // rax

  if ( a1 == 256 )
  {
    v13 = sub_26D00();
    *((int *)v13 + 4) = 0;
    v13[3] = a2;
    if ( dword_82630 )
    {
      if ( dword_82630 == 1 )
      {
        dword_82630 = 2;
        return 0;
      }
    }
    else
    {
      dword_82630 = 1;
    }
    return 0;
  }
  else
  {
    if ( a1 > 256 )
    {
      if ( a1 == 16777221 )
      {
        v4 = 7;
        v6 = **(long long **)(a3 + 40);
        if ( *(int *)v6 == 2 )
        {
          v7 = *(long long *)(v6 + 16);
          v8 = *(long long *)(v6 + 8);
          v9 = dcgettext(0, "%s:%lu: unrecognized option", 5);
          error(0, 0, v9, v8, v7);
          sub_2FD00(2);
        }
        return v4;
      }
    }
    else
    {
      if ( a1 == 67 )
      {
        v12 = sub_26D00();
        v12[3] = a2;
        *((int *)v12 + 4) = 1;
        return 0;
      }
      if ( a1 == 84 )
      {
        v3 = sub_26D00();
        dword_82630 = 2;
        *((int *)v3 + 4) = 2;
        v3[4] = 0;
        v3[6] = 0;
        v3[3] = a2;
        return 0;
      }
    }
    for ( i = &unk_7F280; ; i += 12 )
    {
      v11 = i[2];
      if ( !*(long long *)i && !v11 && !*((long long *)i + 4) )
        return 7;
      if ( a1 == v11 )
        break;
    }
    v14 = sub_26D00();
    v14[4] = a2;
    *((int *)v14 + 4) = 4;
    *((int *)v14 + 6) = a1;
    return 0;
  }
}



// Function: name_next_elt @ 0x27340
void *name_next_elt(char *a1, char *a2, double a3)
{
  void *v3; // r12
  const char *v4; // rbp
  size_t v5; // rax
  char *v6; // rdi
  unsigned long long k; // rbx
  char *v8; // rdi
  char *v9; // rax
  int v10; // eax
  void *result; // rax
  char *v12; // r15
  int v13; // r12d
  _IO_FILE *v14; // r14
  int v15; // r15d
  long long v16; // rax
  char *IO_read_ptr; // rdx
  char *m; // rsi
  int v19; // r13d
  long long n; // rcx
  long long v21; // rbp
  char *v22; // rax
  bool v23; // al
  char *v24; // rdi
  const char *v25; // rax
  char *v26; // rax
  long long v27; // rdx
  bool v28; // cl
  bool v29; // al
  char *v30; // rax
  const char *v31; // r15
  char v32; // al
  void *v33; // rax
  char *v34; // rbp
  char *v35; // rax
  char *v36; // rax
  long long *v37; // rax
  long long *v38; // r14
  int v39; // eax
  long long v40; // rdi
  long long v41; // r15
  int *i; // rax
  long long v43; // r13
  int v44; // edx
  char *v45; // rax
  void *v46; // rdi
  const char *v47; // rsi
  long long v48; // r13
  char *v49; // rax
  long long *v50; // r13
  unsigned int v51; // r12d
  long long v52; // r13
  long long v53; // r15
  char *v54; // rax
  long long *v55; // rax
  long long v56; // rdx
  const char *v57; // rdi
  FILE *v58; // rax
  long long *v59; // rax
  int v60; // ebp
  const char *v61; // rax
  char **v62; // r15
  char **v63; // r15
  long long *j; // rax
  int v65; // edx
  long long v66; // [rsp+0h] [rbp-E8h]
  bool v67; // [rsp+0h] [rbp-E8h]
  int v68; // [rsp+8h] [rbp-E0h]
  bool v69; // [rsp+Fh] [rbp-D9h]
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v71; // [rsp+A8h] [rbp-40h]

  v3 = qword_826B0;
  v68 = (int)a1;
  v71 = __readfsqword(0x28u);
  while ( 2 )
  {
    if ( !v3 )
    {
LABEL_70:
      if ( !qword_82628 )
        return v3;
      if ( qword_82D50 )
        qword_82D50(a1);
      v36 = dcgettext(
              0,
              "The following options were used after any non-optional arguments in archive create or update mode.  These "
              "options are positional and affect only arguments that follow them.  Please, rearrange them properly.",
              5);
      error(0, 0, v36);
      v37 = qword_82628;
      dword_82D58 = 2;
      do
      {
        v38 = v37;
        v37 = (long long *)v37[1];
      }
      while ( v37 );
      while ( 1 )
      {
        v39 = *((int *)v38 + 4);
        if ( v39 == 1 )
          break;
        if ( v39 == 4 )
        {
          v40 = v38[4];
          if ( v40 )
          {
            if ( qword_82D50 )
            {
              qword_82D50(v40);
              v40 = v38[4];
            }
            v41 = sub_4D2D0(v40);
            for ( i = &unk_7F280; ; i += 12 )
            {
              v43 = *(long long *)i;
              v44 = i[2];
              if ( !*(long long *)i && !v44 && !*((long long *)i + 4) )
                break;
              if ( *((int *)v38 + 6) == v44 )
                break;
            }
            v45 = dcgettext(0, "--%s %s has no effect", 5);
            error(0, 0, v45, v43, v41);
            dword_82D58 = 2;
            goto LABEL_87;
          }
          if ( qword_82D50 )
            qword_82D50(0);
          for ( j = (long long *)&unk_7F280; ; j += 6 )
          {
            v48 = *j;
            v65 = *((int *)j + 2);
            if ( !*j && !v65 && !j[4] )
              break;
            if ( *((int *)v38 + 6) == v65 )
              break;
          }
          v47 = "--%s has no effect";
          goto LABEL_95;
        }
LABEL_87:
        v38 = (long long *)*v38;
        if ( !v38 )
        {
          while ( 1 )
          {
            v46 = qword_82628;
            if ( !qword_82628 )
              break;
            qword_82628 = (void *)*((long long *)qword_82628 + 1);
            free(v46);
          }
          return v3;
        }
      }
      if ( qword_82D50 )
        qword_82D50(0);
      v47 = "-C %s has no effect";
      v48 = sub_4D2D0(v38[3]);
LABEL_95:
      v49 = dcgettext(0, v47, 5);
      error(0, 0, v49, v48);
      dword_82D58 = 2;
      goto LABEL_87;
    }
    switch ( *((int *)v3 + 4) )
    {
      case 0:
        goto LABEL_6;
      case 1:
        if ( v68 )
        {
          v26 = (char *)sub_4EE30(*((void **)v3 + 3));
          a1 = (char *)(unsigned int)sub_26180(v26, (long long)a2, v27);
          sub_26300(a1);
          sub_270F0();
          v3 = qword_826B0;
          continue;
        }
LABEL_6:
        v4 = (const char *)*((long long *)v3 + 3);
        v5 = strlen(v4);
        v6 = qword_826A8;
        for ( k = v5; k >= qword_826A0; v6 = qword_826A8 )
          qword_826A8 = (char *)sub_4ED20(v6, &qword_826A0);
        v8 = strcpy(v6, v4);
        v9 = &v8[strlen(v8) - 1];
        if ( v9 > v8 )
        {
          do
          {
            if ( *v9 != 47 )
              break;
            *v9-- = 0;
          }
          while ( v9 != v8 );
        }
        if ( byte_827E8 )
        {
          sub_251B0(v8);
          v8 = qword_826A8;
        }
        v10 = *((int *)v3 + 4);
        qword_82678 = (long long)v8;
        v3 = &unk_82660;
        dword_82670 = v10;
        sub_270F0();
        return v3;
      case 2:
        v14 = (_IO_FILE *)*((long long *)v3 + 6);
        if ( v14 )
        {
          v15 = *((int *)v3 + 10);
        }
        else
        {
          v31 = (const char *)*((long long *)v3 + 3);
          if ( !strcmp(v31, "-") )
          {
            sub_2DF30("-T", a3);
            v14 = (_IO_FILE *)stdin;
            *((long long *)v3 + 6) = stdin;
          }
          else
          {
            if ( stat(v31, &buf) )
              sub_333F0(v31);
            v33 = v3;
            do
            {
              if ( *((int *)v33 + 4) == 2 && *((long long *)v33 + 6) )
              {
                v34 = (char *)*((long long *)v33 + 3);
                goto LABEL_97;
              }
              v33 = *(void **)v33;
            }
            while ( v33 );
            v34 = dcgettext(0, "command line", 5);
LABEL_97:
            v50 = (long long *)qword_82698;
            if ( qword_82698 )
            {
              while ( v50[1] != buf.st_ino || v50[2] != buf.st_dev )
              {
                v50 = (long long *)*v50;
                if ( !v50 )
                  goto LABEL_105;
              }
              v51 = sub_4C780(0, 58, 1);
              if ( qword_82D50 )
                qword_82D50(0);
              v52 = v50[3];
              v53 = sub_4CA40(0, v31);
              v54 = dcgettext(0, "%s: file list requested from %s already read from %s", 5);
              error(0, 0, v54, v53, v34, v52);
              a2 = (char *)&word_3A;
              a1 = 0;
              dword_82D58 = 2;
              sub_4C780(0, 58, v51);
              sub_270F0();
              v3 = qword_826B0;
              continue;
            }
LABEL_105:
            v55 = (long long *)sub_4ECC0(32);
            v56 = qword_82698;
            v57 = (const char *)*((long long *)v3 + 3);
            v55[3] = (long long)v34;
            *v55 = v56;
            qword_82698 = (long long)v55;
            v55[1] = buf.st_ino;
            v55[2] = buf.st_dev;
            v58 = fopen(v57, "r");
            *((long long *)v3 + 6) = v58;
            v14 = v58;
            if ( !v58 )
              sub_32EC0(*((long long *)v3 + 3), a3);
          }
          v15 = byte_7F23C;
          v32 = byte_82700;
          *((int *)v3 + 10) = byte_7F23C;
          *((char *)v3 + 44) = v32;
        }
        v16 = *((long long *)v3 + 4);
        IO_read_ptr = v14->_IO_read_ptr;
        for ( m = v14->_IO_read_end; ; m = v14->_IO_read_end )
        {
          while ( 1 )
          {
            *((long long *)v3 + 4) = ++v16;
            if ( IO_read_ptr >= m )
              break;
            v14->_IO_read_ptr = IO_read_ptr + 1;
            v19 = (unsigned char)*IO_read_ptr;
            if ( v15 != v19 )
              goto LABEL_22;
            ++IO_read_ptr;
          }
          v19 = __uflow(v14);
          v29 = v19 != -1;
          if ( v15 != v19 && v19 != -1 )
          {
LABEL_22:
            for ( n = 0; ; n = v21 )
            {
              v24 = qword_826A8;
              if ( n == qword_826A0 )
              {
                v66 = n;
                v24 = (char *)sub_4ED20(qword_826A8, &qword_826A0);
                qword_826A8 = v24;
                v24[v66] = v19;
                v21 = v66 + 1;
                if ( !v19 )
                {
LABEL_30:
                  if ( (dword_81B80 & 0x200) != 0 )
                  {
                    if ( qword_82D50 )
                      qword_82D50(v24);
                    v25 = (const char *)sub_4CE40(*((long long *)v3 + 3));
                    error(0, 0, "%s: file name read contains nul character", v25);
                  }
                  *((int *)v3 + 10) = 0;
                  a1 = qword_826A8;
                  goto LABEL_35;
                }
              }
              else
              {
                qword_826A8[n] = v19;
                v21 = n + 1;
                if ( !v19 )
                  goto LABEL_30;
              }
              v22 = v14->_IO_read_ptr;
              if ( v22 >= v14->_IO_read_end )
              {
                v19 = __uflow(v14);
                v23 = v19 != -1;
                if ( v15 == v19 )
                {
LABEL_44:
                  v28 = v21 == 0;
                  v29 = v21 == 0 && v23;
                  goto LABEL_45;
                }
              }
              else
              {
                v14->_IO_read_ptr = v22 + 1;
                v19 = (unsigned char)*v22;
                v23 = 1;
                if ( v15 == v19 )
                  goto LABEL_44;
              }
              if ( !v23 )
                goto LABEL_44;
            }
          }
          v28 = 1;
          v21 = 0;
LABEL_45:
          if ( !v29 )
            break;
          v14 = (_IO_FILE *)*((long long *)v3 + 6);
          v15 = *((int *)v3 + 10);
          v16 = *((long long *)v3 + 4);
          IO_read_ptr = v14->_IO_read_ptr;
        }
        a1 = qword_826A8;
        if ( qword_826A0 == v21 )
        {
          v67 = v28;
          v35 = (char *)sub_4ED20(qword_826A8, &qword_826A0);
          v28 = v67;
          qword_826A8 = v35;
          a1 = v35;
        }
        a1[v21] = 0;
        v69 = v28;
        v30 = &a1[strlen(a1) - 1];
        if ( v30 > a1 )
        {
          do
          {
            if ( *v30 != 47 )
              break;
            *v30-- = 0;
          }
          while ( v30 != a1 );
        }
        if ( v19 == -1 && v69 )
        {
          a1 = (char *)*((long long *)v3 + 3);
          a2 = "-";
          if ( strcmp(a1, "-") )
          {
            a1 = (char *)*((long long *)v3 + 6);
            fclose((FILE *)a1);
          }
          *((long long *)v3 + 6) = 0;
          sub_270F0();
          v3 = qword_826B0;
        }
        else
        {
LABEL_35:
          if ( *((char *)v3 + 44) )
            goto LABEL_40;
          if ( byte_827E8 )
          {
            sub_251B0(a1);
            a1 = qword_826A8;
          }
          a2 = (char *)v3;
          if ( (unsigned int)sub_26DB0(a1, (long long)v3) )
          {
            a1 = qword_826A8;
LABEL_40:
            qword_82678 = (long long)a1;
            v3 = &unk_82660;
            dword_82670 = 0;
            return v3;
          }
          v3 = qword_826B0;
          if ( !qword_826B0 )
            goto LABEL_70;
          v59 = (long long *)*((long long *)qword_826B0 + 1);
          if ( v59 )
          {
            do
            {
              v3 = v59;
              v59 = (long long *)v59[1];
            }
            while ( v59 );
            qword_826B0 = v3;
          }
        }
        continue;
      case 3:
        goto LABEL_5;
      case 4:
        v12 = (char *)*((long long *)v3 + 4);
        v13 = *((int *)v3 + 6);
        switch ( v13 )
        {
          case 88:
            a2 = (char *)qword_82B60;
            a1 = (char *)sub_40C90;
            if ( (unsigned int)sub_41210(
                                 sub_40C90,
                                 qword_82B60,
                                 v12,
                                 dword_82AD4 | dword_826F8 | ((unsigned char)(dword_826FC != 1) << 28),
                                 10) )
            {
              v60 = *__errno_location();
              if ( qword_82D50 )
                qword_82D50(sub_40C90);
              v61 = (const char *)sub_4CE40(v12);
              error(0, v60, "%s", v61);
              sub_16FD0();
            }
            return result;
          case 257:
            v62 = off_7D708;
            a2 = ".#*";
            do
            {
              a1 = (char *)qword_82B60;
              ++v62;
              sub_40C90(qword_82B60, a2, 0x10000000);
              a2 = *(v62 - 1);
            }
            while ( a2 );
            break;
          case 258:
            a2 = (char *)(&dword_0 + 1);
            a1 = "CACHEDIR.TAG";
            sub_12E10("CACHEDIR.TAG", 1, (long long)sub_12F10);
            break;
          case 259:
            a2 = (char *)(&dword_0 + 2);
            a1 = "CACHEDIR.TAG";
            sub_12E10("CACHEDIR.TAG", 2, (long long)sub_12F10);
            break;
          case 260:
            a2 = (char *)(&dword_0 + 3);
            a1 = "CACHEDIR.TAG";
            sub_12E10("CACHEDIR.TAG", 3, (long long)sub_12F10);
            break;
          case 261:
            a2 = v12;
            a1 = (char *)qword_82B60;
            sub_40C90(qword_82B60, v12, ((dword_826FC != 1) << 28) | dword_82AD4 | (unsigned int)dword_826F8);
            break;
          case 262:
            a2 = (char *)(&dword_0 + 2);
            a1 = v12;
            sub_173D0(v12, 2);
            break;
          case 263:
            a2 = (char *)(&dword_0 + 1);
            a1 = v12;
            sub_173D0(v12, 1);
            break;
          case 264:
            a2 = (char *)(&dword_0 + 1);
            a1 = v12;
            sub_12E10(v12, 1, 0);
            break;
          case 265:
            a2 = (char *)(&dword_0 + 2);
            a1 = v12;
            sub_12E10(v12, 2, 0);
            break;
          case 266:
            a2 = (char *)(&dword_0 + 3);
            a1 = v12;
            sub_12E10(v12, 3, 0);
            break;
          case 267:
            v63 = off_7D728;
            a2 = "CVS";
            do
            {
              a1 = (char *)qword_82B60;
              ++v63;
              sub_40C90(qword_82B60, a2, 0);
              a2 = *(v63 - 1);
            }
            while ( a2 );
            break;
          case 268:
            sub_17810();
            break;
          case 269:
            dword_826F8 |= 0x10u;
            break;
          case 270:
            dword_826F8 &= ~0x10u;
            break;
          case 271:
            dword_826F8 |= 0x40000000u;
            break;
          case 272:
            dword_826F8 &= ~0x40000000u;
            dword_7F238 = 0;
            break;
          case 273:
            dword_82AD4 = 8;
            break;
          case 274:
            dword_82AD4 = 0;
            break;
          case 275:
            byte_827E8 = 1;
            break;
          case 276:
            byte_827E8 = 0;
            break;
          case 277:
            byte_82700 = 0;
            break;
          case 278:
            dword_826F8 |= 1u;
            break;
          case 279:
            dword_826FC = 1;
            break;
          case 280:
            byte_7F23C = 0;
            byte_82700 = 1;
            break;
          case 281:
            byte_7F23C = 10;
            byte_82700 = 0;
            break;
          case 282:
            byte_82700 = 1;
            break;
          case 283:
            dword_826F8 &= ~1u;
            break;
          case 284:
            dword_826FC = 2;
            break;
          default:
            if ( qword_82D50 )
              qword_82D50(a1);
            error(0, 0, "unhandled positional option %d", v13);
            sub_16FD0();
        }
LABEL_5:
        sub_270F0();
        v3 = qword_826B0;
        continue;
      default:
        continue;
    }
  }
}



// Function: uid_to_uname @ 0x27eb0
long long uid_to_uname(__uid_t uid, long long *a2)
{
  char *v4; // rdi
  struct passwd *v5; // rax
  char *pw_name; // rsi
  long long result; // rax

  if ( !uid || dword_826CC != uid )
  {
    v4 = qword_826F0;
    if ( qword_826F0 && dword_826E4 == uid )
      goto LABEL_7;
    v5 = getpwuid(uid);
    if ( v5 )
    {
      pw_name = v5->pw_name;
      dword_826E4 = uid;
      sub_25120(&qword_826F0, pw_name);
      v4 = qword_826F0;
LABEL_7:
      result = sub_4EE30(v4);
      *a2 = result;
      return result;
    }
    dword_826CC = uid;
  }
  result = sub_4EE30("");
  *a2 = result;
  return result;
}



// Function: gid_to_gname @ 0x27f40
long long gid_to_gname(__gid_t gid, long long *a2)
{
  char *v4; // rdi
  struct group *v5; // rax
  char *gr_name; // rsi
  long long result; // rax

  if ( !gid || dword_826C8 != gid )
  {
    v4 = qword_826E8;
    if ( qword_826E8 && dword_826E0 == gid )
      goto LABEL_7;
    v5 = getgrgid(gid);
    if ( v5 )
    {
      gr_name = v5->gr_name;
      dword_826E0 = gid;
      sub_25120(&qword_826E8, gr_name);
      v4 = qword_826E8;
LABEL_7:
      result = sub_4EE30(v4);
      *a2 = result;
      return result;
    }
    dword_826C8 = gid;
  }
  result = sub_4EE30("");
  *a2 = result;
  return result;
}



// Function: uname_to_uid @ 0x27fd0
int uname_to_uid(char *name, int *a2)
{
  int result; // eax
  struct passwd *v4; // rax
  char *pw_name; // rsi

  if ( !qword_826D8 || (result = strcmp(name, qword_826D8)) != 0 )
  {
    if ( qword_826F0 && *name == *qword_826F0 )
    {
      if ( !strcmp(name, qword_826F0) )
        goto LABEL_7;
      v4 = getpwnam(name);
      if ( v4 )
      {
LABEL_6:
        pw_name = v4->pw_name;
        dword_826E4 = v4->pw_uid;
        sub_25120(&qword_826F0, pw_name);
LABEL_7:
        *a2 = dword_826E4;
        return 1;
      }
    }
    else
    {
      v4 = getpwnam(name);
      if ( v4 )
        goto LABEL_6;
    }
    sub_25120(&qword_826D8, name);
    return 0;
  }
  return result;
}



// Function: gname_to_gid @ 0x28080
int gname_to_gid(char *name, int *a2)
{
  int result; // eax
  struct group *v3; // rax

  if ( !qword_826D0 || (result = strcmp(name, qword_826D0)) != 0 )
  {
    if ( qword_826E8 && *name == *qword_826E8 )
    {
      if ( !strcmp(name, qword_826E8) )
        goto LABEL_7;
      v3 = getgrnam(name);
      if ( v3 )
      {
LABEL_6:
        dword_826E0 = v3->gr_gid;
        sub_25120(&qword_826E8, name);
LABEL_7:
        *a2 = dword_826E0;
        return 1;
      }
    }
    else
    {
      v3 = getgrnam(name);
      if ( v3 )
        goto LABEL_6;
    }
    sub_25120(&qword_826D0, name);
    return 0;
  }
  return result;
}



// Function: name_init @ 0x28180
void *name_init()
{
  char *v0; // rax
  void *result; // rax
  void *v2; // rdx

  v0 = (char *)sub_4ECC0(102);
  qword_826A0 = 100;
  qword_826A8 = v0;
  result = qword_826B0;
  if ( qword_826B0 )
  {
    result = (void *)*((long long *)qword_826B0 + 1);
    if ( result )
    {
      do
      {
        v2 = result;
        result = (void *)*((long long *)result + 1);
      }
      while ( result );
      qword_826B0 = v2;
    }
  }
  return result;
}



// Function: name_term @ 0x281e0
void name_term()
{
  free(qword_826A8);
}



// Function: addname @ 0x28210
long long *addname(char *a1, int a2, char a3, long long a4)
{
  char *v5; // rbp
  long long *v7; // r12
  const char *v8; // rax
  size_t v9; // rax
  long long *v10; // rcx
  bool v11; // zf

  v5 = a1;
  v7 = (long long *)sub_4ED80(96);
  if ( !a1 )
    v5 = "";
  v8 = (const char *)sub_4EE30(v5);
  v7[2] = v8;
  v9 = strlen(v8);
  *((int *)v7 + 10) = a2;
  v10 = (long long *)qword_826B8;
  v7[3] = v9;
  LODWORD(v9) = dword_82AD4 | dword_826F8 | dword_7F238;
  v7[1] = v10;
  v11 = dword_826FC == 2;
  v7[8] = a4;
  *((char *)v7 + 36) = a3;
  *v7 = 0;
  v7[6] = 0;
  *((int *)v7 + 8) = (v11 << 28) | v9;
  v7[7] = 0;
  if ( v10 )
    *v10 = v7;
  else
    qword_826C0 = v7;
  qword_826B8 = (long long)v7;
  return v7;
}



// Function: name_gather @ 0x282f0
long long *name_gather(double a1, long long a2, long long a3)
{
  unsigned int v3; // ebx
  unsigned int i; // ebp
  char *v5; // rdi
  char *v6; // rax
  long long v7; // rdx
  long long *result; // rax
  char *v9; // rax
  long long v10; // rdx
  int v11; // esi
  void **v12; // rbp
  char *v13; // rbp
  int *v14; // rbx
  const char *v15; // rax
  size_t v16; // rax
  bool v17; // zf
  long long *v18; // [rsp+8h] [rbp-20h]

  v3 = 0;
  if ( byte_82AAC )
  {
    while ( 1 )
    {
      result = sub_27340(0, (char *)a3, a1);
      if ( !result )
        break;
      if ( *((int *)result + 4) != 1 )
      {
        v12 = (void **)qword_82638;
        if ( qword_82638 )
        {
          v18 = result;
          free(*((void **)qword_82638 + 2));
          free(v12[11]);
          free(v12);
          result = v18;
        }
        v13 = (char *)result[3];
        v14 = (int *)sub_4ED80(96);
        if ( !v13 )
          v13 = "";
        v15 = (const char *)sub_4EE30(v13);
        *((long long *)v14 + 2) = v15;
        v16 = strlen(v15);
        *(long long *)v14 = 0;
        *((long long *)v14 + 3) = v16;
        LODWORD(v16) = dword_82640;
        *((long long *)v14 + 6) = 0;
        v14[10] = v16;
        LODWORD(v16) = dword_82AD4 | dword_826F8 | dword_7F238;
        *((long long *)v14 + 7) = 0;
        v17 = dword_826FC == 2;
        *((long long *)v14 + 8) = 0;
        *((char *)v14 + 36) = 1;
        qword_82638 = v14;
        result = (long long *)((v17 << 28) | (unsigned int)v16);
        qword_826B8 = (long long)v14;
        v14[8] = (int)result;
        qword_826C0 = v14;
        return result;
      }
      v9 = (char *)sub_4EE30((void *)result[3]);
      dword_82640 = sub_26180(v9, a3, v10);
    }
    v11 = dword_82640;
    if ( !dword_82640 )
      return result;
    return sub_28210(0, v11, 0, 0);
  }
LABEL_2:
  for ( i = v3; ; i = sub_26180(v6, a3, v7) )
  {
    result = sub_27340(0, (char *)a3, a1);
    if ( !result )
      break;
    v5 = (char *)result[3];
    if ( *((int *)result + 4) != 1 )
    {
      a3 = i;
      v3 = i;
      sub_28210(v5, i, 1, 0);
      goto LABEL_2;
    }
    v6 = (char *)sub_4EE30(v5);
  }
  if ( v3 != i )
  {
    v11 = i;
    return sub_28210(0, v11, 0, 0);
  }
  return result;
}



// Function: add_hierarchy_to_namelist @ 0x28480
unsigned long long add_hierarchy_to_namelist(long long a1, long long a2)
{
  long long *v3; // rax
  long long v4; // rax
  unsigned long long v5; // r15
  const char *v6; // rbx
  long long v7; // rdi
  char *v8; // r13
  char v9; // r14
  size_t v10; // rbp
  struct stat *p_buf; // rsi
  struct stat *v13; // rax
  struct stat *v14; // r15
  int v15; // edx
  int v16; // eax
  unsigned long long v17; // [rsp+0h] [rbp-238h]
  struct stat *v18; // [rsp+8h] [rbp-230h]
  struct stat *v20; // [rsp+20h] [rbp-218h]
  unsigned int v21; // [rsp+2Ch] [rbp-20Ch]
  unsigned long long v22; // [rsp+38h] [rbp-200h] BYREF
  long long v23[11]; // [rsp+40h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+98h] [rbp-1A0h] BYREF
  long long v25; // [rsp+1D8h] [rbp-60h]
  int v26; // [rsp+1E8h] [rbp-50h]
  unsigned long long v27; // [rsp+1F8h] [rbp-40h]

  v27 = __readfsqword(0x28u);
  v3 = sub_1F810(a1);
  *(long long *)(a2 + 56) = v3;
  v4 = sub_1FED0(v3);
  if ( v4 )
  {
    v5 = *(long long *)(a2 + 24);
    v6 = (const char *)v4;
    v7 = v5 + 102;
    v17 = v5;
    if ( v5 <= 0x63 )
      v7 = 102;
    v22 = v7;
    v8 = (char *)sub_4ECC0(v7);
    v21 = *(int *)(a2 + 40);
    strcpy(v8, *(const char **)(a2 + 16));
    if ( v8[v5 - 1] != 47 )
    {
      v8[v5] = 47;
      v8[v5 + 1] = 0;
      v17 = v5 + 1;
    }
    v18 = 0;
    v9 = *v6;
    if ( *v6 )
    {
      v20 = 0;
      while ( 1 )
      {
        v10 = strlen(v6);
        if ( v9 == 68 )
          break;
LABEL_9:
        v6 += v10 + 1;
        v9 = *v6;
        if ( !*v6 )
          goto LABEL_10;
      }
      for ( ; v22 < v17 + v10; v8 = (char *)sub_4ED20(v8, &v22) )
        ;
      strcpy(&v8[v17], v6 + 1);
      p_buf = (struct stat *)v21;
      v13 = (struct stat *)sub_28210(v8, v21, 0, a2);
      v14 = v13;
      if ( v18 )
        v20->st_atim.tv_nsec = (__syscall_slong_t)v13;
      else
        v18 = v13;
      sub_2FB40(v23);
      v15 = *(int *)(a1 + 424);
      v25 = a1;
      if ( v15 >= 0 )
      {
        p_buf = (struct stat *)(v6 + 1);
        v16 = sub_14950(a1, (long long)(v6 + 1), dword_82808 | 0x10000u);
        if ( v16 >= 0 )
        {
          p_buf = &buf;
          v26 = v16;
          if ( fstat(v16, &buf) )
          {
            sub_26460((long long)v8);
          }
          else
          {
            p_buf = v14;
            v23[0] = sub_4EE30(v8);
            add_hierarchy_to_namelist(v23, v14);
            sub_14A80((long long)v23);
          }
          goto LABEL_19;
        }
      }
      else
      {
        *__errno_location() = -v15;
      }
      sub_26370((long long)v8);
LABEL_19:
      sub_2FC00(v23, p_buf);
      v20 = v14;
      goto LABEL_9;
    }
LABEL_10:
    free(v8);
    *(long long *)(a2 + 72) = v18;
  }
  return v27 - __readfsqword(0x28u);
}



// Function: add_starting_file @ 0x286f0
long long add_starting_file(char *a1)
{
  char *v1; // rbp
  long long *v2; // rbx
  const char *v3; // rax
  size_t v4; // rax
  bool v5; // zf
  long long *v6; // rbp
  void *v7; // rax
  int v8; // eax
  int v9; // eax
  long long result; // rax
  void **v11; // r12
  long long *v12; // rax

  v1 = a1;
  v2 = (long long *)sub_4ED80(96);
  if ( !a1 )
    v1 = "";
  v3 = (const char *)sub_4EE30(v1);
  v2[2] = v3;
  v4 = strlen(v3);
  v5 = byte_82A78 == 0;
  v6 = (long long *)qword_826B8;
  v2[3] = v4;
  if ( !v5 )
  {
    v11 = (void **)qword_826C0;
    v6 = (long long *)*((long long *)qword_826C0 + 1);
    v12 = *(long long **)qword_826C0;
    if ( v6 )
    {
      *v6 = v12;
      v12 = *v11;
    }
    else
    {
      qword_826C0 = *(void **)qword_826C0;
    }
    if ( v12 )
    {
      v12[1] = v6;
      v6 = (long long *)qword_826B8;
    }
    else
    {
      qword_826B8 = (long long)v6;
    }
    free(v11[2]);
    free(v11[11]);
    free(v11);
  }
  v7 = qword_826C0;
  v2[1] = 0;
  qword_826C0 = v2;
  *v2 = v7;
  if ( !v6 )
    qword_826B8 = (long long)v2;
  v8 = dword_826F8 | dword_7F238;
  v2[6] = 0;
  v9 = dword_82AD4 | v8;
  v5 = dword_826FC == 2;
  *((int *)v2 + 10) = 0;
  v2[7] = 0;
  v2[8] = 0;
  result = (v5 << 28) | (unsigned int)v9;
  *((char *)v2 + 36) = 1;
  *((int *)v2 + 8) = result;
  byte_82A78 = 1;
  return result;
}



// Function: name_match @ 0x28880
bool name_match(void *src, double a2)
{
  unsigned int *v3; // rbx
  char *v4; // rdi
  long long v6; // rax

  v3 = (unsigned int *)qword_826C0;
  while ( 1 )
  {
    if ( !v3 )
      return 1;
    v4 = (char *)*((long long *)v3 + 2);
    if ( !*v4 )
      break;
    while ( !(unsigned char)sub_40A20(v4, src) )
    {
      v3 = *(unsigned int **)v3;
      if ( !v3 )
      {
LABEL_8:
        if ( byte_82A78 && !qword_826C0 )
          byte_82A78 = 0;
        goto LABEL_9;
      }
      while ( 1 )
      {
        v4 = (char *)*((long long *)v3 + 2);
        if ( *v4 )
          break;
        v3 = *(unsigned int **)v3;
        if ( !v3 )
          goto LABEL_8;
      }
    }
    if ( !byte_82A78 )
      goto LABEL_18;
    if ( qword_826C0 == v3 )
    {
      byte_82A78 = 0;
LABEL_18:
      v6 = *((long long *)v3 + 6);
      if ( *((char *)src + *((long long *)v3 + 3)) != 47 || !dword_82AD4 || !v6 )
        *((long long *)v3 + 6) = v6 + 1;
      sub_26300(v3[10]);
      if ( qword_82B38 )
        return *((long long *)v3 + 6) == qword_82B38;
      else
        return *((long long *)v3 + 6) != 0;
    }
LABEL_9:
    if ( byte_82AAC )
    {
      if ( *((long long *)qword_826C0 + 6) )
      {
        sub_282F0(a2, (long long)v4, (long long)src);
        v3 = (unsigned int *)qword_826C0;
        if ( !*((long long *)qword_826C0 + 6) )
          continue;
      }
    }
    return 0;
  }
  sub_26300(v3[10]);
  qword_826C0 = 0;
  qword_826B8 = 0;
  return 1;
}



// Function: all_names_found @ 0x289e0
long long all_names_found(long long a1)
{
  long long v1; // rbx
  unsigned long long v2; // rbp
  unsigned int v3; // r12d
  size_t v4; // rax
  long long *v5; // rdx
  size_t v6; // rcx

  v1 = *(long long *)(a1 + 8);
  if ( v1 )
  {
    v2 = qword_82B38;
    v3 = 0;
    if ( !qword_82B38 )
      return v3;
    v3 = *(unsigned char *)(a1 + 16);
    if ( !(char)v3 )
    {
      v4 = strlen(*(const char **)(a1 + 8));
      v5 = qword_826C0;
      if ( qword_826C0 )
      {
        while ( !*(char *)v5[2] || v2 <= v5[6] )
        {
          v6 = v5[3];
          if ( v6 <= v4 && *(char *)(v1 + v6) == 47 )
            break;
          v5 = (long long *)*v5;
          if ( !v5 )
            return 1;
        }
      }
      else
      {
        return 1;
      }
      return v3;
    }
  }
  return 0;
}



// Function: names_notfound @ 0x28a70
void names_notfound(double a1, long long a2, char *a3)
{
  long long *v3; // rbx
  char *v4; // rdi
  unsigned long long v5; // rdx
  long long v6; // r13
  char *v7; // rdx
  long long v8; // rbp
  long long v9; // rdi
  long long v10; // r12
  char *v11; // rax
  long long *v12; // rax

  v3 = qword_826C0;
  if ( qword_826C0 )
  {
    do
    {
      while ( 1 )
      {
        v5 = v3[6];
        if ( qword_82B38 )
          break;
        if ( !v5 )
        {
          v4 = (char *)v3[2];
          if ( *v4 )
            goto LABEL_9;
        }
LABEL_5:
        v3 = (long long *)*v3;
        if ( !v3 )
          goto LABEL_14;
      }
      if ( qword_82B38 <= v5 )
        goto LABEL_5;
      v4 = (char *)v3[2];
      if ( !*v4 )
        goto LABEL_5;
LABEL_9:
      sub_26F30((long long)v4);
      if ( qword_82D50 )
        qword_82D50(v4);
      v6 = sub_4CE40(v3[2]);
      if ( v3[6] )
        v7 = dcgettext(0, "%s: Required occurrence not found in archive", 5);
      else
        v7 = dcgettext(0, "%s: Not found in archive", 5);
      a3 = 0;
      error(0, 0, v7, v6);
      v3 = (long long *)*v3;
      dword_82D58 = 2;
    }
    while ( v3 );
  }
LABEL_14:
  qword_826C0 = 0;
  qword_826B8 = 0;
  if ( byte_82AAC )
  {
    while ( 1 )
    {
      v12 = sub_27340((char *)&dword_0 + 1, a3, a1);
      if ( !v12 )
        break;
      v8 = v12[3];
      if ( !v8 )
        break;
      v9 = v12[3];
      sub_26F30(v9);
      if ( qword_82D50 )
        qword_82D50(v9);
      v10 = sub_4CE40(v8);
      v11 = dcgettext(0, "%s: Not found in archive", 5);
      a3 = 0;
      error(0, 0, v11, v10);
      dword_82D58 = 2;
    }
  }
}



// Function: label_notfound @ 0x28c00
long long *label_notfound(double a1, long long a2, char *a3)
{
  long long *result; // rax
  unsigned long long v4; // rdx
  long long *v5; // rbx
  long long v6; // rdi
  char *v7; // rax

  result = qword_826C0;
  if ( qword_826C0 )
  {
    do
    {
      v4 = result[6];
      if ( qword_82B38 )
      {
        if ( qword_82B38 <= v4 )
          return result;
      }
      else if ( v4 )
      {
        return result;
      }
      result = (long long *)*result;
    }
    while ( result );
    if ( dword_82A54 )
    {
      v7 = dcgettext(0, "Archive label mismatch", 5);
      a3 = 0;
      error(0, 0, v7);
    }
    result = (long long *)sub_2FD00(1);
    v5 = qword_826C0;
    if ( qword_826C0 )
    {
      do
      {
        result = (long long *)sub_26F30(v5[2]);
        if ( (int)result )
          break;
        v5 = (long long *)*v5;
      }
      while ( v5 );
    }
    qword_826C0 = 0;
    qword_826B8 = 0;
    if ( byte_82AAC )
    {
      do
      {
        result = sub_27340((char *)&dword_0 + 1, a3, a1);
        if ( !result )
          break;
        v6 = result[3];
        if ( !v6 )
          break;
        result = (long long *)sub_26F30(v6);
      }
      while ( !(int)result );
    }
  }
  return result;
}



// Function: collect_and_sort_names @ 0x28cf0
unsigned long long collect_and_sort_names(char *a1, long long a2, double a3)
{
  struct stat *v3; // rbp
  int v4; // eax
  int v5; // r12d
  struct stat *p_buf; // rsi
  long long *v7; // rax
  long long *v8; // rcx
  long long *i; // rdx
  int v10; // r13d
  long long v11; // r15
  long long v12; // rax
  unsigned int *v13; // rbx
  void *v14; // r14
  long long *v15; // rax
  void *v16; // r12
  long long v17; // rdi
  long long *v18; // rax
  long long *v19; // rdx
  unsigned int *v20; // rbp
  long long *v21; // rax
  long long *v22; // rdx
  long long *v23; // rcx
  long long v25; // rdi
  long long *v26; // rax
  long long *v27; // rdx
  int v28; // eax
  int v29; // eax
  void *st_nlink; // rdi
  long long v31; // rax
  const char *v32; // rsi
  char *v33; // rax
  long long v34[11]; // [rsp+0h] [rbp-1F8h] BYREF
  struct stat buf; // [rsp+58h] [rbp-1A0h] BYREF
  int v36; // [rsp+1A8h] [rbp-50h]
  unsigned long long v37; // [rsp+1B8h] [rbp-40h]

  v37 = __readfsqword(0x28u);
  sub_282F0(a3, (long long)a1, a2);
  v3 = (struct stat *)qword_826C0;
  if ( qword_826C0 )
  {
    if ( !qword_82B28 )
    {
LABEL_8:
      v5 = 0;
      while ( 1 )
      {
        if ( v3->st_size )
          goto LABEL_9;
        if ( v3->st_blksize )
          goto LABEL_9;
        if ( (v3->st_gid & 0x10000000) != 0 )
          goto LABEL_9;
        sub_26300(LODWORD(v3->st_rdev));
        if ( !*(char *)v3->st_nlink )
          goto LABEL_9;
        sub_2FB40(v34);
        p_buf = &buf;
        if ( (unsigned int)sub_25FC0((char *)v3->st_nlink, &buf) )
        {
          sub_26460(v3->st_nlink);
LABEL_9:
          v3 = (struct stat *)v3->st_dev;
          ++v5;
          if ( !v3 )
            goto LABEL_17;
        }
        else
        {
          if ( (buf.st_mode & 0xF000) == 0x4000 )
          {
            p_buf = (struct stat *)v3->st_nlink;
            v28 = __openat_2((unsigned int)oldfd, p_buf, dword_82808 | 0x10000u);
            if ( v28 < 0 )
            {
              sub_26370(v3->st_nlink);
            }
            else
            {
              p_buf = &buf;
              v36 = v28;
              v29 = fstat(v28, &buf);
              st_nlink = (void *)v3->st_nlink;
              if ( v29 )
              {
                sub_26460((long long)st_nlink);
              }
              else
              {
                v31 = sub_4EE30(st_nlink);
                ++v3->st_size;
                p_buf = v3;
                v34[0] = v31;
                sub_28480((long long)v34, (long long)v3);
              }
            }
          }
          ++v5;
          sub_2FC00(v34, p_buf);
          v3 = (struct stat *)v3->st_dev;
          if ( !v3 )
          {
LABEL_17:
            v3 = (struct stat *)qword_826C0;
            goto LABEL_18;
          }
        }
      }
    }
LABEL_3:
    v4 = sub_26160();
    if ( v4 )
    {
      if ( v4 == 1 )
      {
        if ( *((int *)qword_826C0 + 10) )
          goto LABEL_6;
        if ( qword_82D50 )
          qword_82D50(a1);
        v32 = "Using -C option inside file list is not allowed with --listed-incremental";
      }
      else
      {
        if ( qword_82D50 )
          qword_82D50(a1);
        v32 = "Only one -C option is allowed with --listed-incremental";
      }
      v33 = dcgettext(0, v32, 5);
      error(0, 0, v33);
      sub_2DEA0(2);
    }
LABEL_6:
    sub_203B0();
    goto LABEL_7;
  }
  a1 = ".";
  sub_28210(".", 0, 0, 0);
  if ( qword_82B28 )
    goto LABEL_3;
LABEL_7:
  v3 = (struct stat *)qword_826C0;
  if ( qword_826C0 )
    goto LABEL_8;
  v5 = 0;
LABEL_18:
  v7 = sub_26B90(v3, v5, (int (*)(long long *, long long *))sub_26D90);
  v8 = 0;
  for ( i = v7; i; i = (long long *)*i )
  {
    i[1] = v8;
    v8 = i;
  }
  qword_826C0 = v7;
  v10 = 0;
  v11 = 0;
  v12 = sub_43BF0(0, 0, sub_26FD0, sub_26D70, 0);
  v13 = (unsigned int *)qword_826C0;
  v14 = (void *)v12;
  if ( qword_826C0 )
  {
    do
    {
      while ( 1 )
      {
        v20 = v13;
        v13 = *(unsigned int **)v13;
        *((long long *)v20 + 11) = sub_26700((long long *)v20[10], *((char **)v20 + 2));
        if ( v11 )
        {
          v15 = (long long *)sub_43960(v14, v20);
          v16 = v15;
          if ( v15 )
            break;
        }
LABEL_32:
        *((long long *)v20 + 6) = 0;
        if ( !sub_44240(v14, v20) )
          sub_17020();
        ++v10;
        v11 = (long long)v20;
        if ( !v13 )
          goto LABEL_34;
      }
      if ( !*((long long *)v20 + 8) )
      {
        v25 = v15[9];
        if ( v25 )
          sub_27040(v25, *((long long *)v20 + 2), *((long long *)v20 + 3));
        sub_442A0(v14, v20);
        v26 = (long long *)*((long long *)v16 + 1);
        v27 = *(long long **)v16;
        if ( v26 )
          *v26 = v27;
        else
          qword_826C0 = *(void **)v16;
        if ( v27 )
          v27[1] = v26;
        else
          qword_826B8 = (long long)v26;
        --v10;
        free(*((void **)v16 + 2));
        free(*((void **)v16 + 11));
        free(v16);
        goto LABEL_32;
      }
      v17 = *((long long *)v20 + 9);
      if ( v17 )
        sub_27040(v17, v15[2], v15[3]);
      v18 = (long long *)*((long long *)v20 + 1);
      v19 = *(long long **)v20;
      if ( v18 )
        *v18 = v19;
      else
        qword_826C0 = *(void **)v20;
      if ( v19 )
        v19[1] = v18;
      else
        qword_826B8 = (long long)v18;
      free(*((void **)v20 + 2));
      free(*((void **)v20 + 11));
      free(v20);
    }
    while ( v13 );
  }
LABEL_34:
  qword_826B8 = v11;
  sub_43DA0(v14);
  v21 = sub_26B90(qword_826C0, v10, (int (*)(long long *, long long *))sub_26FE0);
  if ( !v21 )
  {
    qword_826C0 = 0;
    return v37 - __readfsqword(0x28u);
  }
  v22 = v21;
  v23 = 0;
  do
  {
    v22[1] = v23;
    v23 = v22;
    v22 = (long long *)*v22;
  }
  while ( v22 );
  qword_826C0 = v21;
  if ( !qword_82B28 )
    return v37 - __readfsqword(0x28u);
  while ( !*(char *)v21[2] )
  {
    v21 = (long long *)*v21;
    if ( !v21 )
      return v37 - __readfsqword(0x28u);
  }
  return sub_1FF30(v21[7]);
}



// Function: name_scan @ 0x291d0
long long *name_scan(void *src, long long a2, double a3)
{
  long long *v4; // r12
  char *v5; // rdi

  v4 = qword_826C0;
LABEL_2:
  if ( v4 )
  {
    while ( 1 )
    {
      v5 = (char *)v4[2];
      if ( *v5 )
      {
        a2 = (long long)src;
        if ( (unsigned char)sub_40A20(v5, src) )
          break;
      }
      v4 = (long long *)*v4;
      if ( !v4 )
      {
        if ( byte_82AAC )
        {
          if ( qword_826C0 )
          {
            if ( *((long long *)qword_826C0 + 6) )
            {
              sub_282F0(a3, (long long)v5, a2);
              if ( !*((long long *)qword_826C0 + 6) )
              {
                v4 = qword_826C0;
                goto LABEL_2;
              }
            }
          }
        }
        return v4;
      }
    }
  }
  return v4;
}



// Function: name_from_list @ 0x29260
long long name_from_list()
{
  long long *v0; // rax

  v0 = (long long *)qword_82620;
  if ( !qword_82620 )
  {
    v0 = qword_826C0;
    qword_82620 = (long long)qword_826C0;
    if ( !qword_826C0 )
      return 0;
  }
  while ( v0[6] || !*(char *)v0[2] )
  {
    v0 = (long long *)*v0;
    qword_82620 = (long long)v0;
    if ( !v0 )
      return 0;
  }
  v0[6] = 1;
  sub_26300(*((unsigned int *)v0 + 10));
  return qword_82620;
}



// Function: blank_name_list @ 0x292e0
long long *blank_name_list()
{
  long long *result; // rax

  result = qword_826C0;
  qword_82620 = 0;
  if ( qword_826C0 )
  {
    do
    {
      result[6] = 0;
      result = (long long *)*result;
    }
    while ( result );
  }
  return result;
}



// Function: make_file_name @ 0x29320
char *make_file_name(const char *src, const char *a2)
{
  size_t v2; // rbp
  size_t v3; // rbx
  size_t v4; // r15
  char *v5; // r12

  v2 = 0;
  v3 = strlen(src);
  v4 = strlen(a2) + 1;
  if ( v3 )
  {
    v2 = v3;
    if ( src[v3 - 1] != 47 )
      v2 = v3 + 1;
  }
  v5 = (char *)sub_4ECC0(v4 + v2);
  memcpy(v5, src, v3);
  v5[v3] = 47;
  memcpy(&v5[v2], a2, v4);
  return v5;
}



// Function: stripped_prefix_len @ 0x293b0
long long stripped_prefix_len(char *a1, long long a2)
{
  char v2; // dl
  char *v3; // rax

  v2 = *a1;
  v3 = a1;
  if ( *a1 != 47 )
    goto LABEL_6;
  do
    v2 = *++v3;
  while ( v2 == 47 );
  if ( !v2 )
    return -1;
  while ( 1 )
  {
    ++v3;
    if ( v2 == 47 )
      break;
    v2 = *v3;
LABEL_6:
    if ( !v2 )
      return -1;
  }
  if ( --a2 )
  {
    while ( 1 )
    {
      v2 = *v3;
      if ( *v3 != 47 )
        break;
      ++v3;
    }
    goto LABEL_6;
  }
  return v3 - a1;
}



// Function: contains_dot_dot @ 0x29420
bool contains_dot_dot(char *a1)
{
  char v1; // dl
  bool result; // al

  while ( 1 )
  {
    v1 = *a1;
    if ( *a1 == 46 && a1[1] == 46 )
    {
      result = a1[2] == 0 || a1[2] == 47;
      if ( result )
        break;
    }
    while ( 1 )
    {
      ++a1;
      if ( v1 == 47 )
        break;
      if ( !v1 )
        return 0;
      v1 = *a1;
    }
  }
  return result;
}



// Function: pax_sparse_member_p @ 0x29490
bool pax_sparse_member_p(long long a1)
{
  long long v1; // rdx
  bool result; // al

  v1 = *(long long *)(a1 + 24);
  result = 1;
  if ( !*(long long *)(v1 + 304) )
    return *(int *)(v1 + 292) != 0;
  return result;
}



// Function: star_fixup_header @ 0x294c0
long long star_fixup_header(long long a1)
{
  long long v1; // rax
  long long v2; // rdx
  long long v3; // rcx

  v1 = sub_231E0(qword_82408 + 452, 12);
  v2 = *(long long *)(a1 + 24);
  *(long long *)(v2 + 280) = *(long long *)(v2 + 136);
  v3 = 0;
  if ( v1 >= 0 )
    v3 = v1;
  *(long long *)(v2 + 136) = v3;
  return v1 >= 0;
}



// Function: oldgnu_fixup_header @ 0x29510
long long oldgnu_fixup_header(long long a1)
{
  long long v1; // rax
  long long v2; // rdx
  long long v3; // rcx

  v1 = sub_231E0(qword_82408 + 483, 12);
  v2 = *(long long *)(a1 + 24);
  *(long long *)(v2 + 280) = *(long long *)(v2 + 136);
  v3 = 0;
  if ( v1 >= 0 )
    v3 = v1;
  *(long long *)(v2 + 136) = v3;
  return v1 >= 0;
}



// Function: sparse_add_map @ 0x29560
long long sparse_add_map(long long *a1, const __m128i *a2, unsigned long long a3, unsigned long long a4)
{
  long long v6; // rdi
  long long v7; // rbp
  unsigned long long v8; // rax
  long long v10; // rsi
  long long v11; // rax

  v6 = a1[40];
  v7 = a1[38];
  v8 = a1[39];
  if ( v8 == v7 )
  {
    if ( v6 )
    {
      if ( v8 <= 0x555555555555554LL )
      {
        a3 = v8 >> 1;
        v8 += (v8 >> 1) + 1;
        v10 = 16 * v8;
LABEL_6:
        a1[39] = v8;
        v11 = sub_4ECE0(v6, v10, a3, a4);
        a1[40] = v11;
        v6 = v11;
        goto LABEL_2;
      }
    }
    else
    {
      if ( !v8 )
      {
        v10 = 128;
        v8 = 8;
        goto LABEL_6;
      }
      a4 = v8 >> 60;
      a3 = v8 >> 60 != 0;
      v10 = 16 * v8;
      if ( (v8 & 0x800000000000000LL) == 0 && !(v8 >> 60) )
        goto LABEL_6;
    }
    sub_17020();
  }
LABEL_2:
  *(__m128i *)(v6 + 16 * v7) = _mm_loadu_si128(a2);
  a1[38] = v7 + 1;
  return 16 * v7;
}



// Function: pax_dump_header @ 0x29630
long long pax_dump_header(long long a1, long long a2)
{
  long long v3; // rdx
  int v4; // eax
  long long v5; // rax
  long long v6; // rsi
  long long v7; // r12
  long long v8; // r13
  long long *v9; // rsi
  long long v10; // rbp
  unsigned int v11; // r13d
  unsigned char *v12; // rax
  long long v13; // rdi
  long long *v14; // rax
  long long *v15; // rbp
  const char *v16; // rax
  size_t v17; // rax
  long long v18; // rsi
  size_t v19; // r12
  long long *v20; // r15
  unsigned long long v21; // r13
  long long v22; // rdi
  const char *v23; // rax
  size_t v24; // r14
  const char *v25; // rax
  size_t v26; // rax
  long long v27; // rax
  long long v28; // r12
  unsigned long long v29; // rax
  long long *v30; // r12
  long long *v31; // r12
  long long v32; // r13
  long long v33; // r13
  unsigned char *v34; // rax
  long long v35; // rdi
  unsigned char *v36; // rsi
  void *v37; // rdi
  long long v38; // rcx
  long long v39; // r8
  long long v40; // r15
  char *v41; // r14
  long long v42; // rax
  char *v43; // rcx
  char *i; // r12
  char *v45; // r12
  unsigned long long v46; // r13
  unsigned long long v47; // rsi
  long long v48; // rax
  char *v49; // rcx
  char *j; // r14
  char *v51; // rax
  long long v52; // rdx
  long long v53; // rcx
  long long v54; // r8
  long long v55; // rdx
  long long v56; // rcx
  long long v57; // r8
  char v58; // dl
  char *v59; // rax
  long long v60; // rdx
  long long v61; // rcx
  long long v62; // r8
  long long v63; // rdx
  long long v64; // rcx
  long long v65; // r8
  unsigned long long v66; // rsi
  char *v67; // r14
  long long v68; // rax
  char *v69; // rcx
  char *k; // r12
  char *v71; // rax
  long long v72; // rdx
  long long v73; // rcx
  long long v74; // r8
  long long v75; // rdx
  long long v76; // rcx
  long long v77; // r8
  long long *v79; // rbp
  long long v80; // rdx
  unsigned long long v81; // rax
  const char *v82; // rax
  const char *v83; // rax
  long long v84; // rbp
  long long v85; // r14
  unsigned char *v86; // rax
  long long v87; // [rsp+8h] [rbp-80h]
  char *v88; // [rsp+8h] [rbp-80h]
  long long v89; // [rsp+8h] [rbp-80h]
  long long v90; // [rsp+18h] [rbp-70h]
  unsigned long long v91; // [rsp+28h] [rbp-60h] BYREF
  char v92[24]; // [rsp+30h] [rbp-58h] BYREF
  unsigned long long v93; // [rsp+48h] [rbp-40h]

  v3 = *(long long *)(a1 + 24);
  v93 = __readfsqword(0x28u);
  v4 = dword_82A84;
  *(int *)(v3 + 296) = dword_82A80;
  *(int *)(v3 + 292) = v4;
  if ( v4 )
  {
    v87 = sub_CC60(a1, a2);
    v14 = *(long long **)(a1 + 24);
    v15 = (long long *)v14[40];
    v90 = v14[1];
    v16 = (const char *)sub_45370(v14[38], v92);
    v17 = strlen(v16);
    v18 = *(long long *)(a1 + 24);
    v19 = v17 + 1;
    if ( *(long long *)(v18 + 304) )
    {
      v20 = v15;
      v21 = 0;
      do
      {
        v22 = *v20;
        ++v21;
        v20 += 2;
        v23 = (const char *)sub_45370(v22, v92);
        v24 = strlen(v23);
        v25 = (const char *)sub_45370(*(v20 - 1), v92);
        v26 = strlen(v25);
        v18 = *(long long *)(a1 + 24);
        v19 += v26 + v24 + 2;
      }
      while ( v21 < *(long long *)(v18 + 304) );
    }
    v27 = v19 + 1022;
    v28 = v19 + 511;
    if ( v28 >= 0 )
      v27 = v28;
    v29 = v27 & 0xFFFFFFFFFFFFFE00LL;
    *(long long *)(v18 + 280) += v29;
    *(long long *)(a1 + 16) += v29;
    sub_1DA70("GNU.sparse.major", v18, 0);
    sub_1DA70("GNU.sparse.minor", *(long long *)(a1 + 24), 0);
    sub_1DA70("GNU.sparse.name", *(long long *)(a1 + 24), 0);
    sub_1DA70("GNU.sparse.realsize", *(long long *)(a1 + 24), 0);
    v30 = *(long long **)(a1 + 24);
    v30[1] = sub_1CDF0(v30, "%d/GNUSparseFile.%p/%f", 0);
    v31 = *(long long **)(a1 + 24);
    v32 = v31[1];
    if ( strlen((const char *)v32) > 0x64 )
    {
      *(char *)(v32 + 100) = 0;
      v31 = *(long long **)(a1 + 24);
    }
    v33 = v31[17];
    v31[17] = v31[35];
    v34 = (unsigned char *)sub_13570((long long)v31);
    v35 = *(long long *)(a1 + 24);
    v31[17] = v33;
    v36 = v34;
    sub_13320(v35, v34, v87);
    v37 = *(void **)(*(long long *)(a1 + 24) + 8LL);
    free(v37);
    *(long long *)(*(long long *)(a1 + 24) + 8LL) = v90;
    v40 = sub_D360(v37, v36, v90, v38, v39);
    v41 = (char *)v40;
    v42 = sub_45370(*(long long *)(*(long long *)(a1 + 24) + 304LL), v92);
    v43 = (char *)(v40 + 512);
    for ( i = (char *)v42; *i; *v59 = v58 )
    {
      v59 = v41;
      if ( v43 == v41 )
      {
        sub_CCD0(v40);
        v59 = (char *)sub_D360(v40, v92, v55, v56, v57);
        v40 = (long long)v59;
        v43 = v59 + 512;
      }
      v58 = *i++;
      v41 = v59 + 1;
    }
    if ( v43 == v41 )
    {
      sub_CCD0(v40);
      v40 = sub_D360(v40, v92, v60, v61, v62);
      v41 = (char *)v40;
    }
    *v41 = 10;
    v45 = v41 + 1;
    if ( *(long long *)(*(long long *)(a1 + 24) + 304LL) )
    {
      v46 = 0;
      do
      {
        v47 = (unsigned long long)v92;
        v48 = sub_45370(*v15, v92);
        v49 = (char *)(v40 + 512);
        for ( j = (char *)v48; *j; *v51 = v47 )
        {
          v51 = v45;
          if ( v45 == v49 )
          {
            sub_CCD0(v40);
            v51 = (char *)sub_D360(v40, v47, v52, v53, v54);
            v40 = (long long)v51;
            v49 = v51 + 512;
          }
          v47 = (unsigned char)*j++;
          v45 = v51 + 1;
        }
        if ( v49 == v45 )
        {
          sub_CCD0(v40);
          v40 = sub_D360(v40, v47, v63, v64, v65);
          v45 = (char *)v40;
          v49 = (char *)(v40 + 512);
        }
        *v45 = 10;
        v66 = (unsigned long long)v92;
        v67 = v45 + 1;
        v88 = v49;
        v68 = sub_45370(v15[1], v92);
        v69 = v88;
        for ( k = (char *)v68; *k; *v71 = v66 )
        {
          v71 = v67;
          if ( v67 == v69 )
          {
            sub_CCD0(v40);
            v71 = (char *)sub_D360(v40, v66, v72, v73, v74);
            v40 = (long long)v71;
            v69 = v71 + 512;
          }
          v66 = (unsigned char)*k++;
          v67 = v71 + 1;
        }
        if ( v69 == v67 )
        {
          sub_CCD0(v40);
          v40 = sub_D360(v40, v66, v75, v76, v77);
          v67 = (char *)v40;
        }
        *v67 = 10;
        v45 = v67 + 1;
        ++v46;
        v15 += 2;
      }
      while ( v46 < *(long long *)(*(long long *)(a1 + 24) + 304LL) );
    }
    v11 = 1;
    memset(v45, 0, v40 - (long long)v45 + 512);
    sub_CCD0(v40);
  }
  else
  {
    v5 = sub_CC60(a1, a2);
    v6 = *(long long *)(a1 + 24);
    v7 = v5;
    v8 = *(long long *)(v6 + 320);
    sub_1DA70("GNU.sparse.size", v6, 0);
    sub_1DA70("GNU.sparse.numblocks", *(long long *)(a1 + 24), 0);
    if ( (unsigned char)sub_1C9C0("GNU.sparse.map") || !dword_82A80 )
    {
      v91 = 0;
      v9 = *(long long **)(a1 + 24);
      if ( v9[38] )
      {
        do
        {
          sub_1DA70("GNU.sparse.offset", (long long)v9, (long long)&v91);
          sub_1DA70("GNU.sparse.numbytes", *(long long *)(a1 + 24), (long long)&v91);
          v9 = *(long long **)(a1 + 24);
          ++v91;
        }
        while ( v91 < v9[38] );
      }
      v10 = v9[17];
      v11 = 1;
      v9[17] = v9[35];
      v12 = (unsigned char *)sub_13570((long long)v9);
      v13 = *(long long *)(a1 + 24);
      v9[17] = v10;
      sub_13320(v13, v12, v7);
    }
    else
    {
      sub_1DA70("GNU.sparse.name", *(long long *)(a1 + 24), 0);
      v79 = *(long long **)(a1 + 24);
      v89 = v79[1];
      v79[1] = sub_1CDF0(v79, "%d/GNUSparseFile.%p/%f", 0);
      sub_1DE70(*(long long *)(a1 + 24) + 360LL);
      v80 = *(long long *)(a1 + 24);
      v81 = 0;
      v91 = 0;
      if ( *(long long *)(v80 + 304) )
      {
        while ( 1 )
        {
          v82 = (const char *)sub_45370(*(long long *)(v8 + 16 * v81), v92);
          sub_1DE80((long long *)(*(long long *)(a1 + 24) + 360LL), v82);
          sub_1DE80((long long *)(*(long long *)(a1 + 24) + 360LL), ",");
          v83 = (const char *)sub_45370(*(long long *)(v8 + 16 * v91 + 8), v92);
          sub_1DE80((long long *)(*(long long *)(a1 + 24) + 360LL), v83);
          v80 = *(long long *)(a1 + 24);
          v81 = v91 + 1;
          v91 = v81;
          if ( v81 >= *(long long *)(v80 + 304) )
            break;
          if ( v81 )
          {
            sub_1DE80((long long *)(v80 + 360), ",");
            v81 = v91;
          }
        }
      }
      v11 = sub_1DF00((long long *)(v80 + 360), "GNU.sparse.map");
      if ( !(char)v11
        || (v84 = *(long long *)(a1 + 24),
            v85 = *(long long *)(v84 + 136),
            *(long long *)(v84 + 136) = *(long long *)(v84 + 280),
            v86 = (unsigned char *)sub_13570(v84),
            *(long long *)(v84 + 136) = v85,
            sub_13320(*(long long *)(a1 + 24), v86, v7),
            v89) )
      {
        free(*(void **)(*(long long *)(a1 + 24) + 8LL));
        *(long long *)(*(long long *)(a1 + 24) + 8LL) = v89;
      }
    }
  }
  return v11;
}



// Function: star_get_sparse_info @ 0x29e30
long long star_get_sparse_info(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v5; // r12
  long long v6; // rbx
  bool v7; // zf
  long long v8; // rax
  long long v9; // r13
  long long v10; // rbx
  unsigned long long v11; // rbp
  int v13; // eax
  long long v14; // r12
  char *v15; // rax
  long long v16; // rbp
  char *v17; // rax

  v5 = a1;
  v6 = qword_82408;
  v7 = *(char *)(qword_82408 + 345) == 0;
  *(long long *)(*(long long *)(a1 + 24) + 304LL) = 0;
  if ( v7 && *(char *)(v6 + 366) )
  {
    v16 = v6 + 356;
    while ( 1 )
    {
      if ( !*(char *)(v16 + 12) )
        return 1;
      a2 = v16;
      a1 = v5;
      v13 = sub_29D90(v5, v16);
      if ( v13 )
        break;
      v16 += 24;
      if ( v6 + 452 == v16 )
      {
        if ( *(char *)(v6 + 355) )
          goto LABEL_3;
        return 1;
      }
    }
LABEL_12:
    if ( v13 != 2 )
      return 1;
    if ( qword_82D50 )
      qword_82D50(a1);
    v14 = **(long long **)(v5 + 24);
    v15 = dcgettext(0, "%s: invalid sparse archive member", 5);
    error(0, 0, v15, v14);
    dword_82D58 = 2;
    return 0;
  }
  else
  {
LABEL_3:
    while ( 1 )
    {
      v8 = sub_D360(a1, a2, a3, a4, a5);
      v9 = v8;
      if ( !v8 )
        break;
      v10 = v8;
      v11 = 0;
      sub_CCD0(v8);
      do
      {
        if ( !*(char *)(v10 + 12) )
        {
          *(long long *)(v5 + 16) += 512LL;
          return 1;
        }
        a2 = v10;
        a1 = v5;
        ++v11;
        v13 = sub_29D90(v5, v10);
        LOBYTE(a3) = v13 == 0;
        v10 += 24;
      }
      while ( v11 <= 0x14 && !v13 );
      *(long long *)(v5 + 16) += 512LL;
      if ( !*(char *)(v9 + 504) || v13 )
        goto LABEL_12;
    }
    if ( qword_82D50 )
      qword_82D50(a1);
    v17 = dcgettext(0, "Unexpected EOF in archive", 5);
    error(0, 0, v17);
    dword_82D58 = 2;
    return 0;
  }
}



// Function: oldgnu_get_sparse_info @ 0x29fd0
long long oldgnu_get_sparse_info(long long a1)
{
  long long v2; // rbp
  long long v3; // rbx
  long long v4; // r13
  long long v6; // rsi
  long long v7; // rdi
  int v8; // eax
  long long v9; // rdx
  long long v10; // rcx
  long long v11; // r8
  long long v12; // rax
  long long v13; // r13
  long long v14; // rbx
  unsigned long long v15; // rbp
  long long v16; // r12
  char *v17; // rax
  char *v18; // rax

  v2 = qword_82408;
  v3 = qword_82408 + 386;
  v4 = qword_82408 + 482;
  *(long long *)(*(long long *)(a1 + 24) + 304LL) = 0;
  do
  {
    if ( !*(char *)(v3 + 12) )
      return 1;
    v6 = v3;
    v7 = a1;
    v8 = sub_29D90(a1, v3);
    if ( v8 )
    {
LABEL_14:
      if ( v8 == 2 )
      {
        if ( qword_82D50 )
          qword_82D50(v7);
        v16 = **(long long **)(a1 + 24);
        v17 = dcgettext(0, "%s: invalid sparse archive member", 5);
        error(0, 0, v17, v16);
        dword_82D58 = 2;
        return 0;
      }
      return 1;
    }
    v3 += 24;
  }
  while ( v3 != v4 );
  if ( !*(char *)(v2 + 482) )
    return 1;
LABEL_7:
  v12 = sub_D360(v7, v6, v9, v10, v11);
  v13 = v12;
  if ( v12 )
  {
    v14 = v12;
    v15 = 0;
    sub_CCD0(v12);
    while ( *(char *)(v14 + 12) )
    {
      v6 = v14;
      v7 = a1;
      ++v15;
      v8 = sub_29D90(a1, v14);
      LOBYTE(v9) = v8 == 0;
      v14 += 24;
      if ( v15 > 0x14 || v8 )
      {
        if ( *(char *)(v13 + 504) && !v8 )
          goto LABEL_7;
        goto LABEL_14;
      }
    }
    return 1;
  }
  if ( qword_82D50 )
    qword_82D50(v7);
  v18 = dcgettext(0, "Unexpected EOF in archive", 5);
  error(0, 0, v18);
  dword_82D58 = 2;
  return 0;
}



// Function: sparse_extract_region @ 0x2a650
long long sparse_extract_region(long long a1, long long a2)
{
  long long v2; // rbp
  long long v3; // rbx
  char *v4; // rsi
  long long v5; // rdx
  long long v6; // rcx
  long long v7; // r8
  unsigned int v8; // r14d
  long long v9; // r13
  unsigned int v10; // ebp
  size_t v12; // r12
  long long v13; // rax
  char *v14; // rbp
  int v15; // edi
  unsigned long long v16; // rbp
  char *v17; // rax

  v2 = 16 * a2;
  v3 = a1;
  v4 = *(char **)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * a2);
  v8 = sub_2A5B0(a1, v4);
  if ( (char)v8 )
  {
    v9 = *(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + v2 + 8);
    if ( v9 )
    {
      while ( v9 > 0 )
      {
        v12 = 512;
        if ( v9 <= 512 )
          v12 = v9;
        v13 = sub_D360(a1, v4, v5, v6, v7);
        v14 = (char *)v13;
        if ( !v13 )
        {
          if ( qword_82D50 )
            qword_82D50(a1);
          v17 = dcgettext(0, "Unexpected EOF in archive", 5);
          error(0, 0, v17);
          dword_82D58 = 2;
          return 0;
        }
        sub_CCD0(v13);
        v15 = *(int *)v3;
        v4 = v14;
        *(long long *)(v3 + 16) += 512LL;
        v16 = sub_26070(v15, v14, v12);
        v9 -= v16;
        a1 = *(long long *)(*(long long *)(v3 + 24) + 280LL) - *(long long *)(v3 + 16);
        sub_C710(a1);
        *(long long *)(v3 + 8) += v16;
        if ( v12 != v16 )
        {
          sub_33510(**(long long **)(v3 + 24), v16, v12);
          return 0;
        }
      }
    }
    else
    {
      v10 = *(unsigned char *)(a1 + 4);
      if ( (char)v10 )
      {
        if ( (unsigned int)sub_2C110(*(int *)a1) )
        {
          v8 = v10;
          sub_33470(**(long long **)(a1 + 24));
        }
      }
    }
  }
  return v8;
}



// Function: sparse_dump_region @ 0x2a7c0
long long sparse_dump_region(int *a1, long long a2)
{
  int *v2; // rbp
  long long *v3; // rsi
  long long v4; // rbx
  long long v5; // rsi
  long long v6; // rdx
  long long v7; // rcx
  long long v8; // r8
  size_t v9; // r12
  char *v10; // r15
  long long v11; // rax
  long long v12; // r14
  long long v14; // rdi
  unsigned long long v15; // r12
  long long v16; // rbx
  long long v17; // r13
  char *v18; // rax
  unsigned char v19; // [rsp+7h] [rbp-F1h]
  long long v20; // [rsp+8h] [rbp-F0h]
  struct stat buf; // [rsp+10h] [rbp-E8h] BYREF
  char v22[24]; // [rsp+A0h] [rbp-58h] BYREF
  unsigned long long v23; // [rsp+B8h] [rbp-40h]

  v2 = a1;
  v23 = __readfsqword(0x28u);
  v20 = 16 * a2;
  v3 = (long long *)(*(long long *)(*((long long *)a1 + 3) + 320LL) + 16 * a2);
  v4 = v3[1];
  v5 = *v3;
  v19 = sub_2A5B0(a1, v5);
  if ( v19 && v4 > 0 )
  {
    while ( 1 )
    {
      v9 = 512;
      if ( v4 <= 512 )
        v9 = v4;
      v10 = (char *)sub_D360(a1, v5, v6, v7, v8);
      v11 = sub_4D570(*v2, v10, v9);
      v12 = v11;
      if ( v11 == -1 )
      {
        sub_263A0();
        return 0;
      }
      if ( !v11 )
        break;
      v5 = 0;
      v4 -= v11;
      memset(&v10[v11], 0, 512 - v11);
      *((long long *)v2 + 2) += v12;
      a1 = (int *)v10;
      sub_CCD0(v10);
      if ( v4 <= 0 )
        return v19;
    }
    v14 = (unsigned int)*v2;
    if ( fstat(v14, &buf) )
      v15 = *(long long *)(*((long long *)v2 + 3) + 136LL)
          + v4
          - (*(long long *)(*(long long *)(*((long long *)v2 + 3) + 320LL) + v20)
           + *(long long *)(*(long long *)(*((long long *)v2 + 3) + 320LL) + v20 + 8));
    else
      v15 = *(long long *)(*((long long *)v2 + 3) + 136LL) - buf.st_size;
    if ( (dword_81B80 & 0x80u) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(v14);
      v16 = sub_45370(v15, v22);
      v17 = sub_4CE40(**((long long **)v2 + 3));
      v18 = dcngettext(
              0,
              "%s: File shrank by %s byte; padding with zeros",
              "%s: File shrank by %s bytes; padding with zeros",
              v15,
              5);
      error(0, 0, v18, v17, v16);
    }
    if ( !byte_82B52 )
      sub_2FD00(1);
    return 0;
  }
  return v19;
}



// Function: check_sparse_region @ 0x2a9d0
long long check_sparse_region(long long a1, long long a2, long long a3)
{
  long long v3; // r14
  unsigned int v5; // r12d
  size_t v6; // r13
  long long v7; // rax
  char *v8; // rdx
  char *v10; // rax
  long long v11; // r12
  char *v12; // rax
  long long v13; // [rsp+0h] [rbp-268h] BYREF
  char buf[520]; // [rsp+20h] [rbp-248h] BYREF
  unsigned long long v15; // [rsp+228h] [rbp-40h]

  v3 = a2;
  v15 = __readfsqword(0x28u);
  v5 = sub_2A5B0(a1, a2);
  if ( !(char)v5 || a2 >= a3 )
    return v5;
  while ( 2 )
  {
    v6 = a3 - v3;
    if ( a3 - v3 > 512 )
      v6 = 512;
    v7 = sub_4D570(*(int *)a1, buf, v6);
    if ( v7 == -1 )
    {
      sub_263A0();
      return 0;
    }
    if ( !v7 )
    {
      v10 = dcgettext(0, "Size differs", 5);
      sub_11120(*(long long *)(a1 + 24), (long long)v10);
      return 0;
    }
    v8 = buf;
    do
    {
      if ( *v8++ )
      {
        v11 = sub_452D0(v3, &v13);
        v12 = dcgettext(0, "File fragment at %s is not a hole", 5);
        sub_11120(*(long long *)(a1 + 24), (long long)v12, v11);
        return 0;
      }
    }
    while ( &buf[v7] != v8 );
    v3 += v7;
    if ( a3 > v3 )
      continue;
    break;
  }
  return v5;
}



// Function: check_data_region @ 0x2ab20
long long check_data_region(long long a1, long long a2)
{
  long long v2; // r15
  char *v4; // rsi
  long long v6; // rax
  long long v7; // rdi
  long long v8; // rbp
  long long v9; // rdx
  long long v10; // rcx
  long long v11; // r8
  int v12; // edi
  long long v13; // rax
  size_t v14; // r14
  size_t v15; // r12
  long long v16; // rax
  const void *v17; // r15
  char *v18; // rax
  char *v19; // rax
  char *v20; // rax
  unsigned char v21; // [rsp+7h] [rbp-251h]
  char s2[520]; // [rsp+10h] [rbp-248h] BYREF
  unsigned long long v23; // [rsp+218h] [rbp-40h]

  v2 = 16 * a2;
  v23 = __readfsqword(0x28u);
  v4 = *(char **)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + 16 * a2);
  v21 = sub_2A5B0(a1, v4);
  if ( v21 )
  {
    v6 = *(long long *)(a1 + 24);
    v7 = *(long long *)(v6 + 280) - *(long long *)(a1 + 16);
    v8 = *(long long *)(*(long long *)(v6 + 320) + v2 + 8);
    sub_C710(v7);
    while ( 1 )
    {
      if ( v8 <= 0 )
        return v21;
      v15 = 512;
      if ( v8 <= 512 )
        v15 = v8;
      v16 = sub_D360(v7, v4, v9, v10, v11);
      v17 = (const void *)v16;
      if ( !v16 )
      {
        if ( qword_82D50 )
          qword_82D50(v7);
        v18 = dcgettext(0, "Unexpected EOF in archive", 5);
        error(0, 0, v18);
        dword_82D58 = 2;
        return 0;
      }
      sub_CCD0(v16);
      v12 = *(int *)a1;
      *(long long *)(a1 + 16) += 512LL;
      v13 = sub_4D570(v12, s2, v15);
      v14 = v13;
      if ( v13 == -1 )
        break;
      if ( !v13 )
      {
        v19 = dcgettext(0, "Size differs", 5);
        sub_11120((long long)&qword_82840, (long long)v19);
        return 0;
      }
      v8 -= v13;
      sub_C710(*(long long *)(*(long long *)(a1 + 24) + 280LL) - *(long long *)(a1 + 16));
      v4 = s2;
      v7 = (long long)v17;
      if ( memcmp(v17, s2, v14) )
      {
        v20 = dcgettext(0, "Contents differ", 5);
        sub_11120(*(long long *)(a1 + 24), (long long)v20);
        return 0;
      }
    }
    sub_263A0();
    return 0;
  }
  return v21;
}



// Function: oldgnu_dump_header @ 0x2ad50
long long oldgnu_dump_header(long long a1, long long a2)
{
  long long v3; // r14
  long long *v4; // rax
  unsigned char *v5; // r13
  long long v6; // rax
  unsigned char *v7; // rbx
  long long v8; // rdi
  unsigned long long v9; // rbp
  long long v10; // r15
  unsigned char *v11; // rsi
  bool v12; // al
  unsigned char *v13; // rsi
  long long v14; // rdx
  long long v15; // rcx
  long long v16; // r8
  long long *v17; // r13
  long long *v18; // rbx
  long long v19; // rax
  long long v20; // r15

  v3 = sub_CC60(a1, a2);
  v4 = sub_13570(*(long long *)(a1 + 24));
  *((char *)v4 + 156) = 83;
  v5 = (unsigned char *)v4;
  v6 = *(long long *)(a1 + 24);
  if ( *(long long *)(v6 + 304) > 4u )
    v5[482] = 1;
  v7 = v5 + 386;
  sub_12F80(*(long long *)(v6 + 136), v5 + 483, 12);
  sub_12F80(*(long long *)(*(long long *)(a1 + 24) + 280LL), v5 + 124, 12);
  v8 = *(long long *)(a1 + 24);
  v9 = *(long long *)(v8 + 304);
  if ( v9 )
  {
    v9 = 0;
    do
    {
      v10 = 16 * v9++;
      sub_12F80(*(long long *)(*(long long *)(v8 + 320) + v10), v7, 12);
      v11 = v7 + 12;
      v7 += 24;
      sub_12F80(*(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + v10 + 8), v11, 12);
      v8 = *(long long *)(a1 + 24);
      v12 = v9 < *(long long *)(v8 + 304);
    }
    while ( v9 != 4 && v9 < *(long long *)(v8 + 304) );
  }
  else
  {
    v12 = 0;
  }
  v5[482] = v12;
  v13 = v5;
  sub_13320(v8, v5, v3);
  while ( *(long long *)(*(long long *)(a1 + 24) + 304LL) > v9 )
  {
    v17 = (long long *)sub_D360(v8, v13, v14, v15, v16);
    v18 = v17;
    *v17 = 0;
    v17[63] = 0;
    memset(
      (void *)((unsigned long long)(v17 + 1) & 0xFFFFFFFFFFFFFFF8LL),
      0,
      8LL * (((unsigned int)v17 - (((int)v17 + 8) & 0xFFFFFFF8) + 512) >> 3));
    v19 = *(long long *)(a1 + 24);
    if ( *(long long *)(v19 + 304) > v9 )
    {
      while ( 1 )
      {
        v20 = 16 * v9++;
        sub_12F80(*(long long *)(*(long long *)(v19 + 320) + v20), v18, 12);
        v13 = (unsigned char *)v18 + 12;
        v18 += 3;
        sub_12F80(*(long long *)(*(long long *)(*(long long *)(a1 + 24) + 320LL) + v20 + 8), v13, 12);
        v19 = *(long long *)(a1 + 24);
        if ( v9 >= *(long long *)(v19 + 304) )
          break;
        if ( v17 + 63 == v18 )
        {
          *((char *)v17 + 504) = 1;
          break;
        }
      }
    }
    v8 = (long long)v17;
    sub_CCD0(v17);
  }
  return 1;
}



// Function: sparse_dump_file @ 0x2af80
long long (*sparse_dump_file(int fd, __m128i *a2))(int *)
{
  unsigned int v2; // eax
  unsigned long long v3; // rdx
  unsigned long long v4; // rcx
  long long (*result)(int *); // rax
  bool v6; // zf
  long long *v7; // rbp
  long long *v8; // rax
  unsigned char (*v9)(int *, long long, long long); // rax
  long long v10; // r12
  long long v11; // rax
  unsigned long long v12; // rdx
  unsigned long long v13; // rcx
  long long v14; // r13
  char *v15; // rdx
  char *v16; // rax
  long long v17; // rdx
  __off_t v18; // r12
  __off_t v19; // rax
  unsigned long long v20; // rcx
  __off_t v21; // rax
  long long v22; // r15
  long long *v23; // rax
  unsigned char (*v24)(int *, long long, char *); // rax
  char *v25; // rdx
  long long *v26; // rax
  unsigned char (*v27)(int *, long long, long long); // rcx
  void (*v28)(int *); // rax
  __m128i *v29; // rbx
  unsigned char (*v30)(int *, __m128i *); // rax
  int fda; // [rsp+8h] [rbp-2A0h]
  char v32; // [rsp+17h] [rbp-291h]
  __m128i v33; // [rsp+20h] [rbp-288h] BYREF
  int v34; // [rsp+30h] [rbp-278h] BYREF
  char v35; // [rsp+34h] [rbp-274h]
  long long v36; // [rsp+40h] [rbp-268h]
  long long *v37; // [rsp+48h] [rbp-260h]
  long long *v38; // [rsp+50h] [rbp-258h]
  char buf[520]; // [rsp+60h] [rbp-248h] BYREF
  unsigned long long v40; // [rsp+268h] [rbp-40h]

  v40 = __readfsqword(0x28u);
  v2 = sub_29D10(&v34);
  v4 = v2;
  v32 = v2;
  result = (long long (*)(int *))(&dword_0 + 3);
  if ( !(char)v4 )
    return result;
  v6 = a2[9].m128i_i64[1] == 0;
  v37 = (long long *)a2;
  v7 = (long long *)a2;
  v34 = fd;
  v35 = 1;
  v33.m128i_i64[1] = 0;
  if ( v6 )
  {
    a2[17].m128i_i64[1] = 0;
LABEL_48:
    a2 = &v33;
    v33.m128i_i64[0] = v7[17];
    sub_29560(v7, &v33, v3, v4);
    v26 = v38;
    goto LABEL_40;
  }
  if ( dword_82A7C != 1 )
  {
    if ( (dword_82A7C & 0xFFFFFFFD) != 0 )
      goto LABEL_5;
    v17 = 0;
    v18 = 0;
    v33.m128i_i64[0] = 0;
    while ( 1 )
    {
      a2[17].m128i_i64[1] = v17;
      v21 = lseek(fd, v18, 3);
      v22 = v21;
      if ( v21 == -1 )
        break;
      v19 = lseek(fd, v21, 4);
      if ( !(v22 | v18) && v19 == a2[8].m128i_i64[1] )
      {
        lseek(fd, 0, 0);
        v7 = v37;
        fda = v34;
        goto LABEL_10;
      }
      v33.m128i_i64[0] = v22;
      v18 = v19;
      v33.m128i_i64[1] = v19 - v22;
      sub_29560(a2, &v33, v19 - v22, v20);
      v17 = a2[17].m128i_i64[1] + v33.m128i_i64[1];
    }
    if ( *__errno_location() != 6 )
    {
      v7 = v37;
      fda = v34;
      goto LABEL_10;
    }
    v33.m128i_i64[1] = 0;
    goto LABEL_48;
  }
  fda = fd;
LABEL_10:
  v33 = 0u;
  v8 = v38;
  v7[35] = 0;
  v9 = (unsigned char (*)(int *, long long, long long))v8[6];
  if ( v9 )
  {
    a2 = 0;
    if ( !v9(&v34, 0, 0) )
    {
LABEL_32:
      v7 = v37;
LABEL_5:
      sub_147D0(v7[35] - v36, a2, v3);
      v32 = 0;
      result = (long long (*)(int *))v38[1];
      if ( !result )
        return (long long (*)(int *))(&dword_0 + 1);
      return (long long (*)(int *))((unsigned char)(v32 & result(&v34)) ^ 1u);
    }
  }
  v10 = 0;
  while ( 2 )
  {
    v11 = sub_25FE0(fda, buf, 0x200u);
    v14 = v11;
    if ( v11 && v11 != -1 )
    {
      v15 = &buf[v11];
      v16 = buf;
      do
      {
        if ( *v16++ )
        {
          if ( !v33.m128i_i64[1] )
            v33.m128i_i64[0] = v10;
          v33.m128i_i64[1] += v14;
          v23 = v38;
          v7[35] += v14;
          v24 = (unsigned char (*)(int *, long long, char *))v23[6];
          if ( !v24 )
            goto LABEL_19;
          v25 = buf;
          goto LABEL_31;
        }
      }
      while ( v16 != v15 );
      if ( !v33.m128i_i64[1] )
        goto LABEL_19;
      sub_29560(v7, &v33, (unsigned long long)v15, v13);
      v25 = 0;
      v33.m128i_i64[1] = 0;
      v24 = (unsigned char (*)(int *, long long, char *))v38[6];
      if ( !v24 )
        goto LABEL_19;
LABEL_31:
      a2 = (__m128i *)(&dword_0 + 1);
      if ( v24(&v34, 1, v25) )
      {
LABEL_19:
        v10 += v14;
        continue;
      }
      goto LABEL_32;
    }
    break;
  }
  if ( !v33.m128i_i64[1] )
    v33.m128i_i64[0] = v10;
  a2 = &v33;
  sub_29560(v7, &v33, v12, v13);
  v26 = v38;
  v7[35] += v14;
  v27 = (unsigned char (*)(int *, long long, long long))v26[6];
  if ( v27 )
  {
    a2 = (__m128i *)(&dword_0 + 2);
    if ( v27(&v34, 2, 0) )
    {
      v26 = v38;
      goto LABEL_40;
    }
    goto LABEL_32;
  }
LABEL_40:
  if ( v26[7] )
  {
    v28 = (void (*)(int *))v26[3];
    if ( v28 )
      v28(&v34);
    v7 = v37;
    v3 = v37[35];
    if ( fd >= 0 )
    {
      a2 = (__m128i *)v37[17];
      v29 = 0;
      sub_C640(v37[1], a2, v3 - v36);
      while ( 1 )
      {
        v7 = v37;
        if ( v37[38] <= (unsigned long long)v29 )
          break;
        v30 = (unsigned char (*)(int *, __m128i *))v38[7];
        if ( !v30 )
          goto LABEL_5;
        a2 = v29;
        v29 = (__m128i *)((char *)v29 + 1);
        if ( !v30(&v34, a2) )
          goto LABEL_32;
      }
    }
  }
  else
  {
    v7 = v37;
  }
  sub_147D0(v7[35] - v36, a2, v3);
  result = (long long (*)(int *))v38[1];
  if ( result )
    return (long long (*)(int *))((unsigned char)(v32 & result(&v34)) ^ 1u);
  return result;
}



// Function: sparse_member_p @ 0x2b3a0
long long sparse_member_p()
{
  long long v0; // rdx
  long long (*v1)(void); // rax
  long long v3[9]; // [rsp+0h] [rbp-48h] BYREF

  v3[7] = __readfsqword(0x28u);
  if ( (unsigned char)sub_29D10(v3) && (v3[3] = v0, (v1 = *(long long (**)(void))(v3[4] + 16LL)) != 0) )
    return v1();
  else
    return 0;
}



// Function: sparse_fixup_header @ 0x2b400
long long sparse_fixup_header()
{
  long long result; // rax
  long long v1; // rdx
  long long (*v2)(void); // rdx
  long long v3[9]; // [rsp+0h] [rbp-48h] BYREF

  v3[7] = __readfsqword(0x28u);
  result = sub_29D10(v3);
  if ( (char)result )
  {
    v3[3] = v1;
    v2 = *(long long (**)(void))(v3[4] + 32LL);
    if ( v2 )
      return v2();
  }
  return result;
}



// Function: sparse_extract_file @ 0x2b460
long long (*sparse_extract_file(long long a1, long long a2))(int *)
{
  long long *v2; // rdx
  int v3; // r8d
  char v4; // r12
  long long (*result)(int *); // rax
  long long *v6; // r13
  __off_t v7; // rax
  bool v8; // zf
  long long v9; // rax
  unsigned char (*v10)(int *); // rdx
  long long v11; // rcx
  unsigned long long v12; // rbx
  unsigned char (*v13)(int *, long long); // rdx
  long long v14; // rsi
  int vars0; // [rsp+0h] [rbp+0h] BYREF
  bool vars4; // [rsp+4h] [rbp+4h]
  long long vars8; // [rsp+8h] [rbp+8h]
  long long vars10; // [rsp+10h] [rbp+10h]
  long long vars18; // [rsp+18h] [rbp+18h]
  long long vars20; // [rsp+20h] [rbp+20h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  v4 = sub_29D10(&vars0);
  result = (long long (*)(int *))(&dword_0 + 3);
  if ( v4 )
  {
    vars18 = a2;
    v6 = v2;
    vars0 = v3;
    v7 = lseek(v3, 0, 0);
    vars8 = 0;
    v8 = v7 == 0;
    v9 = vars20;
    vars4 = v8;
    v10 = *(unsigned char (**)(int *))(vars20 + 40);
    if ( v10 )
    {
      if ( !v10(&vars0) )
      {
LABEL_12:
        v11 = vars18;
        v9 = vars20;
        goto LABEL_13;
      }
      v9 = vars20;
    }
    v11 = vars18;
    v12 = 0;
    if ( !*(long long *)(vars18 + 304) )
    {
LABEL_9:
      result = *(long long (**)(int *))(v9 + 8);
      *v6 = *(long long *)(v11 + 280) - vars10;
      if ( !result )
        return result;
      return (long long (*)(int *))((unsigned char)(v4 & result(&vars0)) ^ 1u);
    }
    while ( 1 )
    {
      v13 = *(unsigned char (**)(int *, long long))(v9 + 64);
      if ( !v13 )
        break;
      v14 = v12++;
      if ( !v13(&vars0, v14) )
        goto LABEL_12;
      v11 = vars18;
      v9 = vars20;
      if ( *(long long *)(vars18 + 304) <= v12 )
        goto LABEL_9;
    }
LABEL_13:
    result = *(long long (**)(int *))(v9 + 8);
    v4 = 0;
    *v6 = *(long long *)(v11 + 280) - vars10;
    if ( !result )
      return (long long (*)(int *))(&dword_0 + 1);
    return (long long (*)(int *))((unsigned char)(v4 & result(&vars0)) ^ 1u);
  }
  return result;
}



// Function: sparse_skip_file @ 0x2b5b0
long long (*sparse_skip_file(long long a1, long long a2))(int *)
{
  long long v2; // rdx
  unsigned char v3; // bl
  long long (*result)(int *); // rax
  long long (*v5)(int *); // rax
  char v6; // r8
  int vars0[4]; // [rsp+0h] [rbp+0h] BYREF
  long long vars10; // [rsp+10h] [rbp+10h]
  long long vars18; // [rsp+18h] [rbp+18h]
  long long vars20; // [rsp+20h] [rbp+20h]
  unsigned long long vars38; // [rsp+38h] [rbp+38h]

  vars38 = __readfsqword(0x28u);
  v3 = sub_29D10(vars0);
  result = (long long (*)(int *))(&dword_0 + 3);
  if ( v3 )
  {
    vars18 = v2;
    vars0[0] = -1;
    v5 = *(long long (**)(int *))(vars20 + 40);
    if ( v5 )
    {
      v3 = v5(vars0);
      sub_23D40(*(long long *)(vars18 + 280) - vars10, a2);
      result = *(long long (**)(int *))(vars20 + 8);
      if ( !result )
        return (long long (*)(int *))(v3 ^ 1u);
    }
    else
    {
      sub_23D40(*(long long *)(v2 + 280) - vars10, a2);
      result = *(long long (**)(int *))(vars20 + 8);
      if ( !result )
        return result;
    }
    v6 = result(vars0);
    result = (long long (*)(int *))(&dword_0 + 1);
    if ( !v6 )
      return result;
    return (long long (*)(int *))(v3 ^ 1u);
  }
  return result;
}



// Function: sparse_diff_file @ 0x2b690
long long sparse_diff_file(long long a1, void **a2)
{
  int v2; // edx
  unsigned int v3; // r13d
  long long (*v4)(int *); // rax
  char v5; // bl
  void **v6; // rax
  unsigned long long v7; // rbp
  long long v8; // rbx
  void **v9; // rdx
  void (*v10)(int *, void **); // rax
  long long v12; // rsi
  char v13; // r8
  int v14; // [rsp+0h] [rbp-68h] BYREF
  char v15; // [rsp+4h] [rbp-64h]
  long long v16; // [rsp+10h] [rbp-58h]
  void **v17; // [rsp+18h] [rbp-50h]
  long long v18; // [rsp+20h] [rbp-48h]
  unsigned long long v19; // [rsp+38h] [rbp-30h]

  v19 = __readfsqword(0x28u);
  v3 = sub_29D10(&v14);
  if ( !(char)v3 )
    return v3;
  v17 = a2;
  v14 = v2;
  v4 = *(long long (**)(int *))(v18 + 40);
  v15 = 1;
  if ( v4 )
  {
    v5 = v4(&v14);
    sub_C670(a2);
    if ( !v5 )
    {
LABEL_7:
      v9 = v17;
LABEL_8:
      v3 = 0;
      sub_23D40((signed long long)v9[35] - v16, (long long)a2);
      goto LABEL_9;
    }
  }
  else
  {
    sub_C670(a2);
  }
  v6 = v17;
  a2 = 0;
  v7 = 0;
  while ( (unsigned long long)v6[38] > v7 )
  {
    v8 = 16 * v7;
    if ( !(unsigned char)sub_2A9D0((long long)&v14, (long long)a2, *((long long *)v6[40] + 2 * v7)) )
      goto LABEL_7;
    v12 = v7++;
    v13 = sub_2AB20((long long)&v14, v12);
    v6 = v17;
    v9 = v17;
    a2 = (void **)(*(long long *)((char *)v17[40] + v8) + *(long long *)((char *)v17[40] + v8 + 8));
    if ( !v13 )
      goto LABEL_8;
  }
LABEL_9:
  sub_C6A0();
  v10 = *(void (**)(int *, void **))(v18 + 8);
  if ( v10 )
    v10(&v14, a2);
  return v3;
}



// Function: find_compression_suffix @ 0x2b7d0
const void *find_compression_suffix(char *s, long long *a2)
{
  char *v2; // rax
  const void *v3; // rbp
  const void *v4; // r15
  size_t v5; // rax
  size_t v6; // r12
  void **i; // rbx

  v2 = strrchr(s, 46);
  v3 = v2;
  if ( v2 )
  {
    v4 = v2 + 1;
    v5 = strlen(v2 + 1);
    v3 = off_7F920;
    v6 = v5;
    if ( off_7F920 )
    {
      for ( i = &off_7F920; i[1] != (void *)v6 || memcmp(v3, v4, v6); i += 3 )
      {
        v3 = i[3];
        if ( !v3 )
          return v3;
      }
      v3 = i;
      if ( a2 )
        *a2 = strlen(s) - v6 - 1;
    }
  }
  return v3;
}



// Function: set_compression_program_by_suffix @ 0x2b880
const void *set_compression_program_by_suffix(char *a1, char *a2)
{
  const void *result; // rax

  result = sub_2B7D0(a1, 0);
  if ( result )
    a2 = (char *)*((long long *)result + 2);
  if ( a2 )
    qword_82B70 = a2;
  return result;
}



// Function: strip_compression_suffix @ 0x2b8b0
char **strip_compression_suffix(char *src)
{
  char **v1; // r12
  size_t v2; // rbx
  size_t n[7]; // [rsp+0h] [rbp-38h] BYREF

  n[1] = __readfsqword(0x28u);
  v1 = (char **)sub_2B7D0(src, n);
  if ( v1 )
  {
    v2 = n[0];
    if ( n[0] <= 4 )
    {
      v1 = 0;
      if ( !n[0] )
        return v1;
    }
    else if ( !strncmp(&src[n[0] - 4], ".tar", 4u) && **v1 != 116 )
    {
      v2 -= 4LL;
    }
    v1 = (char **)sub_4ECC0(v2 + 1);
    memcpy(v1, src, v2);
    *((char *)v1 + v2) = 0;
  }
  return v1;
}



// Function: xexec @ 0x2b970
void xexec(char *a1)
{
  char *v1[7]; // [rsp+0h] [rbp-38h] BYREF

  v1[5] = (char *)__readfsqword(0x28u);
  v1[0] = "/bin/sh";
  v1[1] = "-c";
  v1[2] = a1;
  v1[3] = 0;
  execv("/bin/sh", v1);
  sub_32DA0(a1);
}



// Function: wait_for_grandchild @ 0x2b9c0
void wait_for_grandchild(__pid_t pid)
{
  int v1; // edi
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v3; // [rsp+8h] [rbp-20h]

  v3 = __readfsqword(0x28u);
  do
  {
    if ( waitpid(pid, &stat_loc, 0) != -1 )
      goto LABEL_4;
  }
  while ( *__errno_location() == 4 );
  sub_334D0(qword_82B70);
LABEL_4:
  v1 = BYTE1(stat_loc);
  if ( !__OFSUB__((stat_loc & 0x7F) + 1, 1) && (stat_loc & 0x7F) != 0 )
  {
    raise(stat_loc & 0x7F);
    v1 = 0;
  }
  exit(v1);
}



// Function: run_decompress_program @ 0x2ba30
void run_decompress_program()
{
  char *v0; // rax
  long long v1; // r12
  long long i; // rdx
  const char **v3; // rax
  long long v4; // r14
  int *v5; // rax
  char *v6; // rax
  long long v7; // rbx
  char *v8; // rax
  char *v9; // rax
  const char *format; // [rsp+8h] [rbp-140h]
  int v11; // [rsp+1Ch] [rbp-12Ch] BYREF
  long long v12; // [rsp+20h] [rbp-128h] BYREF
  void *dest; // [rsp+28h] [rbp-120h]
  long long v14; // [rsp+30h] [rbp-118h]
  long long v15; // [rsp+90h] [rbp-B8h]
  unsigned long long v16; // [rsp+108h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  v14 = 1;
  v15 = _environ;
  v0 = sub_C810(&v11);
  if ( v0 )
  {
    v1 = (long long)v0;
    for ( i = 34082310; !(unsigned int)sub_375A0(v1, &v12, i); i = 34082318 )
    {
      memmove(dest, (char *)dest + 8 * v14, 8 * v12);
      v3 = (const char **)dest;
      *((long long *)dest + v12) = "-d";
      execvp(*v3, (char *const *)v3);
      *((long long *)dest + v12) = 0;
      v4 = sub_C890(&v11);
      if ( !v4 )
        sub_32DA0(v1);
      if ( (dword_81B80 & 0x80000) != 0 )
      {
        if ( qword_82D50 )
          qword_82D50(&v11);
        format = dcgettext(0, "cannot run %s", 5);
        v5 = __errno_location();
        error(0, *v5, format, v1);
        if ( (dword_81B80 & 0x80000) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(0);
          v6 = dcgettext(0, "trying %s", 5);
          error(0, 0, v6, v4);
        }
      }
      v1 = v4;
    }
    if ( qword_82D50 )
      qword_82D50(v1);
    v7 = sub_358F0(&v12);
    v8 = dcgettext(0, "cannot split string '%s': %s", 5);
    error(0, 0, v8, v1, v7);
    sub_16FD0();
  }
  if ( qword_82D50 )
    qword_82D50(&v11);
  v9 = dcgettext(0, "unable to run decompression program", 5);
  error(0, 0, v9);
  sub_16FD0();
}



// Function: sys_get_archive_stat @ 0x2bd50
bool sys_get_archive_stat()
{
  return fstat(fd, &buf) == 0;
}



// Function: sys_file_is_archive @ 0x2bd80
long long sys_file_is_archive(long long a1)
{
  unsigned int v1; // r8d

  v1 = 0;
  if ( !qword_82818 || qword_82818 != *(long long *)(a1 + 88) )
    return 0;
  LOBYTE(v1) = *(long long *)(a1 + 96) == qword_82810;
  return v1;
}



// Function: sys_save_archive_dev_ino @ 0x2bdc0
__ino_t sys_save_archive_dev_ino()
{
  __ino_t result; // rax

  if ( fd <= 0x3FFFFFFF && (result = buf.st_mode & 0xF000, (int)result == 0x8000) )
  {
    qword_82818 = buf.st_dev;
    qword_82810 = buf.st_ino;
    return buf.st_ino;
  }
  else
  {
    qword_82818 = 0;
  }
  return result;
}



// Function: sys_detect_dev_null_output @ 0x2be10
unsigned long long sys_detect_dev_null_output()
{
  int v0; // r8d
  bool v1; // al
  struct stat v3; // [rsp+0h] [rbp-A8h] BYREF
  unsigned long long v4; // [rsp+98h] [rbp-10h]

  v4 = __readfsqword(0x28u);
  v0 = strcmp(*(const char **)ptr, "/dev/null");
  v1 = 1;
  if ( v0 )
    v1 = fd <= 0x3FFFFFFF
      && (buf.st_mode & 0xF000) == 0x2000
      && !stat("/dev/null", &v3)
      && buf.st_dev == v3.st_dev
      && buf.st_ino == v3.st_ino;
  byte_82A30 = v1;
  return v4 - __readfsqword(0x28u);
}



// Function: sys_wait_for_child @ 0x2bee0
unsigned long long sys_wait_for_child(unsigned int pid, char a2)
{
  char *v3; // rdi
  short v4; // cx
  int v5; // ecx
  unsigned int v7; // ebx
  char *v8; // rax
  char *v9; // rax
  int v10; // [rsp+Ch] [rbp-2Ch]
  unsigned int v11; // [rsp+Ch] [rbp-2Ch]
  int stat_loc; // [rsp+14h] [rbp-24h] BYREF
  unsigned long long v13; // [rsp+18h] [rbp-20h]

  v13 = __readfsqword(0x28u);
  if ( !pid )
    return v13 - __readfsqword(0x28u);
  do
  {
    v3 = (char *)pid;
    if ( waitpid(pid, &stat_loc, 0) != -1 )
    {
      v4 = stat_loc;
      if ( __OFSUB__((stat_loc & 0x7F) + 1, 1) || (stat_loc & 0x7F) == 0 )
        goto LABEL_9;
LABEL_6:
      v5 = v4 & 0x7F;
      if ( v5 != 13 || a2 )
      {
        if ( qword_82D50 )
        {
          v10 = v5;
          qword_82D50(v3);
          v5 = v10;
        }
        v11 = v5;
        v9 = dcgettext(0, "Child died with signal %d", 5);
        error(0, 0, v9, v11);
        goto LABEL_18;
      }
      return v13 - __readfsqword(0x28u);
    }
  }
  while ( *__errno_location() == 4 );
  v3 = qword_82B70;
  sub_334D0(qword_82B70);
  v4 = stat_loc;
  if ( !__OFSUB__((stat_loc & 0x7F) + 1, 1) && (stat_loc & 0x7F) != 0 )
    goto LABEL_6;
LABEL_9:
  if ( HIBYTE(v4) )
  {
    if ( qword_82D50 )
    {
      qword_82D50(v3);
      HIBYTE(v4) = BYTE1(stat_loc);
    }
    v7 = HIBYTE(v4);
    v8 = dcgettext(0, "Child returned status %d", 5);
    error(0, 0, v8, v7);
LABEL_18:
    sub_16FD0();
  }
  return v13 - __readfsqword(0x28u);
}



// Function: sys_spawn_shell @ 0x2c010
unsigned long long sys_spawn_shell()
{
  char *v0; // r12
  __pid_t v1; // eax
  __pid_t v2; // ebx
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v5; // [rsp+8h] [rbp-20h]

  v5 = __readfsqword(0x28u);
  v0 = getenv("SHELL");
  if ( !v0 )
    v0 = "/bin/sh";
  v1 = sub_26520();
  if ( !v1 )
  {
    execlp(v0, "-sh", "-i", 0);
    sub_32DA0(v0);
  }
  v2 = v1;
  while ( waitpid(v2, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_334D0(v0);
      return v5 - __readfsqword(0x28u);
    }
  }
  return v5 - __readfsqword(0x28u);
}



// Function: sys_compare_uid @ 0x2c0d0
bool sys_compare_uid(long long a1, long long a2)
{
  return *(int *)(a1 + 28) == *(int *)(a2 + 28);
}



// Function: sys_compare_gid @ 0x2c0e0
bool sys_compare_gid(long long a1, long long a2)
{
  return *(int *)(a1 + 32) == *(int *)(a2 + 32);
}



// Function: sys_compare_links @ 0x2c0f0
bool sys_compare_links(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a2 == *a1 )
    return a2[1] == a1[1];
  return result;
}



// Function: sys_truncate @ 0x2c110
int sys_truncate(int fd)
{
  long long v1; // rax

  v1 = lseek(fd, 0, 1);
  if ( v1 < 0 )
    return -1;
  else
    return ftruncate(fd, v1);
}



// Function: sys_write_archive_buffer @ 0x2c140
long long sys_write_archive_buffer()
{
  if ( fd <= 0x3FFFFFFF )
    return sub_431A0(fd, s1, nbytes);
  else
    return sub_345C0((unsigned int)(fd - 0x40000000), s1, nbytes);
}



// Function: sys_child_open_for_compress @ 0x2c180
long long sys_child_open_for_compress(double a1)
{
  __pid_t v1; // eax
  char *v2; // rax
  const void **v3; // rbx
  char *v4; // rax
  char *v5; // rdi
  const char *v6; // rbp
  int v7; // eax
  unsigned int v8; // r12d
  void *v10; // rax
  __pid_t v11; // ebp
  const void **v12; // rbx
  const char *v13; // r12
  size_t v14; // rdx
  char *v15; // r12
  unsigned long long v16; // rbx
  long long v17; // rax
  long long v18; // rdi
  char *v19; // rax
  int v20; // eax
  int *v21; // rax
  int v22; // ebp
  int *v23; // rbx
  long long *v24; // rax
  long long v25; // rax
  char *v26; // rax
  char *v27; // [rsp+8h] [rbp-40h]
  int fd; // [rsp+18h] [rbp-30h] BYREF
  int v29; // [rsp+1Ch] [rbp-2Ch]
  int v30; // [rsp+20h] [rbp-28h] BYREF
  int v31; // [rsp+24h] [rbp-24h]
  unsigned long long v32; // [rsp+28h] [rbp-20h]

  v32 = __readfsqword(0x28u);
  signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  sub_26560(&fd);
  v1 = sub_26520();
  if ( v1 <= 0 )
  {
    v2 = dcgettext(0, "tar (child)", 5);
    sub_4AF40(v2);
    signal(13, 0);
    if ( fd )
      sub_2BC90(fd, 0);
    sub_CD20(v29);
    if ( byte_82D70 )
    {
      v5 = *(char **)ptr;
    }
    else
    {
      v3 = (const void **)ptr;
      v4 = strchr(*(const char **)ptr, 58);
      qword_82D78 = (long long)v4;
      v5 = (char *)*v3;
      if ( v4 )
      {
        if ( v4 > v5 )
        {
          v27 = (char *)*v3;
          v10 = memchr(v5, 47, v4 - v5);
          v5 = v27;
          if ( !v10 )
            goto LABEL_17;
        }
      }
    }
    if ( (unsigned int)sub_2BC20(v5) )
    {
      if ( byte_82B84 )
        sub_25C10(*(char **)ptr, 1);
      v6 = *(const char **)ptr;
      if ( strcmp(*(const char **)ptr, "-") )
      {
        v7 = creat(v6, 0x1B6u);
        ::fd = v7;
        if ( v7 < 0 )
        {
          v21 = __errno_location();
          v22 = *v21;
          v23 = v21;
          if ( byte_82B84 )
            sub_25EA0();
          v24 = ptr;
          *v23 = v22;
          sub_32EC0(*v24, a1);
        }
        if ( v7 != 1 )
          sub_2BC90(v7, 1);
      }
      goto LABEL_14;
    }
LABEL_17:
    sub_26560(&v30);
    v11 = sub_26520();
    if ( v11 )
    {
      if ( v30 )
        sub_2BC90(v30, 0);
      sub_CD20(v31);
      v12 = (const void **)ptr;
      v13 = *(const char **)ptr;
      if ( !strcmp(*(const char **)ptr, "-") )
      {
        ::fd = 1;
      }
      else
      {
        if ( byte_82D70
          || (v19 = strchr(v13, 58), qword_82D78 = (long long)v19, v13 = (const char *)*v12, !v19)
          || v19 <= v13
          || memchr(*v12, 47, v19 - v13) )
        {
          v20 = creat(v13, 0x1B6u);
        }
        else
        {
          v20 = sub_33A80(v13, 65, 0x40000000, qword_82AB0);
        }
        ::fd = v20;
        if ( v20 < 0 )
          sub_32EC0(*(long long *)ptr, a1);
      }
      v14 = nbytes;
      while ( 1 )
      {
        v15 = (char *)s1;
        if ( !v14 )
          goto LABEL_45;
        v16 = 0;
        do
        {
          v17 = sub_4D570(0, v15, v14 - v16);
          if ( v17 == -1 )
            sub_33090(qword_82B70);
          if ( !v17 )
          {
            if ( v16 )
            {
              memset((char *)s1 + v16, 0, nbytes - v16);
              v18 = sub_2C140();
              if ( nbytes != v18 )
LABEL_30:
                sub_DA70(v18);
            }
LABEL_45:
            sub_2B9C0(v11);
          }
          v14 = nbytes;
          v16 += v17;
          v15 += v17;
        }
        while ( nbytes > v16 );
        v25 = sub_2C140();
        v14 = v25;
        if ( nbytes != v25 )
        {
          v18 = v25;
          goto LABEL_30;
        }
      }
    }
    v26 = dcgettext(0, "tar (grandchild)", 5);
    sub_4AF40(v26);
    if ( v31 != 1 )
      sub_2BC90(v31, 1);
    sub_CD20(v30);
LABEL_14:
    sub_2B970(qword_82B70);
  }
  v8 = v1;
  ::fd = v29;
  sub_CD20(fd);
  return v8;
}



// Function: sys_child_open_for_uncompress @ 0x2c570
long long sys_child_open_for_uncompress(double a1)
{
  __pid_t v1; // eax
  char *v2; // rax
  const void **v3; // rbx
  const char *v4; // rbp
  char *v5; // rax
  __pid_t v6; // r12d
  const void **v7; // rbx
  const char *v8; // rbp
  unsigned long long v9; // rbx
  char *v10; // r13
  size_t v11; // rbp
  unsigned int v12; // r12d
  char *v14; // rax
  int v15; // eax
  int v16; // eax
  char *v17; // rax
  int v18; // [rsp+8h] [rbp-40h] BYREF
  int v19; // [rsp+Ch] [rbp-3Ch]
  int v20; // [rsp+10h] [rbp-38h] BYREF
  int fd; // [rsp+14h] [rbp-34h]
  unsigned long long v22; // [rsp+18h] [rbp-30h]

  v22 = __readfsqword(0x28u);
  sub_26560(&v18);
  v1 = sub_26520();
  if ( v1 <= 0 )
  {
    v2 = dcgettext(0, "tar (child)", 5);
    sub_4AF40(v2);
    signal(13, 0);
    if ( v19 != 1 )
      sub_2BC90(v19, 1);
    sub_CD20(v18);
    v3 = (const void **)ptr;
    v4 = *(const char **)ptr;
    if ( strcmp(*(const char **)ptr, "-")
      && (byte_82D70
       || (v5 = strchr(v4, 58), qword_82D78 = (long long)v5, v4 = (const char *)*v3, !v5)
       || v5 <= v4
       || memchr(*v3, 47, v5 - v4))
      && (unsigned int)sub_2BC20(v4) )
    {
      v16 = open(*(const char **)ptr, 0, 438);
      ::fd = v16;
      if ( v16 < 0 )
        goto LABEL_36;
      if ( v16 )
        sub_2BC90(v16, 0);
    }
    else
    {
      sub_26560(&v20);
      v6 = sub_26520();
      if ( v6 )
      {
        if ( fd != 1 )
          sub_2BC90(fd, 1);
        sub_CD20(v20);
        v7 = (const void **)ptr;
        v8 = *(const char **)ptr;
        if ( !strcmp(*(const char **)ptr, "-") )
        {
          ::fd = 0;
          goto LABEL_15;
        }
        if ( byte_82D70
          || (v14 = strchr(v8, 58), qword_82D78 = (long long)v14, v8 = (const char *)*v7, !v14)
          || v14 <= v8
          || memchr(*v7, 47, v14 - v8) )
        {
          v15 = open(v8, 0, 438);
        }
        else
        {
          v15 = sub_33A80(v8, 0, 0x40000000, qword_82AB0);
        }
        ::fd = v15;
        if ( v15 >= 0 )
        {
LABEL_15:
          sub_C730();
          while ( 1 )
          {
            if ( ::fd > 0x3FFFFFFF )
            {
              v9 = sub_34460((unsigned int)(::fd - 0x40000000), s1, nbytes);
              if ( v9 != -1 )
                goto LABEL_20;
            }
            else
            {
              v9 = sub_4D570(::fd, s1, nbytes);
              if ( v9 != -1 )
              {
LABEL_20:
                if ( !v9 )
                {
                  sub_CD20(1);
                  sub_2B9C0(v6);
                }
                v10 = (char *)s1;
                do
                {
                  v11 = 512;
                  if ( v9 <= 0x200 )
                    v11 = v9;
                  if ( sub_431A0(1, v10, v11) != v11 )
                    sub_334F0(qword_82B70);
                  v10 += v11;
                  v9 -= v11;
                }
                while ( v9 );
                goto LABEL_15;
              }
            }
            sub_CD60();
          }
        }
LABEL_36:
        sub_32EC0(*(long long *)ptr, a1);
      }
      v17 = dcgettext(0, "tar (grandchild)", 5);
      sub_4AF40(v17);
      if ( v20 )
        sub_2BC90(v20, 0);
      sub_CD20(fd);
    }
    sub_2BA30();
  }
  v12 = v1;
  ::fd = v18;
  sub_CD20(v19);
  return v12;
}



// Function: sys_exec_command @ 0x2c8b0
long long sys_exec_command(char *value, char a2, long long a3)
{
  const char *v6; // rax
  const char *v7; // rsi
  const char *v8; // rsi
  const char *v9; // rsi
  const char *v10; // rsi
  const char *v11; // rsi
  const char *v12; // rax
  const char *v13; // rax
  const char *v14; // rax
  const char *v15; // rax
  const char *v16; // rax
  const char *v17; // rax
  unsigned long long v18; // rax
  unsigned int v19; // edi
  const char *v20; // rax
  const char *v21; // rax
  const char *v22; // rsi
  int fd; // [rsp+4h] [rbp-1A4h] BYREF
  unsigned int v24; // [rsp+8h] [rbp-1A0h]
  char valuea[2]; // [rsp+Eh] [rbp-19Ah] BYREF
  char v26[32]; // [rsp+10h] [rbp-198h] BYREF
  char v27[32]; // [rsp+30h] [rbp-178h] BYREF
  char v28[32]; // [rsp+50h] [rbp-158h] BYREF
  char v29[32]; // [rsp+70h] [rbp-138h] BYREF
  char v30[32]; // [rsp+90h] [rbp-118h] BYREF
  char v31[32]; // [rsp+B0h] [rbp-F8h] BYREF
  char v32[32]; // [rsp+D0h] [rbp-D8h] BYREF
  char v33[32]; // [rsp+F0h] [rbp-B8h] BYREF
  char v34[32]; // [rsp+110h] [rbp-98h] BYREF
  char v35[32]; // [rsp+130h] [rbp-78h] BYREF
  char v36[40]; // [rsp+150h] [rbp-58h] BYREF
  unsigned long long v37; // [rsp+178h] [rbp-30h]

  v37 = __readfsqword(0x28u);
  sub_26560(&fd);
  handler = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  pid = sub_26520();
  if ( !pid )
  {
    if ( fd )
      sub_2BC90(fd, 0);
    sub_CD20(v24);
    if ( setenv("TAR_VERSION", "1.34", 1) )
      goto LABEL_40;
    if ( *(long long *)qword_82828 )
    {
      if ( setenv("TAR_ARCHIVE", *(const char **)qword_82828, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_ARCHIVE");
    }
    v6 = (const char *)sub_45370(((qword_82828 - (long long)ptr) >> 3) + 1, v27);
    if ( setenv("TAR_VOLUME", v6, 1) )
      goto LABEL_40;
    v7 = (const char *)sub_45370(dword_82BA0, v26);
    if ( setenv("TAR_BLOCKING_FACTOR", v7, 1) )
      goto LABEL_40;
    v8 = (const char *)sub_2DCE0();
    if ( v8 )
    {
      if ( setenv("TAR_FORMAT", v8, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_FORMAT");
    }
    valuea[0] = a2;
    valuea[1] = 0;
    if ( setenv("TAR_FILETYPE", valuea, 1) )
      goto LABEL_40;
    __snprintf_chk(v33, 24, 1, 24, "0%lo", *(unsigned int *)(a3 + 112));
    if ( setenv("TAR_MODE", v33, 1) )
      goto LABEL_40;
    if ( value )
    {
      if ( setenv("TAR_FILENAME", value, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_FILENAME");
    }
    v9 = *(const char **)(a3 + 8);
    if ( v9 )
    {
      if ( setenv("TAR_REALNAME", v9, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_REALNAME");
    }
    v10 = *(const char **)(a3 + 32);
    if ( v10 )
    {
      if ( setenv("TAR_UNAME", v10, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_UNAME");
    }
    v11 = *(const char **)(a3 + 40);
    if ( v11 )
    {
      if ( setenv("TAR_GNAME", v11, 1) )
        goto LABEL_40;
    }
    else
    {
      unsetenv("TAR_GNAME");
    }
    v12 = (const char *)sub_257A0(*(long long *)(a3 + 232), *(long long *)(a3 + 240), v36);
    if ( setenv("TAR_ATIME", v12, 1)
      || (v13 = (const char *)sub_257A0(*(long long *)(a3 + 248), *(long long *)(a3 + 256), v35), setenv("TAR_MTIME", v13, 1))
      || (v14 = (const char *)sub_257A0(*(long long *)(a3 + 264), *(long long *)(a3 + 272), v34), setenv("TAR_CTIME", v14, 1))
      || (v15 = (const char *)sub_45370(*(long long *)(a3 + 136), v30), setenv("TAR_SIZE", v15, 1))
      || (v16 = (const char *)sub_45370(*(unsigned int *)(a3 + 116), v29), setenv("TAR_UID", v16, 1))
      || (v17 = (const char *)sub_45370(*(unsigned int *)(a3 + 120), v28), setenv("TAR_GID", v17, 1)) )
    {
LABEL_40:
      sub_17020();
    }
    if ( a2 != 104 )
    {
      if ( a2 <= 104 )
      {
        if ( (unsigned char)(a2 - 98) <= 1u )
        {
          v18 = *(long long *)(a3 + 128);
          v19 = v18 >> 12;
          LOBYTE(v19) = 0;
          v20 = (const char *)sub_45370((unsigned char)v18 | v19, v32);
          if ( !setenv("TAR_MINOR", v20, 1) )
          {
            v21 = (const char *)sub_45370(
                                  (*(long long *)(a3 + 128) >> 8) & 0xFFF
                                | (unsigned int)HIDWORD(*(long long *)(a3 + 128)) & 0xFFFFF000,
                                  v31);
            if ( !setenv("TAR_MAJOR", v21, 1) )
              goto LABEL_33;
          }
          goto LABEL_40;
        }
        goto LABEL_36;
      }
      if ( a2 != 108 )
      {
LABEL_36:
        unsetenv("TAR_MINOR");
        unsetenv("TAR_MAJOR");
        goto LABEL_33;
      }
    }
    unsetenv("TAR_MINOR");
    unsetenv("TAR_MAJOR");
    v22 = *(const char **)(a3 + 24);
    if ( v22 )
    {
      if ( !setenv("TAR_LINKNAME", v22, 1) )
LABEL_34:
        sub_2B970((char *)qword_82A60);
      goto LABEL_40;
    }
LABEL_33:
    unsetenv("TAR_LINKNAME");
    goto LABEL_34;
  }
  sub_CD20(fd);
  return v24;
}



// Function: sys_wait_command @ 0x2ce00
unsigned long long sys_wait_command()
{
  unsigned int v0; // ebp
  unsigned int v1; // ebx
  long long v2; // r12
  char *v3; // rax
  unsigned int v5; // ebx
  long long v6; // r12
  char *v7; // rax
  long long v8; // r12
  char *v9; // rax
  int stat_loc; // [rsp+4h] [rbp-24h] BYREF
  unsigned long long v11; // [rsp+8h] [rbp-20h]

  v0 = pid;
  v11 = __readfsqword(0x28u);
  if ( pid >= 0 )
  {
    signal(13, handler);
    while ( waitpid(v0, &stat_loc, 0) == -1 )
    {
      if ( *__errno_location() != 4 )
      {
        pid = -1;
        sub_334D0(qword_82A60);
        return v11 - __readfsqword(0x28u);
      }
      v0 = pid;
    }
    v1 = stat_loc & 0x7F;
    if ( (stat_loc & 0x7F) != 0 )
    {
      if ( (char)(v1 + 1) <= 1 )
      {
        if ( qword_82D50 )
          qword_82D50(v0);
        v8 = pid;
        v9 = dcgettext(0, "%lu: Child terminated on unknown reason", 5);
        error(0, 0, v9, v8);
        dword_82D58 = 2;
      }
      else
      {
        if ( qword_82D50 )
        {
          qword_82D50(v0);
          v1 = stat_loc & 0x7F;
        }
        v2 = pid;
        v3 = dcgettext(0, "%lu: Child terminated on signal %d", 5);
        error(0, 0, v3, v2, v1);
      }
    }
    else if ( !byte_82A59 )
    {
      v5 = BYTE1(stat_loc);
      if ( BYTE1(stat_loc) )
      {
        if ( qword_82D50 )
        {
          qword_82D50(v0);
          v5 = BYTE1(stat_loc);
        }
        v6 = pid;
        v7 = dcgettext(0, "%lu: Child returned status %d", 5);
        error(0, 0, v7, v6, v5);
        dword_82D58 = 2;
      }
    }
    pid = -1;
  }
  return v11 - __readfsqword(0x28u);
}



// Function: sys_exec_info_script @ 0x2cfc0
long long sys_exec_info_script(const char **a1, int a2)
{
  __pid_t v2; // eax
  __pid_t v3; // ebx
  FILE *v4; // r12
  int v5; // ebp
  int v6; // r14d
  char *v7; // rax
  unsigned int v8; // r12d
  const char *v10; // rax
  const char *v11; // rax
  const char *v12; // rsi
  const char *v13; // rax
  const char *v14; // rax
  int stat_loc; // [rsp+4h] [rbp-64h] BYREF
  char *lineptr; // [rsp+8h] [rbp-60h] BYREF
  size_t n; // [rsp+10h] [rbp-58h] BYREF
  int fd; // [rsp+18h] [rbp-50h] BYREF
  int v19; // [rsp+1Ch] [rbp-4Ch]
  char v20[24]; // [rsp+20h] [rbp-48h] BYREF
  unsigned long long v21; // [rsp+38h] [rbp-30h]

  v21 = __readfsqword(0x28u);
  sub_26560(&fd);
  qword_82720 = signal(13, (__sighandler_t)((char *)&dword_0 + 1));
  v2 = sub_26520();
  if ( !v2 )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", *a1, 1);
    v10 = (const char *)sub_45370(a2, v20);
    setenv("TAR_VOLUME", v10, 1);
    v11 = (const char *)sub_45370(dword_82BA0, v20);
    setenv("TAR_BLOCKING_FACTOR", v11, 1);
    v12 = (const char *)sub_2DD30((unsigned int)dword_82BA8);
    setenv("TAR_SUBCOMMAND", v12, 1);
    v13 = (const char *)sub_2DCE0();
    setenv("TAR_FORMAT", v13, 1);
    v14 = (const char *)sub_45370(v19, v20);
    setenv("TAR_FD", v14, 1);
    sub_CD20(fd);
    sub_2B970((char *)qword_82B48);
  }
  lineptr = 0;
  v3 = v2;
  n = 0;
  sub_CD20(v19);
  v4 = fdopen(fd, "r");
  v5 = __getdelim(&lineptr, &n, 10, v4);
  v6 = v5;
  fclose(v4);
  if ( v5 > 0 )
  {
    v7 = &lineptr[v5 - 1];
    if ( *v7 == 10 )
    {
      *v7 = 0;
      v6 = v5 - 1;
    }
  }
  while ( 1 )
  {
    v8 = waitpid(v3, &stat_loc, 0);
    if ( v8 != -1 )
      break;
    if ( *__errno_location() != 4 )
    {
      signal(13, qword_82720);
      sub_334D0(qword_82B48);
      return v8;
    }
  }
  signal(13, qword_82720);
  v8 = stat_loc & 0x7F;
  if ( (stat_loc & 0x7F) != 0 )
  {
    free(lineptr);
    return (unsigned int)-1;
  }
  else if ( BYTE1(stat_loc) || v6 <= 0 )
  {
    free(lineptr);
    return BYTE1(stat_loc);
  }
  else
  {
    *a1 = lineptr;
  }
  return v8;
}



// Function: sys_exec_checkpoint_script @ 0x2d260
unsigned long long sys_exec_checkpoint_script(char *a1, const char *a2, int a3)
{
  __pid_t v4; // eax
  __pid_t v5; // ebx
  const char *v7; // rax
  const char *v8; // rax
  const char *v9; // rsi
  const char *v10; // rax
  int stat_loc; // [rsp+Ch] [rbp-4Ch] BYREF
  char v12[24]; // [rsp+10h] [rbp-48h] BYREF
  unsigned long long v13; // [rsp+28h] [rbp-30h]

  v13 = __readfsqword(0x28u);
  v4 = sub_26520();
  if ( !v4 )
  {
    setenv("TAR_VERSION", "1.34", 1);
    setenv("TAR_ARCHIVE", a2, 1);
    v7 = (const char *)sub_45370(a3, v12);
    setenv("TAR_CHECKPOINT", v7, 1);
    v8 = (const char *)sub_45370(dword_82BA0, v12);
    setenv("TAR_BLOCKING_FACTOR", v8, 1);
    v9 = (const char *)sub_2DD30((unsigned int)dword_82BA8);
    setenv("TAR_SUBCOMMAND", v9, 1);
    v10 = (const char *)sub_2DCE0();
    setenv("TAR_FORMAT", v10, 1);
    sub_2B970(a1);
  }
  v5 = v4;
  while ( waitpid(v5, &stat_loc, 0) == -1 )
  {
    if ( *__errno_location() != 4 )
    {
      sub_334D0(a1);
      return v13 - __readfsqword(0x28u);
    }
  }
  return v13 - __readfsqword(0x28u);
}



// Function: find_argp_option_key @ 0x2d3c0
long long find_argp_option_key(long long a1, int a2)
{
  long long result; // rax
  int v3; // edx

  for ( result = a1; ; result += 48 )
  {
    v3 = *(int *)(result + 8);
    if ( !*(long long *)result
      && !v3
      && !*(long long *)(result + 16)
      && !*(int *)(result + 24)
      && !*(long long *)(result + 32) )
    {
      break;
    }
    if ( a2 == v3 )
      return result;
  }
  return 0;
}



// Function: format_default_settings @ 0x2d400
long long format_default_settings()
{
  char *v0; // rsi
  _UNKNOWN **i; // rax

  v0 = "oldgnu";
  for ( i = &off_7DA00; ; v0 = (char *)i[2] )
  {
    i += 2;
    if ( !v0 )
      break;
    if ( *((int *)i + 2) == 6 )
      return sub_4F3C0(
               (unsigned int)"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",
               (int)v0,
               (unsigned int)"-",
               20,
               (unsigned int)"escape",
               (unsigned int)"/usr/local/libexec/rmt",
               (char)"/usr/bin/rsh");
  }
  v0 = "unknown?";
  return sub_4F3C0(
           (unsigned int)"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s --rsh-command=%s",
           (int)v0,
           (unsigned int)"-",
           20,
           (unsigned int)"escape",
           (unsigned int)"/usr/local/libexec/rmt",
           (char)"/usr/bin/rsh");
}



// Function: optloc_save @ 0x2d480
long long optloc_save(int a1, const char **a2)
{
  size_t v3; // rdi
  long long v5; // rax
  char *v6; // rsi
  long long v7; // rbx
  int v8; // eax
  long long v9; // r8

  v3 = 32;
  if ( a2[1] )
    v3 = strlen(a2[1]) + 33;
  v5 = sub_4ECC0(v3);
  v6 = (char *)a2[1];
  v7 = v5;
  if ( v6 )
    v6 = strcpy((char *)(v5 + 32), v6);
  v8 = *(int *)a2;
  *(long long *)(v7 + 8) = v6;
  *(int *)v7 = v8;
  *(long long *)(v7 + 16) = a2[2];
  v9 = qword_82BC0[a1];
  qword_82BC0[a1] = v7;
  *(long long *)(v7 + 24) = v9;
  return v9;
}



// Function: sigstat @ 0x2d500
void sigstat(int a1)
{
  sub_C770();
  sub_CC30();
  signal(a1, sigstat);
}



// Function: tar_help_filter @ 0x2d670
long long tar_help_filter(int a1, long long a2, long long a3, long long a4, long long a5, long long a6, char a7)
{
  char *v7; // rbp
  size_t v8; // rax
  char *next_free; // rdi
  size_t v10; // rbx
  char *v11; // rbx
  char *v12; // r15
  const char *v13; // r14
  size_t v14; // rbp
  _UNKNOWN **v15; // r12
  char *v16; // r15
  size_t v17; // rbx
  char *v18; // rax
  int v19; // edx
  int v20; // ecx
  unsigned int v21; // edi
  int v22; // r8d
  int v23; // r9d
  char *v24; // rsi
  char *v27; // rbp
  size_t v28; // rax
  char *v29; // rdi
  size_t v30; // rbx
  const char *v31; // rbp
  size_t v32; // rax
  char *v33; // rdi
  size_t v34; // rbx
  char *v35; // rax
  char *v36; // rax
  char *object_base; // rdi
  char *chunk_limit; // rdx
  struct obstack v39; // [rsp+0h] [rbp-98h] BYREF
  unsigned long long v40; // [rsp+58h] [rbp-40h]

  v40 = __readfsqword(0x28u);
  if ( a1 == 148 )
  {
    v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
    v24 = "lzip";
    return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
  }
  if ( a1 <= 148 )
  {
    if ( a1 == 106 )
    {
      v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v24 = "bzip2";
      return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
    }
    if ( a1 <= 106 )
    {
      if ( a1 == 74 )
      {
        v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
        v24 = "xz";
        return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
      }
      if ( a1 == 90 )
      {
        v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
        v24 = "compress";
        return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
      }
    }
    else if ( a1 == 122 )
    {
      v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v24 = "gzip";
      return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
    }
  }
  else
  {
    if ( a1 == 207 )
    {
      v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v24 = "zstd";
      return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
    }
    if ( a1 > 207 )
    {
      if ( a1 == 33554436 )
      {
        obstack_begin(&v39, 0, 0, sub_4ECC0, &free);
        v7 = dcgettext(0, "Valid arguments for the --quoting-style option are:", 5);
        v8 = strlen(v7);
        next_free = v39.next_free;
        v10 = v8;
        if ( v8 > v39.chunk_limit - v39.next_free )
        {
          obstack_newchunk(&v39, v8);
          next_free = v39.next_free;
        }
        memcpy(next_free, v7, v10);
        v11 = &v39.next_free[v10];
        v39.next_free = v11;
        if ( (char *)(v39.chunk_limit - v11) <= (char *)&dword_0 + 1 )
        {
          obstack_newchunk(&v39, 2);
          v11 = v39.next_free;
        }
        *(short *)v11 = 2570;
        v12 = v39.next_free + 2;
        v39.next_free += 2;
        v13 = "literal";
        v14 = strlen("  ");
        if ( "literal" )
        {
          v15 = &off_7E6E8;
          while ( 1 )
          {
            if ( v14 > v39.chunk_limit - v12 )
            {
              obstack_newchunk(&v39, v14);
              v12 = v39.next_free;
            }
            memcpy(v12, "  ", v14);
            v39.next_free += v14;
            v16 = v39.next_free;
            v17 = strlen(v13);
            if ( v17 > v39.chunk_limit - v16 )
            {
              obstack_newchunk(&v39, v17);
              v16 = v39.next_free;
            }
            memcpy(v16, v13, v17);
            v18 = &v39.next_free[v17];
            v39.next_free = v18;
            if ( v18 == v39.chunk_limit )
            {
              obstack_newchunk(&v39, 1);
              v18 = v39.next_free;
            }
            ++v15;
            v39.next_free = v18 + 1;
            *v18 = 10;
            v13 = (const char *)*(v15 - 1);
            if ( !v13 )
              break;
            v12 = v39.next_free;
          }
        }
        v27 = dcgettext(0, "\n*This* tar defaults to:\n", 5);
        v28 = strlen(v27);
        v29 = v39.next_free;
        v30 = v28;
        if ( v28 > v39.chunk_limit - v39.next_free )
        {
          obstack_newchunk(&v39, v28);
          v29 = v39.next_free;
        }
        memcpy(v29, v27, v30);
        v39.next_free += v30;
        v31 = (const char *)sub_2D400();
        v32 = strlen(v31);
        v33 = v39.next_free;
        v34 = v32;
        if ( v32 > v39.chunk_limit - v39.next_free )
        {
          obstack_newchunk(&v39, v32);
          v33 = v39.next_free;
        }
        memcpy(v33, v31, v34);
        v35 = &v39.next_free[v34];
        v39.next_free = v35;
        if ( v35 == v39.chunk_limit )
        {
          obstack_newchunk(&v39, 1);
          v35 = v39.next_free;
        }
        v39.next_free = v35 + 1;
        *v35 = 10;
        v36 = v39.next_free;
        if ( v39.chunk_limit == v39.next_free )
        {
          obstack_newchunk(&v39, 1);
          v36 = v39.next_free;
        }
        v39.next_free = v36 + 1;
        *v36 = 0;
        object_base = v39.object_base;
        if ( v39.next_free == v39.object_base )
          *((char *)&v39 + 80) |= 2u;
        chunk_limit = v39.chunk_limit;
        if ( (~*(long long *)&v39.alignment_mask & (long long)&v39.next_free[*(long long *)&v39.alignment_mask])
           - (unsigned long long)v39.chunk <= v39.chunk_limit - (char *)v39.chunk )
          chunk_limit = (char *)(~*(long long *)&v39.alignment_mask
                               & (long long)&v39.next_free[*(long long *)&v39.alignment_mask]);
        v39.next_free = chunk_limit;
        v39.object_base = chunk_limit;
        a2 = sub_4EE30(object_base);
        obstack_free(&v39, 0);
      }
      return a2;
    }
    if ( a1 == 149 )
    {
      v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v24 = "lzma";
      return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
    }
    if ( a1 == 150 )
    {
      v21 = (unsigned int)dcgettext(0, "filter the archive through %s", 5);
      v24 = "lzop";
      return sub_4F3C0(v21, (int)v24, v19, v20, v22, v23, a7);
    }
  }
  return a2;
}



// Function: subcommand_string @ 0x2dd30
const char *subcommand_string(int a1)
{
  const char *result; // rax

  switch ( a1 )
  {
    case 0:
      result = "unknown?";
      break;
    case 1:
      result = "-r";
      break;
    case 2:
      result = "-A";
      break;
    case 3:
      result = "-c";
      break;
    case 4:
      result = "-D";
      break;
    case 5:
      result = "-d";
      break;
    case 6:
      result = "-x";
      break;
    case 7:
      result = "-t";
      break;
    case 8:
      result = "-u";
      break;
    case 9:
      result = "--test-label";
      break;
    default:
      abort();
  }
  return result;
}



// Function: decode_signal @ 0x2ddf0
long long decode_signal(char *s2)
{
  const char *v2; // rbp
  char **v3; // rbx
  int v4; // eax
  const char *v5; // rdi
  char *v7; // rax

  v2 = s2 + 3;
  v3 = &off_7D940;
  v4 = strncmp(s2, "SIG", 3u);
  v5 = "USR1";
  if ( v4 )
    v2 = s2;
  if ( strcmp("USR1", v2) )
  {
    do
    {
      v3 += 2;
      if ( v3 == &off_7D990 )
      {
        if ( qword_82D50 )
          qword_82D50(v5);
        v7 = dcgettext(0, "Unknown signal name: %s", 5);
        error(0, 0, v7, s2);
        sub_16FD0();
      }
      v5 = *v3;
    }
    while ( strcmp(*v3, v2) );
  }
  return *((unsigned int *)v3 + 2);
}



// Function: usage @ 0x2dea0
void usage(int status)
{
  argp_help(&off_7FB20, stderr, 4, qword_82DD0);
  sub_40180();
  exit(status);
}



// Function: confirm @ 0x2df50
long long confirm(const char *a1, long long a2, double a3)
{
  const char *v3; // rax
  unsigned int v4; // r12d
  char *IO_write_ptr; // rax
  int v7; // eax
  int v8; // ebp
  char *lineptr; // [rsp+8h] [rbp-30h] BYREF
  size_t n[5]; // [rsp+10h] [rbp-28h] BYREF

  n[1] = __readfsqword(0x28u);
  if ( !qword_82BB0 )
  {
    if ( fd && !qword_82C10 )
    {
      qword_82C10 = (long long)"-w";
      qword_82BB0 = (FILE *)stdin;
    }
    else
    {
      qword_82BB0 = fopen("/dev/tty", "r");
      if ( !qword_82BB0 )
        sub_32EC0("/dev/tty", a3);
    }
  }
  v3 = (const char *)sub_4D2D0(a2);
  __fprintf_chk(qword_81F28, 1, "%s %s?", a1, v3);
  fflush_unlocked(qword_81F28);
  v4 = dword_82BAC;
  if ( dword_82BAC )
  {
    v4 = 0;
  }
  else
  {
    lineptr = 0;
    n[0] = 0;
    if ( __getdelim(&lineptr, n, 10, qword_82BB0) >= 0 )
    {
      v7 = rpmatch(lineptr);
      v8 = dword_82BAC;
      v4 = v7 > 0;
      free(lineptr);
      if ( !v8 )
        return v4;
    }
    else
    {
      dword_82BAC = 1;
      free(lineptr);
    }
  }
  IO_write_ptr = qword_81F28->_IO_write_ptr;
  if ( IO_write_ptr >= qword_81F28->_IO_write_end )
  {
    __overflow(qword_81F28, 10);
  }
  else
  {
    qword_81F28->_IO_write_ptr = IO_write_ptr + 1;
    *IO_write_ptr = 10;
  }
  fflush_unlocked(qword_81F28);
  return v4;
}



// Function: get_date_or_file @ 0x2e120
int get_date_or_file(long long a1, long long a2, char *a3, __m128i *a4)
{
  int result; // eax
  long long v9; // rbp
  char *v10; // r12
  char *v11; // rax
  long long v12; // rax
  __m128i v13; // xmm0
  long long *v14; // r14
  long long v15; // rax
  char *v16; // rax
  struct stat v17; // [rsp+0h] [rbp-C8h] BYREF
  unsigned long long v18; // [rsp+98h] [rbp-30h]

  v18 = __readfsqword(0x28u);
  if ( (unsigned char)(*a3 - 46) > 1u )
  {
    if ( (unsigned char)sub_4AED0(a4, a3, 0) )
    {
      v12 = sub_4ECC0(40);
      v13 = _mm_loadu_si128(a4);
      *(long long *)(v12 + 24) = a2;
      v14 = (long long *)v12;
      *(__m128i *)(v12 + 8) = v13;
      *(long long *)(v12 + 32) = sub_4EE30(a3);
      v15 = *(long long *)(a1 + 8);
      *(long long *)(a1 + 8) = v14;
      *v14 = v15;
      return 0;
    }
    else
    {
      if ( qword_82D50 )
        qword_82D50(a4);
      v9 = sub_4D2D0(a3);
      v10 = sub_23240(a4->m128i_i64[0], a4->m128i_i64[1], 0);
      v11 = dcgettext(0, "Substituting %s for unknown date format %s", 5);
      error(0, 0, v11, v10, v9);
      a4->m128i_i64[1] = 0;
      return 1;
    }
  }
  else
  {
    result = stat(a3, &v17);
    if ( result )
    {
      sub_33410(a3);
      if ( qword_82D50 )
        qword_82D50(a3);
      v16 = dcgettext(0, "Date sample file not found", 5);
      error(0, 0, v16);
      sub_2DEA0(2);
    }
    *a4 = (__m128i)v17.st_mtim;
  }
  return result;
}



// Function: option_conflict_error @ 0x2e2f0
void option_conflict_error(long long a1, long long a2)
{
  char *v2; // rax

  if ( qword_82D50 )
    qword_82D50(a1);
  v2 = dcgettext(0, "'%s' cannot be used with '%s'", 5);
  error(0, 0, v2, a1, a2);
  sub_2DEA0(2);
}



// Function: set_old_files_option @ 0x2e340
int set_old_files_option(int a1, const char **a2)
{
  long long v2; // rax

  v2 = sub_2D480(9, a2);
  if ( v2 )
  {
    if ( *(int *)a2 == *(int *)v2 )
    {
      if ( *(int *)a2 == 1 || (LODWORD(v2) = strcmp(a2[1], *(const char **)(v2 + 8)), !(int)v2) )
      {
        LODWORD(v2) = dword_82B34;
        if ( dword_82B34 != a1 )
          sub_2E2F0((long long)off_7D900[a1], (long long)off_7D900[dword_82B34]);
      }
    }
  }
  dword_82B34 = a1;
  return v2;
}



// Function: parse_opt @ 0x2e4c0
long long parse_opt(char *a1, char *a2, unsigned long long a3)
{
  long long *v3; // rbp
  int **v4; // r14
  char *v5; // r15
  long long *v6; // rax
  long long v7; // rdx
  long long v8; // rax
  long long v9; // rcx
  long long result; // rax
  long long v11; // rsi
  long long v12; // rsi
  unsigned int v13; // eax
  int v14; // edx
  int v15; // ecx
  int v16; // r8d
  int v17; // r9d
  long long v18; // rax
  int v19; // eax
  long long v20; // rdi
  long long v21; // rax
  const char *v22; // rsi
  long long v23; // rbx
  const char *v24; // rsi
  char *next_free; // r13
  char *chunk_limit; // rbp
  unsigned long long v27; // rbp
  size_t v28; // rbx
  char *v29; // rax
  char *v30; // rcx
  char *v31; // r15
  char v32; // al
  char *object_base; // rdi
  char *v34; // rdx
  char *v35; // rbp
  double v36; // xmm0_8
  long long v37; // rax
  unsigned long long v38; // rcx
  long long *v39; // rdi
  long long v40; // rax
  int v41; // eax
  char *v42; // rbp
  size_t v43; // rdx
  char *v44; // rbp
  size_t v45; // r13
  char *v46; // r11
  const unsigned short **v47; // rax
  unsigned char v48; // dl
  char *v49; // rbp
  int v50; // eax
  const void *v51; // r11
  char *v52; // rdi
  size_t v53; // rax
  const char *v54; // r8
  size_t v55; // r13
  double v56; // xmm0_8
  const char **v57; // rax
  int v58; // edx
  const char *v59; // r12
  char *v60; // rax
  const char *v61; // rbx
  const char *v62; // r12
  char *v63; // rax
  long long v64; // rbx
  char *v65; // rax
  char *v66; // rbx
  long long v67; // rax
  const char *v68; // rsi
  char *v69; // rax
  const char *v70; // rsi
  char *v71; // rbx
  const char *v72; // rax
  const char *v73; // rsi
  char *v74; // rax
  char *v75; // rax
  size_t srcd; // [rsp+8h] [rbp-100h]
  unsigned char src; // [rsp+8h] [rbp-100h]
  void *srcc; // [rsp+8h] [rbp-100h]
  char *srce; // [rsp+8h] [rbp-100h]
  char *srca; // [rsp+8h] [rbp-100h]
  const char *srcb; // [rsp+8h] [rbp-100h]
  char *v82; // [rsp+10h] [rbp-F8h]
  char *v83; // [rsp+10h] [rbp-F8h]
  __m128i v84; // [rsp+30h] [rbp-D8h] BYREF
  struct obstack endptr; // [rsp+40h] [rbp-C8h] BYREF
  char v86[40]; // [rsp+A0h] [rbp-68h] BYREF
  unsigned long long v87; // [rsp+C8h] [rbp-40h]

  v3 = (long long *)a3;
  v4 = *(int ***)(a3 + 40);
  v87 = __readfsqword(0x28u);
  if ( (int)a1 <= 207 )
  {
    if ( (int)a1 < 0 )
      return 7;
    v5 = a2;
    switch ( (int)a1 )
    {
      case 0:
        sub_28130(a2);
        return (unsigned int)a1;
      case 48:
      case 49:
      case 50:
      case 51:
      case 52:
      case 53:
      case 54:
      case 55:
        v13 = (unsigned int)dcgettext(0, "Options '-[0-7][lmh]' not supported by *this* tar", 5);
        argp_error((int)v3, v13, v14, v15, v16, v17);
        exit(64);
      case 65:
        sub_2E2A0(2);
        return 0;
      case 66:
        byte_82AB9 = 1;
        return 0;
      case 70:
        qword_82B48 = (long long)a2;
        result = 0;
        byte_82B10 = 1;
        return result;
      case 71:
        goto LABEL_24;
      case 72:
        sub_2E430(a2);
        return 0;
      case 73:
        sub_2E3B0(a2);
        return 0;
      case 74:
        sub_2E3B0("xz");
        return 0;
      case 75:
        sub_2D480(5, (const char **)*v4);
        sub_286F0(a2);
        return 0;
      case 76:
        if ( !(unsigned int)sub_4EEC0(a2) )
        {
          if ( endptr.chunk_size <= (unsigned long long)a2 || strchr("bBcGgkKMmPTtw", *(char *)(endptr.chunk_size - 1)) )
          {
            if ( v84.m128i_i64[0] < 0 )
              v36 = (double)(int)(v84.m128i_i8[0] & 1 | ((unsigned long long)v84.m128i_i64[0] >> 1))
                  + (double)(int)(v84.m128i_i8[0] & 1 | ((unsigned long long)v84.m128i_i64[0] >> 1));
            else
              v36 = (double)v84.m128i_i32[0];
          }
          else
          {
            if ( v84.m128i_i64[0] < 0 )
              v56 = (double)(int)(v84.m128i_i8[0] & 1 | ((unsigned long long)v84.m128i_i64[0] >> 1))
                  + (double)(int)(v84.m128i_i8[0] & 1 | ((unsigned long long)v84.m128i_i64[0] >> 1));
            else
              v56 = (double)v84.m128i_i32[0];
            v36 = v56 * 1024.0;
          }
          byte_82B10 = 1;
          qword_82A70 = *(long long *)&v36;
          return 0;
        }
        if ( qword_82D50 )
          qword_82D50(a2);
        v70 = "Invalid tape length";
        goto LABEL_262;
      case 77:
        byte_82B10 = 1;
        return 0;
      case 78:
        dword_82B8C = 1;
        if ( xmmword_82B00 >= 0 )
          goto LABEL_252;
        v24 = "--after-date";
        goto LABEL_74;
      case 79:
        byte_82A6A = 1;
        return 0;
      case 80:
        sub_2D480(8, (const char **)*v4);
        byte_82B92 = 1;
        return 0;
      case 82:
        byte_82B7C = 1;
        return 0;
      case 83:
        byte_82A88 = 1;
        return 0;
      case 85:
        sub_2E340(3, (const char **)*v4);
        return 0;
      case 86:
        pattern = a2;
        return 0;
      case 87:
        sub_2D480(4, (const char **)*v4);
        byte_82A50 = 1;
        return 0;
      case 90:
        sub_2E3B0("compress");
        return 0;
      case 97:
        *((char *)v4 + 18) = 1;
        return 0;
      case 98:
        if ( !(unsigned int)sub_4EEC0(a2) )
        {
          dword_82BA0 = endptr.chunk_size;
          if ( SLODWORD(endptr.chunk_size) > 0 && SLODWORD(endptr.chunk_size) == endptr.chunk_size )
          {
            nbytes = endptr.chunk_size << 9;
            if ( endptr.chunk_size == (endptr.chunk_size & 0x7FFFFFFFFFFFFFLL) )
              return 0;
          }
        }
        if ( qword_82D50 )
          qword_82D50(a2);
        v70 = "Invalid blocking factor";
        goto LABEL_262;
      case 99:
        sub_2E2A0(3);
        return 0;
      case 100:
        sub_2E2A0(5);
        return 0;
      case 102:
        v37 = qword_82830;
        v38 = qword_82C18;
        v39 = ptr;
        if ( qword_82830 != qword_82C18 )
          goto LABEL_141;
        if ( ptr )
        {
          if ( (unsigned long long)qword_82C18 <= 0xAAAAAAAAAAAAAA9LL )
          {
            v38 = qword_82C18 + ((unsigned long long)qword_82C18 >> 1) + 1;
            goto LABEL_227;
          }
LABEL_241:
          sub_17020();
        }
        if ( qword_82C18 )
        {
          a3 = ((unsigned long long)qword_82C18 * (unsigned __int128)8uLL) >> 64 != 0;
          if ( (qword_82C18 & 0x1000000000000000LL) != 0 || !is_mul_ok(qword_82C18, 8u) )
            goto LABEL_241;
        }
        else
        {
          v38 = 16;
        }
LABEL_227:
        qword_82C18 = v38;
        ptr = (void *)sub_4ECE0(ptr, 8 * v38, a3, v38);
        v39 = ptr;
        v37 = qword_82830;
LABEL_141:
        qword_82830 = v37 + 1;
        v39[v37] = a2;
        return 0;
      case 103:
        sub_2D480(2, (const char **)*v4);
        qword_82B28 = a2;
        dword_82B8C = 1;
LABEL_24:
        byte_82B50 = 1;
        return 0;
      case 104:
        byte_82B69 = 1;
        return 0;
      case 105:
        byte_82B51 = 1;
        return 0;
      case 106:
        sub_2E3B0("bzip2");
        return 0;
      case 107:
        sub_2E340(4, (const char **)*v4);
        return 0;
      case 108:
        dword_82C20 = 1;
        return 0;
      case 109:
        byte_82A68 = 1;
        return 0;
      case 110:
        dword_827FC = 1;
        return 0;
      case 111:
        *((char *)v4 + 16) = 1;
        return 0;
      case 112:
        dword_82AA4 = 1;
        return 0;
      case 114:
        sub_2E2A0(1);
        return 0;
      case 115:
        sub_2D480(6, (const char **)*v4);
        byte_82AAC = 1;
        return 0;
      case 116:
        sub_2E2A0(7);
        ++dword_82A54;
        return 0;
      case 117:
        sub_2E2A0(8);
        return 0;
      case 118:
        ++dword_82A54;
        result = 0;
        dword_81B80 |= 0x583000u;
        return result;
      case 119:
        byte_82B40 = 1;
        return 0;
      case 120:
        sub_2E2A0(6);
        return 0;
      case 122:
        sub_2E3B0("gzip");
        return 0;
      case 128:
        sub_2E430("posix");
        result = 0;
        dword_82A9C = 1;
        return result;
      case 129:
        v41 = 1;
        if ( a2 )
          v41 = dword_69A90[sub_3A4E0("--atime-preserve", a2, &off_7D9E0, dword_69A90, 4, off_81DD8)];
        dword_82B88 = v41;
        return 0;
      case 130:
        byte_82B84 = 1;
        if ( !a2 )
          return 0;
        v4[4] = (int *)a2;
        return 0;
      case 131:
        byte_82B20 = 1;
        return 0;
      case 132:
        if ( a2 )
        {
          if ( *a2 == 46 )
          {
            v5 = a2 + 1;
            sub_10710(".");
          }
          dword_82B78 = strtoul(v5, (char **)&endptr, 0);
          if ( *(char *)endptr.chunk_size )
          {
            if ( qword_82D50 )
              qword_82D50(v5);
            v73 = "--checkpoint value is not an integer";
LABEL_266:
            v74 = dcgettext(0, v73, 5);
            error(0, 0, v74);
            sub_16FD0();
          }
          return 0;
        }
        else
        {
          dword_82B78 = 10;
          return 0;
        }
      case 133:
        sub_10710(a2);
        return 0;
      case 134:
        dword_82AF0 = 2;
        return 0;
      case 135:
        byte_827E0 = 1;
        return 0;
      case 136:
        byte_82B68 = 1;
        return 0;
      case 137:
        sub_2E2A0(4);
        return 0;
      case 138:
        byte_82D70 = 1;
        return 0;
      case 139:
        byte_82B90 = 1;
        return 0;
      case 140:
        v40 = sub_2D520(a2);
        if ( v40 != -1 )
        {
          dword_82B54 = v40;
          return 0;
        }
        dword_82B54 = -1;
        if ( !qword_82B58 )
          return 0;
        sub_28080(qword_82B58, &dword_82B54);
        return 0;
      case 141:
        sub_24EB0((long long)a2);
        return 0;
      case 142:
        byte_82A59 = 1;
        return 0;
      case 143:
        byte_82B52 = 1;
        return 0;
      case 144:
        filename = a2;
        return 0;
      case 145:
        byte_82B30 = 1;
        return 0;
      case 146:
        sub_2E340(6, (const char **)*v4);
        return 0;
      case 147:
        dword_82B24 = strtoul(a2, (char **)&endptr, 10);
        if ( !*(char *)endptr.chunk_size )
          return 0;
        if ( qword_82D50 )
          qword_82D50(a2);
        v68 = "Invalid incremental level value";
        goto LABEL_255;
      case 148:
        sub_2E3B0("lzip");
        return 0;
      case 149:
        sub_2E3B0("lzma");
        return 0;
      case 150:
        sub_2E3B0("lzop");
        return 0;
      case 151:
        qword_82B18 = sub_458B0(a2);
        if ( qword_82B18 )
        {
          dword_82B14 = umask(0);
          umask(dword_82B14);
          return 0;
        }
        if ( qword_82D50 )
          qword_82D50(a2);
        v73 = "Invalid mode given on option";
        goto LABEL_266;
      case 152:
        sub_2E120((long long)v4, (long long)"--mtime", a2, (__m128i *)&xmmword_82AE0);
        if ( dword_82AF0 )
          return 0;
        dword_82AF0 = 1;
        return 0;
      case 153:
        if ( xmmword_82B00 >= 0 )
        {
LABEL_252:
          if ( qword_82D50 )
            qword_82D50(a1);
          v68 = "More than one threshold date";
LABEL_255:
          v69 = dcgettext(0, v68, 5);
          error(0, 0, v69);
          sub_2DEA0(2);
        }
        v24 = "--newer-mtime";
LABEL_74:
        sub_2E120((long long)v4, (long long)v24, v5, (__m128i *)&xmmword_82B00);
        sub_2D480(3, (const char **)*v4);
        return 0;
      case 154:
        dword_82A9C = -1;
        return 0;
      case 155:
        *((char *)v4 + 18) = 0;
        return 0;
      case 156:
        byte_82B20 = 0;
        return 0;
      case 157:
        byte_827E0 = 0;
        return 0;
      case 158:
        byte_82A59 = 0;
        return 0;
      case 159:
        sub_2E340(1, (const char **)*v4);
        return 0;
      case 160:
        while ( 1 )
        {
          v12 = (unsigned int)*v5;
          if ( !*v5 )
            break;
          ++v5;
          sub_4C780(0, v12, 0);
        }
        return 0;
      case 161:
        dword_82AA8 = -1;
        return 0;
      case 162:
        dword_82AA4 = -1;
        return 0;
      case 163:
        dword_827FC = 0;
        return 0;
      case 164:
        dword_82AA0 = -1;
        return 0;
      case 165:
        dword_82A98 = -1;
        return 0;
      case 166:
        byte_82AD2 = 1;
        return 0;
      case 167:
        sub_2D480(1, (const char **)*v4);
        if ( a2 )
        {
          if ( (unsigned int)sub_4EEC0(a2) )
          {
            if ( qword_82D50 )
              qword_82D50(a2);
            v66 = dcgettext(0, "Invalid number", 5);
            v67 = sub_4CE40(a2);
            error(0, 0, "%s: %s", v67, v66);
            goto LABEL_248;
          }
          qword_82B38 = endptr.chunk_size;
          return 0;
        }
        else
        {
          qword_82B38 = 1;
          return 0;
        }
      case 168:
        sub_2E430("v7");
        return 0;
      case 169:
        byte_82AD1 = 1;
        return 0;
      case 170:
        sub_2D480(7, (const char **)*v4);
        byte_82AD0 = 1;
        result = 0;
        qword_82AC8 = a2;
        return result;
      case 171:
        sub_2E340(0, (const char **)*v4);
        return 0;
      case 172:
        sub_2E340(2, (const char **)*v4);
        return 0;
      case 173:
        v18 = sub_2D520(a2);
        if ( v18 != -1 )
        {
          dword_82ABC = v18;
          return 0;
        }
        dword_82ABC = -1;
        if ( !qword_82AC0 )
          return 0;
        sub_27FD0(qword_82AC0, &dword_82ABC);
        return 0;
      case 174:
        sub_24DE0((long long)a2);
        return 0;
      case 175:
        obstack_begin(&endptr, 0, 0, sub_4ECC0, &free);
        while ( 2 )
        {
          next_free = endptr.next_free;
          chunk_limit = endptr.chunk_limit;
          if ( !*v5 )
            goto LABEL_106;
          v27 = endptr.chunk_limit - endptr.next_free;
          v28 = strcspn(v5, ",");
          v29 = memchr(v5, 61, v28);
          v30 = v29;
          if ( !v29 )
          {
            if ( v28 > v27 )
            {
              obstack_newchunk(&endptr, v28);
              next_free = endptr.next_free;
            }
            memcpy(next_free, v5, v28);
            next_free = &endptr.next_free[v28];
            endptr.next_free += v28;
            goto LABEL_104;
          }
          v43 = v29 - v5 + 1;
          if ( v43 > v27 )
          {
            srcc = (void *)(v29 - v5 + 1);
            v83 = v29;
            obstack_newchunk(&endptr, srcc);
            next_free = endptr.next_free;
            v30 = v83;
            v43 = (size_t)srcc;
          }
          v82 = v30;
          srcd = v43;
          memcpy(next_free, v5, v43);
          v44 = &endptr.next_free[srcd];
          v45 = v28 - srcd;
          endptr.next_free += srcd;
          v46 = v82 + 1;
          src = v82[1];
          if ( !src )
            goto LABEL_204;
          v47 = __ctype_b_loc();
          v46 = v82 + 1;
          v48 = src;
          while ( ((*v47)[v48] & 0x2000) != 0 )
          {
            v48 = *++v46;
            --v45;
            if ( !v48 )
              goto LABEL_204;
          }
          if ( v48 != 123 || v46[v45 - 1] != 125 )
          {
LABEL_204:
            if ( endptr.chunk_limit - v44 < v45 )
            {
              srce = v46;
              obstack_newchunk(&endptr, v45);
              v44 = endptr.next_free;
              v46 = srce;
            }
            memcpy(v44, v46, v45);
            next_free = &endptr.next_free[v45];
            endptr.next_free = next_free;
            goto LABEL_104;
          }
          srca = v46;
          v49 = (char *)sub_4ECC0(v45);
          memcpy(v49, srca + 1, v45 - 2);
          v49[v45 - 2] = 0;
          v50 = sub_2E120((long long)v4, (long long)"--pax-option", v49, &v84);
          v51 = srca;
          if ( v50 )
          {
            v52 = endptr.next_free;
            if ( endptr.chunk_limit - endptr.next_free < v45 )
            {
              obstack_newchunk(&endptr, v45);
              v52 = endptr.next_free;
              v51 = srca;
            }
            memcpy(v52, v51, v45);
            endptr.next_free += v45;
          }
          else
          {
            srcb = (const char *)sub_257A0(v84.m128i_i64[0], v84.m128i_u32[2], v86);
            v53 = strlen(srcb);
            v54 = srcb;
            v55 = v53;
            if ( v53 > endptr.chunk_limit - endptr.next_free )
            {
              obstack_newchunk(&endptr, v53);
              v54 = srcb;
            }
            memcpy(endptr.next_free, v54, v55);
            endptr.next_free += v55;
          }
          free(v49);
          next_free = endptr.next_free;
LABEL_104:
          v31 = &v5[v28];
          v32 = *v31;
          if ( *v31 )
          {
            if ( endptr.chunk_limit == next_free )
            {
              obstack_newchunk(&endptr, 1);
              next_free = endptr.next_free;
              v32 = *v31;
            }
            v5 = v31 + 1;
            endptr.next_free = next_free + 1;
            *next_free = v32;
            continue;
          }
          break;
        }
        chunk_limit = endptr.chunk_limit;
LABEL_106:
        if ( next_free == chunk_limit )
        {
          obstack_newchunk(&endptr, 1);
          next_free = endptr.next_free;
        }
        endptr.next_free = next_free + 1;
        *next_free = 0;
        object_base = endptr.object_base;
        if ( endptr.object_base == endptr.next_free )
          *((char *)&endptr + 80) |= 2u;
        v34 = endptr.chunk_limit;
        if ( (~*(long long *)&endptr.alignment_mask & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask])
           - (unsigned long long)endptr.chunk <= endptr.chunk_limit - (char *)endptr.chunk )
          v34 = (char *)(~*(long long *)&endptr.alignment_mask
                       & (long long)&endptr.next_free[*(long long *)&endptr.alignment_mask]);
        endptr.next_free = v34;
        endptr.object_base = v34;
        v35 = (char *)sub_4EE30(object_base);
        obstack_free(&endptr, 0);
        *((char *)v4 + 17) = 1;
        sub_1CA20(v35);
        free(v35);
        return 0;
      case 176:
        sub_2E430("posix");
        return 0;
      case 177:
        while ( 1 )
        {
          v11 = (unsigned int)*v5;
          if ( !*v5 )
            break;
          ++v5;
          sub_4C780(0, v11, 1);
        }
        return 0;
      case 178:
        v22 = "literal";
        if ( !"literal" )
          goto LABEL_245;
        v23 = 0;
        while ( 1 )
        {
          a1 = v5;
          if ( !strcmp(v5, v22) )
            break;
          v22 = (&off_7E6E0)[++v23];
          if ( !v22 )
          {
LABEL_245:
            if ( qword_82D50 )
              qword_82D50(a1);
            v64 = qword_82DD0;
            v65 = dcgettext(0, "Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.", 5);
            error(0, 0, v65, v5, v64);
LABEL_248:
            sub_16FD0();
          }
        }
        sub_4C760(0, (unsigned int)v23);
        return 0;
      case 179:
        if ( (unsigned int)sub_4EEC0(a2) )
        {
          if ( qword_82D50 )
            qword_82D50(a2);
          v70 = "Invalid record size";
LABEL_262:
          v71 = dcgettext(0, v70, 5);
          v72 = (const char *)sub_4CE40(v5);
          error(0, 0, "%s: %s", v72, v71);
          sub_2DEA0(2);
        }
        nbytes = endptr.chunk_size;
        if ( (endptr.chunk_size & 0x1FF) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(a2);
          v75 = dcgettext(0, "Record size must be a multiple of %d.", 5);
          error(0, 0, v75, 512);
          sub_2DEA0(2);
        }
        dword_82BA0 = (unsigned long long)endptr.chunk_size >> 9;
        return 0;
      case 180:
        byte_82ABA = 1;
        return 0;
      case 181:
        byte_82AB8 = 1;
        return 0;
      case 182:
        byte_82A58 = 1;
        return 0;
      case 183:
        off_81C60 = a2;
        return 0;
      case 184:
        qword_82AB0 = (long long)a2;
        return 0;
      case 185:
        dword_82AA8 = 1;
        return 0;
      case 186:
        sub_2E430("posix");
        result = 0;
        dword_82AA0 = 1;
        return result;
      case 187:
        v42 = (char *)sub_2D400();
        __printf_chk(1, "%s\n", v42);
        sub_40180();
        free(v42);
        exit(0);
      case 188:
        byte_82A89 = 1;
        return 0;
      case 189:
        sub_202A0();
        sub_40180();
        exit(0);
      case 190:
        byte_827E1 = 1;
        return 0;
      case 191:
        sub_2E340(5, (const char **)*v4);
        return 0;
      case 192:
        dword_827E4 = dword_7FAE8[sub_3A4E0("--sort", a2, &off_7D9C0, dword_7FAE8, 4, off_81DD8)];
        return 0;
      case 193:
        v21 = sub_3A4E0("--hole-detection", a2, &off_7D990, dword_69A88, 4, off_81DD8);
        byte_82A88 = 1;
        dword_82A7C = dword_69A88[v21];
        return 0;
      case 194:
        v20 = (long long)a2;
        byte_82A88 = 1;
        dword_82A84 = strtoul(a2, (char **)&endptr, 10);
        if ( !*(char *)endptr.chunk_size )
          return 0;
        if ( *(char *)endptr.chunk_size == 46 )
        {
          v20 = endptr.chunk_size + 1;
          dword_82A80 = strtoul((const char *)(endptr.chunk_size + 1), (char **)&endptr, 10);
          if ( !*(char *)endptr.chunk_size )
            return 0;
        }
        if ( qword_82D50 )
          qword_82D50(v20);
        v68 = "Invalid sparse version value";
        goto LABEL_255;
      case 195:
        if ( !(unsigned int)sub_4EEC0(a2) )
        {
          qword_82A90 = endptr.chunk_size;
          return 0;
        }
        if ( qword_82D50 )
          qword_82D50(a2);
        v70 = "Invalid number of elements";
        goto LABEL_262;
      case 196:
        byte_82B84 = 1;
        result = 0;
        v4[3] = (int *)a2;
        return result;
      case 197:
        sub_2E2A0(9);
        return 0;
      case 198:
        if ( a2 )
        {
          v19 = sub_2DDF0(a2);
          signal(v19, sub_2D500);
        }
        else
        {
          byte_82A69 = 1;
        }
        return 0;
      case 199:
        if ( !qword_82A60 )
        {
          qword_82A60 = (long long)a2;
          return 0;
        }
        if ( qword_82D50 )
          qword_82D50(a1);
        v68 = "Only one --to-command option allowed";
        goto LABEL_255;
      case 200:
        sub_30870(a2);
        return 0;
      case 201:
        byte_82B91 = 1;
        return 0;
      case 202:
        qword_82A48 = a2;
        return 0;
      case 203:
        sub_31AB0(a2);
        return 0;
      case 204:
        sub_2E430("posix");
        result = 0;
        dword_82A98 = 1;
        return result;
      case 205:
      case 206:
        sub_2E430("posix");
        dword_82A98 = 1;
        sub_31FD0(a2, (int)a1 == 206);
        return 0;
      case 207:
        sub_2E3B0("zstd");
        return 0;
      default:
        return 7;
    }
  }
  if ( (int)a1 == 16777219 )
  {
    v6 = *(long long **)(*(long long *)a3 + 32LL);
    if ( v6 && *v6 )
    {
      v7 = 32;
      v8 = 0;
      while ( 1 )
      {
        *(long long *)(v3[6] + v8) = v4;
        v8 += 8;
        v9 = *(long long *)(*(long long *)(*v3 + 32LL) + v7);
        v7 += 32;
        if ( !v9 )
          break;
        v4 = (int **)v3[5];
      }
    }
    return 0;
  }
  else
  {
    if ( (int)a1 == 16777221 )
    {
      v57 = (const char **)*v4;
      v58 = **v4;
      if ( v58 == 2 )
      {
        v61 = v57[2];
        v62 = v57[1];
        v63 = dcgettext(0, "%s:%lu: location of the error", 5);
        error(0, 0, v63, v62, v61);
      }
      else if ( !v58 )
      {
        v59 = v57[1];
        v60 = dcgettext(0, "error parsing %s", 5);
        error(0, 0, v60, v59);
      }
      exit(64);
    }
    return 7;
  }
}



// Function: more_options @ 0x2fac0
unsigned long long more_options(unsigned int a1, long long a2, long long a3)
{
  long long v4[2]; // [rsp+0h] [rbp-38h] BYREF
  short v5; // [rsp+10h] [rbp-28h]
  char v6; // [rsp+12h] [rbp-26h]
  long long v7; // [rsp+18h] [rbp-20h]
  long long v8; // [rsp+20h] [rbp-18h]
  unsigned long long v9; // [rsp+28h] [rbp-10h]

  v9 = __readfsqword(0x28u);
  v4[0] = a3;
  v5 = 0;
  v4[1] = 0;
  v6 = 0;
  v7 = 0;
  v8 = 0;
  argp_parse(&off_7F240, a1, a2, 42, 0, v4);
  return v9 - __readfsqword(0x28u);
}



// Function: tar_stat_init @ 0x2fb40
long long tar_stat_init(long long *a1)
{
  int v1; // ecx
  unsigned long long v2; // rdi

  *a1 = 0;
  v1 = (int)a1;
  v2 = (unsigned long long)(a1 + 1);
  *(long long *)(v2 + 424) = 0;
  memset((void *)(v2 & 0xFFFFFFFFFFFFFFF8LL), 0, 8 * ((v1 - (v2 & 0xFFFFFFF8) + 440) >> 3));
  return 0;
}



// Function: tar_stat_close @ 0x2fb80
long long tar_stat_close(long long a1, long long a2)
{
  DIR *v3; // rdi
  int v4; // eax
  long long v5; // rdx
  long long v6; // rcx
  unsigned int v7; // r8d
  int v9; // edi

  v3 = *(DIR **)(a1 + 416);
  if ( v3 )
  {
    v4 = closedir(v3);
  }
  else
  {
    v9 = *(int *)(a1 + 424);
    if ( v9 <= 0 )
    {
      *(int *)(a1 + 424) = 0;
      return 1;
    }
    v4 = close(v9);
  }
  *(int *)(a1 + 424) = 0;
  v7 = 1;
  *(long long *)(a1 + 416) = 0;
  if ( v4 )
  {
    sub_26340(*(long long *)a1, a2, v5, v6, 1);
    return 0;
  }
  return v7;
}



// Function: tar_stat_destroy @ 0x2fc00
long long tar_stat_destroy(long long a1, long long a2, double a3)
{
  sub_2FB80(a1, a2);
  sub_1D580(*(void ***)(a1 + 352), *(long long *)(a1 + 344));
  free(*(void **)a1);
  free(*(void **)(a1 + 8));
  free(*(void **)(a1 + 24));
  free(*(void **)(a1 + 32));
  free(*(void **)(a1 + 40));
  free(*(void **)(a1 + 48));
  free(*(void **)(a1 + 56));
  free(*(void **)(a1 + 72));
  free(*(void **)(a1 + 320));
  free(*(void **)(a1 + 400));
  sub_1DE20(a1 + 360, a3);
  sub_17680(a1);
  *(long long *)a1 = 0;
  *(long long *)(a1 + 432) = 0;
  memset(
    (void *)((a1 + 8) & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8LL * (((unsigned int)a1 - (((int)a1 + 8) & 0xFFFFFFF8) + 440) >> 3));
  return 0;
}



// Function: tar_timespec_cmp @ 0x2fcc0
long long tar_timespec_cmp(long long a1, long long a2, long long a3, long long a4)
{
  int v4; // r8d

  v4 = 0;
  if ( dword_82400 == 4 )
    v4 = (a2 > a4) - (a2 < a4);
  return v4 + 2 * ((a1 > a3) - (unsigned int)(a1 < a3));
}



// Function: parse_xform_flags @ 0x2fd20
long long parse_xform_flags(int *a1, int a2)
{
  long long result; // rax

  switch ( a2 )
  {
    case 'H':
      *a1 &= ~2u;
      result = 0;
      break;
    case 'R':
      *a1 &= ~1u;
      result = 0;
      break;
    case 'S':
      *a1 &= ~4u;
      result = 0;
      break;
    case 'h':
      *a1 |= 2u;
      result = 0;
      break;
    case 'r':
      *a1 |= 1u;
      result = 0;
      break;
    case 's':
      *a1 |= 4u;
      result = 0;
      break;
    default:
      result = 1;
      break;
  }
  return result;
}



// Function: run_case_conv @ 0x2fdb0
unsigned char *run_case_conv(unsigned int a1, const void *a2, size_t a3, long long a4)
{
  unsigned char *v5; // r12
  unsigned char *v7; // rbx
  const int_t **v8; // rax
  unsigned char *v9; // rdx
  long long v10; // rsi
  unsigned char *v11; // rbx
  const int_t **v12; // rax
  unsigned char *v13; // rdx
  long long v14; // rsi

  v5 = (unsigned char *)dest;
  if ( qword_82C40 < a3 )
  {
    qword_82C40 = a3;
    dest = (void *)sub_4ECE0(dest, a3, a3, a4);
    v5 = (unsigned char *)dest;
  }
  memcpy(v5, a2, a3);
  if ( a1 == 3 )
  {
    v11 = &v5[a3];
    if ( v11 <= v5 )
      return v5;
    v12 = __ctype_toupper_loc();
    v13 = v5;
    do
    {
      v14 = *v13++;
      *(v13 - 1) = (*v12)[v14];
    }
    while ( v13 != v11 );
    return v5;
  }
  else if ( a1 > 3 )
  {
    if ( a1 != 4 )
      return v5;
    v7 = &v5[a3];
    if ( v7 <= v5 )
      return v5;
    v8 = __ctype_tolower_loc();
    v9 = v5;
    do
    {
      v10 = *v9++;
      *(v9 - 1) = (*v8)[v10];
    }
    while ( v9 != v7 );
    return v5;
  }
  else
  {
    if ( a1 != 1 )
    {
      if ( a1 == 2 )
        *v5 = (*__ctype_tolower_loc())[*v5];
      return v5;
    }
    *v5 = (*__ctype_toupper_loc())[*v5];
    return v5;
  }
}



// Function: add_char_segment @ 0x2fef0
char *add_char_segment(long long *a1, char a2)
{
  long long *v2; // rax
  long long *v3; // rbx
  long long *v4; // rax
  char *result; // rax

  v2 = (long long *)sub_4ECC0(32);
  *v2 = 0;
  v3 = v2;
  v4 = (long long *)a1[12];
  if ( v4 )
    *v4 = v3;
  else
    a1[11] = v3;
  ++a1[13];
  a1[12] = v3;
  *((int *)v3 + 2) = 0;
  result = (char *)sub_4ECC0(2);
  v3[3] = 1;
  *result = a2;
  result[1] = 0;
  v3[2] = result;
  return result;
}



// Function: add_literal_segment @ 0x2ff60
void add_literal_segment(long long *a1, char *a2, char *a3)
{
  size_t v3; // rbx
  long long *v4; // rax
  long long *v5; // r13
  long long *v6; // rax
  void *v7; // rax
  char *v8; // rax

  v3 = a3 - a2;
  if ( a3 != a2 )
  {
    v4 = (long long *)sub_4ECC0(32);
    *v4 = 0;
    v5 = v4;
    v6 = (long long *)a1[12];
    if ( v6 )
      *v6 = v5;
    else
      a1[11] = v5;
    ++a1[13];
    a1[12] = v5;
    *((int *)v5 + 2) = 0;
    v7 = (void *)sub_4ECC0(v3 + 1);
    v5[2] = v7;
    v8 = memcpy(v7, a2, v3);
    v5[3] = v3;
    v8[v3] = 0;
  }
}



// Function: parse_transform_expr @ 0x30000
char *parse_transform_expr(char *s1)
{
  char *v2; // rdi
  long long v3; // r14
  bool v4; // zf
  int v5; // esi
  char *v6; // r12
  int *v7; // rcx
  int *v8; // rdi
  int v10; // r9d
  long long v11; // r12
  char v12; // al
  int v13; // r13d
  long long v14; // rdx
  int v15; // r15d
  char v16; // bl
  char *v17; // rdi
  int v18; // eax
  int v19; // esi
  char *v20; // rax
  char *v21; // rax
  char *v22; // rbp
  unsigned int v23; // eax
  int v24; // r15d
  char *v25; // rbp
  char *v26; // rsi
  char *v27; // rdx
  char v28; // al
  char *v29; // rdi
  int v30; // eax
  unsigned long long v31; // r13
  long long v32; // rax
  long long *v33; // rdx
  long long v34; // rax
  long long *v35; // rdx
  char *v36; // rax
  char v37; // si
  long long v38; // rax
  long long *v39; // rdx
  char *v40; // rax
  long long v41; // rax
  long long *v42; // rdx
  char *v43; // rax
  long long v44; // rax
  long long *v45; // rdx
  char *v46; // rax
  long long v47; // rax
  long long *v48; // rdx
  char *v49; // rax
  long long v50; // rax
  long long *v51; // rdx
  char *v52; // rax
  unsigned int v53; // r12d
  const char *v54; // rsi
  char *v55; // rax
  const char *v56; // rsi
  char *v57; // rax
  long long v58; // rdi
  char *v59; // rax
  unsigned int v60; // [rsp+0h] [rbp-278h]
  char *src; // [rsp+8h] [rbp-270h]
  char *v62; // [rsp+10h] [rbp-268h]
  int v63; // [rsp+18h] [rbp-260h]
  int v64; // [rsp+18h] [rbp-260h]
  char v65; // [rsp+1Ch] [rbp-25Ch]
  char *v66; // [rsp+20h] [rbp-258h] BYREF
  char *endptr; // [rsp+28h] [rbp-250h] BYREF
  char v68[2]; // [rsp+30h] [rbp-248h] BYREF
  char v69[518]; // [rsp+32h] [rbp-246h] BYREF
  unsigned long long v70; // [rsp+238h] [rbp-40h]

  v2 = (char *)&qword_70;
  v70 = __readfsqword(0x28u);
  v3 = sub_4ED80(112);
  if ( qword_82CC0 )
    *(long long *)qword_82CC0 = v3;
  else
    qword_82CC8 = v3;
  v4 = *s1 == 115;
  qword_82CC0 = v3;
  if ( !v4 )
  {
    v2 = s1;
    if ( !strncmp(s1, "flags=", 6u) )
    {
      v5 = s1[6];
      v6 = s1 + 6;
      dword_81B60 = 0;
      if ( (char)v5 )
      {
        v7 = &dword_81B60;
        while ( (char)v5 != 59 )
        {
          v8 = v7;
          if ( (unsigned int)sub_2FD20(v7, v5) )
          {
            if ( qword_82D50 )
              qword_82D50(v8);
            v53 = *v6;
            v54 = "Unknown transform flag: %c";
            goto LABEL_105;
          }
          v5 = *++v6;
          if ( !(char)v5 )
            return v6;
        }
        ++v6;
      }
      return v6;
    }
LABEL_109:
    if ( qword_82D50 )
      qword_82D50(v2);
    v56 = "Invalid transform expression";
    goto LABEL_112;
  }
  v65 = s1[1];
  if ( !v65 )
    goto LABEL_109;
  v10 = 2;
  v11 = 2;
  src = s1 + 2;
  v12 = s1[2];
  if ( !v12 )
    goto LABEL_109;
  while ( 1 )
  {
    v13 = v10 + 1;
    if ( s1[1] == v12 )
      break;
    if ( v12 == 92 )
      v13 -= (s1[v11 + 1] == 0) - 1;
    v11 = v13;
    v12 = s1[v13];
    v10 = v13;
    if ( !v12 )
      goto LABEL_109;
  }
  v14 = v13;
  v15 = v10 + 1;
  v16 = s1[v13];
  v62 = &s1[v13];
  if ( !v16 )
    goto LABEL_109;
  while ( v16 != v12 )
  {
    if ( v16 == 92 )
      v15 -= (s1[v14 + 1] == 0) - 1;
    v14 = ++v15;
    v16 = s1[v15];
    if ( !v16 )
      goto LABEL_109;
  }
  v17 = &s1[v14 + 1];
  v18 = dword_81B60;
  *(int *)(v3 + 8) = 0;
  v19 = *v17;
  endptr = v17;
  *(int *)(v3 + 12) = v18;
  v60 = 0;
  if ( (char)v19 && (char)v19 != 59 )
  {
    while ( 1 )
    {
      if ( (char)v19 == 105 )
      {
        v20 = endptr;
        v60 |= 2u;
        v17 = endptr + 1;
        goto LABEL_31;
      }
      if ( (char)v19 > 105 )
        break;
      if ( (char)v19 <= 57 )
      {
        if ( (char)v19 <= 47 )
          goto LABEL_29;
        v64 = v10;
        v30 = strtoul(v17, &endptr, 0);
        v17 = endptr;
        v10 = v64;
        *(int *)(v3 + 16) = v30;
        v20 = v17 - 1;
        goto LABEL_31;
      }
      if ( (char)v19 != 103 )
        goto LABEL_29;
      v21 = endptr;
      *(int *)(v3 + 8) = 1;
      v17 = v21 + 1;
      endptr = v21 + 1;
      v19 = v21[1];
      if ( !v21[1] )
        goto LABEL_38;
LABEL_32:
      if ( (char)v19 == 59 )
        goto LABEL_94;
    }
    if ( (char)v19 == 120 )
    {
      v20 = endptr;
      v60 |= 1u;
      v17 = endptr + 1;
    }
    else
    {
LABEL_29:
      if ( (unsigned int)sub_2FD20((int *)(v3 + 12), v19) )
      {
        if ( qword_82D50 )
          qword_82D50(v3 + 12);
        v54 = "Unknown flag in transform expression: %c";
        v53 = *endptr;
LABEL_105:
        v55 = dcgettext(0, v54, 5);
        error(0, 0, v55, v53);
        sub_2DEA0(2);
      }
      v20 = endptr;
      v17 = endptr + 1;
    }
LABEL_31:
    endptr = v17;
    v19 = v20[1];
    if ( !v20[1] )
      goto LABEL_38;
    goto LABEL_32;
  }
LABEL_38:
  if ( (char)v19 == 59 )
LABEL_94:
    endptr = v17 + 1;
  v63 = v10;
  v22 = (char *)sub_4ECC0(v10 - 1);
  memcpy(v22, src, v63 - 2);
  v22[v11 - 2] = 0;
  v23 = sub_5DC90(v3 + 24, v22, v60);
  if ( v23 )
  {
    v58 = v23;
    sub_5DDB0(v23, v3 + 24, v68, 512);
    if ( qword_82D50 )
      qword_82D50(v58);
    v59 = dcgettext(0, "Invalid transform expression: %s", 5);
    error(0, 0, v59, v68);
    sub_2DEA0(2);
  }
  if ( *v22 == 94 || v63 != 2 && v22[v11 - 3] == 36 )
    *(int *)(v3 + 8) = 0;
  v24 = v15 - v13;
  free(v22);
  v25 = (char *)sub_4ECC0(v24 + 1);
  memcpy(v25, v62, v24);
  v66 = v25;
  v26 = v25;
  v27 = v25;
  v25[v24] = 0;
  v28 = *v25;
  if ( *v25 )
  {
    while ( 1 )
    {
      while ( v28 != 92 )
      {
        if ( v28 == 38 )
        {
          sub_2FF60((long long *)v3, v26, v27);
          v50 = sub_4ECC0(32);
          v51 = *(long long **)(v3 + 96);
          *(long long *)v50 = 0;
          if ( v51 )
            *v51 = v50;
          else
            *(long long *)(v3 + 88) = v50;
          *(long long *)(v3 + 96) = v50;
          *(int *)(v50 + 8) = 1;
          *(long long *)(v50 + 16) = 0;
          v52 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v52 + 1;
          v66 = v52 + 1;
          v27 = v52 + 1;
        }
        else
        {
          v66 = ++v27;
        }
        v28 = *v27;
        if ( !*v27 )
          goto LABEL_63;
      }
      sub_2FF60((long long *)v3, v26, v27);
      v29 = v66 + 1;
      v66 = v29;
      switch ( *v29 )
      {
        case '&':
          v37 = 38;
          goto LABEL_68;
        case '0':
        case '1':
        case '2':
        case '3':
        case '4':
        case '5':
        case '6':
        case '7':
        case '8':
        case '9':
          v31 = strtoul(v29, &v66, 10);
          if ( *(long long *)(v3 + 72) < v31 )
          {
            if ( qword_82D50 )
              qword_82D50(v29);
            v56 = "Invalid transform replacement: back reference out of range";
LABEL_112:
            v57 = dcgettext(0, v56, 5);
            error(0, 0, v57);
            sub_2DEA0(2);
          }
          v32 = sub_4ECC0(32);
          v33 = *(long long **)(v3 + 96);
          *(long long *)v32 = 0;
          if ( v33 )
            *v33 = v32;
          else
            *(long long *)(v3 + 88) = v32;
          ++*(long long *)(v3 + 104);
          v26 = v66;
          *(long long *)(v3 + 96) = v32;
          *(int *)(v32 + 8) = 1;
          *(long long *)(v32 + 16) = v31;
LABEL_62:
          v27 = v26;
          v28 = *v26;
          if ( !*v26 )
            goto LABEL_63;
          break;
        case 'E':
          v44 = sub_4ECC0(32);
          v45 = *(long long **)(v3 + 96);
          *(long long *)v44 = 0;
          if ( v45 )
            *v45 = v44;
          else
            *(long long *)(v3 + 88) = v44;
          *(long long *)(v3 + 96) = v44;
          *(int *)(v44 + 8) = 2;
          *(int *)(v44 + 16) = 0;
          v46 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v46 + 1;
          v66 = v46 + 1;
          goto LABEL_62;
        case 'L':
          v41 = sub_4ECC0(32);
          v42 = *(long long **)(v3 + 96);
          *(long long *)v41 = 0;
          if ( v42 )
            *v42 = v41;
          else
            *(long long *)(v3 + 88) = v41;
          *(long long *)(v3 + 96) = v41;
          *(int *)(v41 + 8) = 2;
          *(int *)(v41 + 16) = 4;
          v43 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v43 + 1;
          v66 = v43 + 1;
          goto LABEL_62;
        case 'U':
          v47 = sub_4ECC0(32);
          v48 = *(long long **)(v3 + 96);
          *(long long *)v47 = 0;
          if ( v48 )
            *v48 = v47;
          else
            *(long long *)(v3 + 88) = v47;
          *(long long *)(v3 + 96) = v47;
          *(int *)(v47 + 8) = 2;
          *(int *)(v47 + 16) = 3;
          v49 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v49 + 1;
          v66 = v49 + 1;
          goto LABEL_62;
        case '\\':
          v37 = 92;
          goto LABEL_68;
        case 'a':
          v37 = 7;
          goto LABEL_68;
        case 'b':
          v37 = 8;
          goto LABEL_68;
        case 'f':
          v37 = 12;
          goto LABEL_68;
        case 'l':
          v38 = sub_4ECC0(32);
          v39 = *(long long **)(v3 + 96);
          *(long long *)v38 = 0;
          if ( v39 )
            *v39 = v38;
          else
            *(long long *)(v3 + 88) = v38;
          *(long long *)(v3 + 96) = v38;
          *(int *)(v38 + 8) = 2;
          *(int *)(v38 + 16) = 2;
          v40 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v40 + 1;
          v66 = v40 + 1;
          goto LABEL_62;
        case 'n':
          v37 = 10;
          goto LABEL_68;
        case 'r':
          v37 = 13;
          goto LABEL_68;
        case 't':
          v37 = 9;
          goto LABEL_68;
        case 'u':
          v34 = sub_4ECC0(32);
          v35 = *(long long **)(v3 + 96);
          *(long long *)v34 = 0;
          if ( v35 )
            *v35 = v34;
          else
            *(long long *)(v3 + 88) = v34;
          *(long long *)(v3 + 96) = v34;
          *(int *)(v34 + 8) = 2;
          *(int *)(v34 + 16) = 1;
          v36 = v66;
          ++*(long long *)(v3 + 104);
          v26 = v36 + 1;
          v66 = v36 + 1;
          goto LABEL_62;
        case 'v':
          v37 = 11;
LABEL_68:
          sub_2FEF0((long long *)v3, v37);
          v26 = ++v66;
          goto LABEL_62;
        default:
          if ( *v29 == v16 )
          {
            sub_2FEF0((long long *)v3, v65);
          }
          else
          {
            v68[1] = *v29;
            v68[0] = 92;
            sub_2FF60((long long *)v3, v68, v69);
          }
          v26 = ++v66;
          goto LABEL_62;
      }
    }
  }
LABEL_63:
  sub_2FF60((long long *)v3, v26, v27);
  free(v25);
  return endptr;
}



// Function: set_transform_expr @ 0x30870
void set_transform_expr(char *a1)
{
  for ( ; *a1; a1 = sub_30000(a1) )
    ;
}



// Function: transform_name_fp @ 0x308a0
long long transform_name_fp(
        struct _obstack_chunk **a1,
        int a2,
        long long (*a3)(struct _obstack_chunk *, long long),
        long long a4)
{
  struct _obstack_chunk *object_base; // r15
  long long v5; // r13
  unsigned int v6; // r12d
  size_t *v7; // r12
  unsigned int v8; // ebp
  int i; // eax
  size_t v10; // r14
  size_t v11; // rbx
  char *v12; // rdi
  unsigned long long v13; // rax
  long long v14; // rcx
  char *v15; // rdi
  struct _obstack_chunk *v16; // rsi
  size_t v17; // rax
  char *v18; // rdi
  size_t v19; // rax
  char *v20; // rdi
  size_t v21; // rbx
  char *next_free; // rax
  bool v23; // zf
  char *chunk_limit; // rdx
  long long v25; // rsi
  long long *v27; // r14
  unsigned int v28; // ebx
  int v29; // eax
  unsigned int v30; // eax
  size_t *v31; // rdx
  size_t v32; // rax
  size_t v33; // rdx
  size_t v34; // rdx
  unsigned char *v35; // r8
  size_t v36; // rbp
  char *v37; // rdi
  long long v38; // rax
  unsigned long long v39; // [rsp+0h] [rbp-78h]
  size_t v40; // [rsp+10h] [rbp-68h]
  unsigned char *v41; // [rsp+18h] [rbp-60h]
  unsigned int v42; // [rsp+20h] [rbp-58h]

  object_base = *a1;
  if ( !byte_82C48 )
  {
    obstack_begin(&obstack, 0, 0, sub_4ECC0, &free);
    byte_82C48 = 1;
  }
  v5 = qword_82CC8;
  if ( !qword_82CC8 )
    goto LABEL_72;
  v6 = 0;
  do
  {
    while ( (*(int *)(v5 + 12) & a2) == 0 )
    {
      v5 = *(long long *)v5;
      if ( !v5 )
        goto LABEL_33;
    }
    v7 = (size_t *)sub_4ECC0(16 * (*(long long *)(v5 + 72) + 1LL));
    if ( !LOBYTE(object_base->limit) )
    {
LABEL_70:
      next_free = obstack.next_free;
      if ( obstack.chunk_limit != obstack.next_free )
        goto LABEL_28;
LABEL_71:
      obstack_newchunk(&obstack, 1);
      next_free = obstack.next_free;
      goto LABEL_28;
    }
    v42 = 0;
    v8 = 0;
    v39 = 0;
    for ( i = sub_5DE90(v5 + 24, object_base, *(long long *)(v5 + 72) + 1LL, v7, 0);
          ;
          i = sub_5DE90(v5 + 24, object_base, *(long long *)(v5 + 72) + 1LL, v7, 0) )
    {
      if ( !i )
      {
        v10 = *v7;
        v11 = v7[1];
        if ( *v7 )
        {
          v12 = obstack.next_free;
          if ( v10 > obstack.chunk_limit - obstack.next_free )
          {
            obstack_newchunk(&obstack, *v7);
            v12 = obstack.next_free;
          }
          memcpy(v12, object_base, v10);
          obstack.next_free += v10;
        }
        v13 = *(unsigned int *)(v5 + 16);
        v14 = ++v39;
        if ( (int)v13 && v39 < v13 )
        {
          v15 = obstack.next_free;
          if ( v11 > obstack.chunk_limit - obstack.next_free )
          {
            obstack_newchunk(&obstack, v11);
            v15 = obstack.next_free;
          }
          v16 = object_base;
          object_base = (struct _obstack_chunk *)((char *)object_base + v11);
          memcpy(v15, v16, v11);
          obstack.next_free += v11;
          goto LABEL_18;
        }
        v27 = *(long long **)(v5 + 88);
        if ( !v27 )
          goto LABEL_24;
        v40 = v11;
        v28 = v8;
        while ( 1 )
        {
          v29 = *((int *)v27 + 2);
          if ( v29 == 1 )
            break;
          if ( v29 == 2 )
          {
            v30 = *((int *)v27 + 4);
            if ( v30 > 2 )
            {
              if ( v30 - 3 <= 1 )
                v28 = *((int *)v27 + 4);
              goto LABEL_43;
            }
            if ( v30 )
            {
              v14 = v42;
              if ( v42 && v42 - 3 > 1 )
              {
                v28 = *((int *)v27 + 4);
              }
              else
              {
                v42 = v28;
                v28 = *((int *)v27 + 4);
              }
              goto LABEL_43;
            }
            v27 = (long long *)*v27;
            v28 = 0;
            if ( !v27 )
            {
LABEL_49:
              v8 = v28;
              v11 = v40;
              goto LABEL_24;
            }
          }
          else
          {
            if ( !v29 )
            {
              if ( v28 )
              {
                v35 = sub_2FDB0(v28, (const void *)v27[2], v27[3], v14);
                if ( v28 - 1 <= 1 )
                {
                  v28 = v42;
                  v42 = 0;
                }
              }
              else
              {
                v35 = (unsigned char *)v27[2];
              }
              v37 = obstack.next_free;
              v36 = v27[3];
              if ( v36 > obstack.chunk_limit - obstack.next_free )
                goto LABEL_61;
              goto LABEL_54;
            }
LABEL_43:
            v27 = (long long *)*v27;
            if ( !v27 )
              goto LABEL_49;
          }
        }
        v31 = &v7[2 * v27[2]];
        v32 = *v31;
        if ( *v31 == -1 )
          goto LABEL_43;
        v33 = v31[1];
        if ( v33 == -1 )
          goto LABEL_43;
        v34 = v33 - v32;
        v35 = (unsigned char *)object_base + v32;
        v36 = v34;
        if ( v28 )
        {
          v35 = sub_2FDB0(v28, (char *)object_base + v32, v34, v14);
          if ( v28 - 1 <= 1 )
          {
            v28 = v42;
            v42 = 0;
          }
        }
        v37 = obstack.next_free;
        if ( v36 > obstack.chunk_limit - obstack.next_free )
        {
LABEL_61:
          v41 = v35;
          obstack_newchunk(&obstack, v36);
          v37 = obstack.next_free;
          v35 = v41;
        }
LABEL_54:
        memcpy(v37, v35, v36);
        obstack.next_free += v36;
        goto LABEL_43;
      }
      v17 = strlen((const char *)object_base);
      v18 = obstack.next_free;
      v11 = v17;
      if ( v17 > obstack.chunk_limit - obstack.next_free )
      {
        obstack_newchunk(&obstack, v17);
        v18 = obstack.next_free;
      }
      memcpy(v18, object_base, v11);
      obstack.next_free += v11;
LABEL_24:
      object_base = (struct _obstack_chunk *)((char *)object_base + v11);
      if ( !*(int *)(v5 + 8) )
        break;
LABEL_18:
      if ( !LOBYTE(object_base->limit) )
        goto LABEL_70;
    }
    v19 = strlen((const char *)object_base);
    v20 = obstack.next_free;
    v21 = v19;
    if ( v19 > obstack.chunk_limit - obstack.next_free )
    {
      obstack_newchunk(&obstack, v19);
      v20 = obstack.next_free;
    }
    memcpy(v20, object_base, v21);
    next_free = &obstack.next_free[v21];
    v23 = obstack.chunk_limit == &obstack.next_free[v21];
    obstack.next_free += v21;
    if ( v23 )
      goto LABEL_71;
LABEL_28:
    obstack.next_free = next_free + 1;
    *next_free = 0;
    free(v7);
    object_base = (struct _obstack_chunk *)obstack.object_base;
    if ( obstack.object_base == obstack.next_free )
      *((char *)&obstack + 80) |= 2u;
    v6 = 1;
    chunk_limit = obstack.chunk_limit;
    if ( (~*(long long *)&obstack.alignment_mask & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask])
       - (unsigned long long)obstack.chunk <= obstack.chunk_limit - (char *)obstack.chunk )
      chunk_limit = (char *)(~*(long long *)&obstack.alignment_mask
                           & (long long)&obstack.next_free[*(long long *)&obstack.alignment_mask]);
    obstack.next_free = chunk_limit;
    v5 = *(long long *)v5;
    obstack.object_base = chunk_limit;
  }
  while ( v5 );
LABEL_33:
  if ( (char)v6 )
  {
    v25 = (long long)object_base;
    if ( a3 )
      v25 = a3(object_base, a4);
    sub_25120(a1, v25);
    if ( obstack.chunk >= object_base || obstack.chunk_limit <= (char *)object_base )
    {
      obstack_free(&obstack, object_base);
    }
    else
    {
      obstack.object_base = (char *)object_base;
      obstack.next_free = (char *)object_base;
    }
    return v6;
  }
LABEL_72:
  v6 = 0;
  if ( a3 )
  {
    v6 = 1;
    *a1 = 0;
    v38 = a3(object_base, a4);
    sub_25120(a1, v38);
    free(object_base);
  }
  return v6;
}



// Function: transform_name @ 0x30e70
long long transform_name(struct _obstack_chunk **a1, int a2)
{
  return sub_308A0(a1, a2, 0, 0);
}



// Function: transform_program_p @ 0x30e80
bool transform_program_p()
{
  return qword_82CC8 != 0;
}



// Function: flush_deferred_unlinks @ 0x30e90
long long flush_deferred_unlinks(char a1)
{
  long long *v2; // rbp
  long long *v3; // r15
  long long *v4; // rbx
  const char *v5; // r13
  int v6; // eax
  void *v7; // rdi
  void *v8; // rax
  long long *v9; // rbp
  long long *v10; // rbx
  const char *v11; // r12
  void *v12; // rax
  int *v14; // rax
  unsigned int v15; // [rsp+8h] [rbp-40h]

  v2 = 0;
  v3 = (long long *)qword_82CE8;
  v15 = dword_82580;
  if ( !qword_82CE8 )
    goto LABEL_36;
  do
  {
    while ( 1 )
    {
      v4 = v3;
      v3 = (long long *)*v3;
      if ( !a1 && v4[4] >= (unsigned long long)qword_81F30 )
        goto LABEL_3;
      sub_26300(*((unsigned int *)v4 + 2));
      if ( !*((char *)v4 + 24) )
      {
        if ( unlinkat(oldfd, (const char *)v4[2], 0) )
        {
          v14 = __errno_location();
          v7 = (void *)v4[2];
          if ( *v14 == 2 )
            goto LABEL_16;
          sub_33490(v7);
        }
        break;
      }
      v5 = (const char *)v4[2];
      if ( *((int *)v4 + 2) && (!*v5 || !strcmp((const char *)v4[2], ".")) )
        goto LABEL_3;
      if ( !unlinkat(oldfd, v5, 512) )
        break;
      v6 = *__errno_location();
      if ( v6 != 17 && v6 != 39 )
      {
        if ( v6 != 2 )
        {
          sub_33180(v5);
          v7 = (void *)v4[2];
          goto LABEL_16;
        }
        break;
      }
LABEL_3:
      v2 = v4;
LABEL_4:
      if ( !v3 )
        goto LABEL_18;
    }
    v7 = (void *)v4[2];
LABEL_16:
    free(v7);
    v8 = qword_82CD0;
    --qword_82CD8;
    qword_82CD0 = v4;
    *v4 = v8;
    if ( !v2 )
    {
      qword_82CE8 = (long long)v3;
      goto LABEL_4;
    }
    *v2 = v3;
  }
  while ( v3 );
LABEL_18:
  v9 = (long long *)qword_82CE8;
  if ( !qword_82CE8 )
  {
LABEL_36:
    qword_82CE0 = 0;
    return sub_26300(v15);
  }
  if ( a1 )
  {
    do
    {
      v10 = v9;
      v9 = (long long *)*v9;
      sub_26300(*((unsigned int *)v10 + 2));
      v11 = (const char *)v10[2];
      if ( *((int *)v10 + 2) && *((char *)v10 + 24) && (!*v11 || !strcmp((const char *)v10[2], ".")) )
      {
        v11 = (const char *)sub_26320();
        sub_26300((unsigned int)(*((int *)v10 + 2) - 1));
      }
      if ( unlinkat(oldfd, v11, 512) && *__errno_location() != 2 )
        sub_33180(v11);
      free((void *)v10[2]);
      v12 = qword_82CD0;
      --qword_82CD8;
      qword_82CD0 = v10;
      *v10 = v12;
    }
    while ( v9 );
    qword_82CE0 = 0;
    qword_82CE8 = 0;
  }
  return sub_26300(v15);
}



// Function: finish_deferred_unlinks @ 0x31110
void finish_deferred_unlinks()
{
  long long *v0; // rbx
  void *v1; // rdi

  sub_30E90(1);
  v0 = qword_82CD0;
  if ( qword_82CD0 )
  {
    do
    {
      v1 = v0;
      v0 = (long long *)*v0;
      free(v1);
      qword_82CD0 = v0;
    }
    while ( v0 );
  }
}



// Function: queue_deferred_unlink @ 0x31150
long long queue_deferred_unlink(void *src, char a2)
{
  void *v2; // rbp
  void *v3; // rax
  int v4; // eax
  char *v5; // rax
  long long v6; // rax
  const char *v7; // rdi
  long long v8; // r13
  long long v9; // rbx
  long long *v10; // r14
  const char *v11; // rdi
  long long *v12; // rax
  long long *v13; // rax
  long long v14; // rax

  if ( qword_82CE8 && *(long long *)(qword_82CE8 + 32) < (unsigned long long)qword_81F30 )
  {
    sub_30E90(0);
    v2 = qword_82CD0;
    if ( qword_82CD0 )
      goto LABEL_4;
LABEL_25:
    v2 = (void *)sub_4ECC0(40);
    goto LABEL_5;
  }
  v2 = qword_82CD0;
  if ( !qword_82CD0 )
    goto LABEL_25;
LABEL_4:
  v3 = *(void **)v2;
  *(long long *)v2 = 0;
  qword_82CD0 = v3;
LABEL_5:
  v4 = dword_82580;
  *(long long *)v2 = 0;
  *((int *)v2 + 2) = v4;
  v5 = (char *)sub_4EE30(src);
  *((long long *)v2 + 2) = v5;
  sub_253E0(v5);
  v6 = qword_81F30;
  *((char *)v2 + 24) = a2;
  *((long long *)v2 + 4) = v6;
  if ( !a2 || (v7 = (const char *)*((long long *)v2 + 2), *v7) && strcmp(v7, ".") )
  {
    v13 = (long long *)qword_82CE0;
    if ( qword_82CE0 )
    {
      *(long long *)v2 = *(long long *)qword_82CE0;
      *v13 = v2;
      v14 = *(long long *)v2;
    }
    else
    {
      v14 = qword_82CE8;
      qword_82CE8 = (long long)v2;
      *(long long *)v2 = v14;
    }
    if ( v14 )
      return ++qword_82CD8;
LABEL_18:
    qword_82CE0 = (long long)v2;
    return ++qword_82CD8;
  }
  v8 = qword_82CE8;
  if ( qword_82CE8 )
  {
    v9 = qword_82CE8;
    v10 = 0;
    while ( 1 )
    {
      if ( *(char *)(v9 + 24) )
      {
        v11 = *(const char **)(v9 + 16);
        if ( (!*v11 || !strcmp(v11, ".")) && *(int *)(v9 + 8) < *((int *)v2 + 2) )
          break;
      }
      v10 = (long long *)v9;
      if ( !*(long long *)v9 )
        goto LABEL_15;
      v9 = *(long long *)v9;
    }
    if ( !v10 )
    {
      *(long long *)v2 = v8;
      qword_82CE8 = (long long)v2;
      return ++qword_82CD8;
    }
    *(long long *)v2 = *v10;
    *v10 = v2;
    if ( !*(long long *)v2 )
      goto LABEL_18;
  }
  else
  {
LABEL_15:
    v12 = (long long *)qword_82CE0;
    if ( qword_82CE0 )
    {
      *(long long *)v2 = *(long long *)qword_82CE0;
      *v12 = v2;
      v8 = *(long long *)v2;
    }
    else
    {
      *(long long *)v2 = v8;
      qword_82CE8 = (long long)v2;
    }
    if ( !v8 )
      goto LABEL_18;
  }
  return ++qword_82CD8;
}



// Function: update_archive @ 0x31350
void update_archive(long long a1, long long a2, double a3)
{
  unsigned int v3; // r12d
  long long v4; // rcx
  long long v5; // r8
  struct stat *v6; // rsi
  unsigned int v7; // r15d
  long long v8; // rdi
  long long i; // rax
  struct stat *v10; // r13
  long long v11; // rdx
  long long v12; // rdi
  char *v13; // rax
  char *v14; // rax
  __syscall_slong_t tv_nsec; // rsi
  __time_t tv_sec; // rdi
  int v17; // eax
  unsigned int v18; // ebp
  unsigned long long v19; // rdi
  long long v20; // rdx
  long long v21; // rcx
  long long v22; // r8
  __off_t st_size; // r12
  long long v24; // rax
  struct stat *v25; // rbx
  size_t v26; // r15
  unsigned int v27; // eax
  long long *v28; // rdx
  long long *v29; // rax
  long long *v30; // r12
  void **v31; // r11
  const char *v32; // r12
  unsigned int v33; // r15d
  char *v34; // rax
  long long v35; // rbx
  long long v36; // r13
  char *v37; // rax
  long long *v38; // [rsp+10h] [rbp-108h]
  long long *v39; // [rsp+18h] [rbp-100h]
  char *s; // [rsp+20h] [rbp-F8h]
  unsigned int v41; // [rsp+2Ch] [rbp-ECh]
  struct stat v42; // [rsp+30h] [rbp-E8h] BYREF
  char v43[24]; // [rsp+C0h] [rbp-58h] BYREF
  unsigned long long v44; // [rsp+D8h] [rbp-40h]

  v3 = 0;
  v44 = __readfsqword(0x28u);
  sub_282F0(a3, a1, a2);
  sub_F710(2, a2, a3);
  sub_1D4E0(2);
  while ( 1 )
  {
    v6 = (struct stat *)&qword_82840;
    v7 = sub_22570((char *)&qword_82408, (char *)&qword_82840, 0, v4, v5);
    switch ( v7 )
    {
      case 0u:
      case 2u:
        goto LABEL_60;
      case 1u:
        sub_22C60(qword_82408, (long long)&qword_82840, &dword_82400, 0);
        tv_nsec = (__syscall_slong_t)&qword_82840;
        tv_sec = (unsigned int)*(char *)(qword_82408 + 156);
        sub_22460(tv_sec, &qword_82840);
        dword_82BA4 = dword_82400;
        if ( dword_82BA8 == 8 )
        {
          tv_sec = (__time_t)file;
          v29 = sub_291D0(file, (long long)&qword_82840, a3);
          v30 = v29;
          if ( v29 )
          {
            sub_26300(*((unsigned int *)v29 + 10));
            tv_nsec = (__syscall_slong_t)&v42;
            tv_sec = (__time_t)file;
            if ( !(unsigned int)sub_25FC0(file, &v42) )
            {
              if ( (v42.st_mode & 0xF000) == 0x4000 )
              {
                tv_sec = v30[2];
                tv_nsec = 1;
                s = (char *)sub_26970(tv_sec, 1);
                if ( s )
                {
                  v31 = (void **)sub_265F0((char *)v30[2]);
                  if ( *s )
                  {
                    v41 = v7;
                    v39 = (long long *)v31;
                    v38 = v30;
                    v32 = s;
                    do
                    {
                      v33 = *((int *)v38 + 10);
                      v34 = (char *)sub_26680(v39, v32);
                      tv_nsec = v33;
                      sub_28210(v34, v33, 0, 0);
                      v32 += strlen(v32) + 1;
                    }
                    while ( *v32 );
                    v7 = v41;
                    v30 = v38;
                    v31 = (void **)v39;
                  }
                  sub_26660(v31);
                  free(s);
                  tv_sec = (__time_t)v30;
                  sub_28840(v30);
                }
              }
              else
              {
                tv_sec = v42.st_mtim.tv_sec;
                tv_nsec = v42.st_mtim.tv_nsec;
                if ( (int)sub_2FCC0(
                            v42.st_mtim.tv_sec,
                            v42.st_mtim.tv_nsec,
                            (long long)xmmword_82938,
                            (long long)*(&xmmword_82938 + 1)) <= 0 )
                {
                  tv_sec = (__time_t)v30;
                  sub_28840(v30);
                }
              }
            }
          }
        }
        sub_24660(tv_sec, tv_nsec);
        sub_2FC00((long long)&qword_82840, tv_nsec, a3);
        goto LABEL_20;
      case 3u:
        v8 = (long long)&qword_82840;
        src = (void *)qword_82408;
        sub_2FC00((long long)&qword_82840, (long long)&qword_82840, a3);
        goto LABEL_4;
      case 4u:
        v8 = (long long)&qword_82840;
        sub_2FC00((long long)&qword_82840, (long long)&qword_82840, a3);
LABEL_4:
        sub_CC80();
        byte_82CF8 = 1;
        qword_82CF0 = (long long)src;
LABEL_5:
        for ( i = sub_29260(); i; i = sub_29260() )
        {
          v10 = *(struct stat **)(i + 16);
          v6 = 0;
          v8 = (long long)v10;
          if ( (unsigned char)sub_176D0(v10, 0) )
            goto LABEL_5;
          if ( byte_82B40 )
          {
            v6 = v10;
            v8 = (long long)"add";
            if ( !(unsigned int)sub_2DF50("add", (long long)v10, a3) )
              goto LABEL_5;
            if ( dword_82BA8 == 2 )
            {
LABEL_28:
              v6 = v10;
              v17 = openat(oldfd, (const char *)v10, 0);
              v18 = v17;
              if ( v17 < 0 )
              {
                v8 = (long long)v10;
                sub_32EA0(v10);
              }
              else
              {
                v6 = &v42;
                v19 = (unsigned int)v17;
                if ( fstat(v17, &v42) )
                {
                  sub_33410(v10);
                }
                else
                {
                  st_size = v42.st_size;
                  while ( st_size > 0 )
                  {
                    v25 = (struct stat *)sub_D360(v19, v6, v20, v21, v22);
                    v26 = sub_CD10(v25);
                    if ( v26 > st_size )
                    {
                      v26 = st_size;
                      if ( (st_size & 0x1FF) != 0 )
                      {
                        v27 = 512 - (st_size & 0x1FF);
                        v28 = (__dev_t *)((char *)&v25->st_dev + st_size);
                        if ( v27 >= 8 )
                        {
                          *v28 = 0;
                          *(long long *)((char *)v28 + v27 - 8) = 0;
                          memset(
                            (void *)((unsigned long long)(v28 + 1) & 0xFFFFFFFFFFFFFFF8LL),
                            0,
                            8LL * ((v27 + (int)v28 - (((int)v28 + 8) & 0xFFFFFFF8)) >> 3));
                        }
                        else if ( (-(char)st_size & 4) != 0 )
                        {
                          *(int *)v28 = 0;
                          *(int *)((char *)v28 + v27 - 4) = 0;
                        }
                        else if ( v27 )
                        {
                          *(char *)v28 = 0;
                          if ( (-(char)st_size & 2) != 0 )
                            *(short *)((char *)v28 + v27 - 2) = 0;
                        }
                        v26 = st_size;
                      }
                    }
                    v6 = v25;
                    v24 = sub_4D570(v18, v25, v26);
                    if ( v24 == -1 )
                      sub_330B0(v10, v42.st_size - st_size, v26);
                    if ( !v24 )
                    {
                      if ( qword_82D50 )
                        qword_82D50(v18);
                      v35 = sub_45370(st_size, v43);
                      v36 = sub_4CE40(v10);
                      v37 = dcngettext(0, "%s: File shrank by %s byte", "%s: File shrank by %s bytes", st_size, 5);
                      error(0, 0, v37, v36, v35);
                      sub_16FD0();
                    }
                    st_size -= v24;
                    v19 = (unsigned long long)v25 + ((v24 - 1) & 0xFFFFFFFFFFFFFE00LL);
                    sub_CCD0(v19);
                  }
                }
                v8 = v18;
                if ( close(v18) )
                {
                  v8 = (long long)v10;
                  sub_32D60(v10);
                }
              }
              goto LABEL_5;
            }
          }
          else if ( dword_82BA8 == 2 )
          {
            goto LABEL_28;
          }
          v8 = 0;
          v6 = v10;
          sub_14BB0(0, v10, v10);
        }
        sub_12FE0();
        sub_D8D0(v8, (long long)v6, v11);
        sub_31110();
        sub_28A70(a3, v8, (char *)v6);
        return;
      case 5u:
        v12 = qword_82408;
        sub_CCD0(qword_82408);
        if ( v3 == 1 )
          goto LABEL_17;
        if ( v3 <= 1 )
        {
          if ( qword_82D50 )
            qword_82D50(v12);
          v13 = dcgettext(0, "This does not look like a tar archive", 5);
          v12 = 0;
          error(0, 0, v13);
LABEL_17:
          if ( qword_82D50 )
            qword_82D50(v12);
          v14 = dcgettext(0, "Skipping to next header", 5);
          error(0, 0, v14);
          dword_82D58 = 2;
          sub_2FC00((long long)&qword_82840, 0, a3);
          goto LABEL_20;
        }
        if ( v3 == 3 )
          goto LABEL_17;
        if ( v3 == 4 )
LABEL_60:
          abort();
LABEL_25:
        sub_2FC00((long long)&qword_82840, (long long)&qword_82840, a3);
LABEL_20:
        v3 = v7;
        break;
      default:
        goto LABEL_25;
    }
  }
}



// Function: utf8_convert @ 0x31920
long long utf8_convert(unsigned char a1, void *a2, char **a3)
{
  void *v4; // r12
  char *v5; // r13
  const char *v7; // rax
  char *inbuf; // [rsp+8h] [rbp-50h] BYREF
  char *outbuf; // [rsp+10h] [rbp-48h] BYREF
  size_t inbytesleft; // [rsp+18h] [rbp-40h] BYREF
  size_t outbytesleft[7]; // [rsp+20h] [rbp-38h] BYREF

  v4 = (void *)qword_81B70[a1];
  outbytesleft[1] = __readfsqword(0x28u);
  if ( v4 == (void *)-1LL )
  {
    v7 = (const char *)sub_453D0();
    if ( a1 )
      qword_81B78 = (long long)iconv_open("UTF-8", v7);
    else
      qword_81B70[0] = (long long)iconv_open(v7, "UTF-8");
    v4 = (void *)qword_81B70[a1];
    if ( v4 )
    {
      if ( v4 == (void *)-1LL )
        return 0;
      goto LABEL_3;
    }
LABEL_12:
    *a3 = (char *)sub_4EE30(a2);
    return 1;
  }
  if ( !v4 )
    goto LABEL_12;
LABEL_3:
  inbytesleft = strlen((const char *)a2) + 1;
  outbytesleft[0] = 16 * inbytesleft + 1;
  outbuf = (char *)sub_4ECC0(outbytesleft[0]);
  v5 = outbuf;
  inbuf = (char *)a2;
  if ( iconv(v4, &inbuf, &inbytesleft, &outbuf, outbytesleft) )
  {
    free(v5);
    return 0;
  }
  else
  {
    *outbuf = 0;
    *a3 = v5;
    return 1;
  }
}



// Function: string_ascii_p @ 0x31a80
long long string_ascii_p(char *a1)
{
  while ( 1 )
  {
    if ( !*a1 )
      return 1;
    if ( *a1 < 0 )
      break;
    ++a1;
  }
  return 0;
}



// Function: set_warning_option @ 0x31ab0
int set_warning_option(const char *s1)
{
  int result; // eax
  size_t v2; // rax
  void (*v3)(); // rbx

  result = strcmp(s1, "none");
  if ( !result )
    goto LABEL_5;
  v2 = strlen(s1);
  v3 = off_81DD8;
  if ( v2 > 2 && !memcmp(s1, "no-", 3u) )
  {
    result = dword_81B80 & ~dword_81BA0[sub_3A4E0("--warning", s1 + 3, &off_7DA80, dword_81BA0, 4, v3)];
LABEL_5:
    dword_81B80 = result;
    return result;
  }
  result = dword_81BA0[sub_3A4E0("--warning", s1, &off_7DA80, dword_81BA0, 4, v3)] | dword_81B80;
  dword_81B80 = result;
  return result;
}



// Function: xattrs_masked_out @ 0x31e00
long long xattrs_masked_out(char *name, char a2)
{
  long long v3; // rbx
  long long v4; // rbx

  if ( qword_82D28 )
  {
    if ( !qword_82D30 )
      return 1;
    v3 = 0;
    while ( fnmatch(*((const char **)qword_82D20 + v3), name, 0) )
    {
      if ( qword_82D30 <= (unsigned long long)++v3 )
        return 1;
    }
  }
  else if ( !a2 && strncmp(name, "user.", 5u) )
  {
    return 1;
  }
  if ( qword_82D40 && qword_82D48 )
  {
    v4 = 0;
    while ( fnmatch(*((const char **)qword_82D38 + v4), name, 0) )
    {
      if ( qword_82D48 <= (unsigned long long)++v4 )
        return 0;
    }
    return 1;
  }
  return 0;
}



// Function: xattrs_acls_get @ 0x31ed0
void xattrs_acls_get(long long a1)
{
  char *v1; // rax

  if ( dword_82A9C > 0 )
  {
    if ( dword_82D14 )
    {
      dword_82D14 = 1;
    }
    else
    {
      if ( qword_82D50 )
        qword_82D50(a1);
      v1 = dcgettext(0, "POSIX ACL support is not available", 5);
      error(0, 0, v1);
      dword_82D14 = 1;
    }
  }
}



// Function: xattrs_acls_set @ 0x31f50
void xattrs_acls_set(long long a1, long long a2, char a3)
{
  char *v3; // rax

  if ( dword_82A9C > 0 && a3 != 50 )
  {
    if ( dword_82D10 )
    {
      dword_82D10 = 1;
    }
    else
    {
      if ( qword_82D50 )
        qword_82D50(a1);
      v3 = dcgettext(0, "POSIX ACL support is not available", 5);
      error(0, 0, v3);
      dword_82D10 = 1;
    }
  }
}



// Function: xattrs_mask_add @ 0x31fd0
void *xattrs_mask_add(long long a1, char a2, long long a3)
{
  void **v4; // rbx
  void *result; // rax
  unsigned long long v6; // rcx
  long long v7; // rdi
  long long v8; // rax

  v4 = &qword_82D20;
  if ( !a2 )
    v4 = &qword_82D38;
  result = v4[2];
  v6 = (unsigned long long)v4[1];
  v7 = (long long)*v4;
  if ( result == (void *)v6 )
  {
    if ( v6 )
    {
      if ( v7 )
      {
        if ( v6 <= 0xAAAAAAAAAAAAAA9LL )
        {
LABEL_8:
          v6 += (v6 >> 1) + 1;
LABEL_9:
          v4[1] = (void *)v6;
          v8 = sub_4ECE0(v7, 8 * v6, a3, v6);
          *v4 = (void *)v8;
          v7 = v8;
          result = v4[2];
          goto LABEL_4;
        }
      }
      else
      {
        a3 = (v6 * (unsigned __int128)8uLL) >> 64 != 0;
        if ( (v6 & 0x1000000000000000LL) == 0 && is_mul_ok(v6, 8u) )
          goto LABEL_9;
      }
      sub_17020();
    }
    v4[1] = &byte_4;
    v6 = 4;
    if ( !v7 )
      goto LABEL_9;
    goto LABEL_8;
  }
LABEL_4:
  v4[2] = (char *)result + 1;
  *(long long *)(v7 + 8LL * (long long)result) = a1;
  return result;
}



// Function: xattrs_clear_setup @ 0x320a0
void xattrs_clear_setup()
{
  if ( qword_82D28 )
  {
    free(qword_82D20);
    if ( !qword_82D40 )
      return;
  }
  else if ( !qword_82D40 )
  {
    return;
  }
  free(qword_82D38);
}



// Function: xattrs_xattrs_get @ 0x32100
ssize_t xattrs_xattrs_get(int fd, char *path, long long a3, long long a4)
{
  ssize_t result; // rax
  char *v6; // r12
  int v7; // ebx
  size_t v8; // rsi
  size_t v9; // rdx
  long long v10; // rdx
  long long v11; // rcx
  long long v12; // rbp
  long long v13; // rdx
  long long v14; // rcx
  size_t v15; // rsi
  char *v16; // rax
  const char *v17; // rax
  size_t v18; // rax
  void *v19; // rdx
  ssize_t v20; // rcx
  int v21; // eax
  long long v22; // rdx
  long long v23; // rcx
  size_t v24; // rsi
  char *v25; // rax
  const char *v26; // rdi
  size_t v27; // rsi
  ssize_t v28; // [rsp+0h] [rbp-58h]
  long long v29; // [rsp+8h] [rbp-50h]
  const char *v31; // [rsp+18h] [rbp-40h]

  result = (unsigned int)dword_82A98;
  if ( dword_82A98 <= 0 )
    return result;
  v6 = list;
  v7 = a4;
  v8 = size;
  v9 = size;
  if ( !list )
  {
    if ( size )
    {
      if ( (size & 0x8000000000000000LL) != 0LL )
        goto LABEL_32;
    }
    else
    {
      v8 = 128;
    }
    size = v8;
    v25 = (char *)sub_4ECE0(0, v8, v9, a4);
    v9 = size;
    list = v25;
    v6 = v25;
  }
  while ( v7 )
  {
    result = flistxattr(v7, v6, v9);
    v12 = result;
    if ( result != -1 )
      goto LABEL_12;
LABEL_5:
    if ( *__errno_location() != 34 )
    {
      v26 = "llistxattrat";
      if ( v7 )
        v26 = "flistxattr";
      return sub_32BD0(v26, path);
    }
    v15 = size;
    if ( list )
    {
      if ( size > 0x5555555555555553LL )
        goto LABEL_32;
      v15 = (size >> 1) + size + 1;
    }
    else if ( size )
    {
      if ( (size & 0x8000000000000000LL) != 0LL )
        goto LABEL_32;
    }
    else
    {
      v15 = 128;
    }
    size = v15;
    v16 = (char *)sub_4ECE0(list, v15, v13, v14);
    v9 = size;
    list = v16;
    v6 = v16;
  }
  result = sub_39DD0(fd, path, v6, v9);
  v12 = result;
  if ( result == -1 )
    goto LABEL_5;
  v6 = list;
LABEL_12:
  if ( qword_82D00 )
    goto LABEL_13;
  v27 = qword_81C08;
  if ( !qword_81C08 )
  {
    v27 = 128;
    goto LABEL_49;
  }
  if ( (qword_81C08 & 0x8000000000000000LL) != 0LL )
LABEL_32:
    sub_17020();
LABEL_49:
  qword_81C08 = v27;
  result = sub_4ECE0(0, v27, v10, v11);
  qword_82D00 = (void *)result;
LABEL_13:
  if ( v12 > 0 )
  {
    v17 = "fgetxattr";
    if ( !v7 )
      v17 = "lgetxattrat";
    v31 = v17;
    do
    {
      v18 = strlen(v6);
      v19 = qword_82D00;
      v28 = v18;
      while ( 1 )
      {
        if ( v7 )
        {
          v20 = fgetxattr(v7, v6, v19, qword_81C08);
          if ( v20 != -1 )
          {
LABEL_26:
            v29 = v20;
            if ( !(unsigned char)sub_31E00(v6, 1) )
              sub_1D5D0(a3, v6, qword_82D00, v29);
            goto LABEL_28;
          }
        }
        else
        {
          v20 = sub_398D0(fd, path, v6, v19, qword_81C08);
          if ( v20 != -1 )
            goto LABEL_26;
        }
        v21 = *__errno_location();
        if ( v21 != 34 )
          break;
        v24 = qword_81C08;
        if ( qword_82D00 )
        {
          if ( qword_81C08 > 0x5555555555555553LL )
            goto LABEL_32;
          v24 = (qword_81C08 >> 1) + qword_81C08 + 1;
        }
        else if ( qword_81C08 )
        {
          if ( (qword_81C08 & 0x8000000000000000LL) != 0LL )
            goto LABEL_32;
        }
        else
        {
          v24 = 128;
        }
        qword_81C08 = v24;
        qword_82D00 = (void *)sub_4ECE0(qword_82D00, v24, v22, v23);
        v19 = qword_82D00;
      }
      if ( v21 != 61 )
        sub_32BD0(v31, path);
LABEL_28:
      result = v28;
      v6 += v28 + 1;
      v12 = v12 - v28 - 1;
    }
    while ( v12 > 0 );
  }
  return result;
}



// Function: xattrs_selinux_get @ 0x32430
long long xattrs_selinux_get(int a1, long long a2, long long a3, unsigned int a4)
{
  long long result; // rax
  long long v5; // rdx
  const char *v7; // rdi

  result = (unsigned int)dword_82AA0;
  if ( dword_82AA0 > 0 )
  {
    v5 = a3 + 48;
    if ( a4 )
    {
      result = sub_4FBC0(a4, v5);
      if ( (int)result != -1 )
        return result;
    }
    else
    {
      result = sub_4DD40(a1);
      if ( (int)result != -1 )
        return result;
    }
    result = (unsigned int)*__errno_location();
    if ( (int)result != 95 && (int)result != 61 )
    {
      v7 = "fgetfilecon";
      if ( !a4 )
        v7 = "lgetfileconat";
      return sub_32BD0(v7, a2);
    }
  }
  return result;
}



// Function: xattrs_selinux_set @ 0x324c0
void xattrs_selinux_set(long long a1, long long a2, char a3)
{
  long long v3; // rdi
  int v4; // eax
  char *v5; // r12
  char *v6; // r13
  int *v7; // rax

  if ( dword_82AA0 > 0 && *(long long *)(a1 + 48) )
  {
    v3 = (unsigned int)oldfd;
    if ( a3 == 50 )
    {
      v4 = sub_4E180(oldfd);
      v5 = "lsetfileconat";
    }
    else
    {
      v4 = sub_4DF60(oldfd);
      v5 = "setfileconat";
    }
    if ( v4 == -1 && (dword_81B80 & 0x200000) != 0 )
    {
      if ( qword_82D50 )
        qword_82D50(v3);
      v6 = dcgettext(0, "%s: Cannot set SELinux context for file '%s'", 5);
      v7 = __errno_location();
      error(0, *v7, v6, v5, a2);
    }
  }
}



// Function: xattrs_xattrs_set @ 0x32570
void xattrs_xattrs_set(long long a1, char *a2, char a3, int a4)
{
  unsigned long long v6; // rbx
  long long v7; // r14
  char *v8; // r14
  long long v9; // rdi
  int v10; // eax
  const char *v11; // r12
  int *v12; // rax
  const char *format; // [rsp+18h] [rbp-40h]

  if ( dword_82A98 > 0 && *(long long *)(a1 + 344) )
  {
    v6 = 0;
    do
    {
      v7 = *(long long *)(*(long long *)(a1 + 352) + 24 * v6);
      v8 = (char *)(strlen("SCHILY.xattr.") + v7);
      if ( (a3 != 48 || (strcmp(v8, "security.capability") != 0) != a4)
        && !(unsigned char)sub_31E00(v8, 0)
        && *(long long *)(*(long long *)(a1 + 352) + 24 * v6 + 8) )
      {
        v9 = (unsigned int)oldfd;
        if ( a3 == 50 )
        {
          v10 = sub_393C0(oldfd, a2);
          v11 = "lsetxattr";
        }
        else
        {
          v10 = sub_39140(oldfd, a2);
          v11 = "setxattrat";
        }
        if ( v10 == -1 && (dword_81B80 & 0x200000) != 0 )
        {
          if ( qword_82D50 )
            qword_82D50(v9);
          format = dcgettext(0, "%s: Cannot set '%s' extended attribute for file '%s'", 5);
          v12 = __errno_location();
          error(0, *v12, format, v11, v8, a2);
        }
      }
      ++v6;
    }
    while ( *(long long *)(a1 + 344) > v6 );
  }
}



// Function: xattrs_print_char @ 0x326f0
void xattrs_print_char(long long *a1, char *a2)
{
  long long v2; // r13
  size_t v3; // rax

  if ( dword_82A54 <= 1 )
  {
    *a2 = 0;
    return;
  }
  if ( dword_82A98 > 0 || dword_82AA0 > 0 || dword_82A9C > 0 )
  {
    *(short *)a2 = 32;
    if ( dword_82A98 > 0 && a1[43] )
    {
      v2 = 0;
      while ( 1 )
      {
        v3 = strlen("SCHILY.xattr.");
        if ( !(unsigned char)sub_31E00((char *)(*(long long *)(a1[44] + 24 * v2) + v3), 0) )
          break;
        if ( a1[43] <= (unsigned long long)++v2 )
          goto LABEL_16;
      }
      *a2 = 42;
      if ( dword_82AA0 <= 0 )
        goto LABEL_11;
    }
    else
    {
LABEL_16:
      if ( dword_82AA0 <= 0 )
      {
        if ( dword_82A9C <= 0 )
          return;
        goto LABEL_12;
      }
    }
    if ( a1[6] )
      *a2 = 46;
  }
LABEL_11:
  if ( dword_82A9C <= 0 )
    return;
LABEL_12:
  if ( a1[8] || a1[10] )
    *a2 = 43;
}



// Function: xattrs_print @ 0x32830
double xattrs_print(long long *a1, double result)
{
  const char *v2; // rcx
  double v3; // xmm0_8
  long long v4; // rdx
  unsigned long long v5; // rbp
  long long v6; // r13
  char *v7; // r12

  if ( dword_82A54 > 2 )
  {
    if ( dword_82AA0 > 0 )
    {
      v2 = (const char *)a1[6];
      if ( v2 )
        result = __fprintf_chk(qword_81F28, 1, "  s: %s\n", v2);
    }
    if ( dword_82A9C > 0 && (a1[8] || a1[10]) )
    {
      v3 = __fprintf_chk(qword_81F28, 1, "  a: ");
      sub_31B90("", a1[7], a1[8], v3);
      v4 = a1[10];
      if ( a1[8] && v4 )
      {
        v3 = __fprintf_chk(qword_81F28, 1, ",");
        v4 = a1[10];
      }
      sub_31B90("default:", a1[9], v4, v3);
      result = __fprintf_chk(qword_81F28, 1, "\n");
    }
    if ( dword_82A98 > 0 && a1[43] )
    {
      v5 = 0;
      do
      {
        while ( 1 )
        {
          v6 = 24 * v5;
          v7 = (char *)(*(long long *)(a1[44] + 24 * v5) + strlen("SCHILY.xattr."));
          if ( !(unsigned char)sub_31E00(v7, 0) )
            break;
          if ( a1[43] <= ++v5 )
            return result;
        }
        ++v5;
        result = __fprintf_chk(qword_81F28, 1, "  x: %lu %s\n", *(long long *)(a1[44] + v6 + 16), v7);
      }
      while ( a1[43] > v5 );
    }
  }
  return result;
}


