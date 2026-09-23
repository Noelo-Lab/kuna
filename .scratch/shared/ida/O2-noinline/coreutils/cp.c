// Function: main @ 0x40b0
long long main(int a1, char **a2, char **a3)
{
  char *v5; // rdi
  char **v6; // rsi
  int v7; // eax
  long long v8; // rax
  char *v9; // rax
  int v10; // eax
  long long v11; // rdi
  long long v12; // rdx
  long long v13; // rcx
  char *v15; // rax
  const char *v16; // rsi
  char *v17; // rax
  char *v18; // rax
  char *v19; // rax
  int *v20; // rax
  char *v21; // rax
  long long v22; // r13
  char *v23; // r12
  int *v24; // rax
  long long v25; // [rsp+0h] [rbp-C8h]
  char **v26; // [rsp+8h] [rbp-C0h]
  long long v27; // [rsp+10h] [rbp-B8h]
  long long v28; // [rsp+18h] [rbp-B0h]
  char v29; // [rsp+25h] [rbp-A3h]
  unsigned char v30; // [rsp+26h] [rbp-A2h]
  char v31; // [rsp+27h] [rbp-A1h]
  const char *format; // [rsp+28h] [rbp-A0h]
  int v33; // [rsp+30h] [rbp-98h] BYREF
  int v34; // [rsp+34h] [rbp-94h]
  int v35; // [rsp+38h] [rbp-90h]
  int v36; // [rsp+3Ch] [rbp-8Ch]
  char v37; // [rsp+44h] [rbp-84h]
  char v38; // [rsp+45h] [rbp-83h]
  char v39; // [rsp+46h] [rbp-82h]
  char v40; // [rsp+47h] [rbp-81h]
  char v41; // [rsp+4Ch] [rbp-7Ch]
  char v42; // [rsp+4Dh] [rbp-7Bh]
  short v43; // [rsp+4Eh] [rbp-7Ah]
  long long v44; // [rsp+58h] [rbp-70h]
  char v45; // [rsp+60h] [rbp-68h]
  char v46; // [rsp+61h] [rbp-67h]
  char v47; // [rsp+62h] [rbp-66h]
  char v48; // [rsp+63h] [rbp-65h]
  char v49; // [rsp+64h] [rbp-64h]
  char v50; // [rsp+65h] [rbp-63h]
  short v51; // [rsp+67h] [rbp-61h]
  char v52; // [rsp+6Ah] [rbp-5Eh]
  char v53; // [rsp+6Bh] [rbp-5Dh]
  char v54; // [rsp+6Ch] [rbp-5Ch]
  int v55; // [rsp+74h] [rbp-54h]
  unsigned long long v56; // [rsp+88h] [rbp-40h]

  v5 = *a2;
  v56 = __readfsqword(0x28u);
  sub_127F0(v5);
  setlocale(6, "");
  bindtextdomain("coreutils", "/usr/local/share/locale");
  textdomain("coreutils");
  sub_1A150(sub_E560);
  byte_2515A = (int)is_selinux_enabled() > 0;
  sub_49F0(&v33);
  v30 = 0;
  v28 = 0;
  v25 = 0;
  v31 = 0;
  v26 = 0;
  v27 = 0;
  v29 = 0;
  while ( 1 )
  {
    v6 = a2;
    v7 = getopt_long(a1, a2, "abdfHilLnprst:uvxPRS:TZ", &longopts, 0);
    if ( v7 == -1 )
      break;
    if ( v7 > 136 )
      goto LABEL_84;
    if ( v7 <= 71 )
    {
      if ( v7 == -131 )
      {
        sub_16CC0(
          (int)stdout,
          (unsigned int)"cp",
          (unsigned int)"GNU coreutils",
          (int)off_25020,
          (unsigned int)"Torbjorn Granlund",
          (unsigned int)"David MacKenzie",
          "Jim Meyering",
          0);
        exit(0);
      }
      if ( v7 == -130 )
        sub_57E0(0);
LABEL_84:
      sub_57E0(1);
    }
    switch ( v7 )
    {
      case 72:
        v34 = 3;
        continue;
      case 76:
        v34 = 4;
        continue;
      case 80:
        v34 = 2;
        continue;
      case 82:
      case 114:
        HIBYTE(v51) = 1;
        continue;
      case 83:
        v29 = 1;
        v27 = optarg;
        continue;
      case 84:
        v30 = 1;
        continue;
      case 90:
        if ( byte_2515A )
        {
          if ( optarg )
          {
            v28 = optarg;
          }
          else
          {
            v44 = selabel_open(0, 0, 0);
            if ( !v44 )
            {
              format = dcgettext(0, "warning: ignoring --context", 5);
              v20 = __errno_location();
              error(0, *v20, format);
            }
          }
        }
        else if ( optarg )
        {
          v15 = dcgettext(0, "warning: ignoring --context; it requires an SELinux-enabled kernel", 5);
          error(0, 0, v15);
        }
        continue;
      case 97:
        v45 = 1;
        v34 = 2;
        v42 = 1;
        v43 = 257;
        v47 = 1;
        if ( byte_2515A )
          v48 = 1;
        v50 = 1;
        v51 = 257;
        continue;
      case 98:
        v8 = optarg;
        v29 = 1;
        if ( !optarg )
          v8 = (long long)v26;
        v26 = (char **)v8;
        continue;
      case 100:
        v45 = 1;
        v34 = 2;
        continue;
      case 102:
        v39 = 1;
        continue;
      case 105:
        v35 = 3;
        continue;
      case 108:
        v40 = 1;
        continue;
      case 110:
        v35 = 2;
        continue;
      case 112:
        goto LABEL_7;
      case 115:
        v52 = 1;
        continue;
      case 116:
        if ( v25 )
        {
          v21 = dcgettext(0, "multiple target directories specified", 5);
          error(1, 0, v21);
LABEL_89:
          v16 = "options --backup and --no-clobber are mutually exclusive";
LABEL_83:
          v19 = dcgettext(0, v16, 5);
          error(0, 0, v19);
          goto LABEL_84;
        }
        v25 = optarg;
        break;
      case 117:
        v53 = 1;
        continue;
      case 118:
        v54 = 1;
        continue;
      case 120:
        v41 = 1;
        continue;
      case 128:
        v46 = 0;
        continue;
      case 129:
        v31 = 1;
        continue;
      case 130:
        sub_4A80(optarg, &v33, 0);
        continue;
      case 131:
        byte_25159 = 1;
        continue;
      case 132:
        if ( optarg )
        {
          sub_4A80(optarg, &v33, 1);
          v47 = 1;
        }
        else
        {
LABEL_7:
          v42 = 1;
          v43 = 257;
          v47 = 1;
        }
        continue;
      case 133:
        if ( optarg )
          v55 = dword_1CAD0[sub_D220("--reflink", optarg, &off_24860, dword_1CAD0, 4, off_25028, 1)];
        else
          v55 = 2;
        continue;
      case 134:
        v36 = dword_1CAE0[sub_D220("--sparse", optarg, off_24880, dword_1CAE0, 4, off_25028, 1)];
        continue;
      case 135:
        byte_25158 = 1;
        continue;
      case 136:
        v38 = 1;
        continue;
      default:
        goto LABEL_84;
    }
  }
  if ( v40 && v52 )
  {
    v16 = "cannot make both hard and symbolic links";
    goto LABEL_83;
  }
  if ( v35 == 2 )
  {
    v53 = 0;
    if ( v29 )
      goto LABEL_89;
LABEL_66:
    v10 = 0;
    if ( v55 == 2 && v36 != 2 )
      goto LABEL_82;
    goto LABEL_52;
  }
  if ( !v29 )
    goto LABEL_66;
  if ( v55 == 2 && v36 != 2 )
  {
LABEL_82:
    v16 = "--reflink can be used only with --sparse=auto";
    goto LABEL_83;
  }
  v9 = dcgettext(0, "backup type", 5);
  v6 = v26;
  v10 = sub_DB20(v9, v26);
LABEL_52:
  v11 = v27;
  v33 = v10;
  sub_D750(v27);
  if ( v34 == 1 )
    goto LABEL_72;
LABEL_53:
  if ( HIBYTE(v51) )
    goto LABEL_75;
  while ( 1 )
  {
    v13 = v44 | v28;
    if ( !(v44 | v28) )
    {
      if ( !v49 || byte_2515A )
        goto LABEL_59;
      goto LABEL_71;
    }
    if ( !v49 )
      break;
    if ( v48 )
    {
      v18 = dcgettext(0, "cannot set target context and preserve it", 5);
      v6 = 0;
      v11 = 1;
      error(1, 0, v18);
      goto LABEL_78;
    }
    if ( byte_2515A )
      goto LABEL_57;
LABEL_71:
    v17 = dcgettext(0, "cannot preserve security context without an SELinux-enabled kernel", 5);
    v6 = 0;
    v11 = 1;
    error(1, 0, v17);
LABEL_72:
    if ( !HIBYTE(v51) || v40 )
    {
LABEL_78:
      v34 = 4;
      goto LABEL_53;
    }
    v34 = 2;
LABEL_75:
    v37 = v31;
  }
  v48 = 0;
LABEL_57:
  if ( v28 )
  {
    v11 = v28;
    if ( (int)setfscreatecon(v28, v6, v12, v13) < 0 )
    {
      v22 = sub_14E00(v28);
      v23 = dcgettext(0, "failed to set default file creation context to %s", 5);
      v24 = __errno_location();
      error(1, *v24, v23, v22);
    }
  }
LABEL_59:
  sub_BFE0(v11, v6, v12, v13);
  return (unsigned char)sub_5AE0((unsigned int)(a1 - optind), &a2[optind], v25, v30, &v33) ^ 1u;
}



// Function: cp_option_init @ 0x49f0
char *cp_option_init(long long a1)
{
  char *result; // rax

  sub_83B0();
  *(int *)(a1 + 68) = 1;
  *(long long *)(a1 + 8) = 0x200000004LL;
  *(long long *)(a1 + 16) = 0x100000000LL;
  *(int *)(a1 + 28) = 0;
  *(char *)(a1 + 32) = 0;
  *(int *)(a1 + 4) = 1;
  *(short *)(a1 + 24) = 0;
  *(long long *)(a1 + 40) = 0;
  *(long long *)(a1 + 48) = 256;
  *(int *)(a1 + 56) = 0;
  *(short *)(a1 + 60) = 0;
  result = getenv("POSIXLY_CORRECT");
  *(long long *)(a1 + 72) = 0;
  *(long long *)(a1 + 80) = 0;
  *(char *)(a1 + 62) = result != 0;
  return result;
}



// Function: decode_preserve_arg @ 0x4a80
void decode_preserve_arg(void *a1, char *a2, char a3)
{
  const char *v5; // r15
  const char *v6; // rax
  const char *v7; // r14
  char *v8; // rax
  bool v9; // zf
  const char *v10; // [rsp+0h] [rbp-48h]
  char *ptr; // [rsp+8h] [rbp-40h]

  ptr = (char *)sub_17230(a1);
  v5 = ptr;
  v6 = "--no-preserve";
  if ( a3 )
    v6 = "--preserve";
  v10 = v6;
  while ( 2 )
  {
    v7 = 0;
    v8 = strchr(v5, 44);
    if ( v8 )
    {
      *v8 = 0;
      v7 = v8 + 1;
    }
    switch ( sub_D220(v10, v5, &off_24460, &unk_1CAB0, 4, off_25028, 1) )
    {
      case 0LL:
      case 7LL:
      case 10LL:
      case 11LL:
        a2[30] = a3;
        a2[32] = a3 ^ 1;
        goto LABEL_8;
      case 1LL:
      case 8LL:
      case 12LL:
        a2[31] = a3;
        goto LABEL_8;
      case 2LL:
      case 9LL:
      case 13LL:
        a2[29] = a3;
        goto LABEL_8;
      case 3LL:
      case 14LL:
        a2[48] = a3;
        goto LABEL_8;
      case 4LL:
        a2[52] = a3;
        a2[51] = a3;
        if ( !v7 )
          goto LABEL_11;
        goto LABEL_9;
      case 5LL:
        a2[53] = a3;
        a2[54] = a3;
        goto LABEL_8;
      case 6LL:
        a2[30] = a3;
        v9 = byte_2515A == 0;
        a2[31] = a3;
        a2[29] = a3;
        a2[48] = a3;
        a2[32] = a3 ^ 1;
        if ( !v9 )
          a2[51] = a3;
        a2[53] = a3;
LABEL_8:
        if ( v7 )
        {
LABEL_9:
          v5 = v7;
          continue;
        }
LABEL_11:
        free(ptr);
        return;
      default:
        abort();
    }
  }
}



// Function: make_dir_parents_private @ 0x4bf0
long long make_dir_parents_private(
        const char *src,
        size_t a2,
        unsigned int a3,
        long long a4,
        long long *a5,
        char *a6,
        long long a7)
{
  size_t v9; // r14
  size_t v11; // rax
  struct stat **v12; // rcx
  signed long long v13; // rdx
  void *v14; // rsp
  long long *v15; // rdx
  signed long long v16; // rcx
  void *v17; // rsp
  const char *v18; // rsi
  char *v19; // rbx
  bool v20; // zf
  char *v21; // r14
  const char *v22; // rdi
  char *v23; // rax
  char *v24; // rbx
  int v25; // r13d
  int v26; // r8d
  long long v27; // rax
  __m128i v28; // xmm1
  __m128i v29; // xmm2
  __m128i v30; // xmm3
  long long v31; // r9
  __m128i v32; // xmm5
  __m128i v33; // xmm6
  __m128i v34; // xmm7
  __m128i v35; // xmm0
  __m128i v36; // xmm4
  long long *v37; // rcx
  long long v38; // rax
  long long v39; // rdx
  long long v40; // r9
  int v41; // edx
  int v42; // r13d
  short v43; // cx
  int v44; // eax
  long long v45; // r9
  int v46; // eax
  long long v47; // rcx
  long long v48; // r8
  long long v49; // r9
  __mode_t st_mode; // eax
  long long v51; // rdx
  long long v52; // r12
  char *v53; // rax
  int v54; // eax
  char *v55; // rsi
  long long v56; // r12
  char *v57; // rax
  const char *v58; // rsi
  long long v59; // r13
  char *v60; // r12
  int *v61; // rax
  long long v62; // [rsp+0h] [rbp-21A8h]
  long long v63; // [rsp+8h] [rbp-21A0h] BYREF
  char v64[4088]; // [rsp+10h] [rbp-2198h] BYREF
  long long v65; // [rsp+1008h] [rbp-11A0h] BYREF
  long long v66; // [rsp+1010h] [rbp-1198h] BYREF
  struct stat *v67; // [rsp+2008h] [rbp-1A0h] BYREF
  long long v68; // [rsp+2010h] [rbp-198h]
  long long v69; // [rsp+2018h] [rbp-190h]
  long long *v70; // [rsp+2020h] [rbp-188h]
  struct stat *p_buf; // [rsp+2028h] [rbp-180h]
  char *v72; // [rsp+2030h] [rbp-178h]
  char *file; // [rsp+2038h] [rbp-170h]
  void *srca; // [rsp+2040h] [rbp-168h]
  struct stat buf; // [rsp+2048h] [rbp-160h] BYREF
  __m128i v76; // [rsp+20D8h] [rbp-D0h] BYREF
  __m128i v77; // [rsp+20E8h] [rbp-C0h] BYREF
  __m128i v78; // [rsp+20F8h] [rbp-B0h] BYREF
  __m128i v79; // [rsp+2108h] [rbp-A0h] BYREF
  __m128i v80; // [rsp+2118h] [rbp-90h] BYREF
  __m128i v81; // [rsp+2128h] [rbp-80h] BYREF
  __m128i v82; // [rsp+2138h] [rbp-70h] BYREF
  __m128i v83; // [rsp+2148h] [rbp-60h] BYREF
  __m128i v84; // [rsp+2158h] [rbp-50h] BYREF
  unsigned long long v85; // [rsp+2170h] [rbp-38h]

  v68 = a4;
  v70 = a5;
  v72 = a6;
  v85 = __readfsqword(0x28u);
  v9 = sub_E750();
  *v70 = 0;
  if ( v9 <= a2 )
    return 1;
  v11 = strlen(src);
  v12 = (struct stat **)((char *)&v67 - ((v11 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v67 != v12 )
  {
    while ( &v65 != (long long *)v12 )
      ;
  }
  v13 = ((short)v11 + 24) & 0xFF0;
  v14 = alloca(v13);
  if ( (((short)v11 + 24) & 0xFF0) != 0 )
    *(long long *)&v64[v13 + 4080] = *(long long *)&v64[v13 + 4080];
  srca = memcpy(&v66, src, v11 + 1);
  v15 = (long long *)((char *)&v65 - ((v9 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &v65 != v15 )
  {
    while ( &v63 != v15 )
      ;
  }
  v16 = ((short)v9 + 24) & 0xFF0;
  v17 = alloca(v16);
  if ( (((short)v9 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v62 + v16) = *(long long *)((char *)&v62 + v16);
  memcpy(v64, srca, v9);
  v18 = &v64[a2];
  for ( v64[v9] = 0; *v18 == 47; ++v18 )
    ;
  p_buf = &buf;
  if ( fstatat(a3, v18, &buf, 0) )
  {
    v19 = (char *)srca + a2;
    v20 = *v19 == 47;
    file = v19;
    v21 = v19;
    if ( v20 )
    {
      do
        ++v21;
      while ( *v21 == 47 );
    }
    else
    {
      v21 = file;
    }
    v22 = v21;
    v67 = (struct stat *)&v76;
    while ( 1 )
    {
      v23 = strchr(v22, 47);
      v24 = v23;
      if ( !v23 )
        return 1;
      *v23 = 0;
      v25 = fstatat(a3, v21, p_buf, 0);
      if ( !v25 && (*(int *)(a7 + 28) & 0xFFFFFF00) == 0 )
        break;
      if ( stat(file, v67) )
      {
        v26 = *__errno_location();
        if ( v26 )
          goto LABEL_51;
      }
      else if ( (v77.m128i_i16[4] & 0xF000) != 0x4000 )
      {
        v26 = 20;
LABEL_51:
        LODWORD(srca) = v26;
        v52 = sub_14B40(4, file);
        v53 = dcgettext(0, "failed to get attributes of %s", 5);
        error(0, (int)srca, v53, v52);
        return 0;
      }
      v27 = sub_16E90(168);
      v28 = _mm_load_si128(&v77);
      v29 = _mm_load_si128(&v78);
      v30 = _mm_load_si128(&v79);
      v31 = v27;
      *(__m128i *)v27 = _mm_load_si128(&v76);
      v32 = _mm_load_si128(&v81);
      v33 = _mm_load_si128(&v82);
      v34 = _mm_load_si128(&v83);
      v35 = _mm_load_si128(&v84);
      *(__m128i *)(v27 + 16) = v28;
      v36 = _mm_load_si128(&v80);
      v37 = v70;
      *(__m128i *)(v27 + 32) = v29;
      *(__m128i *)(v27 + 48) = v30;
      *(__m128i *)(v27 + 64) = v36;
      *(__m128i *)(v27 + 80) = v32;
      *(__m128i *)(v27 + 96) = v33;
      *(__m128i *)(v27 + 112) = v34;
      *(__m128i *)(v27 + 128) = v35;
      *(long long *)(v27 + 152) = v24 - (char *)srca;
      v38 = *v37;
      *(char *)(v31 + 144) = 0;
      *(long long *)(v31 + 160) = v38;
      *v37 = v31;
      if ( !v25 )
        break;
      v39 = *(unsigned int *)(v31 + 24);
      v69 = v31;
      if ( !(unsigned char)sub_7FF0(file, srca, v39, 1, a7) )
        return 0;
      v40 = v69;
      *v72 = 1;
      v41 = *(int *)(v40 + 24);
      if ( *(char *)(a7 + 29) )
      {
        v42 = *(int *)(v40 + 24) & 0x3F;
        v43 = ~(v41 & 0x3F);
      }
      else if ( *(char *)(a7 + 30) )
      {
        v42 = *(int *)(v40 + 24) & 0x12;
        v43 = ~(v41 & 0x12);
      }
      else
      {
        v43 = -1;
        v42 = 0;
      }
      if ( *(char *)(a7 + 32) )
        LOWORD(v41) = 511;
      v69 = v40;
      v44 = mkdirat(a3, v21, (unsigned short)v43 & (unsigned short)v41 & 0xFFF);
      v45 = v69;
      if ( v44 )
      {
        v58 = "cannot make directory %s";
        v59 = sub_14B40(4, srca);
        goto LABEL_66;
      }
      if ( v68 )
      {
        __printf_chk(1, v68, file, srca);
        v45 = v69;
      }
      v69 = v45;
      v46 = fstatat(a3, v21, p_buf, 256);
      v49 = v69;
      if ( v46 )
      {
        v58 = "failed to get attributes of %s";
        v59 = sub_14B40(4, srca);
LABEL_66:
        v60 = dcgettext(0, v58, 5);
        v61 = __errno_location();
        error(0, *v61, v60, v59);
        return 0;
      }
      st_mode = buf.st_mode;
      if ( !*(char *)(a7 + 30) )
      {
        if ( (v42 & ~buf.st_mode) != 0
          && (v54 = sub_8640(), v49 = v69, v42 &= ~v54, st_mode = buf.st_mode, (v42 & ~buf.st_mode) != 0)
          || (st_mode & 0x1C0) != 0x1C0 )
        {
          *(char *)(v49 + 144) = 1;
          *(int *)(v49 + 24) = st_mode | v42;
        }
      }
      v51 = st_mode | 0x1C0;
      if ( (int)v51 != st_mode && (unsigned int)sub_E520(a3, v21, v51, v47, v48, v49) )
      {
        v58 = "setting permissions for %s";
        v59 = sub_14B40(4, srca);
        goto LABEL_66;
      }
      if ( !*v72 )
        goto LABEL_23;
LABEL_26:
      v20 = v24[1] == 47;
      *v24 = 47;
      v22 = v24 + 1;
      if ( v20 )
      {
        do
          ++v22;
        while ( *v22 == 47 );
      }
    }
    if ( !(unsigned char)sub_7FF0(file, srca, 0, 0, a7) )
      return 0;
    if ( (buf.st_mode & 0xF000) != 0x4000 )
    {
      v55 = srca;
      goto LABEL_62;
    }
    *v72 = 0;
LABEL_23:
    if ( (*(long long *)(a7 + 40) || *(char *)(a7 + 51)) && !(unsigned char)sub_8250(srca, 0, a7) && *(char *)(a7 + 52) )
      return 0;
    goto LABEL_26;
  }
  if ( (buf.st_mode & 0xF000) == 0x4000 )
  {
    *v72 = 0;
    return 1;
  }
  v55 = v64;
LABEL_62:
  v56 = sub_14B40(4, v55);
  v57 = dcgettext(0, "%s exists but is not a directory", 5);
  error(0, 0, v57, v56);
  return 0;
}



// Function: re_protect @ 0x52e0
long long re_protect(const char *src, unsigned int fd, long long a3, long long a4, char *a5)
{
  size_t v7; // rax
  struct timespec **v8; // rcx
  signed long long v9; // rdx
  void *v10; // rsp
  long long v11; // rcx
  long long v12; // r9
  char *v13; // r12
  const char *v14; // r13
  long long v15; // r8
  bool v16; // zf
  long long v17; // rdx
  long long v18; // rax
  long long v19; // rdx
  long long v20; // r13
  char *v21; // r12
  int *v22; // rax
  long long v24; // rdx
  const char *v25; // rsi
  long long v26; // r13
  char *v27; // r12
  int *v28; // rax
  long long v30; // [rsp+0h] [rbp-1078h]
  long long v31; // [rsp+8h] [rbp-1070h] BYREF
  long long v32; // [rsp+10h] [rbp-1068h] BYREF
  struct timespec *times; // [rsp+1008h] [rbp-70h] BYREF
  long long v34; // [rsp+1010h] [rbp-68h]
  long long v35[12]; // [rsp+1018h] [rbp-60h] BYREF

  v34 = a3;
  v35[5] = __readfsqword(0x28u);
  v7 = strlen(src);
  v8 = (struct timespec **)((char *)&times - ((v7 + 24) & 0xFFFFFFFFFFFFF000LL));
  if ( &times != v8 )
  {
    while ( &v31 != (long long *)v8 )
      ;
  }
  v9 = ((short)v7 + 24) & 0xFF0;
  v10 = alloca(v9);
  if ( (((short)v7 + 24) & 0xFF0) != 0 )
    *(long long *)((char *)&v30 + v9) = *(long long *)((char *)&v30 + v9);
  v13 = memcpy(&v32, src, v7 + 1);
  v14 = &v13[v34 - (long long)src];
  if ( !a4 )
    return 1;
  times = (struct timespec *)v35;
  while ( 1 )
  {
    v16 = a5[31] == 0;
    v13[*(long long *)(a4 + 152)] = 0;
    if ( !v16 )
    {
      v35[0] = sub_15960(a4);
      v35[1] = v17;
      v18 = sub_15980(a4);
      v35[3] = v19;
      v35[2] = v18;
      if ( utimensat(fd, v14, times, 0) )
      {
        v20 = sub_14B40(4, v13);
        v21 = dcgettext(0, "failed to preserve times for %s", 5);
        v22 = __errno_location();
        error(0, *v22, v21, v20);
        return 0;
      }
    }
    if ( !a5[29] || !(unsigned int)sub_E540(fd, v14, *(unsigned int *)(a4 + 28), *(unsigned int *)(a4 + 32)) )
    {
      v15 = (unsigned char)a5[30];
      if ( (char)v15 )
        goto LABEL_17;
      goto LABEL_9;
    }
    if ( !(unsigned char)sub_8400(a5) )
      break;
    sub_E540(fd, v14, 0xFFFFFFFFLL, *(unsigned int *)(a4 + 32));
    v15 = (unsigned char)a5[30];
    if ( (char)v15 )
    {
LABEL_17:
      if ( (unsigned int)sub_CAA0(v14, 0xFFFFFFFFLL, v13, 0xFFFFFFFFLL, *(unsigned int *)(a4 + 24)) )
        return 0;
      goto LABEL_10;
    }
LABEL_9:
    if ( *(char *)(a4 + 144) )
    {
      v24 = *(unsigned int *)(a4 + 24);
      LOBYTE(v34) = v15;
      if ( (unsigned int)sub_E520(fd, v14, v24, v11, v15, v12) )
      {
        v25 = "failed to preserve permissions for %s";
        v26 = sub_14B40(4, v13);
        goto LABEL_21;
      }
    }
LABEL_10:
    v13[*(long long *)(a4 + 152)] = 47;
    a4 = *(long long *)(a4 + 160);
    if ( !a4 )
      return 1;
  }
  LOBYTE(v34) = 0;
  v25 = "failed to preserve ownership for %s";
  v26 = sub_14B40(4, v13);
LABEL_21:
  v27 = dcgettext(0, v25, 5);
  v28 = __errno_location();
  error(0, *v28, v27, v26);
  return (unsigned char)v34;
}



// Function: usage @ 0x57e0
void usage(int status)
{
  long long v2; // r12
  char *v3; // rax
  char *v4; // rax
  FILE *v5; // r12
  char *v6; // rax
  FILE *v7; // r12
  char *v8; // rax
  FILE *v9; // r12
  char *v10; // rax
  FILE *v11; // r12
  char *v12; // rax
  FILE *v13; // r12
  char *v14; // rax
  FILE *v15; // r12
  char *v16; // rax
  FILE *v17; // r12
  char *v18; // rax
  FILE *v19; // r12
  char *v20; // rax
  FILE *v21; // r12
  char *v22; // rax
  FILE *v23; // r12
  char *v24; // rax
  FILE *v25; // r12
  char *v26; // rax
  FILE *v27; // r12
  char *v28; // rax
  FILE *v29; // r12
  char *v30; // rax
  FILE *v31; // r12
  char *v32; // rax
  FILE *v33; // r12
  char *v34; // rax
  FILE *v35; // r12
  char *v36; // rax
  FILE *v37; // rsi
  char *v38; // rdi

  v2 = qword_255A0;
  if ( status )
  {
    v3 = dcgettext(0, "Try '%s --help' for more information.\n", 5);
    __fprintf_chk(stderr, 1, v3, v2);
  }
  else
  {
    v4 = dcgettext(
           0,
           "Usage: %s [OPTION]... [-T] SOURCE DEST\n"
           "  or:  %s [OPTION]... SOURCE... DIRECTORY\n"
           "  or:  %s [OPTION]... -t DIRECTORY SOURCE...\n",
           5);
    __printf_chk(1, v4, v2, v2);
    v5 = stdout;
    v6 = dcgettext(0, "Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.\n", 5);
    fputs_unlocked(v6, v5);
    sub_4970();
    v7 = stdout;
    v8 = dcgettext(
           0,
           "  -a, --archive                same as -dR --preserve=all\n"
           "      --attributes-only        don't copy the file data, just the attributes\n"
           "      --backup[=CONTROL]       make a backup of each existing destination file\n"
           "  -b                           like --backup but does not accept an argument\n"
           "      --copy-contents          copy contents of special files when recursive\n"
           "  -d                           same as --no-dereference --preserve=links\n",
           5);
    fputs_unlocked(v8, v7);
    v9 = stdout;
    v10 = dcgettext(
            0,
            "  -f, --force                  if an existing destination file cannot be\n"
            "                                 opened, remove it and try again (this option\n"
            "                                 is ignored when the -n option is also used)\n"
            "  -i, --interactive            prompt before overwrite (overrides a previous -n\n"
            "                                  option)\n"
            "  -H                           follow command-line symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v10, v9);
    v11 = stdout;
    v12 = dcgettext(
            0,
            "  -l, --link                   hard link files instead of copying\n"
            "  -L, --dereference            always follow symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v12, v11);
    v13 = stdout;
    v14 = dcgettext(
            0,
            "  -n, --no-clobber             do not overwrite an existing file (overrides\n"
            "                                 a previous -i option)\n"
            "  -P, --no-dereference         never follow symbolic links in SOURCE\n",
            5);
    fputs_unlocked(v14, v13);
    v15 = stdout;
    v16 = dcgettext(
            0,
            "  -p                           same as --preserve=mode,ownership,timestamps\n"
            "      --preserve[=ATTR_LIST]   preserve the specified attributes (default:\n"
            "                                 mode,ownership,timestamps), if possible\n"
            "                                 additional attributes: context, links, xattr,\n"
            "                                 all\n",
            5);
    fputs_unlocked(v16, v15);
    v17 = stdout;
    v18 = dcgettext(
            0,
            "      --no-preserve=ATTR_LIST  don't preserve the specified attributes\n"
            "      --parents                use full source file name under DIRECTORY\n",
            5);
    fputs_unlocked(v18, v17);
    v19 = stdout;
    v20 = dcgettext(
            0,
            "  -R, -r, --recursive          copy directories recursively\n"
            "      --reflink[=WHEN]         control clone/CoW copies. See below\n"
            "      --remove-destination     remove each existing destination file before\n"
            "                                 attempting to open it (contrast with --force)\n",
            5);
    fputs_unlocked(v20, v19);
    v21 = stdout;
    v22 = dcgettext(
            0,
            "      --sparse=WHEN            control creation of sparse files. See below\n"
            "      --strip-trailing-slashes  remove any trailing slashes from each SOURCE\n"
            "                                 argument\n",
            5);
    fputs_unlocked(v22, v21);
    v23 = stdout;
    v24 = dcgettext(
            0,
            "  -s, --symbolic-link          make symbolic links instead of copying\n"
            "  -S, --suffix=SUFFIX          override the usual backup suffix\n"
            "  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY\n"
            "  -T, --no-target-directory    treat DEST as a normal file\n",
            5);
    fputs_unlocked(v24, v23);
    v25 = stdout;
    v26 = dcgettext(
            0,
            "  -u, --update                 copy only when the SOURCE file is newer\n"
            "                                 than the destination file or when the\n"
            "                                 destination file is missing\n"
            "  -v, --verbose                explain what is being done\n"
            "  -x, --one-file-system        stay on this file system\n",
            5);
    fputs_unlocked(v26, v25);
    v27 = stdout;
    v28 = dcgettext(
            0,
            "  -Z                           set SELinux security context of destination\n"
            "                                 file to default type\n"
            "      --context[=CTX]          like -Z, or if CTX is specified then set the\n"
            "                                 SELinux or SMACK security context to CTX\n",
            5);
    fputs_unlocked(v28, v27);
    v29 = stdout;
    v30 = dcgettext(0, "      --help        display this help and exit\n", 5);
    fputs_unlocked(v30, v29);
    v31 = stdout;
    v32 = dcgettext(0, "      --version     output version information and exit\n", 5);
    fputs_unlocked(v32, v31);
    v33 = stdout;
    v34 = dcgettext(
            0,
            "\n"
            "By default, sparse SOURCE files are detected by a crude heuristic and the\n"
            "corresponding DEST file is made sparse as well.  That is the behavior\n"
            "selected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\n"
            "file whenever the SOURCE file contains a long enough sequence of zero bytes.\n"
            "Use --sparse=never to inhibit creation of sparse files.\n",
            5);
    fputs_unlocked(v34, v33);
    v35 = stdout;
    v36 = dcgettext(
            0,
            "\n"
            "When --reflink[=always] is specified, perform a lightweight copy, where the\n"
            "data blocks are copied only when modified.  If this is not possible the copy\n"
            "fails, or if --reflink=auto is specified, fall back to a standard copy.\n"
            "Use --reflink=never to ensure a standard copy is performed.\n",
            5);
    fputs_unlocked(v36, v35);
    sub_49A0();
    v37 = stdout;
    v38 = dcgettext(
            0,
            "\n"
            "As a special case, cp makes a backup of SOURCE when the force and backup\n"
            "options are given and SOURCE and DEST are the same name for an existing,\n"
            "regular file.\n",
            5);
    fputs_unlocked(v38, v37);
    sub_55B0();
  }
  exit(status);
}



// Function: do_copy @ 0x5ae0
long long do_copy(int a1, const char **a2, const char *a3, char a4, long long a5)
{
  const char **v5; // r13
  long long v6; // rbx
  unsigned int v7; // r14d
  const char *v8; // r15
  const char *v9; // rdi
  const char *v10; // r12
  size_t v11; // rax
  char **v12; // rcx
  signed long long v13; // rdx
  void *v14; // rsp
  const char *v15; // r12
  int v16; // eax
  char *v17; // r12
  const char *v19; // r14
  const char *v20; // r12
  int *v21; // rdi
  long long v22; // rcx
  int *v23; // rsi
  size_t v24; // rax
  char **v25; // rcx
  signed long long v26; // rdx
  void *v27; // rsp
  void *v28; // r12
  const char *v29; // rdi
  const char *v30; // rax
  const char *v31; // rcx
  bool v32; // zf
  char v33; // al
  long long v34; // rdx
  char v35; // al
  long long *v36; // r15
  void *v37; // rdi
  const char *v38; // rbx
  int v39; // r12d
  char *v40; // rax
  char *v41; // rax
  long long v42; // rax
  const char *v43; // rsi
  long long v44; // r12
  char *v45; // rax
  long long v46; // r13
  char *v47; // r12
  int *v48; // rax
  const char *v49; // rsi
  char *v50; // rax
  long long v51; // rax
  const char *v52; // [rsp+0h] [rbp-1128h]
  char v53[15]; // [rsp+8h] [rbp-1120h] BYREF
  char *v54; // [rsp+1008h] [rbp-120h] BYREF
  long long *v55; // [rsp+1010h] [rbp-118h]
  long long v56; // [rsp+1018h] [rbp-110h]
  const char *v57; // [rsp+1020h] [rbp-108h]
  long long v58; // [rsp+1028h] [rbp-100h]
  int fd; // [rsp+1030h] [rbp-F8h]
  int v60; // [rsp+1034h] [rbp-F4h]
  char v61; // [rsp+1046h] [rbp-E2h] BYREF
  char v62; // [rsp+1047h] [rbp-E1h] BYREF
  void *ptr; // [rsp+1048h] [rbp-E0h] BYREF
  long long v64; // [rsp+1050h] [rbp-D8h] BYREF
  struct stat buf; // [rsp+1058h] [rbp-D0h] BYREF
  unsigned long long v66; // [rsp+10F0h] [rbp-38h]

  v5 = a2;
  v60 = a1;
  v57 = a3;
  v58 = a5;
  v66 = __readfsqword(0x28u);
  v61 = 0;
  if ( (a3 == 0) >= a1 )
  {
    if ( a1 != 1 )
    {
      v49 = "missing file operand";
LABEL_67:
      v50 = dcgettext(0, v49, 5);
      error(0, 0, v50);
LABEL_63:
      sub_57E0(1);
    }
    v51 = sub_14B40(4, *a2);
    v43 = "missing destination file operand after %s";
    v44 = v51;
    goto LABEL_62;
  }
  buf.st_mode = 0;
  if ( a4 )
  {
    if ( v57 )
    {
      v41 = dcgettext(0, "cannot combine --target-directory (-t) and --no-target-directory (-T)", 5);
      a2 = 0;
      error(1, 0, v41);
    }
    else if ( v60 <= 2 )
    {
      goto LABEL_19;
    }
    v42 = sub_14B40(4, a2[2]);
    v43 = "extra operand %s";
    v44 = v42;
LABEL_62:
    v45 = dcgettext(0, v43, 5);
    error(0, 0, v45, v44);
    goto LABEL_63;
  }
  if ( v57 )
  {
    fd = sub_15A20(v57, &buf);
    if ( !(unsigned char)sub_15A10((unsigned int)fd) )
    {
      v46 = sub_14B40(4, v57);
      v47 = dcgettext(0, "target directory %s", 5);
      v48 = __errno_location();
      error(1, *v48, v47, v46);
    }
    if ( v60 == 1 )
      goto LABEL_6;
    goto LABEL_48;
  }
  v38 = a2[v60 - 1];
  v57 = v38;
  fd = sub_15A20(v38, &buf);
  v7 = sub_15A10((unsigned int)fd);
  if ( !(char)v7 )
  {
    v39 = *__errno_location();
    if ( v39 == 2 )
      goto LABEL_57;
    while ( v60 > 2 )
    {
      v5 = (const char **)sub_14B40(4, v57);
      v40 = dcgettext(0, "target %s", 5);
      error(1, v39, v40, v5);
LABEL_57:
      v61 = 1;
    }
    goto LABEL_19;
  }
  --v60;
  if ( !v38 )
  {
LABEL_19:
    v19 = *v5;
    v20 = v5[1];
    if ( !byte_25159 )
    {
      if ( *(char *)(v58 + 22)
        && *(int *)v58
        && !strcmp(*v5, v5[1])
        && !v61
        && (buf.st_mode || !stat(v20, &buf))
        && (buf.st_mode & 0xF000) == 0x8000 )
      {
        sub_DAA0(4294967196LL, v20, *(unsigned int *)v58);
        v21 = &dword_25100;
        v22 = 22;
        v23 = (int *)v58;
        while ( v22 )
        {
          *v21++ = *v23++;
          --v22;
        }
        dword_25100 = 0;
        v58 = (long long)&dword_25100;
      }
      return (unsigned int)sub_BAE0((long long)v19, (long long)&v64, 0);
    }
    v49 = "with --parents, the destination must be a directory";
    goto LABEL_67;
  }
  if ( v60 > 1 )
  {
LABEL_48:
    sub_8330(v58);
    sub_8370(v58);
LABEL_6:
    v6 = 0;
    v7 = 1;
    v55 = &v64;
    v56 = (long long)&v62;
    v54 = &v61;
    while ( 1 )
    {
      v8 = a2[v6];
      v64 = 0;
      if ( byte_25158 )
      {
        sub_E800(v8);
        v9 = v8;
        if ( !byte_25159 )
        {
LABEL_9:
          v10 = (const char *)sub_DB60(v9);
          v11 = strlen(v10);
          v12 = (char **)((char *)&v54 - ((v11 + 24) & 0xFFFFFFFFFFFFF000LL));
          if ( &v54 != v12 )
          {
            while ( v53 != (char *)v12 )
              ;
          }
          v13 = ((short)v11 + 24) & 0xFF0;
          v14 = alloca(v13);
          if ( (((short)v11 + 24) & 0xFF0) != 0 )
            *(long long *)&v53[v13 - 8] = *(long long *)&v53[v13 - 8];
          v15 = (const char *)memcpy(v53, v10, v11 + 1);
          sub_E800(v15);
          v16 = strcmp(v15, "..");
          v17 = (char *)sub_EC90(v57, &v15[v16 == 0], v55);
          goto LABEL_14;
        }
      }
      else
      {
        v9 = v8;
        if ( !byte_25159 )
          goto LABEL_9;
      }
      v24 = strlen(v9);
      v25 = (char **)((char *)&v54 - ((v24 + 24) & 0xFFFFFFFFFFFFF000LL));
      if ( &v54 != v25 )
      {
        while ( v53 != (char *)v25 )
          ;
      }
      v26 = ((short)v24 + 24) & 0xFF0;
      v27 = alloca(v26);
      if ( (((short)v24 + 24) & 0xFF0) != 0 )
        *(long long *)&v53[v26 - 8] = *(long long *)&v53[v26 - 8];
      v28 = memcpy(v53, v8, v24 + 1);
      sub_E800(v28);
      v29 = v57;
      v30 = (const char *)sub_EC90(v57, v28, v55);
      v31 = 0;
      v17 = (char *)v30;
      v32 = *(char *)(v58 + 60) == 0;
      v52 = v29;
      if ( !v32 )
        v31 = "%s -> %s\n";
      v33 = sub_4BF0(v30, v64 - (long long)v30, fd, (long long)v31, (long long *)&ptr, v54, v58);
      if ( *(char *)v64 == 47 )
      {
        v34 = v64 + 1;
        do
          v64 = v34++;
        while ( *(char *)(v34 - 1) == 47 );
      }
      if ( !v33 )
      {
        v35 = byte_25159;
        v7 = 0;
LABEL_43:
        if ( v35 )
        {
          v36 = ptr;
          if ( ptr )
          {
            do
            {
              v37 = v36;
              v36 = (long long *)v36[20];
              ptr = v36;
              free(v37);
            }
            while ( v36 );
          }
        }
        goto LABEL_15;
      }
LABEL_14:
      v7 &= sub_BAE0((long long)v8, v56, 0);
      if ( byte_25159 )
      {
        v7 &= sub_52E0(v17, fd, v64, (long long)ptr, (char *)v58);
        v35 = byte_25159;
        goto LABEL_43;
      }
LABEL_15:
      ++v6;
      free(v17);
      if ( v60 <= (int)v6 )
        return v7;
    }
  }
  if ( v60 == 1 )
    goto LABEL_6;
  return v7;
}



// Function: is_ancestor @ 0x61e0
long long is_ancestor(long long *a1, long long *a2)
{
  if ( !a2 )
    return 0;
  while ( a2[1] != a1[1] || a2[2] != *a1 )
  {
    a2 = (long long *)*a2;
    if ( !a2 )
      return 0;
  }
  return 1;
}



// Function: copy_attr_free @ 0x6220
void copy_attr_free()
{
  ;
}



// Function: owner_failure_ok @ 0x6230
char owner_failure_ok(long long a1)
{
  int *v1; // rax
  char result; // al

  v1 = __errno_location();
  result = *v1 == 22 || *v1 == 1;
  if ( result )
    return *(char *)(a1 + 27) ^ 1;
  return result;
}



// Function: copy_attr_quote @ 0x6260
long long copy_attr_quote(long long a1, long long a2)
{
  return sub_14B40(4, a2);
}



// Function: valid_options @ 0x6270
long long valid_options(long long a1)
{
  int v1; // edx
  unsigned int v2; // eax

  if ( *(int *)a1 > 3u )
    __assert_fail("VALID_BACKUP_TYPE (co->backup_type)", "src/copy.c", 0xC0Fu, "valid_options");
  v1 = *(int *)(a1 + 12);
  if ( (unsigned int)(v1 - 1) > 2 )
    __assert_fail("VALID_SPARSE_MODE (co->sparse_mode)", "src/copy.c", 0xC10u, "valid_options");
  v2 = *(int *)(a1 + 68);
  if ( v2 > 2 )
    __assert_fail("VALID_REFLINK_MODE (co->reflink_mode)", "src/copy.c", 0xC11u, "valid_options");
  if ( *(char *)(a1 + 23) && *(char *)(a1 + 58) )
    __assert_fail("!(co->hard_link && co->symbolic_link)", "src/copy.c", 0xC12u, "valid_options");
  if ( v2 == 2 && v1 != 2 )
    __assert_fail(
      "! (co->reflink_mode == REFLINK_ALWAYS && co->sparse_mode != SPARSE_AUTO)",
      "src/copy.c",
      0xC13u,
      "valid_options");
  return 1;
}



// Function: writable_destination @ 0x6370
bool writable_destination(int fd, char *file, short a3)
{
  if ( (a3 & 0xF000) == 0xA000 )
    return 1;
  if ( (unsigned char)sub_16E20() )
    return 1;
  return faccessat(fd, file, 2, 512) == 0;
}



// Function: overwrite_ok @ 0x63e0
long long overwrite_ok(long long a1, long long a2, int a3, char *a4, long long a5)
{
  int v8; // edx
  long long v9; // r14
  char *v10; // rdx
  FILE *v11; // rdi
  long long v12; // rdx
  long long v14; // r12
  char *v15; // rax
  long long v16; // [rsp-8h] [rbp-50h]
  char v17[12]; // [rsp+Ch] [rbp-3Ch] BYREF
  unsigned long long v18; // [rsp+18h] [rbp-30h]

  v8 = *(int *)(a5 + 24);
  v18 = __readfsqword(0x28u);
  if ( sub_6370(a3, a4, v8) )
  {
    sub_14B40(4, a2);
    v14 = qword_255A0;
    v15 = dcgettext(0, "%s: overwrite %s? ", 5);
    v11 = stderr;
    __fprintf_chk(stderr, 1, v15, v14);
  }
  else
  {
    sub_EB60(*(unsigned int *)(a5 + 24), v17);
    v17[10] = 0;
    sub_14B40(4, a2);
    v9 = qword_255A0;
    if ( *(char *)(a1 + 24) || (*(int *)(a1 + 20) & 0xFFFF00) != 0 )
      v10 = dcgettext(0, "%s: replace %s, overriding mode %04lo (%s)? ", 5);
    else
      v10 = dcgettext(0, "%s: unwritable %s (mode %04lo, %s); try anyway? ", 5);
    v11 = stderr;
    __fprintf_chk(stderr, 1, v10, v9);
    v12 = v16;
  }
  return sub_17540(v11, 1, v12);
}



// Function: abandon_move @ 0x6530
long long abandon_move(long long a1, long long a2, int a3, char *a4, long long a5)
{
  long long result; // rax
  int v7; // edx
  char *v8; // [rsp+0h] [rbp-28h]
  long long v9; // [rsp+8h] [rbp-20h]

  result = *(unsigned char *)(a1 + 24);
  if ( !(char)result )
    __assert_fail("x->move_mode", "src/copy.c", 0x710u, "abandon_move");
  v7 = *(int *)(a1 + 8);
  if ( v7 != 2 )
  {
    if ( v7 == 3 )
      return (unsigned int)sub_63E0(a1, a2, a3, a4, a5) ^ 1;
    if ( v7 == 4 )
    {
      if ( *(char *)(a1 + 61) )
      {
        v9 = a5;
        v8 = a4;
        if ( !sub_6370(a3, a4, *(int *)(a5 + 24)) )
        {
          a4 = v8;
          a5 = v9;
          return (unsigned int)sub_63E0(a1, a2, a3, a4, a5) ^ 1;
        }
      }
    }
    return 0;
  }
  return result;
}



// Function: subst_suffix @ 0x65e0
void *subst_suffix(void *src, long long a2, const char *a3)
{
  size_t v4; // r15
  char *v5; // rbp

  v4 = strlen(a3) + 1;
  v5 = (char *)sub_16EB0(v4 + a2 - (long long)src);
  memcpy(&v5[a2 - (long long)src], a3, v4);
  return memcpy(v5, src, a2 - (long long)src);
}



// Function: source_is_dst_backup @ 0x6640
long long source_is_dst_backup(const char *s1, long long *a2, int a3, char *a4)
{
  unsigned int v4; // r14d
  size_t v6; // r12
  size_t v7; // rax
  const char *v8; // r15
  size_t v9; // r13
  size_t v11; // rax
  char *v12; // rbp
  int v13; // ebx
  const char *s2; // [rsp+8h] [rbp-F0h]
  struct stat buf; // [rsp+20h] [rbp-D8h] BYREF
  unsigned long long v17; // [rsp+B8h] [rbp-40h]

  v4 = 0;
  v17 = __readfsqword(0x28u);
  v6 = strlen(s1);
  s2 = (const char *)sub_DB60(a4);
  v7 = strlen(s2);
  v8 = src;
  v9 = v7;
  if ( v7 + strlen(src) == v6 && !memcmp(s1, s2, v9) && !strcmp(&s1[v9], v8) )
  {
    v11 = strlen(a4);
    v12 = (char *)sub_65E0(a4, (long long)&a4[v11], v8);
    v13 = fstatat(a3, v12, &buf, 0);
    free(v12);
    if ( !v13 && a2[1] == buf.st_ino )
      LOBYTE(v4) = *a2 == buf.st_dev;
  }
  return v4;
}



// Function: clone_file @ 0x6770
int clone_file(int a1, unsigned int a2)
{
  return ioctl(a1, 0x40049409u, a2);
}



// Function: infer_scantype @ 0x6780
long long infer_scantype(int a1, long long a2, __off_t *a3)
{
  unsigned int v3; // r12d
  __off_t v6; // rax
  int v7; // edi

  v3 = 1;
  if ( (*(int *)(a2 + 24) & 0xF000) != 0x8000 )
    return v3;
  if ( *(long long *)(a2 + 64) >= *(long long *)(a2 + 48) / 512LL )
    return v3;
  v3 = 3;
  v6 = lseek(a1, 0, 3);
  *a3 = v6;
  if ( v6 >= 0 )
    return v3;
  v7 = *__errno_location();
  if ( v7 == 6 )
    return v3;
  v3 = 2;
  if ( v7 == 22 )
    return v3;
  else
    return 2 * (unsigned int)(unsigned char)sub_61C0();
}



// Function: write_zeros @ 0x6820
long long write_zeros(int fd, size_t n)
{
  size_t v2; // rbp
  size_t v3; // rbx
  long long v4; // rax

  v2 = n;
  if ( !buf )
  {
    buf = calloc(nmemb, 1u);
    if ( !buf )
    {
      nmemb = 1024;
      buf = &unk_25160;
    }
  }
  if ( !n )
    return 1;
  while ( 1 )
  {
    v3 = nmemb;
    if ( v2 <= nmemb )
      v3 = v2;
    v4 = sub_11320(fd, buf, v3);
    if ( v4 != v3 )
      break;
    v2 -= v4;
    if ( !v2 )
      return 1;
  }
  return 0;
}



// Function: punch_hole @ 0x68d0
long long punch_hole(int a1, __off_t a2, __off_t a3)
{
  int v3; // r12d
  int *v5; // rax
  char v6; // al
  int v7; // edx

  v3 = fallocate(a1, 3, a2, a3);
  if ( v3 < 0 && ((v5 = __errno_location(), v6 = sub_61C0((unsigned int)*v5), v7 == 38) || v6) )
    return 0;
  else
    return (unsigned int)v3;
}



// Function: create_hole @ 0x6910
long long create_hole(int a1, long long a2, char a3, __off_t a4)
{
  __off_t v6; // rax
  long long v8; // rax
  const char *v9; // rsi
  long long v10; // r13
  long long v11; // rax
  char *v12; // r12
  int *v13; // rax

  v6 = lseek(a1, a4, 1);
  if ( v6 < 0 )
  {
    v11 = sub_14B40(4, a2);
    v9 = "cannot lseek %s";
    v10 = v11;
  }
  else
  {
    if ( !a3 || (int)sub_68D0(a1, v6 - a4, a4) >= 0 )
      return 1;
    v8 = sub_14B40(4, a2);
    v9 = "error deallocating %s";
    v10 = v8;
  }
  v12 = dcgettext(0, v9, 5);
  v13 = __errno_location();
  error(0, *v13, v12, v10);
  return 0;
}



// Function: sparse_copy @ 0x6a40
long long sparse_copy(
        unsigned int a1,
        unsigned int a2,
        void **a3,
        size_t a4,
        size_t a5,
        char a6,
        char a7,
        long long a8,
        long long a9,
        unsigned long long a10,
        long long *a11,
        char *a12)
{
  int *v12; // rbp
  unsigned long long v13; // r12
  long long v14; // r8
  long long v15; // rax
  unsigned int v16; // r15d
  size_t v17; // rcx
  size_t v18; // r14
  unsigned int v19; // r13d
  char *v20; // r11
  size_t v21; // rdx
  ssize_t v22; // rax
  void *v23; // r11
  int *v24; // rbx
  long long v25; // r12
  char *v26; // rax
  char *v28; // r10
  size_t v29; // rbx
  unsigned long long v30; // r15
  int v31; // eax
  char v32; // r9
  int v33; // r12d
  long long v34; // rax
  char v35; // r9
  unsigned int v36; // eax
  unsigned int v37; // ecx
  unsigned int v38; // eax
  int *v39; // rdx
  unsigned int v40; // ecx
  long long v41; // rbx
  long long v42; // r12
  char *v43; // rax
  int v44; // eax
  bool v45; // zf
  long long v46; // r12
  char *v47; // rax
  long long v48; // r13
  char *v49; // r12
  int *v50; // rax
  __off_t v51; // r13
  char v52; // r15
  char *v53; // [rsp+8h] [rbp-B0h]
  unsigned long long v54; // [rsp+8h] [rbp-B0h]
  char *v55; // [rsp+10h] [rbp-A8h]
  char v56; // [rsp+10h] [rbp-A8h]
  void *bufa; // [rsp+18h] [rbp-A0h]
  char *buf; // [rsp+18h] [rbp-A0h]
  unsigned long long v59; // [rsp+20h] [rbp-98h]
  ssize_t v63; // [rsp+48h] [rbp-70h]
  size_t v65; // [rsp+68h] [rbp-50h]

  *a12 = 0;
  v59 = a10;
  *a11 = 0;
  if ( a5 || !a7 )
  {
    if ( a10 )
      goto LABEL_12;
    return 1;
  }
  if ( !a10 )
    return 1;
  LODWORD(v12) = a2;
  v13 = a10;
  while ( 1 )
  {
    while ( 1 )
    {
      v14 = 0x7FFFFFFFC0000000LL;
      if ( v13 <= 0x7FFFFFFFC0000000LL )
        v14 = v13;
      v15 = copy_file_range(a1, 0, a2, 0, v14, 0);
      if ( !v15 )
      {
        v59 = v13;
        if ( !*a11 )
          goto LABEL_12;
        return 1;
      }
      if ( v15 < 0 )
        break;
      *a11 += v15;
      v13 -= v15;
      if ( !v13 )
        return 1;
    }
    v37 = *__errno_location();
    if ( v37 == 38 || (v38 = sub_61C0(v37), (char)v38) )
    {
LABEL_66:
      v59 = v13;
      goto LABEL_12;
    }
    if ( v40 <= 0x1A )
    {
      if ( ((1LL << v40) & 0x4440200) != 0 )
        goto LABEL_66;
      if ( v40 == 1 )
        break;
    }
    if ( v40 != 4 )
    {
      v12 = v39;
      v16 = v38;
LABEL_62:
      v41 = sub_14A70(1, 4, a9);
      v42 = sub_14A70(0, 4, a8);
      v43 = dcgettext(0, "error copying %s to %s", 5);
      error(0, *v12, v43, v42, v41);
      return v16;
    }
  }
  v16 = v38;
  v59 = v13;
  v12 = v39;
  if ( *a11 )
    goto LABEL_62;
LABEL_12:
  v17 = a4;
  if ( a5 )
    v17 = a5;
  v18 = 0;
  v19 = 0;
  v65 = v17;
  while ( 1 )
  {
LABEL_15:
    v20 = (char *)*a3;
    if ( !*a3 )
    {
      v44 = getpagesize();
      v20 = (char *)sub_16E60(v44, a4);
      *a3 = v20;
    }
    v21 = v59;
    v53 = v20;
    if ( a4 <= v59 )
      v21 = a4;
    v22 = read(a1, v20, v21);
    v23 = v53;
    v63 = v22;
    if ( v22 >= 0 )
      break;
    v24 = __errno_location();
    if ( *v24 != 4 )
    {
      v16 = 0;
      v25 = sub_14B40(4, a8);
      v26 = dcgettext(0, "error reading %s", 5);
      error(0, *v24, v26, v25);
      return v16;
    }
  }
  if ( !v22 )
  {
    v52 = v19;
    v51 = v18;
    LOBYTE(v12) = v52;
    goto LABEL_72;
  }
  v28 = v53;
  v29 = v65;
  *a11 += v22;
  v30 = v22;
  while ( 2 )
  {
    v54 = v30;
    if ( v29 > v30 )
      v29 = v30;
    LOBYTE(v12) = a5 != 0 && v29 != 0;
    if ( !(char)v12 )
    {
      if ( (char)v19 != 1 && v29 == v30 )
        goto LABEL_40;
      LODWORD(v12) = v19;
      if ( !v29 )
        goto LABEL_40;
LABEL_44:
      v18 += v29;
      if ( v18 + 0x8000000000000000LL < v29 )
      {
        v16 = 0;
        v46 = sub_14B40(4, a8);
        v47 = dcgettext(0, "overflow reading %s", 5);
        error(0, 0, v47, v46);
        return v16;
      }
      v30 -= v29;
      v28 += v29;
      v19 = (unsigned int)v12;
      goto LABEL_46;
    }
    bufa = v23;
    v55 = v28;
    v31 = sub_69E0(v28, v29);
    v28 = v55;
    v23 = bufa;
    LODWORD(v12) = v31;
    v32 = (v18 != 0) & (v19 ^ v31);
    if ( (char)v31 == 1 || v29 != v30 )
    {
      if ( v32 )
      {
        v33 = 0;
LABEL_32:
        buf = v55;
        v56 = (v18 != 0) & (v19 ^ v31);
        if ( (char)v19 )
          goto LABEL_41;
        goto LABEL_33;
      }
      goto LABEL_44;
    }
    if ( v32 )
    {
      v33 = 1;
      LODWORD(v12) = 0;
      goto LABEL_32;
    }
    LODWORD(v12) = 0;
LABEL_40:
    buf = v28;
    v18 += v29;
    v33 = 1;
    v56 = 0;
    if ( (char)v19 )
    {
LABEL_41:
      v36 = sub_6910(a2, a9, a6, v18);
      v35 = v56;
      v28 = buf;
      if ( !(char)v36 )
        return v36;
      goto LABEL_34;
    }
LABEL_33:
    v34 = sub_11320(a2, v23, v18);
    v35 = v56;
    v28 = buf;
    if ( v18 != v34 )
    {
      v16 = v19;
      v48 = sub_14B40(4, a9);
      v49 = dcgettext(0, "error writing %s", 5);
      v50 = __errno_location();
      error(0, *v50, v49, v48);
      return v16;
    }
LABEL_34:
    if ( !v33 )
    {
      v23 = v28;
      v30 -= v29;
      v19 = (unsigned int)v12;
      v28 += v29;
      v18 = v29;
      goto LABEL_46;
    }
    if ( v29 )
    {
      if ( v35 )
      {
        v18 = v29;
        v19 = (unsigned int)v12;
        v23 = v28;
        v29 = 0;
        continue;
      }
LABEL_53:
      v23 = v28;
      v19 = (unsigned int)v12;
      v28 += v29;
      v18 = 0;
      v30 = v54 - v29;
LABEL_46:
      if ( !v30 )
        goto LABEL_64;
      continue;
    }
    break;
  }
  if ( !v35 )
  {
    v54 = 0;
    goto LABEL_53;
  }
  v18 = 0;
LABEL_64:
  v45 = v59 == v63;
  v59 -= v63;
  *a12 = (char)v12;
  if ( !v45 )
  {
    v19 = (unsigned int)v12;
    goto LABEL_15;
  }
  v51 = v18;
LABEL_72:
  if ( !(char)v12 )
    return 1;
  return sub_6910(a2, a9, a6, v51);
}



// Function: fchmod_or_lchmod @ 0x7020
int fchmod_or_lchmod(int a1, unsigned int a2, long long a3, long long a4, long long a5, long long a6)
{
  if ( a1 >= 0 )
    return fchmod(a1, a4);
  else
    return sub_E520(a2, a3, (unsigned int)a4, a4, a2, a6);
}



// Function: copy_attr @ 0x7040
bool copy_attr(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  char v7; // dl
  long long (*v8)(); // r8
  long long (*v9)(int, int, int, int, int, int, char); // r9
  long long (**v10)(int, int, int, int, int, int, char); // r9
  long long (*v12)(int, int, int, int, int, int, char); // [rsp+0h] [rbp-28h] BYREF
  long long (*v13)(long long, long long); // [rsp+8h] [rbp-20h]
  void (*v14)(); // [rsp+10h] [rbp-18h]
  unsigned long long v15; // [rsp+18h] [rbp-10h]

  v15 = __readfsqword(0x28u);
  v7 = *(char *)(a5 + 51);
  if ( !*(char *)(a5 + 49) || *(char *)(a5 + 54) )
  {
    v8 = sub_7330;
    v9 = sub_7260;
    if ( !v7 )
    {
      v8 = sub_7330;
      if ( !*(long long *)(a5 + 40) )
        v8 = 0;
      v9 = sub_7260;
    }
LABEL_4:
    v12 = v9;
    v10 = &v12;
    v13 = sub_6260;
    v14 = sub_6220;
    goto LABEL_5;
  }
  if ( !*(char *)(a5 + 55) )
  {
    v8 = sub_7330;
    v9 = sub_7180;
    if ( !v7 && !*(long long *)(a5 + 40) )
      v8 = 0;
    goto LABEL_4;
  }
  v8 = sub_7330;
  v10 = 0;
  if ( !v7 )
  {
    v10 = *(long long (***)(int, int, int, int, int, int, char))(a5 + 40);
    v8 = 0;
    if ( v10 )
    {
      v8 = sub_7330;
      v10 = 0;
    }
  }
LABEL_5:
  if ( (int)(a4 | a2) < 0 )
    return (unsigned int)attr_copy_file(a1, a3, v8, v10, v8, v10, v12, v13, v14, v15) == 0;
  else
    return (unsigned int)attr_copy_fd(a1, a2, a3, a4, v8, v10, v12, v13, v14, v15) == 0;
}



// Function: copy_attr_error @ 0x7180
unsigned long long copy_attr_error(long long a1, long long a2, ...)
{
  long long v3; // rsi
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v6; // [rsp+18h] [rbp-C0h]

  v6 = __readfsqword(0x28u);
  v3 = (unsigned int)*__errno_location();
  if ( !(unsigned char)sub_61D0(v3) )
  {
    va_start(va, a2);
    ((void (*)(long long, long long, long long, __va_list_tag *))sub_16760)(0, v3, a2, va);
  }
  return v6 - __readfsqword(0x28u);
}



// Function: copy_attr_allerror @ 0x7260
unsigned long long copy_attr_allerror(long long a1, long long a2, ...)
{
  int *v2; // rax
  long long v3; // r8
  long long v4; // r9
  gcc_va_list va; // [rsp+0h] [rbp-D8h] BYREF
  unsigned long long v7; // [rsp+18h] [rbp-C0h]

  va_start(va, a2);
  v7 = __readfsqword(0x28u);
  v2 = __errno_location();
  ((void (*)(long long, long long, long long, __va_list_tag *, long long, long long))sub_16760)(
    0,
    (unsigned int)*v2,
    a2,
    va,
    v3,
    v4);
  return v7 - __readfsqword(0x28u);
}



// Function: check_selinux_attr @ 0x7330
int check_selinux_attr(const char *a1, long long a2)
{
  int result; // eax

  result = strncmp(a1, "security.selinux", 0x10u);
  if ( result )
    return attr_copy_check_permissions(a1, a2) != 0;
  return result;
}



// Function: restore_default_fscreatecon_or_die @ 0x7380
long long restore_default_fscreatecon_or_die(long long a1, long long a2, long long a3, long long a4)
{
  long long result; // rax
  char *v5; // r12
  int *v6; // rax

  result = setfscreatecon(0, a2, a3, a4);
  if ( (int)result )
  {
    v5 = dcgettext(0, "failed to restore the default file creation context", 5);
    v6 = __errno_location();
    error(1, *v6, v5);
    return sub_73C0();
  }
  return result;
}



// Function: emit_verbose @ 0x73c0
int emit_verbose(long long a1, long long a2, long long a3)
{
  const char *v4; // r12
  const char *v5; // rax
  long long v6; // r12
  char *v7; // rax

  v4 = (const char *)sub_14A70(1, 4, a2);
  v5 = (const char *)sub_14A70(0, 4, a1);
  __printf_chk(1, "%s -> %s", v5, v4);
  if ( a3 )
  {
    v6 = sub_14B40(4, a3);
    v7 = dcgettext(0, " (backup: %s)", 5);
    __printf_chk(1, v7, v6);
  }
  return putchar_unlocked(10);
}



// Function: create_hard_link @ 0x7460
long long create_hard_link(
        void *a1,
        unsigned int a2,
        const char *a3,
        void *a4,
        unsigned int a5,
        long long a6,
        unsigned char a7,
        unsigned int a8,
        unsigned char a9)
{
  void *v10; // r14
  unsigned int v13; // r12d
  int v14; // ebp
  void *v16; // r9
  long long v17; // rbx
  long long v18; // r12
  char *v19; // rax
  long long v20; // rcx
  long long v21; // r13
  char *v22; // rax
  void *ptr; // [rsp+10h] [rbp-40h]

  v10 = a1;
  v13 = a8;
  v14 = sub_C0C0(a2, a3, a5, a6, a9 << 10, a7, -1);
  if ( v14 > 0 )
  {
    v16 = 0;
    if ( !a1 )
    {
      v10 = sub_65E0(a4, a6, a3);
      v16 = v10;
    }
    ptr = v16;
    v17 = sub_14A70(1, 4, v10);
    v18 = sub_14A70(0, 4, a4);
    v19 = dcgettext(0, "cannot create hard link %s to %s", 5);
    v20 = v18;
    v13 = 0;
    error(0, v14, v19, v20, v17);
    free(ptr);
  }
  else
  {
    LOBYTE(v13) = (v14 < 0) & a8;
    if ( (char)v13 )
    {
      v21 = sub_14B40(4, a4);
      v22 = dcgettext(0, "removed %s\n", 5);
      __printf_chk(1, v22, v21);
    }
    else
    {
      return 1;
    }
  }
  return v13;
}



// Function: set_process_security_ctx @ 0x7ff0
long long set_process_security_ctx(long long a1, long long a2, long long a3, char a4, long long a5)
{
  unsigned int v6; // r12d
  long long v7; // rdx
  long long v8; // rcx
  int *v9; // r13
  long long v10; // r14
  char *v11; // rax
  long long v13; // rdx
  long long v14; // rcx
  long long v15; // rdx
  long long v16; // rcx
  long long v17; // rdi
  int *v18; // rbx
  long long v19; // r13
  char *v20; // rax
  int v21; // eax
  int *v22; // rbp
  long long v23; // r13
  char *v24; // rax
  long long v25[7]; // [rsp+10h] [rbp-38h] BYREF

  v6 = *(unsigned char *)(a5 + 51);
  v25[1] = __readfsqword(0x28u);
  if ( !(char)v6 )
  {
    LOBYTE(v6) = a4 & (*(long long *)(a5 + 40) != 0);
    if ( !(char)v6 )
      return 1;
    if ( (int)sub_C760() >= 0 )
      return 1;
    v18 = __errno_location();
    if ( (unsigned char)sub_61D0((unsigned int)*v18) )
    {
      return 1;
    }
    else
    {
      v19 = sub_14B40(4, a2);
      v20 = dcgettext(0, "failed to set default file creation context for %s", 5);
      error(0, *v18, v20, v19);
    }
    return v6;
  }
  if ( !*(char *)(a5 + 49) || *(char *)(a5 + 52) )
  {
    if ( (int)sub_158E0(a1, v25) < 0 )
    {
      v9 = __errno_location();
LABEL_5:
      v10 = sub_14B40(4, a1);
      v11 = dcgettext(0, "failed to get security context of %s", 5);
      error(0, *v9, v11, v10);
      goto LABEL_6;
    }
    v21 = setfscreatecon(v25[0], v25, v7, v8);
    v17 = v25[0];
    if ( v21 >= 0 )
    {
LABEL_24:
      freecon(v17);
      return v6;
    }
    v22 = __errno_location();
LABEL_27:
    v23 = sub_14E00(v25[0]);
    v24 = dcgettext(0, "failed to set default file creation context to %s", 5);
    error(0, *v22, v24, v23);
LABEL_18:
    v17 = v25[0];
    if ( *(char *)(a5 + 52) )
    {
      freecon(v25[0]);
      return 0;
    }
    goto LABEL_24;
  }
  if ( *(char *)(a5 + 55) )
  {
    if ( (int)sub_158E0(a1, v25) < 0 )
      goto LABEL_6;
    if ( (int)setfscreatecon(v25[0], v25, v15, v16) < 0 )
      goto LABEL_18;
    goto LABEL_28;
  }
  if ( (int)sub_158E0(a1, v25) >= 0 )
  {
    if ( (int)setfscreatecon(v25[0], v25, v13, v14) < 0 )
    {
      v22 = __errno_location();
      if ( (unsigned char)sub_61D0((unsigned int)*v22) )
        goto LABEL_18;
      goto LABEL_27;
    }
LABEL_28:
    v17 = v25[0];
    goto LABEL_24;
  }
  v9 = __errno_location();
  if ( !(unsigned char)sub_61D0((unsigned int)*v9) )
    goto LABEL_5;
LABEL_6:
  if ( *(char *)(a5 + 52) )
    return 0;
  return v6;
}



// Function: set_file_security_ctx @ 0x8250
long long set_file_security_ctx(long long a1, unsigned char a2, long long a3)
{
  long long v4; // rdi
  unsigned int v5; // r13d
  int *v7; // rbx
  long long v8; // r12
  char *v9; // rax

  v4 = *(long long *)(a3 + 40);
  if ( *(char *)(a3 + 49) && !*(char *)(a3 + 52) )
  {
    if ( *(char *)(a3 + 55) )
      return sub_C960(v4, a1, a2);
    v5 = sub_C960(v4, a1, a2);
    if ( (char)v5 )
      return 1;
    v7 = __errno_location();
    if ( (unsigned char)sub_61D0((unsigned int)*v7) )
      return v5;
  }
  else
  {
    if ( (unsigned char)sub_C960(v4, a1, a2) )
      return 1;
    v7 = __errno_location();
  }
  v8 = sub_14A70(0, 4, a1);
  v9 = dcgettext(0, "failed to set the security context of %s", 5);
  error(0, *v7, v9, v8);
  return 0;
}



// Function: dest_info_init @ 0x8330
long long dest_info_init(long long a1)
{
  long long result; // rax

  result = sub_11CD0(61, 0, sub_12540, sub_12510, sub_125B0);
  *(long long *)(a1 + 72) = result;
  if ( !result )
    sub_17250();
  return result;
}



// Function: src_info_init @ 0x8370
long long src_info_init(long long a1)
{
  long long result; // rax

  result = sub_11CD0(61, 0, sub_124F0, sub_12510, sub_125B0);
  *(long long *)(a1 + 80) = result;
  if ( !result )
    sub_17250();
  return result;
}



// Function: cp_options_default @ 0x83b0
bool cp_options_default(long long a1)
{
  long long v2; // rdi
  __uid_t v3; // eax
  bool result; // al

  v2 = a1 + 8;
  *(long long *)(v2 - 8) = 0;
  *(long long *)(v2 + 72) = 0;
  memset(
    (void *)(v2 & 0xFFFFFFFFFFFFFFF8LL),
    0,
    8 * ((unsigned long long)((unsigned int)a1 - (v2 & 0xFFFFFFF8) + 88) >> 3));
  v3 = geteuid();
  *(int *)(a1 + 64) = -1;
  result = v3 == 0;
  *(char *)(a1 + 27) = result;
  *(char *)(a1 + 26) = result;
  return result;
}



// Function: chown_failure_ok @ 0x8400
char chown_failure_ok(long long a1)
{
  int *v1; // rax
  char result; // al

  v1 = __errno_location();
  result = *v1 == 22 || *v1 == 1;
  if ( result )
    return *(char *)(a1 + 26) ^ 1;
  return result;
}



// Function: cached_umask @ 0x8640
long long cached_umask()
{
  __mode_t v0; // r12d

  v0 = dword_25010;
  if ( dword_25010 == -1 )
  {
    v0 = umask(0);
    dword_25010 = v0;
    umask(v0);
  }
  return v0;
}



// Function: copy_reg @ 0x8680
long long copy_reg(
        long long a1,
        const char *a2,
        unsigned int a3,
        const char *a4,
        long long a5,
        unsigned int a6,
        int a7,
        char *a8,
        long long a9)
{
  unsigned int v10; // eax
  bool v11; // zf
  int v12; // eax
  unsigned int v13; // ebp
  long long v14; // r13
  char *v15; // rax
  long long v16; // rcx
  int v17; // esi
  const char *v18; // rdx
  void *v19; // r13
  unsigned int v20; // r14d
  long long v22; // r13
  char *v23; // r12
  int *v24; // rax
  int v25; // ecx
  int v26; // r8d
  int v27; // r9d
  int v28; // r10d
  unsigned int v29; // eax
  int v30; // r14d
  int v31; // eax
  long long v32; // r8
  long long v33; // r9
  long long v34; // rcx
  int v35; // eax
  long long v36; // rcx
  long long v37; // rax
  int v38; // eax
  unsigned int v39; // r10d
  int v40; // r15d
  long long v41; // rax
  unsigned long long v42; // rax
  __off_t v43; // r11
  unsigned int v44; // r10d
  unsigned long long v45; // rcx
  signed long long v46; // r11
  char v47; // al
  char v48; // r9
  size_t v49; // r8
  char v50; // al
  char v51; // al
  long long v52; // rax
  long long v53; // rdx
  int v54; // eax
  unsigned int v55; // edx
  unsigned int v56; // r13d
  long long v57; // r14
  char *v58; // r12
  int *v59; // rax
  long long v60; // rcx
  long long v61; // r14
  char *v62; // r13
  int *v63; // rax
  long long v64; // r14
  char *v65; // rax
  int v66; // r9d
  unsigned int v67; // eax
  int v68; // r9d
  unsigned int v69; // eax
  long long v70; // r13
  char *v71; // rax
  long long v72; // rdx
  long long v73; // rax
  long long v74; // rdx
  int v75; // eax
  long long v76; // r14
  char *v77; // rax
  long long v78; // r13
  char *v79; // rax
  int v80; // eax
  bool v81; // al
  int v82; // eax
  long long v83; // rbx
  long long v84; // r13
  char *v85; // rax
  int v86; // eax
  int v87; // eax
  ssize_t v88; // rax
  unsigned int v89; // eax
  const char *v90; // rsi
  long long v91; // r13
  char *v92; // rax
  int v93; // edx
  char v94; // al
  int v95; // eax
  long long v96; // r13
  char *v97; // rax
  int v98; // eax
  int v99; // eax
  long long v100; // r8
  long long v101; // r9
  int v102; // eax
  long long v103; // r13
  char *v104; // rax
  long long v105; // r13
  char *v106; // rax
  long long v107; // r13
  char *v108; // rax
  long long v109; // rcx
  short v110; // ax
  int v111; // eax
  int v113; // [rsp+10h] [rbp-1E8h]
  int v114; // [rsp+10h] [rbp-1E8h]
  int v115; // [rsp+10h] [rbp-1E8h]
  int v116; // [rsp+10h] [rbp-1E8h]
  int v117; // [rsp+10h] [rbp-1E8h]
  unsigned int v118; // [rsp+10h] [rbp-1E8h]
  char v120; // [rsp+20h] [rbp-1D8h]
  int v121[2]; // [rsp+20h] [rbp-1D8h]
  unsigned int v122; // [rsp+20h] [rbp-1D8h]
  int v123; // [rsp+20h] [rbp-1D8h]
  unsigned int v124; // [rsp+20h] [rbp-1D8h]
  int v125; // [rsp+20h] [rbp-1D8h]
  char v128; // [rsp+30h] [rbp-1C8h]
  int v129; // [rsp+30h] [rbp-1C8h]
  unsigned int v130; // [rsp+34h] [rbp-1C4h]
  int *v131; // [rsp+38h] [rbp-1C0h]
  int fde; // [rsp+40h] [rbp-1B8h]
  int fd; // [rsp+40h] [rbp-1B8h]
  unsigned long long fda; // [rsp+40h] [rbp-1B8h]
  int fdf; // [rsp+40h] [rbp-1B8h]
  int fdb; // [rsp+40h] [rbp-1B8h]
  unsigned int fdc; // [rsp+40h] [rbp-1B8h]
  int fdd; // [rsp+40h] [rbp-1B8h]
  unsigned int fdg; // [rsp+40h] [rbp-1B8h]
  unsigned int v140; // [rsp+48h] [rbp-1B0h]
  int v141[2]; // [rsp+48h] [rbp-1B0h]
  char v142; // [rsp+57h] [rbp-1A1h] BYREF
  void *v143; // [rsp+58h] [rbp-1A0h] BYREF
  __off_t v144; // [rsp+60h] [rbp-198h] BYREF
  __off_t length; // [rsp+68h] [rbp-190h] BYREF
  long long v146[4]; // [rsp+70h] [rbp-188h] BYREF
  struct stat v147; // [rsp+90h] [rbp-168h] BYREF
  struct stat buf; // [rsp+120h] [rbp-D8h] BYREF
  char v149; // [rsp+1B7h] [rbp-41h] BYREF
  unsigned long long v150; // [rsp+1B8h] [rbp-40h]

  v150 = __readfsqword(0x28u);
  v10 = *(int *)(a9 + 24);
  v11 = *(int *)(a5 + 4) == 2;
  v143 = 0;
  v130 = v10;
  v120 = *(char *)(a5 + 49);
  v128 = *(char *)(a5 + 53);
  v12 = sub_E880(a1, (unsigned char)v11 << 17, a3, (int)a4, a5, a6);
  if ( v12 < 0 )
  {
    v20 = 0;
    v22 = sub_14B40(4, a1);
    v23 = dcgettext(0, "cannot open %s for reading", 5);
    v24 = __errno_location();
    error(0, *v24, v23, v22);
    return v20;
  }
  v13 = v12;
  if ( fstat(v12, &buf) )
  {
    v61 = sub_14B40(4, a1);
    v62 = dcgettext(0, "cannot fstat %s", 5);
    v63 = __errno_location();
    v16 = v61;
    v18 = v62;
    v17 = *v63;
    goto LABEL_6;
  }
  if ( *(long long *)(a9 + 8) != buf.st_ino || *(long long *)a9 != buf.st_dev )
  {
    v14 = sub_14B40(4, a1);
    v15 = dcgettext(0, "skipping file %s, as it was replaced while being copied", 5);
    v16 = v14;
    v17 = 0;
    v18 = v15;
LABEL_6:
    v19 = 0;
    v20 = 0;
    error(0, v17, v18, v16);
    goto LABEL_7;
  }
  v20 = (unsigned char)*a8;
  v131 = __errno_location();
  if ( (char)v20 )
    goto LABEL_62;
  v28 = sub_12710(a3, (int)a4, v120 == 0 ? 1 : 513, v25, v26, v27);
  v26 = *v131;
  if ( v28 < 0 )
  {
    if ( v26 != 2 )
    {
      if ( !*(char *)(a5 + 22) )
      {
        if ( !*a8 )
          goto LABEL_68;
        goto LABEL_62;
      }
      if ( unlinkat(a3, a4, 0) )
      {
        if ( *v131 != 2 )
        {
          v107 = sub_14B40(4, a2);
          v108 = dcgettext(0, "cannot remove %s", 5);
          v109 = v107;
          v19 = 0;
          error(0, *v131, v108, v109);
          goto LABEL_7;
        }
      }
      else if ( *(char *)(a5 + 60) )
      {
        v64 = sub_14B40(4, a2);
        v65 = dcgettext(0, "removed %s\n", 5);
        __printf_chk(1, v65, v64);
      }
    }
    if ( *(long long *)(a5 + 40) )
    {
      v20 = sub_7FF0(a1, (long long)a2, a6, 1, a5);
      if ( !(char)v20 )
        goto LABEL_114;
    }
    *a8 = 1;
LABEL_62:
    v66 = a6 & ~a7;
    if ( v128 && !*(char *)(a5 + 27) )
      LOBYTE(v66) = v66 | 0x80;
    fdb = v66;
    v67 = sub_12710(a3, (int)a4, 193, v66, v26, v66);
    v68 = fdb;
    v28 = v67;
    v26 = *v131;
    v69 = v67 >> 31;
    if ( *v131 != 17 || !(char)v69 )
    {
      LOBYTE(v69) = (v26 == 21) & v69;
      goto LABEL_72;
    }
    v26 = 17;
    if ( *(char *)(a5 + 24) )
      goto LABEL_68;
    v88 = readlinkat(a3, a4, &v149, 1u);
    v26 = 17;
    if ( v88 < 0 )
      goto LABEL_68;
    v20 = *(unsigned char *)(a5 + 62);
    if ( (char)v20 )
    {
      v89 = sub_12710(a3, (int)a4, 65, fdb, 17, fdb);
      v68 = fdb;
      v28 = v89;
      v26 = *v131;
      v69 = (v89 >> 31) & (*v131 == 21);
LABEL_72:
      if ( (char)v69 )
      {
        v26 = 21;
        if ( *a2 )
          v26 = (a2[strlen(a2) - 1] != 47) + 20;
      }
      else
      {
        v30 = v68 & ~a6;
        if ( v28 >= 0 )
          goto LABEL_18;
      }
LABEL_68:
      v113 = v26;
      v70 = sub_14B40(4, a2);
      v71 = dcgettext(0, "cannot create regular file %s", 5);
      v17 = v113;
      v16 = v70;
      v18 = v71;
      goto LABEL_6;
    }
    v96 = sub_14B40(4, a2);
    v97 = dcgettext(0, "not writing through dangling symlink %s", 5);
    error(0, 0, v97, v96);
LABEL_114:
    v19 = 0;
    goto LABEL_7;
  }
  if ( *(long long *)(a5 + 40) || *(char *)(a5 + 51) )
  {
    fde = v28;
    v29 = sub_8250((long long)a2, 0, a5);
    v28 = fde;
    v20 = v29;
    if ( !(char)v29 )
    {
      if ( *(char *)(a5 + 52) )
        goto LABEL_52;
    }
  }
  if ( *a8 )
    goto LABEL_62;
  a7 = 0;
  v30 = 0;
LABEL_18:
  if ( v120 )
  {
    if ( !*(int *)(a5 + 68) )
      goto LABEL_20;
    fdd = v28;
    v82 = sub_6770(v28, v13);
    v28 = fdd;
    if ( v82 )
    {
      if ( *(int *)(a5 + 68) == 2 )
      {
        v20 = 0;
        v83 = sub_14A70(1, 4, a1);
        v84 = sub_14A70(0, 4, a2);
        v85 = dcgettext(0, "failed to clone %s from %s", 5);
        error(0, *v131, v85, v84, v83);
        v28 = fdd;
        goto LABEL_52;
      }
LABEL_20:
      fd = v28;
      v31 = fstat(v28, &v147);
      v28 = fd;
      if ( v31 )
      {
        v20 = 0;
        v105 = sub_14B40(4, a2);
        v106 = dcgettext(0, "cannot fstat %s", 5);
        error(0, *v131, v106, v105);
        v28 = fd;
        goto LABEL_52;
      }
      v34 = v30 | v147.st_mode;
      if ( v147.st_mode != (int)v34 )
      {
        v35 = sub_7020(fd, a3, (long long)a4, v34, v32, v33);
        v28 = fd;
        if ( v35 )
          v30 = 0;
      }
      if ( !v120 )
        goto LABEL_38;
      v140 = v28;
      fda = sub_7FD0(v147.st_blksize, &buf);
      v37 = 512;
      if ( (unsigned long long)(v36 - 1) <= 0x1FFFFFFFFFFFFFFFLL )
        v37 = v36;
      *(long long *)v121 = v37;
      v38 = sub_6780(v13, (long long)&buf, &v144);
      v39 = v140;
      v40 = v38;
      if ( !v38 )
      {
        v116 = v140;
        v90 = "cannot lseek %s";
        v91 = sub_14B40(4, a1);
LABEL_108:
        v92 = dcgettext(0, v90, 5);
        error(0, *v131, v92, v91);
        v28 = v116;
        goto LABEL_79;
      }
      if ( (v147.st_mode & 0xF000) == 0x8000 && ((v87 = *(int *)(a5 + 12), v87 == 3) || v87 == 2 && v40 != 1) )
      {
        *(long long *)v141 = fda;
        fdg = v39;
        j__posix_fadvise(v13, 0, 0, 2);
        v142 = 0;
        v44 = fdg;
        v46 = *(long long *)v141;
        if ( v40 != 3 )
        {
          v47 = *(int *)(a5 + 68) != 0;
          v48 = *(int *)(a5 + 12) == 3;
          goto LABEL_35;
        }
        v93 = *(int *)(a5 + 12);
        v47 = *(int *)(a5 + 68) != 0;
      }
      else
      {
        j__posix_fadvise(v13, 0, 0, 2);
        v41 = sub_7FD0(buf.st_blksize, 0);
        v42 = sub_DBF0(v41, fda, 0x7FFFFFFFFFFFFFFFLL);
        v43 = fda;
        v44 = v140;
        v45 = v42;
        if ( (buf.st_mode & 0xF000) == 0x8000 && buf.st_size < fda )
          v43 = buf.st_size + 1;
        v46 = v43 + v42 - 1 - (v43 + v42 - 1) % v42;
        v47 = *(int *)(a5 + 68) != 0;
        if ( v46 > 0 )
        {
          v142 = 0;
          v93 = 1;
          if ( v40 != 3 )
          {
            *(long long *)v121 = 0;
            v48 = *(int *)(a5 + 12) == 3;
            goto LABEL_35;
          }
        }
        else
        {
          v142 = 0;
          if ( v40 != 3 )
          {
            *(long long *)v121 = 0;
            v46 = v45;
            v48 = *(int *)(a5 + 12) == 3;
LABEL_35:
            v49 = *(long long *)v121;
            v122 = v44;
            v50 = sub_6A40(v13, v44, &v143, v46, v49, v48, v47, a1, (long long)a2, 0xFFFFFFFFFFFFFFFFLL, &length, &v142);
            v28 = v122;
            v51 = v50 ^ 1;
            goto LABEL_36;
          }
          v93 = 1;
        }
      }
      v124 = v44;
      v94 = sub_7B50(v13, v44, buf.st_size, v93, v47, a1, (long long)a2);
      v28 = v124;
      v51 = v94 ^ 1;
LABEL_36:
      if ( v51 )
      {
LABEL_79:
        v20 = 0;
        goto LABEL_52;
      }
      if ( v142 )
      {
        v125 = v28;
        v95 = ftruncate(v28, length);
        v28 = v125;
        if ( v95 < 0 )
        {
          v116 = v125;
          v90 = "failed to extend %s";
          v91 = sub_14B40(4, a2);
          goto LABEL_108;
        }
      }
LABEL_38:
      v123 = v30;
      if ( !*(char *)(a5 + 31) )
        goto LABEL_39;
      goto LABEL_77;
    }
  }
  v123 = v30 | *(unsigned char *)(a5 + 29);
  if ( v123 )
  {
    v120 = 0;
    goto LABEL_20;
  }
  v147.st_mode = 0;
  if ( !*(char *)(a5 + 31) )
    goto LABEL_45;
LABEL_77:
  fdc = v28;
  v146[0] = sub_15960(a9);
  v146[1] = v72;
  v73 = sub_15980(a9);
  v146[3] = v74;
  v146[2] = v73;
  v75 = sub_E900(fdc, a3, a4, v146, 0);
  v28 = fdc;
  if ( v75 )
  {
    v76 = sub_14B40(4, a2);
    v77 = dcgettext(0, "preserving times for %s", 5);
    error(0, *v131, v77, v76);
    v28 = fdc;
    if ( *(char *)(a5 + 50) )
      goto LABEL_79;
  }
LABEL_39:
  if ( !*(char *)(a5 + 29)
    || (v52 = *(unsigned int *)(a9 + 28),
        v53 = *(unsigned int *)(a9 + 32),
        *(long long *)&v147.st_uid == __PAIR64__(v53, v52)) )
  {
LABEL_45:
    if ( !v128 || (v129 = v28, v81 = sub_7040(a1, v13, (long long)a2, (unsigned int)v28, a5), v28 = v129, v81) )
      v20 = 1;
    else
      v20 = *(unsigned char *)(a5 + 54) ^ 1;
    if ( (*(long long *)(a5 + 24) & 0xFF0000000000FFLL) != 0 )
    {
      v114 = v28;
      v80 = sub_CAA0(a1, v13, a2, (unsigned int)v28, v130);
      v28 = v114;
      if ( v80 && *(char *)(a5 + 50) )
        v20 = 0;
    }
    else
    {
      v56 = *(unsigned char *)(a5 + 57);
      if ( (char)v56 )
      {
        v115 = v28;
        v86 = sub_CB50(a2, (unsigned int)v28, *(unsigned int *)(a5 + 16));
        v28 = v115;
        if ( v86 )
          v20 = 0;
      }
      else if ( *(char *)(a5 + 32) && *a8 )
      {
        v118 = v28;
        v110 = sub_8640();
        v111 = sub_CB50(a2, v118, ~v110 & 0x1B6);
        v28 = v118;
        if ( v111 )
          v20 = v56;
      }
      else if ( a7 | v123 )
      {
        v117 = v28;
        v98 = sub_8640();
        v28 = v117;
        if ( v123 | a7 & ~v98 )
        {
          v99 = sub_8640();
          v102 = sub_7020(v117, a3, (long long)a4, ~v99 & a6, v100, v101);
          v28 = v117;
          if ( v102 )
          {
            v103 = sub_14B40(4, a2);
            v104 = dcgettext(0, "preserving permissions for %s", 5);
            error(0, *v131, v104, v103);
            v28 = v117;
            if ( *(char *)(a5 + 50) )
              v20 = 0;
          }
        }
      }
    }
    goto LABEL_52;
  }
  v20 = 0;
  fdf = v28;
  v54 = sub_8430(a5, a2, a3, a4, (unsigned int)v28, *(unsigned int *)(a9 + 24), v52, v53, (unsigned char)*a8, &v147);
  v28 = fdf;
  if ( v54 != -1 )
  {
    v55 = v130;
    BYTE1(v55) = BYTE1(v130) & 0xF1;
    if ( v54 )
      v55 = v130;
    v130 = v55;
    goto LABEL_45;
  }
LABEL_52:
  if ( close(v28) >= 0 )
  {
    v19 = v143;
    if ( close(v13) >= 0 )
      goto LABEL_8;
LABEL_54:
    v57 = sub_14B40(4, a1);
    v58 = dcgettext(0, "failed to close %s", 5);
    v59 = __errno_location();
    v60 = v57;
    v20 = 0;
    error(0, *v59, v58, v60);
    goto LABEL_8;
  }
  v20 = 0;
  v78 = sub_14B40(4, a2);
  v79 = dcgettext(0, "failed to close %s", 5);
  error(0, *v131, v79, v78);
  v19 = v143;
LABEL_7:
  if ( close(v13) < 0 )
    goto LABEL_54;
LABEL_8:
  j_free(v19);
  return v20;
}



// Function: copy_internal @ 0x9470
long long copy_internal(
        char *a1,
        void *a2,
        int a3,
        char *a4,
        int a5,
        long long *a6,
        void *a7,
        unsigned int *a8,
        int a9,
        char *a10,
        char *a11,
        long long a12)
{
  void *v12; // r15
  int v14; // r14d
  int v15; // r12d
  int v16; // edi
  char *v17; // rsi
  int v18; // eax
  void *v19; // rsi
  long long v20; // rdi
  short *v21; // r9
  char v22; // r11
  struct stat *v23; // rdx
  unsigned int v24; // eax
  char v25; // al
  int v26; // r12d
  int *v27; // r15
  int v28; // r12d
  unsigned char v29; // r14
  int v30; // eax
  char v31; // r11
  unsigned int v32; // eax
  int v33; // eax
  long long v34; // r8
  int v35; // eax
  const char *v36; // rdx
  long long v37; // rdx
  long long v38; // rcx
  bool v40; // zf
  char v41; // al
  short v42; // r15
  const char *v43; // rax
  char *v44; // r12
  int v45; // eax
  long long v46; // rcx
  long long v47; // r8
  char *v48; // r9
  unsigned int v49; // r12d
  int v50; // eax
  unsigned int v51; // r12d
  int v52; // eax
  __ino_t v53; // rdx
  __nlink_t v54; // rax
  long long v55; // rdx
  char *v56; // r12
  int *v57; // rax
  int v58; // eax
  long long v59; // r13
  char *v60; // r12
  int *v61; // rax
  unsigned int v62; // edx
  int v63; // eax
  char v64; // r10
  int v65; // eax
  char *v66; // rax
  long long v67; // rcx
  long long v68; // r8
  long long v69; // r9
  int v70; // eax
  int *v71; // r12
  long long v72; // r13
  char *v73; // rax
  long long v74; // r12
  char *v75; // rdx
  long long v76; // rax
  const char *v77; // rsi
  long long v78; // r13
  char *v79; // r12
  int *v80; // rax
  int v81; // eax
  const char *v82; // rsi
  long long v83; // r13
  char *v84; // r12
  void *v85; // rdi
  long long v86; // rsi
  long long v87; // rdx
  long long v88; // r13
  long long v89; // r12
  char *v90; // rax
  unsigned int v91; // eax
  short st_mode; // r12
  int v93; // ecx
  int v94; // edx
  long long v95; // rdi
  char v96; // al
  int v97; // eax
  int *v98; // rax
  int *v99; // r12
  long long v100; // rbx
  long long v101; // r12
  char *v102; // rax
  unsigned char v103; // al
  char v104; // al
  unsigned int v105; // eax
  char *v106; // rax
  int v107; // eax
  char *v108; // rax
  long long v109; // rbx
  long long v110; // r12
  char *v111; // rax
  long long v112; // rbx
  const char *v113; // rsi
  long long v114; // r12
  char *v115; // rax
  char *v116; // rax
  long long v117; // rbx
  long long v118; // r12
  char *v119; // rax
  long long v120; // r15
  long long v121; // r13
  char *v122; // rax
  const char *v123; // rsi
  long long v124; // r12
  char *v125; // rax
  const char *v126; // rdi
  char v127; // al
  int v128; // edx
  char v129; // al
  long long v130; // rdx
  const char *v131; // rax
  char *v132; // r12
  size_t v133; // r14
  size_t v134; // rax
  char v135; // r11
  signed long long v136; // rcx
  char *v137; // rsi
  short v138; // cx
  signed long long v139; // rcx
  void *v140; // rsp
  void *v141; // rax
  char v142; // al
  const char *v143; // rsi
  char *v144; // rdx
  long long v145; // r13
  long long v146; // r12
  char *v147; // rax
  char v148; // al
  long long v149; // rdx
  long long v150; // rax
  char *v151; // r12
  long long v152; // rsi
  long long v153; // r13
  char *v154; // rax
  char v155; // al
  void *v156; // r14
  void *v157; // r12
  long long v158; // r15
  long long v159; // r13
  char *v160; // rax
  __mode_t v161; // r12d
  __mode_t v162; // edx
  int v163; // ebx
  long long v164; // r12
  char *v165; // rax
  char *v166; // r12
  long long v167; // rbx
  long long v168; // rax
  long long v169; // r12
  char *v170; // rax
  long long v171; // rdi
  long long v172; // rdx
  unsigned char v173; // al
  const char *v174; // rsi
  long long v175; // r13
  char *v176; // rax
  void *v177; // rax
  void *v178; // rax
  char *v179; // r12
  unsigned char v180; // al
  int *v181; // rax
  void *v182; // [rsp-Eh] [rbp-1320h] BYREF
  long long v183; // [rsp-6h] [rbp-1318h]
  long long v184; // [rsp+2h] [rbp-1310h]
  char v186[15]; // [rsp+12h] [rbp-1300h] BYREF
  char v187[4]; // [rsp+1012h] [rbp-300h] BYREF
  __mode_t v188; // [rsp+1016h] [rbp-2FCh]
  const char *v189; // [rsp+101Ah] [rbp-2F8h]
  int v190; // [rsp+1022h] [rbp-2F0h]
  unsigned char v191; // [rsp+1029h] [rbp-2E9h]
  long long *v192; // [rsp+102Ah] [rbp-2E8h]
  char *v193; // [rsp+1032h] [rbp-2E0h]
  char *v194; // [rsp+103Ah] [rbp-2D8h]
  void **v195; // [rsp+1042h] [rbp-2D0h]
  void *ptr; // [rsp+104Ah] [rbp-2C8h]
  short *v197; // [rsp+1052h] [rbp-2C0h]
  int errnum[2]; // [rsp+105Ah] [rbp-2B8h]
  long long v199; // [rsp+1062h] [rbp-2B0h]
  void *src; // [rsp+106Ah] [rbp-2A8h]
  char *name; // [rsp+1072h] [rbp-2A0h]
  int fd; // [rsp+107Ah] [rbp-298h]
  unsigned int v203; // [rsp+107Eh] [rbp-294h]
  short v204[73]; // [rsp+1090h] [rbp-282h] BYREF
  struct stat v205; // [rsp+1122h] [rbp-1F0h] BYREF
  struct stat v206; // [rsp+11B2h] [rbp-160h] BYREF
  struct stat v207; // [rsp+1242h] [rbp-D0h] BYREF
  unsigned long long v208; // [rsp+12DAh] [rbp-38h]

  src = a2;
  fd = a3;
  name = a4;
  ptr = a7;
  v192 = a6;
  v194 = a10;
  LODWORD(v199) = a9;
  v193 = a11;
  v14 = a8[16];
  *(long long *)errnum = a12;
  LOBYTE(v195) = a9;
  v208 = __readfsqword(0x28u);
  *a11 = 0;
  LOBYTE(v204[0]) = a5 > 0;
  if ( *((char *)a8 + 24) )
  {
    if ( v14 < 0 )
    {
      if ( !(unsigned int)sub_14E40(-100, a1, fd, name) )
      {
        LOBYTE(v204[0]) = 1;
        v15 = 1;
        **(char **)errnum = 1;
        goto LABEL_4;
      }
      v14 = *__errno_location();
    }
    LOBYTE(v204[0]) = v14 == 0;
    **(char **)errnum = v14 == 0;
    v15 = v14 == 0;
    if ( !v14 )
    {
LABEL_4:
      if ( *((char *)a8 + 63) )
      {
        v14 = 0;
        goto LABEL_11;
      }
      v16 = fd;
      v12 = src;
      v14 = 0;
      v17 = name;
LABEL_9:
      v18 = j__fstatat(v16, v17, (struct stat *)&v204[1], (a8[1] == 2) << 8);
      v19 = v12;
      if ( v18 )
      {
LABEL_189:
        v76 = sub_14B40(4, v19);
        v77 = "cannot stat %s";
        v78 = v76;
        goto LABEL_190;
      }
      v203 = *(int *)&v204[13];
      if ( (v204[13] & 0xF000) == 0x4000 )
      {
        LODWORD(v12) = *((unsigned char *)a8 + 56);
        if ( !(char)v12 )
        {
          v74 = sub_14B40(4, a1);
          if ( !*((char *)a8 + 25) )
          {
            v75 = dcgettext(0, "-r not specified; omitting directory %s", 5);
            error(0, 0, v75, v74);
            return (unsigned int)v12;
          }
          v143 = "omitting directory %s";
LABEL_305:
          v144 = dcgettext(0, v143, 5);
          error(0, 0, v144, v74);
          return (unsigned int)v12;
        }
      }
LABEL_11:
      if ( (char)v199 )
        goto LABEL_12;
      goto LABEL_39;
    }
  }
  else
  {
    v15 = a5;
    if ( !v14 )
      goto LABEL_4;
  }
  if ( v14 != 17 || a8[2] != 2 )
  {
    v12 = a1;
    v17 = a1;
    v16 = -100;
    goto LABEL_9;
  }
  if ( (char)v199 )
  {
LABEL_12:
    v20 = *((long long *)a8 + 10);
    if ( v20 )
    {
      v21 = &v204[1];
      if ( (v203 & 0xF000) != 0x4000 && !*a8 )
      {
        v197 = &v204[1];
        LODWORD(v12) = sub_EA90(v20, a1, &v204[1]);
        if ( (char)v12 )
        {
          v143 = "warning: source file %s specified more than once";
          v74 = sub_14B40(4, a1);
          goto LABEL_305;
        }
        v20 = *((long long *)a8 + 10);
        v21 = v197;
      }
      sub_EA00(v20, a1, v21);
    }
    v191 = sub_7FB0(a8[1], 1);
    if ( v15 > 0 )
    {
      v197 = 0;
      v22 = 0;
      goto LABEL_18;
    }
    goto LABEL_55;
  }
LABEL_39:
  v191 = sub_7FB0(a8[1], 0);
  if ( v15 > 0 )
  {
    v197 = 0;
    goto LABEL_23;
  }
LABEL_55:
  if ( v14 == 17 && a8[2] == 2 )
  {
    HIBYTE(v204[0]) = 0;
    v22 = 0;
    goto LABEL_63;
  }
  v30 = v203 & 0xF000;
  if ( v30 != 0x8000 )
  {
    v31 = *((char *)a8 + 20) ^ 1 | (v30 == 0x4000 || v30 == 40960);
    if ( v31 )
      goto LABEL_58;
  }
  v31 = *((char *)a8 + 24);
  if ( v31 )
    goto LABEL_58;
  v31 = *((char *)a8 + 58);
  if ( v31 )
    goto LABEL_58;
  v31 = *((char *)a8 + 23);
  if ( v31 )
    goto LABEL_58;
  if ( *a8 )
  {
    v31 = 1;
    v15 = 256;
    goto LABEL_59;
  }
  v31 = *((char *)a8 + 21);
  if ( v31 )
  {
LABEL_58:
    v15 = 256;
    goto LABEL_59;
  }
  if ( v15 )
    goto LABEL_178;
LABEL_59:
  v12 = &v205;
  LOBYTE(v197) = v31;
  if ( j__fstatat(fd, name, &v205, v15) )
  {
    v71 = __errno_location();
    if ( *v71 == 40 )
    {
      if ( *((char *)a8 + 22) )
        goto LABEL_179;
    }
    else if ( *v71 == 2 )
    {
LABEL_178:
      LOBYTE(v204[0]) = 1;
LABEL_179:
      v197 = 0;
      v22 = 0;
      if ( v14 != 17 )
        goto LABEL_180;
      v32 = a8[2];
      goto LABEL_61;
    }
    LODWORD(v12) = 0;
    v72 = sub_14B40(4, src);
    v73 = dcgettext(0, "cannot stat %s", 5);
    error(0, *v71, v73, v72);
    return (unsigned int)v12;
  }
  v32 = a8[2];
  v22 = (char)v197;
LABEL_61:
  HIBYTE(v204[0]) = 0;
  if ( v32 != 2 )
  {
    LOBYTE(v197) = v22;
    v33 = sub_75B0(a1, (long long)v204 + 1);
    v22 = (char)v197;
    LODWORD(v12) = v33;
    if ( !(char)v33 )
    {
      v112 = sub_14A70(1, 4, src);
      v113 = "%s and %s are the same file";
      v114 = sub_14A70(0, 4, a1);
      goto LABEL_269;
    }
  }
LABEL_63:
  if ( !*((char *)a8 + 59) )
  {
    if ( *((char *)a8 + 24) )
      goto LABEL_258;
    if ( (v203 & 0xF000) == 0x4000 )
      goto LABEL_210;
LABEL_208:
    v91 = a8[2];
    if ( v91 == 2 )
      goto LABEL_86;
    if ( v91 == 3 )
    {
      LOBYTE(v197) = v22;
      v155 = sub_63E0((long long)a8, (long long)src, fd, name, (long long)&v205);
      v22 = (char)v197;
      if ( !v155 )
        goto LABEL_86;
    }
    goto LABEL_210;
  }
  if ( (v203 & 0xF000) == 0x4000 )
  {
    if ( !*((char *)a8 + 24) )
      goto LABEL_210;
    goto LABEL_258;
  }
  v34 = 0;
  if ( *((char *)a8 + 31) )
  {
    v34 = 1;
    if ( *((char *)a8 + 24) )
      v34 = v205.st_dev != *(long long *)&v204[1];
  }
  LOBYTE(v197) = v22;
  v35 = sub_15E70((unsigned int)fd, name, &v205, &v204[1], v34);
  v22 = (char)v197;
  if ( v35 >= 0 )
  {
    if ( *(long long *)errnum )
      **(char **)errnum = 1;
    v36 = (const char *)sub_BF60(name);
    if ( v36 && !(unsigned char)sub_7460(0, fd, v36, src, fd, (long long)name, 1u, *((unsigned char *)a8 + 60), v191) )
    {
      if ( *((char *)a8 + 51) )
        sub_7380(0, (unsigned int)fd, v37, v38);
      goto LABEL_75;
    }
    goto LABEL_86;
  }
  if ( !*((char *)a8 + 24) )
    goto LABEL_208;
LABEL_258:
  LOBYTE(v197) = v22;
  v107 = sub_6530((long long)a8, (long long)src, fd, name, (long long)&v205);
  v22 = (char)v197;
  LODWORD(v12) = v107;
  if ( (char)v107 )
  {
    v108 = *(char **)errnum;
    if ( *(long long *)errnum )
      goto LABEL_260;
    goto LABEL_86;
  }
LABEL_210:
  LODWORD(v12) = HIBYTE(v204[0]);
  if ( HIBYTE(v204[0]) )
    goto LABEL_86;
  st_mode = v205.st_mode;
  v93 = v203 & 0xF000;
  if ( (v205.st_mode & 0xF000) == 0x4000 )
  {
    if ( v93 == 0x4000 )
      goto LABEL_218;
LABEL_284:
    if ( !*((char *)a8 + 24) || (v94 = *a8) == 0 )
    {
      v143 = "cannot overwrite directory %s with non-directory";
      v74 = sub_14B40(4, src);
      goto LABEL_305;
    }
    if ( (v204[13] & 0xF000) != 0x4000 )
      goto LABEL_287;
    goto LABEL_345;
  }
  if ( v93 == 0x4000 )
  {
    if ( !*((char *)a8 + 24) || (v94 = *a8) == 0 )
    {
      v112 = sub_14A70(1, 4, a1);
      v113 = "cannot overwrite non-directory %s with directory %s";
      v114 = sub_14A70(0, 4, src);
      goto LABEL_269;
    }
    if ( !(char)v199 )
      goto LABEL_309;
  }
  else
  {
    v94 = *a8;
    if ( !(char)v199 )
      goto LABEL_219;
  }
  LODWORD(v197) = v203 & 0xF000;
  if ( v94 == 3 )
  {
    if ( !*((char *)a8 + 24) )
      goto LABEL_323;
    goto LABEL_309;
  }
  v95 = *((long long *)a8 + 9);
  LOBYTE(v189) = v22;
  v96 = sub_EA90(v95, name, &v205);
  v22 = (char)v189;
  if ( v96 )
  {
    v112 = sub_14A70(1, 4, a1);
    v113 = "will not overwrite just-created %s with %s";
    v114 = sub_14A70(0, 4, src);
    goto LABEL_269;
  }
  if ( (int)v197 != 0x4000 )
  {
    st_mode = v205.st_mode;
    if ( (v205.st_mode & 0xF000) == 0x4000 )
      goto LABEL_284;
  }
LABEL_218:
  v94 = *a8;
LABEL_219:
  if ( *((char *)a8 + 24) )
  {
LABEL_309:
    if ( (v204[13] & 0xF000) != 0x4000 )
      goto LABEL_310;
    st_mode = v205.st_mode;
LABEL_345:
    if ( (st_mode & 0xF000) == 0x4000 )
    {
LABEL_310:
      if ( !v94 )
        goto LABEL_229;
      goto LABEL_287;
    }
    if ( !v94 )
    {
      v112 = sub_14C40(0, 3, src);
      v113 = "cannot move directory onto non-directory: %s -> %s";
      v114 = sub_14C40(0, 3, a1);
      goto LABEL_269;
    }
LABEL_287:
    LOBYTE(v197) = v22;
    LODWORD(v189) = v94;
    v126 = (const char *)sub_DB60(a1);
    v127 = sub_6190(v126);
    v22 = (char)v197;
    if ( v127 )
      goto LABEL_229;
    v128 = (int)v189;
LABEL_289:
    if ( v128 != 3 )
    {
      LOBYTE(v197) = v22;
      v129 = sub_6640(v126, &v204[1], fd, name);
      v22 = (char)v197;
      if ( v129 )
      {
        if ( *((char *)a8 + 24) )
          v166 = dcgettext(0, "backing up %s might destroy source;  %s not moved", 5);
        else
          v166 = dcgettext(0, "backing up %s might destroy source;  %s not copied", 5);
        v167 = sub_14A70(1, 4, a1);
        v168 = sub_14A70(0, 4, src);
        error(0, 0, v166, v168, v167);
        return (unsigned int)v12;
      }
    }
    v130 = *a8;
    LOBYTE(v197) = v22;
    v131 = (const char *)sub_DA90((unsigned int)fd, name, v130);
    v132 = (char *)v131;
    if ( v131 )
    {
      v133 = name - (char *)src;
      v134 = strlen(v131);
      v135 = (char)v197;
      v12 = (void *)(v134 + 1);
      v136 = name - (char *)src + v134 + 24;
      v137 = &v187[-(v136 & 0xFFFFFFFFFFFFF000LL)];
      v138 = v136 & 0xFFF0;
      if ( v187 != v137 )
      {
        while ( v186 != v137 )
          ;
      }
      v139 = v138 & 0xFFF;
      v140 = alloca(v139);
      if ( v139 )
        *(long long *)&v186[v139 - 8] = *(long long *)&v186[v139 - 8];
      LOBYTE(v189) = v135;
      v197 = v186;
      v141 = mempcpy(v186, src, v133);
      memcpy(v141, v132, (size_t)v12);
      free(v132);
      v22 = (char)v189;
    }
    else
    {
      LOBYTE(v189) = (char)v197;
      v181 = __errno_location();
      v22 = (char)v197;
      v197 = 0;
      v99 = v181;
      if ( *v181 != 2 )
      {
        v174 = "cannot backup %s";
        v175 = sub_14B40(4, src);
        goto LABEL_388;
      }
    }
    LOBYTE(v204[0]) = 1;
    goto LABEL_230;
  }
  st_mode = v205.st_mode;
  if ( !v94 )
    goto LABEL_221;
LABEL_323:
  LODWORD(v197) = v94;
  LOBYTE(v189) = v22;
  v126 = (const char *)sub_DB60(a1);
  v148 = sub_6190(v126);
  v128 = (int)v197;
  v22 = (char)v189;
  if ( v148 )
  {
LABEL_221:
    if ( (st_mode & 0xF000) == 0x4000
      || !*((char *)a8 + 21)
      && (!*((char *)a8 + 49)
       || (!*((char *)a8 + 48) || v205.st_nlink <= 1) && (a8[1] != 2 || (v204[13] & 0xF000) == 0x8000)) )
    {
      goto LABEL_229;
    }
    LOBYTE(v197) = v22;
    v97 = unlinkat(fd, name, 0);
    v22 = (char)v197;
    if ( !v97 || (v98 = __errno_location(), v22 = (char)v197, v99 = v98, *v98 == 2) )
    {
      v40 = *((char *)a8 + 60) == 0;
      LOBYTE(v204[0]) = 1;
      if ( !v40 )
      {
        LOBYTE(v189) = v22;
        v169 = sub_14B40(4, src);
        v170 = dcgettext(0, "removed %s\n", 5);
        __printf_chk(1, v170, v169);
        v22 = (char)v189;
        v197 = 0;
        goto LABEL_230;
      }
      goto LABEL_229;
    }
    v174 = "cannot remove %s";
    v175 = sub_14B40(4, src);
LABEL_388:
    v176 = dcgettext(0, v174, 5);
    error(0, *v99, v176, v175);
    return (unsigned int)v12;
  }
  if ( (st_mode & 0xF000) != 0x4000 )
    goto LABEL_289;
LABEL_229:
  v197 = 0;
LABEL_230:
  v14 = 17;
LABEL_180:
  if ( !(char)v199 )
    goto LABEL_23;
LABEL_18:
  if ( !*((long long *)a8 + 9) )
    goto LABEL_23;
  LODWORD(v12) = *((unsigned char *)a8 + 24);
  if ( (char)v12 )
    goto LABEL_23;
  if ( !*a8 )
  {
    v23 = &v205;
    if ( !v22 )
    {
      if ( fstatat(fd, name, &v207, 256) )
        goto LABEL_23;
      v23 = &v207;
    }
    if ( (v23->st_mode & 0xF000) == 0xA000 && (unsigned char)sub_EA90(*((long long *)a8 + 9), name, v23) )
    {
      v112 = sub_14A70(1, 4, src);
      v113 = "will not copy %s through just-created symlink %s";
      v114 = sub_14A70(0, 4, a1);
LABEL_269:
      v115 = dcgettext(0, v113, 5);
      error(0, 0, v115, v114, v112);
      return (unsigned int)v12;
    }
LABEL_23:
    if ( !*((char *)a8 + 60) || *((char *)a8 + 24) )
      goto LABEL_25;
    goto LABEL_144;
  }
  if ( !*((char *)a8 + 60) )
  {
    if ( !v14 )
      goto LABEL_89;
    goto LABEL_26;
  }
LABEL_144:
  if ( (v203 & 0xF000) != 0x4000 )
    sub_73C0((long long)a1, (long long)src, (long long)v197);
LABEL_25:
  if ( !v14 )
  {
    if ( *((char *)a8 + 24) )
      goto LABEL_78;
    goto LABEL_89;
  }
LABEL_26:
  if ( *((char *)a8 + 56) && (v203 & 0xF000) == 0x4000 )
  {
    if ( (char)v199 )
      v189 = (const char *)sub_BF60(name);
    else
      v189 = (const char *)sub_BF00(*(long long *)&v204[5], *(long long *)&v204[1]);
    if ( !v189 )
      goto LABEL_36;
LABEL_31:
    if ( (unsigned char)sub_15130(-100) )
    {
      v145 = sub_14A70(1, 4, qword_25568);
      v146 = sub_14A70(0, 4, qword_25570);
      v147 = dcgettext(0, "cannot copy a directory, %s, into itself, %s", 5);
      v86 = 0;
      v85 = 0;
      error(0, 0, v147, v146, v145);
      *v193 = 1;
      goto LABEL_251;
    }
    if ( !(unsigned char)sub_15130(fd) )
    {
      v24 = a8[1];
      if ( v24 == 4 || v24 == 3 && (char)v199 )
        goto LABEL_36;
      v156 = src;
      v157 = sub_65E0(src, (long long)name, v189);
      v158 = sub_14A70(1, 4, v157);
      v159 = sub_14A70(0, 4, v156);
      v160 = dcgettext(0, "will not create hard link %s to directory %s", 5);
      v86 = 0;
      error(0, 0, v160, v159, v158);
      v85 = v157;
      free(v157);
LABEL_251:
      if ( !*((char *)a8 + 51) )
        goto LABEL_202;
LABEL_252:
      sub_7380((long long)v85, v86, v87, v46);
LABEL_200:
      if ( !v189 )
        sub_BEA0(*(long long *)&v204[5], *(long long *)&v204[1]);
LABEL_202:
      if ( v197 )
      {
        if ( !renameat(fd, (const char *)v197 + name - (char *)src, fd, name) )
        {
          if ( *((char *)a8 + 60) )
          {
            v88 = sub_14A70(1, 4, src);
            v89 = sub_14A70(0, 4, v197);
            v90 = dcgettext(0, "%s -> %s (unbackup)\n", 5);
            __printf_chk(1, v90, v89, v88);
          }
          goto LABEL_75;
        }
        v77 = "cannot un-backup %s";
        v78 = sub_14B40(4, src);
LABEL_190:
        LODWORD(v12) = 0;
        v79 = dcgettext(0, v77, 5);
        v80 = __errno_location();
        error(0, *v80, v79, v78);
        return (unsigned int)v12;
      }
      goto LABEL_75;
    }
    v164 = sub_14B40(4, qword_25570);
    v165 = dcgettext(0, "warning: source directory %s specified more than once", 5);
    error(0, 0, v165, v164);
    LOBYTE(v12) = a8[6] & (*(long long *)errnum != 0);
    if ( !(char)v12 )
      goto LABEL_86;
    v108 = *(char **)errnum;
LABEL_260:
    *v108 = 1;
    return (unsigned int)v12;
  }
  v25 = *((char *)a8 + 24);
  if ( !v25 )
  {
    if ( *((char *)a8 + 48) )
    {
      if ( *((char *)a8 + 23) )
      {
        v189 = 0;
        goto LABEL_143;
      }
      goto LABEL_138;
    }
LABEL_89:
    v28 = LOBYTE(v204[0]);
    v189 = 0;
    v29 = v204[0];
    goto LABEL_90;
  }
  if ( *(long long *)&v204[9] == 1 )
  {
    v189 = (const char *)sub_BF00(*(long long *)&v204[5], *(long long *)&v204[1]);
    goto LABEL_248;
  }
  if ( !*((char *)a8 + 48) || *((char *)a8 + 23) )
  {
    v189 = 0;
    goto LABEL_46;
  }
LABEL_138:
  if ( *(long long *)&v204[9] > 1u || (v62 = a8[1], (char)v199) && v62 == 3 || (v189 = 0, v62 == 4) )
  {
    v189 = (const char *)sub_BF60(name);
LABEL_248:
    if ( !v189 )
    {
LABEL_36:
      v25 = *((char *)a8 + 24);
      goto LABEL_142;
    }
    if ( (v203 & 0xF000) == 0x4000 )
      goto LABEL_31;
    v85 = 0;
    v86 = (unsigned int)fd;
    if ( !(unsigned char)sub_7460(0, fd, v189, src, fd, (long long)name, 1u, *((unsigned char *)a8 + 60), v191) )
      goto LABEL_251;
LABEL_86:
    LODWORD(v12) = 1;
    return (unsigned int)v12;
  }
LABEL_142:
  if ( !v25 )
  {
LABEL_143:
    v28 = LOBYTE(v204[0]);
    v29 = v204[0];
    goto LABEL_90;
  }
LABEL_46:
  if ( v14 == 17 && (!renameat(-100, a1, fd, name) || (v14 = *__errno_location()) == 0) )
  {
LABEL_78:
    if ( *((char *)a8 + 60) )
    {
      v116 = dcgettext(0, "renamed ", 5);
      __printf_chk(1, v116);
      sub_73C0((long long)a1, (long long)src, (long long)v197);
    }
    if ( *((long long *)a8 + 5) )
      sub_8250((long long)src, 1u, (long long)a8);
    if ( *(long long *)errnum )
      **(char **)errnum = 1;
    if ( (char)v199 && !*((char *)a8 + 63) )
    {
      sub_EA00(*((long long *)a8 + 9), name, &v204[1]);
      LODWORD(v12) = (unsigned char)v199;
      return (unsigned int)v12;
    }
    goto LABEL_86;
  }
  if ( v14 == 22 )
  {
    v117 = sub_14A70(1, 4, qword_25568);
    v118 = sub_14A70(0, 4, qword_25570);
    v119 = dcgettext(0, "cannot move %s to a subdirectory of itself, %s", 5);
    error(0, 0, v119, v118, v117);
    *v193 = 1;
    goto LABEL_86;
  }
  if ( v14 != 18 )
  {
    v100 = sub_14A70(1, 4, src);
    v101 = sub_14A70(0, 4, a1);
    v102 = dcgettext(0, "cannot move %s to %s", 5);
    error(0, v14, v102, v101, v100);
LABEL_236:
    sub_BEA0(*(long long *)&v204[5], *(long long *)&v204[1]);
    goto LABEL_75;
  }
  v26 = v203 & 0xF000;
  if ( unlinkat(fd, name, (v26 == 0x4000) << 9) )
  {
    v27 = __errno_location();
    if ( *v27 != 2 )
    {
      v109 = sub_14A70(1, 4, src);
      v110 = sub_14A70(0, 4, a1);
      v111 = dcgettext(0, "inter-device move failed: %s to %s; unable to remove target", 5);
      error(0, *v27, v111, v110, v109);
      goto LABEL_236;
    }
  }
  if ( *((char *)a8 + 60) && v26 != 0x4000 )
  {
    v106 = dcgettext(0, "copied ", 5);
    __printf_chk(1, v106);
    sub_73C0((long long)a1, (long long)src, (long long)v197);
  }
  LOBYTE(v204[0]) = 1;
  v28 = 1;
  v29 = 1;
LABEL_90:
  v40 = *((char *)a8 + 57) == 0;
  errnum[0] = v203 & 0xFFF;
  if ( !v40 )
    errnum[0] = a8[4] & 0xFFF;
  if ( *((char *)a8 + 29) )
  {
    v42 = v203;
    if ( !(unsigned char)sub_7FF0((long long)a1, (long long)src, v203, v28, (long long)a8) )
      goto LABEL_75;
    LODWORD(v12) = v42 & 0xF000;
    v190 = errnum[0] & 0x3F;
    if ( (int)v12 != 0x4000 )
    {
LABEL_99:
      LOBYTE(ptr) = *((char *)a8 + 58);
      if ( (char)ptr )
      {
        if ( *a1 == 47 )
          goto LABEL_105;
        v43 = (const char *)sub_E730(name);
        v44 = (char *)v43;
        if ( fd == -100 && (*(long long *)errnum = ".", !strcmp(".", v43))
          || stat(".", &v206)
          || fstatat(fd, v44, &v207, 0)
          || v206.st_ino == v207.st_ino && v206.st_dev == v207.st_dev )
        {
          free(v44);
LABEL_105:
          v45 = sub_C230(a1, (unsigned int)fd, name, *((unsigned char *)a8 + 22), 0xFFFFFFFFLL);
          v191 = 0;
          v49 = v45;
          if ( v45 > 0 )
          {
            v120 = sub_14A70(1, 4, a1);
            v121 = sub_14A70(0, 4, src);
            v122 = dcgettext(0, "cannot create symbolic link %s to %s", 5);
            v86 = v49;
            v85 = 0;
            error(0, v49, v122, v121, v120);
            goto LABEL_199;
          }
          goto LABEL_106;
        }
        free(v44);
        v123 = "%s: can make relative symbolic links only in current directory";
        v124 = sub_14C40(0, 3, src);
LABEL_282:
        v125 = dcgettext(0, v123, 5);
        v86 = 0;
        v85 = 0;
        error(0, 0, v125, v124);
        goto LABEL_199;
      }
      if ( *((char *)a8 + 23) )
      {
        v103 = 1;
        if ( !*((char *)a8 + 22) )
          v103 = a8[2] == 3;
        v86 = 4294967196LL;
        v85 = a1;
        v104 = sub_7460(a1, 0xFFFFFF9C, a1, src, fd, (long long)name, v103, 0, v191);
        v191 = 0;
        if ( !v104 )
          goto LABEL_199;
        goto LABEL_106;
      }
      if ( (int)v12 == 0x8000 || (v191 = a8[5] & ((int)v12 != 40960)) != 0 )
      {
        v85 = a1;
        v86 = (long long)src;
        v191 = sub_8680(
                 (long long)a1,
                 (const char *)src,
                 fd,
                 name,
                 (long long)a8,
                 errnum[0] & 0x1FF,
                 v190,
                 v204,
                 (long long)&v204[1]);
        if ( !v191 )
          goto LABEL_199;
        LOBYTE(ptr) = 0;
        v29 = v204[0];
        goto LABEL_106;
      }
      if ( (int)v12 == 4096 )
      {
        v161 = v203 & ~v190;
        if ( mknodat(fd, name, v161, 0) )
        {
          v162 = v161;
          BYTE1(v162) = BYTE1(v161) & 0xEF;
          if ( mkfifoat(fd, name, v162) )
          {
            v82 = "cannot create fifo %s";
            v83 = sub_14B40(4, src);
            goto LABEL_198;
          }
        }
        goto LABEL_106;
      }
      v81 = (int)v12;
      BYTE1(v81) = BYTE1(v12) & 0xBF;
      if ( (int)v12 == 49152 || v81 == 0x2000 )
      {
        if ( mknodat(fd, name, v203 & ~v190, *(__dev_t *)&v204[21]) )
        {
          v82 = "cannot create special file %s";
          v83 = sub_14B40(4, src);
LABEL_198:
          v84 = dcgettext(0, v82, 5);
          v85 = 0;
          v86 = (unsigned int)*__errno_location();
          error(0, v86, v84, v83);
          goto LABEL_199;
        }
        goto LABEL_106;
      }
      if ( (int)v12 != 40960 )
      {
        v123 = "%s has unknown file type";
        v124 = sub_14B40(4, a1);
        goto LABEL_282;
      }
      v150 = sub_CBF0(a1);
      v151 = (char *)v150;
      if ( !v150 )
      {
        v82 = "cannot read symbolic link %s";
        v83 = sub_14B40(4, a1);
        goto LABEL_198;
      }
      v152 = (unsigned int)fd;
      errnum[0] = sub_C230(v150, (unsigned int)fd, name, *((unsigned char *)a8 + 22), 0xFFFFFFFFLL);
      if ( errnum[0] > 0 )
      {
        if ( *((char *)a8 + 59) == 1 && !v29 && (v205.st_mode & 0xF000) == 0xA000 )
        {
          ptr = (void *)v205.st_size;
          v177 = (void *)strlen(v151);
          if ( ptr == v177 )
          {
            v178 = (void *)sub_CD80(fd, name);
            if ( v178 )
            {
              v152 = (long long)v151;
              ptr = v178;
              if ( !strcmp((const char *)v178, v151) )
              {
                free(ptr);
                v171 = (long long)v151;
                free(v151);
                goto LABEL_380;
              }
              free(ptr);
            }
          }
        }
        free(v151);
        v153 = sub_14B40(4, src);
        v154 = dcgettext(0, "cannot create symbolic link %s", 5);
        v86 = (unsigned int)errnum[0];
        v85 = 0;
        error(0, errnum[0], v154, v153);
        goto LABEL_199;
      }
      v171 = (long long)v151;
      free(v151);
LABEL_380:
      if ( *((char *)a8 + 51) )
        sub_7380(v171, v152, v172, v46);
      v173 = *((char *)a8 + 29);
      LOBYTE(ptr) = 1;
      v191 = v173;
      if ( !v173 )
        goto LABEL_106;
      if ( (unsigned int)sub_E540((unsigned int)fd, name, *(unsigned int *)&v204[15], *(unsigned int *)&v204[17]) )
      {
        LOBYTE(ptr) = sub_8400((long long)a8);
        if ( (char)ptr )
        {
          v191 = 0;
          goto LABEL_106;
        }
        v179 = dcgettext(0, "failed to preserve ownership for %s", 5);
        v85 = 0;
        v86 = (unsigned int)*__errno_location();
        error(0, v86, v179, src);
        if ( *((char *)a8 + 50) )
          goto LABEL_199;
      }
      v180 = v191;
      v191 = 0;
      LOBYTE(ptr) = v180;
LABEL_106:
      LOBYTE(v47) = (int)v12 != 0x4000;
      if ( v29 )
        goto LABEL_108;
      v50 = *((unsigned char *)a8 + 20) ^ 1;
      LOBYTE(v50) = v47 & (*((char *)a8 + 20) ^ 1);
      v51 = v50;
      if ( !(char)v50 )
        goto LABEL_108;
      if ( !*((long long *)a8 + 5) && !*((char *)a8 + 51) )
        goto LABEL_256;
      v85 = src;
      v86 = 0;
      v105 = sub_8250((long long)src, 0, (long long)a8);
      v47 = v105;
      if ( (char)v105 )
      {
LABEL_108:
        LODWORD(v12) = 1;
        if ( !(char)v199 )
          goto LABEL_113;
        LOBYTE(v195) = 0;
        LODWORD(v12) = v199;
LABEL_110:
        if ( *((long long *)a8 + 9) )
        {
          LOBYTE(v199) = v47;
          v52 = fstatat(fd, name, &v207, 256);
          v47 = (unsigned char)v199;
          if ( !v52 )
          {
            sub_EA00(*((long long *)a8 + 9), name, &v207);
            v47 = (unsigned char)v199;
          }
        }
LABEL_113:
        if ( *((char *)a8 + 23) && (char)v47 || v191 )
          return (unsigned int)v12;
        goto LABEL_116;
      }
      v29 = *((char *)a8 + 52);
      if ( !v29 )
      {
LABEL_256:
        v47 = v51;
        goto LABEL_108;
      }
LABEL_199:
      if ( !*((char *)a8 + 51) )
        goto LABEL_200;
      goto LABEL_252;
    }
  }
  else
  {
    LODWORD(v12) = v203 & 0xF000;
    if ( (int)v12 != 0x4000 )
    {
      v41 = sub_7FF0((long long)a1, (long long)src, v203, v28, (long long)a8);
      v190 = 0;
      if ( !v41 )
        goto LABEL_75;
      goto LABEL_99;
    }
    if ( !(unsigned char)sub_7FF0((long long)a1, (long long)src, v203, v28, (long long)a8) )
      goto LABEL_75;
    v190 = errnum[0] & 0x12;
  }
  v191 = sub_61E0(&v204[1], ptr);
  if ( v191 )
  {
    v123 = "cannot copy cyclic symbolic link %s";
    v124 = sub_14B40(4, a1);
    goto LABEL_282;
  }
  v195 = &v182;
  v182 = ptr;
  v183 = *(long long *)&v204[5];
  v184 = *(long long *)&v204[1];
  if ( !v29 && (v205.st_mode & 0xF000) == 0x4000 )
  {
    if ( *((long long *)a8 + 5) || *((char *)a8 + 51) )
    {
      v85 = src;
      v86 = 0;
      *(long long *)errnum = v48;
      v142 = sub_8250((long long)src, 0, (long long)a8);
      v48 = *(char **)errnum;
      if ( !v142 )
      {
        if ( *((char *)a8 + 52) )
          goto LABEL_199;
      }
    }
    v190 = 0;
    v64 = 0;
  }
  else
  {
    ptr = v48;
    if ( mkdirat(fd, name, errnum[0] & ~v190) )
    {
      v82 = "cannot create directory %s";
      v83 = sub_14B40(4, src);
      goto LABEL_198;
    }
    *(long long *)errnum = ptr;
    v63 = fstatat(fd, name, &v205, 256);
    v48 = *(char **)errnum;
    if ( v63 )
    {
      v82 = "cannot stat %s";
      v83 = sub_14B40(4, src);
      goto LABEL_198;
    }
    LODWORD(v12) = v205.st_mode;
    v64 = 0;
    if ( (v205.st_mode & 0x1C0) != 0x1C0 )
    {
      v65 = sub_E520((unsigned int)fd, name, v205.st_mode | 0x1C0, v46, v47, *(long long *)errnum);
      v188 = (unsigned int)v12;
      v48 = *(char **)errnum;
      v64 = 1;
      if ( v65 )
      {
        v82 = "setting permissions for %s";
        v83 = sub_14B40(4, src);
        goto LABEL_198;
      }
    }
    if ( !*v194 )
    {
      v197 = v48;
      LOBYTE(errnum[0]) = v64;
      sub_BF60(name);
      v48 = v197;
      v64 = errnum[0];
      *v194 = 1;
    }
    if ( *((char *)a8 + 60) )
    {
      v40 = *((char *)a8 + 24) == 0;
      v197 = v48;
      LOBYTE(errnum[0]) = v64;
      if ( v40 )
      {
        sub_73C0((long long)a1, (long long)src, 0);
      }
      else
      {
        v12 = (void *)sub_14B40(4, src);
        v66 = dcgettext(0, "created directory %s\n", 5);
        __printf_chk(1, v66, v12, v67, v68, v69, v182, v183, v184);
      }
      v64 = errnum[0];
      v48 = v197;
    }
  }
  LOBYTE(v12) = a8[7] & (v192 != 0);
  if ( !(char)v12 || *v192 == *(long long *)&v204[1] )
  {
    LOBYTE(errnum[0]) = v64;
    v70 = sub_BBC0(
            (int)a1,
            (int)src,
            fd,
            (int)name,
            v28,
            (int)v48,
            (long long)v195,
            (long long)a8,
            (long long)v194,
            (long long)v193);
    v64 = errnum[0];
    LODWORD(v12) = v70;
  }
  if ( (char)v199 )
  {
    LOBYTE(ptr) = 0;
    v47 = 0;
    LOBYTE(v195) = v64;
    goto LABEL_110;
  }
  LOBYTE(ptr) = 0;
  LOBYTE(v195) = v64;
LABEL_116:
  if ( *((char *)a8 + 31) )
  {
    v207.st_dev = sub_15960(&v204[1]);
    v207.st_ino = v53;
    v54 = sub_15980(&v204[1]);
    *(long long *)&v207.st_mode = v55;
    v207.st_nlink = v54;
    if ( utimensat(fd, name, (const struct timespec *)&v207, (unsigned char)ptr << 8) )
    {
      v199 = sub_14B40(4, src);
      v56 = dcgettext(0, "preserving times for %s", 5);
      v57 = __errno_location();
      error(0, *v57, v56, v199);
      if ( *((char *)a8 + 50) )
        goto LABEL_75;
    }
  }
  if ( !(char)ptr )
  {
    if ( !*((char *)a8 + 29) || !v29 && v205.st_uid == *(int *)&v204[15] && v205.st_gid == *(int *)&v204[17] )
      goto LABEL_126;
    v58 = sub_8430(
            a8,
            src,
            (unsigned int)fd,
            name,
            0xFFFFFFFFLL,
            *(unsigned int *)&v204[13],
            *(unsigned int *)&v204[15],
            *(unsigned int *)&v204[17],
            v29,
            &v205);
    if ( v58 != -1 )
    {
      if ( !v58 )
        v203 &= 0xFFFFF1FF;
LABEL_126:
      if ( *((char *)a8 + 53)
        && !sub_7040((long long)a1, 0xFFFFFFFFLL, (long long)src, 0xFFFFFFFFLL, (long long)a8)
        && *((char *)a8 + 54) )
      {
        goto LABEL_75;
      }
      if ( (*((long long *)a8 + 3) & 0xFF0000000000FFLL) == 0 )
      {
        if ( *((char *)a8 + 57) )
        {
          v149 = a8[4];
        }
        else
        {
          if ( ((char)a8[8] & v29) == 0 )
          {
            if ( v190 && (v190 &= ~(unsigned int)sub_8640()) != 0 )
            {
              if ( (char)v195 != 1 )
              {
                if ( v29 && fstatat(fd, name, &v205, 256) )
                {
                  v19 = src;
                  goto LABEL_189;
                }
                v188 = v205.st_mode;
                if ( (~v205.st_mode & v190) == 0 )
                  return (unsigned int)v12;
              }
            }
            else if ( !(char)v195 )
            {
              return (unsigned int)v12;
            }
            if ( !(unsigned int)sub_E520((unsigned int)fd, name, v190 | v188, v46, v47, v48) )
              return (unsigned int)v12;
            v59 = sub_14B40(4, src);
            v60 = dcgettext(0, "preserving permissions for %s", 5);
            v61 = __errno_location();
            error(0, *v61, v60, v59);
LABEL_134:
            if ( !*((char *)a8 + 50) )
              return (unsigned int)v12;
            goto LABEL_75;
          }
          v163 = 511;
          if ( (v203 & 0x7000) != 0x4000 )
            v163 = 438;
          v149 = v163 & ~(unsigned int)sub_8640();
        }
        if ( !(unsigned int)sub_CB50(src, 0xFFFFFFFFLL, v149) )
          return (unsigned int)v12;
        goto LABEL_75;
      }
      if ( !(unsigned int)sub_CAA0(a1, 0xFFFFFFFFLL, src, 0xFFFFFFFFLL, v203) )
        return (unsigned int)v12;
      goto LABEL_134;
    }
LABEL_75:
    LODWORD(v12) = 0;
    return (unsigned int)v12;
  }
  if ( *((char *)a8 + 53)
    && !sub_7040((long long)a1, 0xFFFFFFFFLL, (long long)src, 0xFFFFFFFFLL, (long long)a8)
    && *((char *)a8 + 54) )
  {
    goto LABEL_75;
  }
  return (unsigned int)v12;
}



// Function: copy @ 0xbae0
long long copy(char *a1, void *a2, int a3, char *a4, int a5, unsigned int *a6, char *a7, long long a8)
{
  char v13[65]; // [rsp+17h] [rbp-41h] BYREF

  *(long long *)&v13[1] = __readfsqword(0x28u);
  if ( !(unsigned char)sub_6270((long long)a6) )
    __assert_fail("valid_options (options)", "src/copy.c", 0xC2Au, "copy");
  v13[0] = 0;
  qword_25570 = (long long)a1;
  qword_25568 = (long long)a2;
  return sub_9470(a1, a2, a3, a4, a5, 0, 0, a6, 1, v13, a7, a8);
}



// Function: copy_dir @ 0xbbc0
long long copy_dir(
        long long a1,
        long long a2,
        int a3,
        long long a4,
        unsigned char a5,
        long long *a6,
        void *a7,
        const __m128i *a8,
        char *a9,
        char *a10)
{
  long long v12; // rax
  const char *v13; // r14
  long long i; // rax
  char *v15; // r12
  void *v16; // rbp
  char v17; // r15
  long long v19; // r13
  char *v20; // r12
  int *v21; // rax
  unsigned char v22; // [rsp+Eh] [rbp-FAh]
  char v23; // [rsp+Fh] [rbp-F9h]
  int v26; // [rsp+24h] [rbp-E4h]
  long long v27; // [rsp+28h] [rbp-E0h]
  char *s; // [rsp+58h] [rbp-B0h]
  char v29; // [rsp+6Dh] [rbp-9Bh] BYREF
  char v30; // [rsp+6Eh] [rbp-9Ah] BYREF
  char v31; // [rsp+6Fh] [rbp-99h] BYREF
  _OWORD v32[5]; // [rsp+70h] [rbp-98h] BYREF
  long long v33; // [rsp+C0h] [rbp-48h]
  unsigned long long v34; // [rsp+C8h] [rbp-40h]

  v34 = __readfsqword(0x28u);
  v12 = a8[5].m128i_i64[0];
  v32[0] = _mm_loadu_si128(a8);
  v32[1] = _mm_loadu_si128(a8 + 1);
  v33 = v12;
  v32[2] = _mm_loadu_si128(a8 + 2);
  v32[3] = _mm_loadu_si128(a8 + 3);
  v32[4] = _mm_loadu_si128(a8 + 4);
  s = (char *)sub_15610(a1, 2);
  if ( s )
  {
    if ( a8->m128i_i32[1] == 3 )
      DWORD1(v32[0]) = 2;
    if ( *s )
    {
      v23 = 0;
      v26 = a5;
      v27 = a4 - a2;
      v13 = s;
      v22 = 1;
      for ( i = sub_EC90(a1, s, 0); ; i = sub_EC90(a1, v13, 0) )
      {
        v15 = (char *)i;
        v16 = (void *)sub_EC90(a2, v13, 0);
        v30 = *a9;
        v22 &= sub_9470(v15, v16, a3, (char *)v16 + v27, v26, a6, a7, (unsigned int *)v32, 0, &v30, &v29, (long long)&v31);
        v17 = v29;
        *a10 |= v29;
        free(v16);
        free(v15);
        if ( v17 )
          break;
        v23 |= v30;
        v13 += strlen(v13) + 1;
        if ( !*v13 )
          break;
      }
    }
    else
    {
      v23 = 0;
      v22 = 1;
    }
    free(s);
    *a9 = v23;
  }
  else
  {
    v19 = sub_14B40(4, a1);
    v20 = dcgettext(0, "cannot access %s", 5);
    v21 = __errno_location();
    error(0, *v21, v20, v19);
    return 0;
  }
  return v22;
}



// Function: src_to_dest_hash @ 0xbe50
unsigned long long src_to_dest_hash(long long *a1, unsigned long long a2)
{
  return *a1 % a2;
}



// Function: src_to_dest_compare @ 0xbe60
bool src_to_dest_compare(long long *a1, long long *a2)
{
  bool result; // al

  result = 0;
  if ( *a1 == *a2 )
    return a1[1] == a2[1];
  return result;
}



// Function: src_to_dest_free @ 0xbe80
void src_to_dest_free(void **ptr)
{
  free(ptr[2]);
  free(ptr);
}



// Function: forget_created @ 0xbea0
unsigned long long forget_created(long long a1, long long a2)
{
  void **v2; // rax
  long long v4[3]; // [rsp+0h] [rbp-28h] BYREF
  unsigned long long v5; // [rsp+18h] [rbp-10h]

  v5 = __readfsqword(0x28u);
  v4[0] = a1;
  v4[1] = a2;
  v4[2] = 0;
  v2 = (void **)((long long (*)(long long, long long *))sub_12350)(qword_25578, v4);
  if ( v2 )
    sub_BE80(v2);
  return v5 - __readfsqword(0x28u);
}



// Function: src_to_dest_lookup @ 0xbf00
long long src_to_dest_lookup(long long a1, long long a2)
{
  long long result; // rax
  long long v3[5]; // [rsp+0h] [rbp-28h] BYREF

  v3[3] = __readfsqword(0x28u);
  v3[0] = a1;
  v3[1] = a2;
  result = sub_11A70(qword_25578, v3);
  if ( result )
    return *(long long *)(result + 16);
  return result;
}



// Function: remember_copied @ 0xbf60
long long remember_copied(void *src, long long a2, long long a3)
{
  long long v4; // rbp
  long long v5; // rax
  long long v6; // rdi
  long long v7; // rax
  long long v8; // rbx
  long long result; // rax

  v4 = sub_16E90(24);
  v5 = sub_17230(src);
  v6 = qword_25578;
  *(long long *)v4 = a2;
  *(long long *)(v4 + 16) = v5;
  *(long long *)(v4 + 8) = a3;
  v7 = sub_122F0(v6, v4);
  if ( !v7 )
    sub_17250();
  v8 = v7;
  result = 0;
  if ( v4 != v8 )
  {
    sub_BE80((void **)v4);
    return *(long long *)(v8 + 16);
  }
  return result;
}



// Function: hash_init @ 0xbfe0
long long hash_init()
{
  long long result; // rax

  result = sub_11CD0(103, 0, sub_BE50, sub_BE60, sub_BE80);
  qword_25578 = result;
  if ( !result )
    sub_17250();
  return result;
}



// Function: try_link @ 0xc020
int try_link(char *to, long long a2)
{
  return linkat(*(int *)a2, *(const char **)(a2 + 8), *(int *)(a2 + 16), to, *(int *)(a2 + 20));
}



// Function: samedir_template @ 0xc040
void *samedir_template(void *src, void *dest)
{
  long long v3; // rax
  size_t v4; // r12

  v3 = sub_DB60(src) - (long long)src;
  v4 = v3;
  if ( (unsigned long long)(v3 + 9) > 0x100 )
  {
    dest = malloc(v3 + 9);
    if ( !dest )
      return 0;
  }
  strcpy((char *)mempcpy(dest, src, v4), "CuXXXXXX");
  return dest;
}



// Function: try_symlink @ 0xc0a0
int try_symlink(char *to, long long a2)
{
  return symlinkat(*(const char **)a2, *(int *)(a2 + 8), to);
}



// Function: force_linkat @ 0xc0c0
long long force_linkat(int a1, const char *a2, int a3, char *a4, int a5, char a6, int a7)
{
  unsigned int v10; // eax
  char *v11; // rax
  char *v12; // r15
  unsigned int v13; // r12d
  char dest[264]; // [rsp+30h] [rbp-148h] BYREF
  unsigned long long v16; // [rsp+138h] [rbp-40h]

  v16 = __readfsqword(0x28u);
  v10 = a7;
  if ( a7 < 0 )
  {
    v13 = linkat(a1, a2, a3, a4, a5);
    if ( !v13 )
      return v13;
    v10 = *__errno_location();
  }
  if ( a6 == 1 && v10 == 17 )
  {
    v11 = (char *)sub_C040(a4, dest);
    v12 = v11;
    if ( v11 )
    {
      if ( (unsigned int)sub_15BA0(v11) )
      {
        v13 = *__errno_location();
      }
      else
      {
        v13 = -1;
        if ( renameat(a3, v12, a3, a4) )
          v13 = *__errno_location();
        unlinkat(a3, v12, 0);
      }
      if ( v12 != dest )
        free(v12);
    }
    else
    {
      return (unsigned int)*__errno_location();
    }
  }
  else
  {
    return v10;
  }
  return v13;
}



// Function: force_symlinkat @ 0xc230
long long force_symlinkat(const char *a1, int a2, char *a3, char a4, int a5)
{
  char *v7; // rax
  char *v8; // r15
  unsigned int v9; // r12d
  char dest[264]; // [rsp+10h] [rbp-148h] BYREF
  unsigned long long v12; // [rsp+118h] [rbp-40h]

  v12 = __readfsqword(0x28u);
  if ( a5 < 0 )
  {
    v9 = symlinkat(a1, a2, a3);
    if ( !v9 )
      return v9;
    a5 = *__errno_location();
  }
  if ( a4 == 1 && a5 == 17 )
  {
    v7 = (char *)sub_C040(a3, dest);
    v8 = v7;
    if ( v7 )
    {
      if ( (unsigned int)sub_15BA0(v7) )
      {
        v9 = *__errno_location();
      }
      else
      {
        v9 = -1;
        if ( renameat(a2, v8, a2, a3) )
        {
          v9 = *__errno_location();
          unlinkat(a2, v8, 0);
        }
      }
      if ( v8 != dest )
        free(v8);
    }
    else
    {
      return (unsigned int)*__errno_location();
    }
  }
  else
  {
    return (unsigned int)a5;
  }
  return v9;
}



// Function: computecon @ 0xc370
long long computecon(long long a1, unsigned int a2, long long a3)
{
  void *v4; // rbp
  unsigned short v5; // ax
  unsigned int v6; // r12d
  int *v7; // rax
  int v8; // r13d
  int *v9; // rbx
  long long v11; // [rsp+8h] [rbp-40h] BYREF
  long long v12[7]; // [rsp+10h] [rbp-38h] BYREF

  v12[1] = __readfsqword(0x28u);
  v11 = 0;
  v12[0] = 0;
  v4 = (void *)sub_E730(a1);
  if ( (int)getcon(&v11) >= 0 && (int)sub_158C0(v4, v12) >= 0 && (v5 = mode_to_security_class(a2)) != 0 )
    v6 = security_compute_create(v11, v12[0], v5, a3);
  else
    v6 = -1;
  v7 = __errno_location();
  v8 = *v7;
  v9 = v7;
  free(v4);
  freecon(v11);
  freecon(v12[0]);
  *v9 = v8;
  return v6;
}



// Function: restorecon_private @ 0xc450
long long restorecon_private(long long a1, const char *a2)
{
  struct stat *p_buf; // rsi
  unsigned int v4; // r12d
  int *v5; // rax
  int *v6; // rbx
  long long v7; // r13
  long long v8; // r15
  int v9; // r14d
  unsigned int v10; // ebp
  int *v11; // rax
  int *v12; // rbx
  unsigned int v13; // eax
  int v14; // r12d
  unsigned int v16; // eax
  long long v17; // [rsp+0h] [rbp-E8h] BYREF
  long long v18; // [rsp+8h] [rbp-E0h] BYREF
  struct stat buf; // [rsp+10h] [rbp-D8h] BYREF
  unsigned long long v20; // [rsp+A8h] [rbp-40h]

  v20 = __readfsqword(0x28u);
  v17 = 0;
  v18 = 0;
  if ( a1 )
  {
    p_buf = (struct stat *)&unk_20000;
    v4 = open(a2, (int)&unk_20000);
    v5 = __errno_location();
    v6 = v5;
    if ( v4 == -1 )
    {
      v9 = *v5;
      if ( *v5 != 40 )
      {
        v8 = 0;
        v7 = 0;
        goto LABEL_17;
      }
      p_buf = &buf;
      if ( lstat(a2, &buf) < 0 )
      {
        v9 = *v6;
        v8 = 0;
        v7 = 0;
        goto LABEL_17;
      }
    }
    else
    {
      p_buf = &buf;
      if ( fstat(v4, &buf) < 0 )
      {
        v8 = 0;
        v7 = 0;
        v10 = -1;
LABEL_30:
        v9 = *v6;
        goto LABEL_13;
      }
    }
    p_buf = (struct stat *)&v17;
    if ( (int)selabel_lookup(a1, &v17, a2, buf.st_mode) < 0 )
    {
      v9 = *v6;
      v8 = 0;
      v7 = 0;
      if ( *v6 == 2 )
      {
        *v6 = 61;
        v9 = 61;
      }
    }
    else
    {
      v7 = context_new(v17);
      if ( v7 )
      {
        p_buf = (struct stat *)&v18;
        if ( v4 == -1 )
        {
          if ( (int)sub_158E0(a2, &v18) < 0 )
          {
            v9 = *v6;
            v8 = 0;
            goto LABEL_17;
          }
          v8 = context_new(v18);
          if ( !v8 )
          {
            v9 = *v6;
            v10 = -1;
            goto LABEL_18;
          }
        }
        else
        {
          if ( (int)sub_15900(v4, &v18) < 0 )
          {
            v8 = 0;
            v10 = -1;
            goto LABEL_30;
          }
          v8 = context_new(v18);
          if ( !v8 )
          {
            v9 = *v6;
            v10 = -1;
            goto LABEL_13;
          }
        }
        p_buf = (struct stat *)context_type_get(v7);
        if ( p_buf )
        {
          if ( !(unsigned int)context_type_set(v8, p_buf) )
          {
            p_buf = (struct stat *)context_str(v8);
            if ( p_buf )
            {
              if ( v4 != -1 )
              {
                v10 = fsetfilecon(v4, p_buf);
                goto LABEL_30;
              }
              v16 = lsetfilecon(a2, p_buf);
              v9 = *v6;
              v4 = v16;
LABEL_17:
              v10 = v4;
              goto LABEL_18;
            }
          }
        }
        v9 = *v6;
      }
      else
      {
        v9 = *v6;
        v8 = 0;
      }
    }
    v10 = -1;
    if ( v4 == -1 )
    {
LABEL_18:
      context_free(v7, p_buf);
      context_free(v8, p_buf);
      freecon(v17);
      freecon(v18);
      *v6 = v9;
      return v10;
    }
LABEL_13:
    close(v4);
    goto LABEL_18;
  }
  if ( (int)getfscreatecon(&v18) < 0 )
  {
    return (unsigned int)-1;
  }
  else
  {
    v11 = __errno_location();
    v12 = v11;
    if ( v18 )
    {
      v13 = lsetfilecon(a2, v18);
      v14 = *v12;
      v10 = v13;
      freecon(v18);
      *v12 = v14;
    }
    else
    {
      *v11 = 61;
      return (unsigned int)-1;
    }
  }
  return v10;
}



// Function: defaultcon @ 0xc760
long long defaultcon(long long a1, char *a2, unsigned int a3)
{
  long long v5; // rbp
  int *v6; // rbx
  long long v8; // rsi
  long long v9; // rax
  void *v10; // r13
  long long v11; // r12
  long long v12; // rdx
  long long v13; // rcx
  long long v14; // rdi
  unsigned int v15; // eax
  int v16; // r15d
  unsigned int v17; // r14d
  long long v19; // [rsp+8h] [rbp-50h] BYREF
  long long v20[9]; // [rsp+10h] [rbp-48h] BYREF

  v5 = (long long)a2;
  v20[1] = __readfsqword(0x28u);
  v19 = 0;
  v20[0] = 0;
  v6 = __errno_location();
  if ( *a2 == 47 )
  {
    v10 = 0;
  }
  else
  {
    v8 = 2;
    v9 = sub_E4C0(a2, 2);
    v5 = v9;
    if ( !v9 )
    {
      v16 = *v6;
      v10 = 0;
      v11 = 0;
      v17 = -1;
      goto LABEL_12;
    }
    v10 = (void *)v9;
  }
  v8 = (long long)&v19;
  if ( (int)selabel_lookup(a1, &v19, v5, a3) < 0 )
  {
    v16 = *v6;
    if ( *v6 == 2 )
    {
      *v6 = 61;
      v16 = 61;
    }
    v11 = 0;
    v5 = 0;
    v17 = -1;
  }
  else
  {
    v8 = a3;
    if ( (int)sub_C370(v5, a3, (long long)v20) < 0 )
    {
      v16 = *v6;
      v11 = 0;
      v5 = 0;
      v17 = -1;
    }
    else
    {
      v5 = context_new(v19);
      if ( v5 )
      {
        v11 = context_new(v20[0]);
        if ( v11
          && (v8 = context_type_get(v5)) != 0
          && !(unsigned int)context_type_set(v11, v8)
          && (v14 = context_str(v11)) != 0 )
        {
          v15 = setfscreatecon(v14, v8, v12, v13);
          v16 = *v6;
          v17 = v15;
        }
        else
        {
          v16 = *v6;
          v17 = -1;
        }
      }
      else
      {
        v16 = *v6;
        v11 = 0;
        v17 = -1;
      }
    }
  }
LABEL_12:
  context_free(v5, v8);
  context_free(v11, v8);
  freecon(v19);
  freecon(v20[0]);
  free(v10);
  *v6 = v16;
  return v17;
}



// Function: restorecon @ 0xc960
long long restorecon(long long a1, const char *a2, char a3)
{
  void *v4; // r13
  unsigned int v5; // r12d
  const char *v6; // rbp
  int *v7; // rbx
  int v8; // r14d
  const char **v9; // rbp
  long long v11; // rax
  int v12; // eax
  int v13; // ebp
  long long v14[9]; // [rsp+0h] [rbp-48h] BYREF

  v4 = 0;
  v5 = a1;
  v6 = a2;
  v14[3] = __readfsqword(0x28u);
  if ( *a2 != 47 )
  {
    v11 = sub_E4C0(a2, 2);
    v6 = (const char *)v11;
    if ( !v11 )
      return 0;
    v4 = (void *)v11;
    v7 = __errno_location();
    if ( a3 )
      goto LABEL_3;
LABEL_15:
    v12 = sub_C450(a1, v6);
    v13 = *v7;
    LOBYTE(v5) = v12 != -1;
    free(v4);
    *v7 = v13;
    return v5;
  }
  v7 = __errno_location();
  if ( !a3 )
    goto LABEL_15;
LABEL_3:
  v14[0] = v6;
  v14[1] = 0;
  v8 = 0;
  v9 = (const char **)sub_17290(v14, 16, 0);
  while ( sub_10B30(v9) )
  {
    while ( (int)sub_C450(a1, v9[4]) < 0 )
    {
      v8 = *v7;
      if ( !sub_10B30(v9) )
        goto LABEL_7;
    }
  }
LABEL_7:
  if ( *v7 )
    v8 = *v7;
  if ( (unsigned int)sub_10940(v9) )
    v8 = *v7;
  free(v4);
  LOBYTE(v5) = v8 == 0;
  return v5;
}



// Function: dir_name @ 0xe730
long long dir_name(void *a1)
{
  long long result; // rax

  result = sub_E790(a1);
  if ( !result )
    sub_17250();
  return result;
}



// Function: is_prime @ 0x113a0
bool is_prime(unsigned long long a1)
{
  unsigned long long v1; // rdx
  long long v2; // r8
  unsigned long long v3; // rsi
  unsigned long long v4; // rcx

  v1 = a1 - (a1 / 3 + (((0xAAAAAAAAAAAAAAABLL * (unsigned __int128)a1) >> 64) & 0xFFFFFFFFFFFFFFFELL));
  if ( a1 > 9 && v1 )
  {
    v2 = 16;
    v3 = 9;
    v4 = 3;
    do
    {
      v4 += 2LL;
      v3 += v2;
      v1 = a1 % v4;
      if ( a1 <= v3 )
        break;
      v2 += 8;
    }
    while ( v1 );
  }
  return v1 != 0;
}



// Function: next_prime @ 0x11410
unsigned long long next_prime(unsigned long long a1)
{
  long long v1; // rax
  unsigned long long v2; // r9

  v1 = 10;
  if ( a1 >= 0xA )
    v1 = a1;
  v2 = v1 | 1;
  if ( (v1 | 1) != 0xFFFFFFFFFFFFFFFFLL )
  {
    do
    {
      if ( sub_113A0(v2) )
        break;
      v2 += 2LL;
    }
    while ( v2 != -1 );
  }
  return v2;
}



// Function: raw_hasher @ 0x11450
unsigned long long raw_hasher(long long a1, unsigned long long a2)
{
  return sub_17720(a1, 3) % a2;
}



// Function: raw_comparator @ 0x11470
bool raw_comparator(long long a1, long long a2)
{
  return a2 == a1;
}



// Function: check_tuning @ 0x11480
long long check_tuning(long long a1)
{
  long long v1; // rax
  float v2; // xmm0_4
  float v3; // xmm1_4
  float v4; // xmm2_4
  long long result; // rax

  v1 = *(long long *)(a1 + 40);
  if ( (_UNKNOWN *)v1 == &unk_1D660 )
    return 1;
  v2 = *(float *)(v1 + 8);
  if ( v2 <= 0.1
    || v2 >= 0.89999998
    || *(float *)(v1 + 12) <= 1.1
    || *(float *)v1 < 0.0
    || (v3 = *(float *)v1 + 0.1, v4 = *(float *)(v1 + 4), v4 <= v3)
    || v4 > 1.0
    || (result = 1, v2 <= v3) )
  {
    *(long long *)(a1 + 40) = &unk_1D660;
    return 0;
  }
  return result;
}



// Function: free_entry @ 0x11510
long long free_entry(long long a1, long long *a2)
{
  long long result; // rax

  result = *(long long *)(a1 + 72);
  *a2 = 0;
  a2[1] = result;
  *(long long *)(a1 + 72) = a2;
  return result;
}



// Function: safe_hasher @ 0x11530
long long safe_hasher(long long a1, long long a2)
{
  unsigned long long v2; // rax

  v2 = (*(long long (**)(long long, long long))(a1 + 48))(a2, *(long long *)(a1 + 16));
  if ( *(long long *)(a1 + 16) <= v2 )
    abort();
  return *(long long *)a1 + 16 * v2;
}



// Function: hash_find_entry @ 0x11560
long long hash_find_entry(long long a1, long long a2, __m128i **a3, char a4)
{
  __m128i *v6; // rax
  __m128i *v7; // rbx
  long long result; // rax
  __m128i *v9; // rsi
  long long *v10; // rax
  long long *v11; // rsi
  long long v12; // [rsp+8h] [rbp-30h]
  long long v13; // [rsp+8h] [rbp-30h]

  v6 = (__m128i *)sub_11530(a1, a2);
  *a3 = v6;
  if ( !v6->m128i_i64[0] )
    return 0;
  v7 = v6;
  if ( v6->m128i_i64[0] != a2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(a2) )
    {
      result = v7->m128i_i64[0];
      goto LABEL_5;
    }
    while ( 1 )
    {
      v10 = (long long *)v7->m128i_i64[1];
      if ( !v10 )
        break;
      if ( *v10 == a2 || (*(unsigned char (**)(long long))(a1 + 56))(a2) )
      {
        v11 = (long long *)v7->m128i_i64[1];
        result = *v11;
        if ( a4 )
        {
          v13 = *v11;
          v7->m128i_i64[1] = v11[1];
          sub_11510(a1, v11);
          return v13;
        }
        return result;
      }
      v7 = (__m128i *)v7->m128i_i64[1];
    }
    return 0;
  }
  result = v6->m128i_i64[0];
LABEL_5:
  if ( a4 )
  {
    v9 = (__m128i *)v7->m128i_i64[1];
    if ( v9 )
    {
      v12 = result;
      *v7 = _mm_loadu_si128(v9);
      sub_11510(a1, v9);
      return v12;
    }
    else
    {
      v7->m128i_i64[0] = 0;
    }
  }
  return result;
}



// Function: allocate_entry @ 0x11660
long long *allocate_entry(long long a1)
{
  long long *result; // rax

  result = *(long long **)(a1 + 72);
  if ( !result )
    return malloc(0x10u);
  *(long long *)(a1 + 72) = result[1];
  return result;
}



// Function: transfer_entries @ 0x11690
long long transfer_entries(long long a1, long long a2, char a3)
{
  long long *v5; // rbp
  long long v6; // r15
  long long *v7; // r14
  long long v8; // r15
  long long *v9; // rax
  long long *v10; // rsi
  long long *v11; // rax
  long long *v12; // r14
  long long *v13; // rax
  long long v14; // rdx

  v5 = *(long long **)a2;
  if ( *(long long *)a2 >= *(long long *)(a2 + 8) )
    return 1;
  while ( 1 )
  {
    while ( 1 )
    {
      v6 = *v5;
      if ( *v5 )
      {
        v7 = (long long *)v5[1];
        if ( v7 )
        {
          do
          {
            while ( 1 )
            {
              v8 = *v7;
              v9 = (long long *)sub_11530(a1, *v7);
              v10 = v7;
              v7 = (long long *)v7[1];
              if ( !*v9 )
                break;
              v10[1] = v9[1];
              v9[1] = v10;
              if ( !v7 )
                goto LABEL_10;
            }
            *v9 = v8;
            ++*(long long *)(a1 + 24);
            sub_11510(a1, v10);
          }
          while ( v7 );
LABEL_10:
          v6 = *v5;
        }
        v5[1] = 0;
        if ( !a3 )
          break;
      }
      v5 += 2;
      if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
        return 1;
    }
    v11 = (long long *)sub_11530(a1, v6);
    v12 = v11;
    if ( !*v11 )
    {
      *v11 = v6;
      ++*(long long *)(a1 + 24);
      goto LABEL_15;
    }
    v13 = sub_11660(a1);
    if ( !v13 )
      return 0;
    v14 = v12[1];
    *v13 = v6;
    v13[1] = v14;
    v12[1] = v13;
LABEL_15:
    *v5 = 0;
    v5 += 2;
    --*(long long *)(a2 + 24);
    if ( *(long long *)(a2 + 8) <= (unsigned long long)v5 )
      return 1;
  }
}



// Function: hash_get_n_buckets @ 0x11850
long long hash_get_n_buckets(long long a1)
{
  return *(long long *)(a1 + 16);
}



// Function: hash_get_n_buckets_used @ 0x11860
long long hash_get_n_buckets_used(long long a1)
{
  return *(long long *)(a1 + 24);
}



// Function: hash_get_n_entries @ 0x11870
long long hash_get_n_entries(long long a1)
{
  return *(long long *)(a1 + 32);
}



// Function: hash_get_max_bucket_length @ 0x11880
unsigned long long hash_get_max_bucket_length(long long a1)
{
  long long *v1; // rcx
  unsigned long long v2; // rsi
  unsigned long long v3; // r8
  long long v4; // rax
  unsigned long long i; // rdx

  v1 = *(long long **)a1;
  v2 = *(long long *)(a1 + 8);
  v3 = 0;
  if ( *(long long *)a1 < v2 )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( (unsigned long long)v1 >= v2 )
          return v3;
      }
      v4 = v1[1];
      for ( i = 1; v4; ++i )
        v4 = *(long long *)(v4 + 8);
      if ( v3 < i )
        v3 = i;
      v1 += 2;
    }
    while ( (unsigned long long)v1 < v2 );
  }
  return v3;
}



// Function: hash_print_statistics @ 0x11950
long long hash_print_statistics(long long a1, long long a2)
{
  long long v2; // rbx
  long long v3; // r13
  unsigned long long v4; // r12
  long long v5; // r9
  double v6; // xmm0_8
  double v7; // xmm1_8

  sub_11870(a1);
  v2 = sub_11850(a1);
  v3 = sub_11860(a1);
  v4 = sub_11880(a1);
  __fprintf_chk(a2, 1, "# entries:         %lu\n", v5);
  __fprintf_chk(a2, 1, "# buckets:         %lu\n", v2);
  if ( v3 < 0 )
  {
    v6 = ((double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)) + (double)(int)(v3 & 1 | ((unsigned long long)v3 >> 1)))
       * 100.0;
    if ( v2 >= 0 )
      goto LABEL_3;
LABEL_6:
    v7 = (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1)) + (double)(int)(v2 & 1 | ((unsigned long long)v2 >> 1));
    goto LABEL_4;
  }
  v6 = (double)(int)v3 * 100.0;
  if ( v2 < 0 )
    goto LABEL_6;
LABEL_3:
  v7 = (double)(int)v2;
LABEL_4:
  __fprintf_chk(a2, 1, "# buckets used:    %lu (%.2f%%)\n", v3, v6 / v7);
  return __fprintf_chk(a2, 1, "max bucket length: %lu\n", v4);
}



// Function: hash_lookup @ 0x11a70
long long hash_lookup(long long a1, long long a2)
{
  long long v2; // rbp
  long long *v3; // rax
  long long v4; // rsi
  long long *v5; // rbx

  v2 = a2;
  v3 = (long long *)sub_11530(a1, a2);
  v4 = *v3;
  if ( !*v3 )
    return 0;
  v5 = v3;
  while ( v4 != v2 )
  {
    if ( (*(unsigned char (**)(long long))(a1 + 56))(v2) )
      return *v5;
    v5 = (long long *)v5[1];
    if ( !v5 )
      return 0;
    v4 = *v5;
  }
  return v2;
}



// Function: hash_get_next @ 0x11b20
long long hash_get_next(long long a1, long long a2)
{
  long long *v2; // rax
  long long *v3; // rdx
  long long v4; // rcx
  long long v5; // r8

  v2 = (long long *)sub_11530(a1, a2);
  v3 = v2;
  while ( 1 )
  {
    v4 = *v3;
    v3 = (long long *)v3[1];
    if ( v4 == a2 )
      break;
    if ( !v3 )
      goto LABEL_7;
  }
  if ( v3 )
    return *v3;
LABEL_7:
  while ( 1 )
  {
    v2 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v2 )
      break;
    v5 = *v2;
    if ( *v2 )
      return v5;
  }
  return 0;
}



// Function: hash_do_for_each @ 0x11be0
long long hash_do_for_each(long long a1, unsigned char (*a2)(long long, long long), long long a3)
{
  long long *v3; // r14
  long long v6; // r12
  long long v7; // rdi
  long long *v9; // rbx

  v3 = *(long long **)a1;
  if ( *(long long *)a1 >= *(long long *)(a1 + 8) )
    return 0;
  v6 = 0;
  while ( 1 )
  {
    v7 = *v3;
    if ( *v3 )
      break;
LABEL_4:
    v3 += 2;
    if ( *(long long *)(a1 + 8) <= (unsigned long long)v3 )
      return v6;
  }
  v9 = v3;
  while ( a2(v7, a3) )
  {
    v9 = (long long *)v9[1];
    ++v6;
    if ( !v9 )
      goto LABEL_4;
    v7 = *v9;
  }
  return v6;
}



// Function: hash_initialize @ 0x11cd0
long long *hash_initialize(
        long long a1,
        float *a2,
        unsigned long long (*a3)(long long a1, unsigned long long a2),
        bool (*a4)(long long a1, long long a2),
        long long a5)
{
  float *v5; // r15
  unsigned long long (*v7)(long long, unsigned long long); // rbp
  bool (*v8)(long long, long long); // rbx
  long long *v9; // r12
  size_t v10; // rax
  char *v11; // rax
  long long v12; // rdx
  void *v14; // rdi

  v5 = a2;
  v7 = a3;
  v8 = a4;
  if ( !a3 )
    v7 = sub_11450;
  if ( !a4 )
    v8 = sub_11470;
  v9 = malloc(0x50u);
  if ( v9 )
  {
    if ( !a2 )
      v5 = (float *)&unk_1D660;
    v9[5] = v5;
    if ( (unsigned char)sub_11480((long long)v9)
      && (v10 = sub_117C0(a1, *((unsigned char *)v5 + 16), v5[2]), (v9[2] = v10) != 0)
      && (v11 = (char *)calloc(v10, 0x10u), (*v9 = v11) != 0) )
    {
      v12 = v9[2];
      v9[6] = v7;
      v9[3] = 0;
      v9[7] = v8;
      v9[8] = a5;
      v9[1] = &v11[16 * v12];
      v9[4] = 0;
      v9[9] = 0;
    }
    else
    {
      v14 = v9;
      v9 = 0;
      free(v14);
    }
  }
  return v9;
}



// Function: hash_clear @ 0x11de0
void hash_clear(long long *a1)
{
  long long *v1; // r12
  long long *v2; // rbx
  void (*v3)(long long); // rdx
  long long *v4; // rax
  long long v5; // rcx

  v1 = (long long *)*a1;
  if ( *a1 < a1[1] )
  {
    do
    {
      while ( !*v1 )
      {
        v1 += 2;
        if ( a1[1] <= (unsigned long long)v1 )
          goto LABEL_14;
      }
      v2 = (long long *)v1[1];
      v3 = (void (*)(long long))a1[8];
      if ( v2 )
      {
        while ( 1 )
        {
          if ( v3 )
          {
            v3(*v2);
            v3 = (void (*)(long long))a1[8];
          }
          v4 = (long long *)v2[1];
          v5 = a1[9];
          *v2 = 0;
          v2[1] = v5;
          a1[9] = v2;
          if ( !v4 )
            break;
          v2 = v4;
        }
      }
      if ( v3 )
        v3(*v1);
      *v1 = 0;
      v1 += 2;
      *(v1 - 1) = 0;
    }
    while ( a1[1] > (unsigned long long)v1 );
  }
LABEL_14:
  a1[3] = 0;
  a1[4] = 0;
}



// Function: hash_free @ 0x11e90
void hash_free(long long *ptr)
{
  long long *v2; // r13
  unsigned long long v3; // rax
  long long *v4; // rbp
  long long *v5; // rbx
  long long *v6; // rbx
  void *v7; // rdi
  long long *v8; // rbx
  void *v9; // rdi

  v2 = (long long *)*ptr;
  v3 = ptr[1];
  v4 = (long long *)*ptr;
  if ( ptr[8] && ptr[4] )
  {
    if ( (unsigned long long)v2 >= v3 )
      goto LABEL_15;
    do
    {
      while ( !*v2 )
      {
        v2 += 2;
        if ( v3 <= (unsigned long long)v2 )
          goto LABEL_10;
      }
      v5 = v2;
      do
      {
        ((void (*)(void))ptr[8])();
        v5 = (long long *)v5[1];
      }
      while ( v5 );
      v3 = ptr[1];
      v2 += 2;
    }
    while ( v3 > (unsigned long long)v2 );
LABEL_10:
    v4 = (long long *)*ptr;
  }
  if ( v3 > (unsigned long long)v4 )
  {
    do
    {
      v6 = (long long *)v4[1];
      while ( v6 )
      {
        v7 = v6;
        v6 = (long long *)v6[1];
        free(v7);
      }
      v4 += 2;
    }
    while ( ptr[1] > (unsigned long long)v4 );
  }
LABEL_15:
  v8 = (long long *)ptr[9];
  while ( v8 )
  {
    v9 = v8;
    v8 = (long long *)v8[1];
    free(v9);
  }
  free((void *)*ptr);
  free(ptr);
}



// Function: hash_rehash @ 0x11f80
long long hash_rehash(long long a1, long long a2)
{
  size_t v2; // rax
  size_t v3; // rbx
  char *v4; // rax
  long long v5; // rax
  unsigned int v6; // r12d
  void *ptr; // [rsp+0h] [rbp-88h] BYREF
  char *v9; // [rsp+8h] [rbp-80h]
  size_t v10; // [rsp+10h] [rbp-78h]
  long long v11; // [rsp+18h] [rbp-70h]
  long long v12; // [rsp+20h] [rbp-68h]
  long long v13; // [rsp+28h] [rbp-60h]
  long long v14; // [rsp+30h] [rbp-58h]
  long long v15; // [rsp+38h] [rbp-50h]
  long long v16; // [rsp+40h] [rbp-48h]
  long long v17; // [rsp+48h] [rbp-40h]
  unsigned long long v18; // [rsp+58h] [rbp-30h]

  v18 = __readfsqword(0x28u);
  v2 = sub_117C0(a2, *(unsigned char *)(*(long long *)(a1 + 40) + 16LL), *(float *)(*(long long *)(a1 + 40) + 8LL));
  if ( !v2 )
    return 0;
  v3 = v2;
  if ( *(long long *)(a1 + 16) == v2 )
    return 1;
  v4 = (char *)calloc(v2, 0x10u);
  ptr = v4;
  if ( !v4 )
    return 0;
  v10 = v3;
  v11 = 0;
  v9 = &v4[16 * v3];
  v5 = *(long long *)(a1 + 40);
  v12 = 0;
  v13 = v5;
  v14 = *(long long *)(a1 + 48);
  v15 = *(long long *)(a1 + 56);
  v16 = *(long long *)(a1 + 64);
  v17 = *(long long *)(a1 + 72);
  v6 = sub_11690((long long)&ptr, a1, 0);
  if ( (char)v6 )
  {
    free(*(void **)a1);
    *(long long *)a1 = ptr;
    *(long long *)(a1 + 8) = v9;
    *(long long *)(a1 + 16) = v10;
    *(long long *)(a1 + 24) = v11;
    *(long long *)(a1 + 72) = v17;
  }
  else
  {
    *(long long *)(a1 + 72) = v17;
    if ( !(unsigned char)sub_11690(a1, (long long)&ptr, 1) || !(unsigned char)sub_11690(a1, (long long)&ptr, 0) )
      abort();
    free(ptr);
  }
  return v6;
}



// Function: hash_insert_if_absent @ 0x12100
long long hash_insert_if_absent(long long *a1, long long a2, long long *a3)
{
  long long v5; // rax
  unsigned int v6; // r8d
  long long v8; // rax
  float v9; // xmm5_4
  long long v10; // rax
  float v11; // xmm4_4
  long long *v12; // rax
  __m128i *v13; // rdx
  float v14; // xmm4_4
  long long v15; // rax
  float v16; // xmm0_4
  float v17; // xmm5_4
  float v18; // xmm4_4
  long long v19; // rdx
  long long v20; // rdx
  long long v21; // rsi
  __m128i *v22[7]; // [rsp+0h] [rbp-38h] BYREF

  v22[1] = (__m128i *)__readfsqword(0x28u);
  if ( !a2 )
LABEL_26:
    abort();
  v5 = sub_11560((long long)a1, a2, v22, 0);
  if ( !v5 )
  {
    v8 = a1[3];
    if ( v8 < 0 )
    {
      v20 = a1[3] & 1LL | (a1[3] >> 1);
      v9 = (float)(int)v20 + (float)(int)v20;
    }
    else
    {
      v9 = (float)(int)v8;
    }
    v10 = a1[2];
    if ( v10 < 0 )
    {
      v19 = a1[2] & 1LL | (a1[2] >> 1);
      v11 = (float)(int)v19 + (float)(int)v19;
    }
    else
    {
      v11 = (float)(int)v10;
    }
    if ( v9 > (float)(*(float *)(a1[5] + 8LL) * v11) )
    {
      sub_11480((long long)a1);
      v15 = a1[5];
      v16 = *(float *)(v15 + 8);
      if ( v17 > (float)(v14 * v16) )
      {
        v18 = v14 * *(float *)(v15 + 12);
        if ( !*(char *)(v15 + 16) )
          v18 = v18 * v16;
        if ( v18 >= 1.8446744e19 )
          return (unsigned int)-1;
        v21 = v18 >= 9.223372e18
            ? (unsigned int)(int)(float)(v18 - 9.223372e18) ^ 0x8000000000000000LL
            : (unsigned int)(int)v18;
        if ( !(unsigned char)sub_11F80((long long)a1, v21) )
          return (unsigned int)-1;
        if ( sub_11560((long long)a1, a2, v22, 0) )
          goto LABEL_26;
      }
    }
    if ( !*(long long *)v22[0] )
    {
      *(long long *)v22[0] = a2;
      v6 = 1;
      ++a1[4];
      ++a1[3];
      return v6;
    }
    v12 = sub_11660((long long)a1);
    if ( v12 )
    {
      v13 = v22[0];
      *v12 = a2;
      v6 = 1;
      v12[1] = v13->m128i_i64[1];
      v13->m128i_i64[1] = (long long)v12;
      ++a1[4];
      return v6;
    }
    return (unsigned int)-1;
  }
  v6 = 0;
  if ( a3 )
    *a3 = v5;
  return v6;
}



// Function: hash_insert @ 0x122f0
long long hash_insert(long long *a1, long long a2)
{
  int v2; // eax
  bool v3; // zf
  long long result; // rax
  long long v5[3]; // [rsp+0h] [rbp-18h] BYREF

  v5[1] = __readfsqword(0x28u);
  v2 = sub_12100(a1, a2, v5);
  if ( v2 == -1 )
    return 0;
  v3 = v2 == 0;
  result = a2;
  if ( v3 )
    return v5[0];
  return result;
}



// Function: hash_remove @ 0x12350
long long hash_remove(long long *a1, long long a2)
{
  long long v3; // r12
  __m128i *v4; // rax
  long long v6; // rax
  float v7; // xmm5_4
  long long v8; // rax
  float v9; // xmm4_4
  float v10; // xmm4_4
  long long v11; // rax
  float v12; // xmm5_4
  float v13; // xmm4_4
  unsigned long long v14; // rsi
  long long *v15; // rbp
  void *v16; // rdi
  unsigned long long v17; // rdx
  __m128i *v18[5]; // [rsp+0h] [rbp-28h] BYREF

  v18[1] = (__m128i *)__readfsqword(0x28u);
  v3 = sub_11560((long long)a1, a2, v18, 1);
  if ( !v3 )
    return v3;
  v4 = v18[0];
  --a1[4];
  if ( v4->m128i_i64[0] )
    return v3;
  v6 = a1[3] - 1LL;
  a1[3] = v6;
  if ( v6 < 0 )
  {
    v17 = v6 & 1 | ((unsigned long long)v6 >> 1);
    v8 = a1[2];
    v7 = (float)(int)v17 + (float)(int)v17;
    if ( v8 >= 0 )
      goto LABEL_6;
LABEL_18:
    v9 = (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1))
       + (float)(v8 & 1 | (unsigned int)((unsigned long long)v8 >> 1));
    goto LABEL_7;
  }
  v7 = (float)(int)v6;
  v8 = a1[2];
  if ( v8 < 0 )
    goto LABEL_18;
LABEL_6:
  v9 = (float)(int)v8;
LABEL_7:
  if ( (float)(*(float *)a1[5] * v9) > v7 )
  {
    sub_11480((long long)a1);
    v11 = a1[5];
    if ( (float)(*(float *)v11 * v10) > v12 )
    {
      v13 = v10 * *(float *)(v11 + 4);
      if ( !*(char *)(v11 + 16) )
        v13 = v13 * *(float *)(v11 + 8);
      if ( v13 >= 9.223372e18 )
        v14 = (unsigned int)(int)(float)(v13 - 9.223372e18) ^ 0x8000000000000000LL;
      else
        v14 = (unsigned int)(int)v13;
      if ( !(unsigned char)sub_11F80((long long)a1, v14) )
      {
        v15 = (long long *)a1[9];
        while ( v15 )
        {
          v16 = v15;
          v15 = (long long *)v15[1];
          free(v16);
        }
        a1[9] = 0;
      }
    }
  }
  return v3;
}


