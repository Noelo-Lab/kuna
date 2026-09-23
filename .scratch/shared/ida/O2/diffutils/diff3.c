// Function: main @ 0x2a20
long long main(int a1, char **a2, char **a3)
{
  unsigned long long v3; // r15
  char *v5; // rdi
  int v6; // eax
  long long v7; // r12
  char *v8; // rax
  char **v9; // rbx
  char *v10; // rax
  FILE *v11; // rbp
  char *v12; // rax
  const char *v13; // rsi
  char *v14; // rax
  char *v15; // rax
  FILE *v16; // rbp
  char *v17; // rax
  char *v18; // r12
  char *v19; // rax
  const char *v20; // rdx
  const char *v21; // rcx
  FILE *v22; // rdi
  char v24; // cl
  bool v25; // dl
  char v26; // al
  char v27; // al
  long long v28; // rax
  const char **v29; // rdi
  long long v30; // rax
  char *v31; // rbx
  int v32; // ebp
  struct stat *p_buf; // rbp
  FILE *v34; // r13
  _IO_FILE *v35; // r12
  const char *v36; // r14
  char *v37; // r13
  const char *v38; // rbp
  long long v39; // rax
  signed long long v40; // rsi
  bool v41; // zf
  bool v42; // sf
  bool v43; // of
  long long v44; // rdi
  int v45; // esi
  long long v46; // rax
  long long v47; // r8
  __blkcnt_t st_blocks; // rdx
  long long v49; // rcx
  long long *v50; // rdx
  long long v51; // rdi
  struct stat *v52; // rdi
  long long v53; // rdx
  int v54; // ecx
  long long *v55; // r9
  long long *v56; // rsi
  char *v57; // r15
  const char *v58; // r8
  long long v59; // rax
  const char *v60; // rdi
  long long v61; // rax
  long long *v62; // rdx
  char *v63; // rdx
  long long v64; // r12
  long long v65; // r14
  long long v66; // rbp
  long long v67; // rax
  long long v68; // r14
  long long v69; // rax
  long long v70; // rdi
  long long v71; // rdi
  long long v72; // rbp
  long long v73; // rdi
  long long v74; // r11
  long long v75; // rdx
  long long i; // rax
  long long v77; // rcx
  long long v78; // r14
  long long v79; // rax
  long long v80; // rdx
  long long v81; // rdx
  long long j; // rax
  long long v83; // rcx
  int v84; // eax
  long long v85; // rax
  long long v86; // r15
  long long v87; // r12
  long long v88; // r14
  long long v89; // rbp
  char *v90; // r13
  const void *v91; // rdi
  const void *v92; // rsi
  size_t v93; // rdx
  char *v94; // rdx
  int v95; // ecx
  char *v96; // rbp
  char *v97; // rbx
  char *v98; // r14
  char *v99; // rax
  char *v100; // rax
  long long v102; // rbx
  size_t v103; // rdx
  const void *v104; // rdi
  char *v105; // rax
  char *IO_read_end; // rcx
  char *v107; // rdx
  int v108; // eax
  char *v109; // rax
  char *v110; // rcx
  signed long long v111; // rbx
  size_t v112; // rdx
  const void *v113; // rdi
  signed long long v114; // rbx
  size_t v115; // rdx
  const void *v116; // rdi
  char *v117; // rax
  _IO_FILE *v118; // r14
  char *IO_read_ptr; // rax
  char *IO_write_ptr; // rdx
  FILE *v121; // r14
  const char *v122; // rax
  int v123; // r15d
  long long v124; // r12
  long long v125; // rax
  struct stat *v126; // rbx
  long long v127; // rax
  int v128; // esi
  FILE *v129; // r15
  FILE *chain; // rax
  int v131; // edx
  __ino_t v132; // r12
  __nlink_t st_nlink; // r14
  long long v134; // r15
  struct stat *v135; // r9
  struct stat *v136; // r8
  const void *v137; // rbx
  char *v138; // rax
  __ino_t st_ino; // r12
  char v140; // [rsp+0h] [rbp-1F8h]
  const char *v141; // [rsp+8h] [rbp-1F0h]
  int v142; // [rsp+8h] [rbp-1F0h]
  char *v143; // [rsp+10h] [rbp-1E8h]
  int v144; // [rsp+10h] [rbp-1E8h]
  const char *v145; // [rsp+10h] [rbp-1E8h]
  long long v146; // [rsp+18h] [rbp-1E0h]
  int v147; // [rsp+18h] [rbp-1E0h]
  const char *v148; // [rsp+18h] [rbp-1E0h]
  char s; // [rsp+20h] [rbp-1D8h]
  char *sa; // [rsp+20h] [rbp-1D8h]
  const char *v151; // [rsp+28h] [rbp-1D0h]
  FILE **v152; // [rsp+28h] [rbp-1D0h]
  long long v153; // [rsp+28h] [rbp-1D0h]
  const char *v154; // [rsp+30h] [rbp-1C8h]
  char *v155; // [rsp+30h] [rbp-1C8h]
  long long v156; // [rsp+30h] [rbp-1C8h]
  const char *v157; // [rsp+30h] [rbp-1C8h]
  const char *v158; // [rsp+38h] [rbp-1C0h]
  struct stat *v159; // [rsp+38h] [rbp-1C0h]
  long long v160; // [rsp+40h] [rbp-1B8h]
  struct stat *v161; // [rsp+40h] [rbp-1B8h]
  long long v162; // [rsp+48h] [rbp-1B0h]
  long long v163; // [rsp+50h] [rbp-1A8h]
  const char **v164; // [rsp+58h] [rbp-1A0h]
  long long v165; // [rsp+60h] [rbp-198h]
  unsigned char v166; // [rsp+6Bh] [rbp-18Dh]
  char v167[8]; // [rsp+78h] [rbp-180h] BYREF
  void *ptr; // [rsp+80h] [rbp-178h] BYREF
  void *v169; // [rsp+88h] [rbp-170h] BYREF
  FILE *v170; // [rsp+90h] [rbp-168h] BYREF
  int v171[3]; // [rsp+98h] [rbp-160h]
  int v172[3]; // [rsp+A4h] [rbp-154h]
  long long v173; // [rsp+B0h] [rbp-148h] BYREF
  long long v174; // [rsp+B8h] [rbp-140h]
  long long v175; // [rsp+C0h] [rbp-138h] BYREF
  long long v176; // [rsp+C8h] [rbp-130h]
  struct stat *v177; // [rsp+D0h] [rbp-128h]
  struct stat *v178; // [rsp+D8h] [rbp-120h]
  long long v179[2]; // [rsp+E0h] [rbp-118h] BYREF
  long long v180[2]; // [rsp+F0h] [rbp-108h] BYREF
  const char *v181; // [rsp+100h] [rbp-F8h]
  const char *v182; // [rsp+108h] [rbp-F0h]
  const char *v183; // [rsp+110h] [rbp-E8h]
  struct stat buf; // [rsp+120h] [rbp-D8h] BYREF
  char v185[2]; // [rsp+1B6h] [rbp-42h] BYREF
  unsigned long long v186; // [rsp+1B8h] [rbp-40h]

  v3 = 0;
  v5 = *a2;
  v186 = __readfsqword(0x28u);
  status = 2;
  sub_50E0(v5);
  setlocale(6, "");
  bindtextdomain("diffutils", "/usr/local/share/locale");
  textdomain("diffutils");
  sub_5020(0);
  sub_7940();
  LODWORD(v141) = 0;
  while ( 2 )
  {
    v6 = getopt_long(a1, a2, "aeimvx3AEL:TX", &longopts, 0);
    if ( v6 == -1 )
    {
      v24 = byte_110E9;
      v25 = (int)v3 != 0;
      byte_110F0 = ((int)v3 != 0) & (byte_110E9 ^ 1);
      v26 = byte_110E9 & ((int)v3 == 0);
      byte_110EB |= v26;
      v27 = byte_110EF | v26;
      byte_110EF = v27;
      if ( ((unsigned int)v3 & ((int)v3 - 1)) != 0
        || (v166 = byte_110EA & byte_110E9, ((unsigned char)byte_110EA & (unsigned char)byte_110E9) != 0)
        || (int)v141 && !v27 )
      {
        sub_42A0("incompatible options");
      }
      else
      {
        v28 = optind;
        if ( a1 - optind != 3 )
        {
          if ( a1 - optind <= 2 )
          {
            sub_42A0("missing operand after '%s'");
LABEL_35:
            sub_42A0(0);
          }
          v28 = sub_42A0("extra operand '%s'");
        }
        v29 = (const char **)&a2[v28];
        v30 = (int)v141;
        v164 = v29;
        while ( (int)v30 <= 2 )
        {
          (&v181)[v30] = v29[v30];
          ++v30;
        }
        v31 = "-";
        v32 = (unsigned char)(v24 | v25);
        LODWORD(v163) = 2 - v32;
        v162 = 2 - v32;
        if ( strcmp(v29[v162], "-")
          || (v3 = (unsigned int)(v32 + 1), LODWORD(v163) = v32 + 1, strcmp(*v29, "-"))
          && (v162 = (int)v3, strcmp(v29[(int)v3], "-")) )
        {
          v171[0] = 0;
          p_buf = &buf;
          v172[0] = 0;
          v171[2] = v163;
          v171[1] = 3 - v163;
          v172[3 - (int)v163] = 1;
          v165 = 3 - (int)v163;
          v172[v162] = 2;
          v34 = (FILE *)v29;
          v35 = (_IO_FILE *)(v29 + 3);
          do
          {
            v36 = *(const char **)&v34->_flags;
            if ( strcmp(*(const char **)&v34->_flags, "-") )
            {
              if ( stat(v36, &buf) < 0 )
              {
LABEL_181:
                sub_4330(*(long long *)&v34->_flags);
LABEL_182:
                v118 = (_IO_FILE *)v3;
                v3 = (unsigned long long)v35;
                while ( 1 )
                {
                  IO_read_ptr = v35->_IO_read_ptr;
                  if ( IO_read_ptr < v35->_IO_read_end )
                  {
                    v35->_IO_read_ptr = IO_read_ptr + 1;
                    LOBYTE(IO_read_ptr) = *IO_read_ptr;
                  }
                  else
                  {
                    LODWORD(IO_read_ptr) = __uflow(v35);
                    if ( (int)IO_read_ptr == -1 && (v35->_flags & 0x30) != 0 )
                      goto LABEL_160;
                  }
                  IO_write_ptr = v118->_IO_write_ptr;
                  if ( IO_write_ptr >= v118->_IO_write_end )
                  {
                    __overflow(v118, (unsigned char)IO_read_ptr);
                  }
                  else
                  {
                    v118->_IO_write_ptr = IO_write_ptr + 1;
                    *IO_write_ptr = (char)IO_read_ptr;
                  }
                }
              }
              if ( (buf.st_mode & 0xF000) == 0x4000 )
              {
                error(2, 21, "%s", *(const char **)&v34->_flags);
LABEL_116:
                v34 = v170;
                LOBYTE(v153) = byte_110F0;
                if ( !byte_110F0 )
                {
                  s = byte_110E9;
                  if ( !byte_110E9 )
                  {
                    v121 = v170;
                    v122 = "  ";
                    if ( byte_110EE )
                      v122 = "\t";
                    v34 = stdout;
                    sa = (char *)v122;
                    while ( 1 )
                    {
                      if ( !v121 )
                      {
                        v166 = 0;
                        goto LABEL_161;
                      }
                      if ( v121->_flags == 4 )
                      {
                        v185[0] = 0;
                        v144 = 3;
                        v142 = 3;
                      }
                      else
                      {
                        v127 = (unsigned int)(v121->_flags - 5);
                        if ( (unsigned int)v127 > 2 )
                        {
                          sub_4300("internal error: invalid diff type passed to output");
                          goto LABEL_213;
                        }
                        v128 = v172[v127];
                        v185[1] = 0;
                        v142 = v128;
                        v185[0] = v128 + 49;
                        v144 = v128 == 0;
                      }
                      v123 = 0;
                      __fprintf_chk(v34, 1, "====%s\n", v185);
                      do
                      {
                        v124 = (int)v171[v123];
                        v156 = v123;
                        v125 = (long long)v121 + 16 * v124;
                        v126 = *(struct stat **)(v125 + 8);
                        p_buf = *(struct stat **)(v125 + 16);
                        v147 = v123 + 1;
                        __fprintf_chk(v34, 1, "%d:", v123 + 1);
                        if ( v126 == p_buf )
                        {
                          __fprintf_chk(v34, 1, "%ldc\n", v126);
                        }
                        else if ( (char *)v126 - (char *)p_buf == 1 )
                        {
                          __fprintf_chk(v34, 1, "%lda\n", (char *)&v126[-1].__unused[2] + 7);
                        }
                        else
                        {
                          __fprintf_chk(v34, 1, "%ld,%ldc\n", v126, p_buf);
                        }
                        if ( (long long)v126 <= (long long)p_buf && v144 != v123 )
                        {
                          v134 = 0;
                          v135 = v126;
                          v136 = p_buf;
                          v153 = v124 + 6;
                          do
                          {
                            v161 = v136;
                            v159 = v135;
                            fputs_unlocked(sa, v34);
                            v137 = *(const void **)(*((long long *)&v121->_IO_read_ptr + v153) + 8 * v134);
                            p_buf = *(struct stat **)(*((long long *)&v121->_IO_backup_base + v124) + 8 * v134++);
                            fwrite_unlocked(v137, 1u, (size_t)p_buf, v34);
                            v136 = v161;
                            v135 = v159;
                          }
                          while ( (char *)v161 - (char *)v159 >= v134 );
                          if ( *((char *)p_buf + (long long)v137 - 1) != 10 )
                          {
                            v138 = dcgettext(0, "No newline at end of file", 5);
                            __fprintf_chk(v34, 1, "\n\\ %s\n", v138);
                          }
                        }
                        v123 = v147;
                        if ( v142 == 1 )
                          v123 = dword_CD00[v156];
                      }
                      while ( v123 <= 2 );
                      v121 = v121->_chain;
                    }
                  }
                  sub_7810(*v164, "r", stdin);
                  v166 = 0;
                  v94 = 0;
                  v151 = v183;
                  v35 = stdin;
                  v3 = (unsigned long long)stdout;
                  v154 = v182;
                  v158 = v181;
                  v160 = 16 * v162;
                  v163 = 16 * v165;
LABEL_119:
                  if ( !v34 )
                    goto LABEL_182;
                  if ( v34->_flags == 4 )
                    goto LABEL_125;
                  v95 = v172[v34->_flags - 5];
                  if ( v95 == 1 )
                  {
                    LOBYTE(v96) = byte_110EB;
                    if ( !byte_110EB )
                      goto LABEL_124;
                    v110 = v34->_IO_read_ptr;
                    LODWORD(v146) = 6;
                    v143 = v110 - 1;
                    v141 = "<<<<<<< %s\n";
                    v31 = (char *)(v110 - v94 - 2);
                    if ( (long long)v31 < 0 )
                      goto LABEL_168;
                    goto LABEL_131;
                  }
                  if ( v95 != 2 )
                  {
                    if ( v95 != -1 )
                      goto LABEL_124;
LABEL_125:
                    if ( byte_110ED )
                      goto LABEL_124;
                    LOBYTE(v96) = byte_110EF;
                    v97 = v34->_IO_read_ptr;
                    v143 = v97 - 1;
                    v31 = (char *)(v97 - v94 - 2);
                    if ( (long long)v31 >= 0 )
                    {
                      LODWORD(v146) = 4;
                      v141 = "||||||| %s\n";
                      goto LABEL_131;
                    }
                    if ( !byte_110EF )
                      goto LABEL_142;
                    v141 = "||||||| %s\n";
LABEL_176:
                    v114 = 0;
                    __fprintf_chk(v3, 1, "<<<<<<< %s\n", v158);
                    while ( v114 <= v34->_IO_read_end - v34->_IO_read_ptr )
                    {
                      v115 = *(long long *)&v34->_IO_backup_base[8 * v114];
                      v116 = *(const void **)&v34->_IO_buf_base[8 * v114++];
                      fwrite_unlocked(v116, 1u, v115, (FILE *)v3);
                    }
LABEL_140:
                    if ( byte_110EB )
                    {
LABEL_168:
                      v111 = 0;
                      __fprintf_chk(v3, 1, v141, v154);
                      while ( v111 <= *(char **)((char *)&v34->_IO_read_end + v163)
                                    - *(char **)((char *)&v34->_IO_read_ptr + v163) )
                      {
                        v112 = *(long long *)(*((long long *)&v34->_IO_backup_base + v165) + 8 * v111);
                        v113 = *(const void **)(*((long long *)&v34->_IO_buf_base + v165) + 8 * v111++);
                        fwrite_unlocked(v113, 1u, v112, (FILE *)v3);
                      }
                    }
                    fputs_unlocked("=======\n", (FILE *)v3);
                    v166 = s;
                    LOBYTE(v96) = s;
                    goto LABEL_142;
                  }
                  LOBYTE(v96) = byte_110EC;
                  if ( byte_110EC )
                    goto LABEL_124;
                  v109 = v34->_IO_read_ptr;
                  LODWORD(v146) = 7;
                  v143 = v109 - 1;
                  v141 = "<<<<<<< %s\n";
                  v31 = (char *)(v109 - v94 - 2);
                  if ( (long long)v31 >= 0 )
                  {
                    do
                    {
LABEL_131:
                      v100 = v35->_IO_read_ptr;
                      if ( v100 < v35->_IO_read_end )
                      {
                        v35->_IO_read_ptr = v100 + 1;
                        LODWORD(v98) = (unsigned char)*v100;
                      }
                      else
                      {
                        LODWORD(v98) = __uflow(v35);
                        if ( (int)v98 == -1 )
                        {
                          if ( (v35->_flags & 0x20) != 0 )
                          {
LABEL_180:
                            v117 = dcgettext(0, "read failed", 5);
                            sub_4330(v117);
                            goto LABEL_181;
                          }
                          if ( (v35->_flags & 0x10) != 0 )
                            goto LABEL_135;
                        }
                      }
                      v99 = *(char **)(v3 + 40);
                      if ( (unsigned long long)v99 >= *(long long *)(v3 + 48) )
                        goto LABEL_136;
                      *(long long *)(v3 + 40) = v99 + 1;
                      *v99 = (char)v98;
                    }
                    while ( (int)v98 != 10 );
                    goto LABEL_137;
                  }
LABEL_142:
                  LOBYTE(v141) = (char)v96;
                  v102 = 0;
                  v98 = (char *)v34 + v160;
                  while ( v102 <= *((long long *)v98 + 2) - *((long long *)v98 + 1) )
                  {
                    v103 = *(long long *)(*((long long *)&v34->_IO_backup_base + v162) + 8 * v102);
                    v104 = *(const void **)(*((long long *)&v34->_IO_buf_base + v162) + 8 * v102++);
                    fwrite_unlocked(v104, 1u, v103, (FILE *)v3);
                  }
                  if ( (char)v96 )
                    __fprintf_chk(v3, 1, ">>>>>>> %s\n", v151);
                  v31 = (char *)(v34->_IO_read_end - v34->_IO_read_ptr + 1);
                  v96 = v31;
LABEL_148:
                  if ( (long long)--v96 < 0 )
                    goto LABEL_173;
                  v105 = v35->_IO_read_ptr;
                  IO_read_end = v35->_IO_read_end;
                  while ( 1 )
                  {
                    if ( v105 < IO_read_end )
                    {
                      v107 = v105 + 1;
                      v35->_IO_read_ptr = v105 + 1;
                      if ( *v105 == 10 )
                        goto LABEL_148;
                    }
                    else
                    {
                      v108 = __uflow(v35);
                      if ( v108 == 10 )
                        goto LABEL_148;
                      if ( v108 == -1 )
                      {
                        if ( (v35->_flags & 0x20) != 0 )
                          goto LABEL_180;
                        if ( (v35->_flags & 0x10) != 0 )
                        {
                          if ( !v96 && !v34->_chain )
                          {
LABEL_160:
                            if ( (stdin->_flags & 0x20) == 0 )
                              goto LABEL_161;
                            sub_4300("read failed");
LABEL_173:
                            v94 = &v143[(long long)v31];
LABEL_124:
                            v34 = v34->_chain;
                            goto LABEL_119;
                          }
LABEL_135:
                          sub_4300("input file shrank");
LABEL_136:
                          __overflow((_IO_FILE *)v3, (unsigned char)v98);
                          if ( (int)v98 != 10 )
                            goto LABEL_131;
LABEL_137:
                          if ( v31-- != 0 )
                            goto LABEL_131;
                          if ( (char)v96 )
                          {
                            if ( (int)v146 != 4 )
                              goto LABEL_140;
                            goto LABEL_176;
                          }
                          goto LABEL_142;
                        }
                      }
                      v107 = v35->_IO_read_ptr;
                      IO_read_end = v35->_IO_read_end;
                    }
                    v105 = v107;
                  }
                }
LABEL_213:
                v129 = stdout;
                v145 = v183;
                v148 = v182;
                v157 = v181;
                while ( v34 )
                {
                  chain = v34->_chain;
                  v34->_chain = (struct _IO_FILE *)p_buf;
                  p_buf = (struct stat *)v34;
                  v34 = chain;
                }
                while ( 1 )
                {
                  if ( !p_buf )
                  {
                    if ( byte_110EA )
                      fputs_unlocked("w\nq\n", v129);
LABEL_161:
                    free(ptr);
                    free(v169);
                    ((void (*)(void))sub_4690)();
                    exit(v166);
                  }
                  if ( LODWORD(p_buf->st_dev) != 4 )
                  {
                    v131 = v172[LODWORD(p_buf->st_dev) - 5];
                    if ( v131 == 1 )
                    {
                      if ( !byte_110EB )
                        goto LABEL_220;
                      st_ino = p_buf->st_ino;
                      __fprintf_chk(v129, 1, "%lda\n", p_buf->st_nlink);
                      __fprintf_chk(v129, 1, ">>>>>>> %s\n", v145);
                      sub_45F0(v129);
                      __fprintf_chk(v129, 1, "%lda\n<<<<<<< %s\n", st_ino - 1, v148);
                      sub_4500(v129);
                      fputs_unlocked("=======\n", v129);
                      goto LABEL_241;
                    }
                    if ( v131 == 2 )
                    {
                      if ( byte_110EC )
                        goto LABEL_220;
                      v132 = p_buf->st_ino;
                      st_nlink = p_buf->st_nlink;
                      goto LABEL_234;
                    }
                    if ( v131 != -1 )
                      goto LABEL_220;
                  }
                  if ( byte_110ED )
                    goto LABEL_220;
                  v132 = p_buf->st_ino;
                  st_nlink = p_buf->st_nlink;
                  if ( byte_110EF )
                  {
                    __fprintf_chk(v129, 1, "%lda\n", p_buf->st_nlink);
                    if ( byte_110EB )
                    {
                      __fprintf_chk(v129, 1, "||||||| %s\n", v148);
                      sub_4500(v129);
                    }
                    fputs_unlocked("=======\n", v129);
                    sub_4500(v129);
                    __fprintf_chk(v129, 1, ">>>>>>> %s\n", v145);
                    sub_45F0(v129);
                    __fprintf_chk(v129, 1, "%lda\n<<<<<<< %s\n", v132 - 1, v157);
LABEL_241:
                    sub_45F0(v129);
                    v166 = v153;
                    goto LABEL_220;
                  }
LABEL_234:
                  if ( *(&p_buf->st_nlink + 2 * v162) - *(&p_buf->st_ino + 2 * v162) == -1 )
                  {
                    if ( v132 == st_nlink )
                      __fprintf_chk(v129, 1, "%ldd\n", v132);
                    else
                      __fprintf_chk(v129, 1, "%ld,%ldd\n", v132, st_nlink);
                  }
                  else
                  {
                    if ( st_nlink - v132 == -1 )
                    {
                      __fprintf_chk(v129, 1, "%lda\n", st_nlink);
                    }
                    else if ( st_nlink == v132 )
                    {
                      __fprintf_chk(v129, 1, "%ldc\n", st_nlink);
                    }
                    else
                    {
                      __fprintf_chk(v129, 1, "%ld,%ldc\n", v132, st_nlink);
                    }
                    sub_4500(v129);
                    sub_45F0(v129);
                  }
LABEL_220:
                  p_buf = (struct stat *)p_buf->st_ctim.tv_sec;
                }
              }
            }
            v34 = (FILE *)((char *)v34 + 8);
          }
          while ( v35 != v34 );
          signal(17, 0);
          v37 = (char *)&unk_CD20;
          v38 = v29[v172[2]];
          v31 = (char *)sub_46E0(v29[v172[1]], v38, v167, &v169);
          v39 = sub_46E0(*v29, v38, v167, &ptr);
          v178 = (struct stat *)v31;
          v177 = (struct stat *)v39;
          v170 = 0;
          v152 = &v170;
LABEL_47:
          p_buf = v178;
          if ( v177 )
          {
            v176 = 0;
            v175 = 0;
            v174 = 0;
            v173 = 0;
            if ( v178 )
            {
              v40 = v178->st_nlink;
              v43 = __OFSUB__(v177->st_nlink, v40);
              v41 = v177->st_nlink == v40;
              v42 = (signed long long)(v177->st_nlink - v40) < 0;
              v44 = (signed long long)v177->st_nlink > v40;
              v45 = (signed long long)v177->st_nlink > v40;
              v46 = (unsigned char)(v42 ^ v43) | v41;
              p_buf = *(&v177 + v44);
              v47 = 8 * v44;
            }
            else
            {
              p_buf = v177;
              v45 = 0;
              v47 = 0;
              v46 = 1;
              v44 = 0;
            }
          }
          else
          {
            if ( !v178 )
              goto LABEL_116;
            v176 = 0;
            v46 = 0;
            v47 = 8;
            v45 = 1;
            v175 = 0;
            v44 = 1;
            v174 = 0;
            v173 = 0;
          }
          st_blocks = p_buf->st_blocks;
          v49 = *(long long *)&p_buf->st_mode;
          *(&v175 + v44) = (long long)p_buf;
          *(&v173 + v44) = (long long)p_buf;
          *(&v177 + v44) = (struct stat *)st_blocks;
          v50 = *(&v177 + v46);
          p_buf->st_blocks = 0;
          if ( v50 )
          {
            do
            {
              if ( v50[2] > v49 + 1 )
                break;
              if ( *(&v173 + v46) )
                *(long long *)(*(&v175 + v46) + 64) = v50;
              else
                *(&v173 + v46) = (long long)v50;
              v51 = (long long)*(&v177 + v46);
              *(&v175 + v46) = (long long)v50;
              v52 = *(struct stat **)(v51 + 64);
              v50[8] = 0;
              v53 = v50[3];
              *(&v177 + v46) = v52;
              LODWORD(v46) = v45 ^ 1;
              if ( v49 < v53 )
              {
                v54 = v45;
                v45 ^= 1u;
                LODWORD(v46) = v54;
                v49 = v53;
              }
              v46 = (int)v46;
              v50 = *(&v177 + (int)v46);
            }
            while ( v50 );
            v44 = v45;
          }
          v55 = v179;
          v56 = v180;
          v57 = (char *)&v175;
          v58 = *(const char **)(*(long long *)((char *)&v173 + v47) + 16);
          v59 = *(&v175 + v44);
          v60 = (const char *)&v173;
          v141 = v58;
          v31 = *(char **)(v59 + 24);
          v61 = 0;
LABEL_61:
          v62 = *(long long **)&v60[v61];
          if ( v62 )
          {
            *(long long *)((char *)v55 + v61) = &v58[*v62 - v62[2]];
            v63 = &v31[*(long long *)(*(long long *)&v57[v61] + 8LL) - *(long long *)(*(long long *)&v57[v61] + 24LL)];
            goto LABEL_63;
          }
          while ( 1 )
          {
            *(long long *)((char *)v55 + v61) = &v58[*(long long *)&v37[2 * v61 + 16] - *((long long *)v37 + 6)];
            v63 = &v31[*(long long *)&v37[2 * v61 + 16] - *((long long *)v37 + 6)];
LABEL_63:
            *(long long *)((char *)v56 + v61) = v63;
            v61 += 8;
            if ( v61 != 16 )
              goto LABEL_61;
            v64 = v179[0];
            v65 = v180[1];
            v66 = v179[1];
            v146 = v180[0];
            v67 = sub_7350(112);
            *(int *)v67 = 0;
            v37 = (char *)v67;
            *(long long *)(v67 + 104) = 0;
            *(long long *)(v67 + 8) = v64;
            *(long long *)(v67 + 24) = v66;
            *(long long *)(v67 + 16) = v146;
            *(long long *)(v67 + 32) = v65;
            *(long long *)(v67 + 40) = v141;
            *(long long *)(v67 + 48) = v31;
            if ( v146 - v64 == -1 )
            {
              *(long long *)(v67 + 56) = 0;
              *(long long *)(v67 + 80) = 0;
            }
            else
            {
              *(long long *)(v67 + 56) = sub_76B0(v146 - v64 + 1, 8);
              v85 = sub_76B0(v146 - v64 + 1, 8);
              v65 = *((long long *)v37 + 4);
              v66 = *((long long *)v37 + 3);
              *((long long *)v37 + 10) = v85;
            }
            v68 = v65 - v66 + 1;
            if ( v68 )
            {
              *((long long *)v37 + 8) = sub_76B0(v68, 8);
              v69 = sub_76B0(v68, 8);
            }
            else
            {
              *((long long *)v37 + 8) = 0;
              v69 = 0;
            }
            v70 = *((long long *)v37 + 6) - *((long long *)v37 + 5);
            *((long long *)v37 + 11) = v69;
            v71 = v70 + 1;
            if ( v71 )
            {
              *((long long *)v37 + 9) = sub_76B0(v71, 8);
              v72 = sub_76B0(v71, 8);
            }
            else
            {
              *((long long *)v37 + 9) = 0;
              v72 = 0;
            }
            *((long long *)v37 + 12) = v72;
            v155 = v57;
            v143 = (char *)&v173;
            while ( 1 )
            {
              v57 = *(char **)v143;
              if ( *(long long *)v143 )
                break;
LABEL_77:
              v143 += 8;
              if ( v155 == v143 )
              {
                v73 = v146;
                v74 = 0;
                while ( 2 )
                {
                  v75 = v64;
                  v31 = *(char **)((char *)&v173 + v74);
                  for ( i = 0; ; i += 8 )
                  {
                    v77 = v73 + 1;
                    if ( v31 )
                      v77 = *(long long *)v31;
                    if ( v77 <= v75 )
                      break;
                    ++v75;
                    *(long long *)(*(long long *)&v37[v74 + 56] + i) = *(long long *)(*((long long *)v37 + 9) + i);
                    *(long long *)(*(long long *)&v37[v74 + 80] + i) = *(long long *)(v72 + i);
                  }
                  if ( v31 )
                  {
                    v146 = v74;
                    v57 = *(char **)&v37[v74 + 56];
                    v78 = *(long long *)&v37[v74 + 80];
                    v143 = (char *)(v73 + 1 - v64);
LABEL_86:
                    v56 = (long long *)*((long long *)v31 + 6);
                    if ( (unsigned char)sub_4460(
                                            *((long long *)v31 + 4),
                                            v56,
                                            &v57[8 * (*(long long *)v31 - v64)],
                                            v78 + 8 * (*(long long *)v31 - v64),
                                            *((long long *)v31 + 1) - *(long long *)v31 + 1LL) )
                    {
                      v79 = *((long long *)v31 + 1);
                      v80 = *((long long *)v31 + 3) - (long long)v141;
                      v31 = (char *)*((long long *)v31 + 8);
                      v81 = 8 * v80 + 8;
                      for ( j = v79 + 1 - v64; ; ++j )
                      {
                        if ( v31 )
                        {
                          if ( j >= *(long long *)v31 - v64 )
                            goto LABEL_86;
                        }
                        else if ( j >= (long long)v143 )
                        {
                          v74 = v146;
                          goto LABEL_93;
                        }
                        *(long long *)&v57[8 * j] = *(long long *)(*((long long *)v37 + 9) + v81);
                        v83 = *(long long *)(v72 + v81);
                        v81 += 8;
                        *(long long *)(v78 + 8 * j) = v83;
                      }
                    }
                    goto LABEL_75;
                  }
LABEL_93:
                  v74 += 8;
                  if ( v74 != 16 )
                  {
                    v64 = *(long long *)((char *)v179 + v74);
                    v73 = *(long long *)((char *)v180 + v74);
                    continue;
                  }
                  break;
                }
                v84 = 6;
                if ( !v173 )
                  goto LABEL_97;
                if ( !v174 )
                {
                  v84 = 5;
                  goto LABEL_97;
                }
                v84 = 4;
                v31 = (char *)(*((long long *)v37 + 2) - *((long long *)v37 + 1));
                if ( v31 != (char *)(*((long long *)v37 + 4) - *((long long *)v37 + 3)) )
                  goto LABEL_97;
                v141 = v37;
                v86 = *((long long *)v37 + 11);
                v87 = *((long long *)v37 + 8);
                v88 = *((long long *)v37 + 10);
                v89 = *((long long *)v37 + 7);
                v90 = 0;
                ++v31;
                do
                {
                  if ( v31 == v90 )
                  {
                    v37 = (char *)v141;
                    v84 = 7;
                    goto LABEL_97;
                  }
                  v91 = *(const void **)(v89 + 8LL * (long long)v90);
                  if ( !v91 )
                    break;
                  v92 = *(const void **)(v87 + 8LL * (long long)v90);
                  if ( !v92 )
                    break;
                  v93 = *(long long *)(v88 + 8LL * (long long)v90);
                  if ( v93 != *(long long *)(v86 + 8LL * (long long)v90) )
                    break;
                  ++v90;
                }
                while ( !memcmp(v91, v92, v93) );
                v37 = (char *)v141;
                v84 = 4;
LABEL_97:
                *(int *)v37 = v84;
                *v152 = (FILE *)v37;
                v152 = (FILE **)(v37 + 104);
                goto LABEL_47;
              }
            }
            v31 = (char *)*((long long *)v37 + 9);
            while ( 1 )
            {
              v56 = (long long *)*((long long *)v57 + 7);
              if ( !(unsigned char)sub_4460(
                                       *((long long *)v57 + 5),
                                       v56,
                                       &v31[8 * (*((long long *)v57 + 2) - (long long)v141)],
                                       v72 + 8LL * (*((long long *)v57 + 2) - (long long)v141),
                                       *((long long *)v57 + 3) - *((long long *)v57 + 2) + 1LL) )
                break;
              v57 = (char *)*((long long *)v57 + 8);
              if ( !v57 )
                goto LABEL_77;
            }
LABEL_75:
            v60 = "internal error: screwup in format of diff blocks";
            v61 = sub_4300("internal error: screwup in format of diff blocks");
          }
        }
      }
      sub_4300("'-' specified for more than one input file");
      start();
    }
    switch ( v6 )
    {
      case 51:
        byte_110ED = 1;
        v3 = (unsigned int)v3 | 1;
        continue;
      case 65:
        byte_110EB = 1;
        v3 = (unsigned int)v3 | 2;
        byte_110EF = 1;
        continue;
      case 69:
        byte_110EF = 1;
        v3 = (unsigned int)v3 | 4;
        continue;
      case 76:
        if ( (int)v141 > 2 )
          sub_42A0("too many file label options");
        (&v181)[(int)v141] = (const char *)optarg;
        LODWORD(v141) = (int)v141 + 1;
        continue;
      case 84:
        byte_110EE = 1;
        continue;
      case 88:
        byte_110EC = 1;
        v3 = (unsigned int)v3 | 8;
        continue;
      case 97:
        byte_110F2 = 1;
        continue;
      case 101:
        v3 = (unsigned int)v3 | 0x10;
        continue;
      case 105:
        byte_110EA = 1;
        continue;
      case 109:
        byte_110E9 = 1;
        continue;
      case 118:
        sub_5580("Randy Smith");
        v22 = stdout;
        sub_7150(stdout, v140);
        sub_4690(v22, "diff3");
        return 0;
      case 120:
        byte_110EC = 1;
        v3 = (unsigned int)v3 | 0x20;
        continue;
      case 128:
        file = (char *)optarg;
        continue;
      case 129:
        v7 = qword_21140;
        v8 = dcgettext(0, "Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n", 5);
        v9 = &off_108A0;
        __printf_chk(1, v8, v7);
        v10 = dcgettext(0, "Compare three files line by line.", 5);
        __printf_chk(1, "%s\n\n", v10);
        v11 = stdout;
        v12 = dcgettext(0, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
        fputs_unlocked(v12, v11);
        v13 = "-A, --show-all              output all changes, bracketing conflicts";
        break;
      case 130:
        byte_110F1 = 1;
        continue;
      default:
        goto LABEL_35;
    }
    break;
  }
  do
  {
    while ( *v13 )
    {
      ++v9;
      v15 = dcgettext(0, v13, 5);
      __printf_chk(1, "  %s\n", v15);
      v13 = *v9;
      if ( !*v9 )
        goto LABEL_11;
    }
    v14 = stdout->_IO_write_ptr;
    if ( v14 >= stdout->_IO_write_end )
    {
      __overflow(stdout, 10);
    }
    else
    {
      stdout->_IO_write_ptr = v14 + 1;
      *v14 = 10;
    }
    v13 = v9[1];
    ++v9;
  }
  while ( v13 );
LABEL_11:
  v16 = stdout;
  v17 = dcgettext(
          0,
          "\n"
          "The default output format is a somewhat human-readable representation of\n"
          "the changes.\n"
          "\n"
          "The -e, -E, -x, -X (and corresponding long) options cause an ed script\n"
          "to be output instead of the default.\n"
          "\n"
          "Finally, the -m (--merge) option causes diff3 to do the merge internally\n"
          "and output the actual merged file.  For unusual input, this is more\n"
          "robust than using ed.\n",
          5);
  fputs_unlocked(v17, v16);
  v18 = dcgettext(0, "Exit status is 0 if successful, 1 if conflicts, 2 if trouble.", 5);
  v19 = dcgettext(0, "If a FILE is '-', read standard input.", 5);
  __printf_chk(1, "\n%s\n%s\n", v19, v18);
  sub_7230();
  sub_4690(1, "\n%s\n%s\n", v20, v21);
  return 0;
}



// Function: try_help @ 0x42a0
long long try_help(char *msgid, long long a2)
{
  char *v2; // rax
  long long v3; // r12
  char *v4; // rax

  if ( msgid )
  {
    v2 = dcgettext(0, msgid, 5);
    error(0, 0, v2, a2);
  }
  v3 = qword_21140;
  v4 = dcgettext(0, "Try '%s --help' for more information.", 5);
  error(2, 0, v4, v3);
  return sub_4300((char *)&dword_0 + 2);
}



// Function: fatal @ 0x4300
long long fatal(char *msgid)
{
  char *v1; // rax

  v1 = dcgettext(0, msgid, 5);
  error(2, 0, "%s", v1);
  return sub_4330(2);
}



// Function: perror_with_exit @ 0x4330
long long perror_with_exit(const char *a1)
{
  int *v1; // rax

  v1 = __errno_location();
  error(2, *v1, "%s", a1);
  return sub_4360();
}



// Function: scan_diff_line @ 0x4360
long long scan_diff_line(char *a1, long long *a2, long long *a3, unsigned long long a4, char a5)
{
  char *v5; // rdi
  char *i; // rbx
  char *v7; // r12
  char v9; // al
  unsigned char *v10; // r12
  int v11; // esi
  char *IO_write_ptr; // rax
  long long v13; // rdx
  long long v14; // rcx
  long long v15; // r8

  if ( *a1 == a5 && a1[1] == 32 )
  {
    v5 = a1 + 2;
    *a2 = v5;
    for ( i = v5; ; ++i )
    {
      v7 = i + 1;
      if ( *i == 10 )
        break;
    }
    *a3 = v7 - v5;
    if ( (unsigned long long)v7 < a4 && *v7 == 92 )
    {
      v9 = byte_110F0;
      if ( byte_110F0 )
      {
        __fprintf_chk(stderr, 1, "%s:", (const char *)qword_21140);
        v9 = byte_110F0;
      }
      else
      {
        *a3 = i - v5;
      }
      v10 = i + 2;
      while ( 1 )
      {
        if ( v9 )
        {
          v11 = *v10;
          IO_write_ptr = stderr->_IO_write_ptr;
          if ( IO_write_ptr >= stderr->_IO_write_end )
          {
            __overflow(stderr, v11);
          }
          else
          {
            stderr->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = v11;
          }
        }
        if ( *v10++ == 10 )
          break;
        v9 = byte_110F0;
      }
      return (long long)v10;
    }
    else
    {
      return (long long)(i + 1);
    }
  }
  else
  {
    sub_4300("invalid diff format; incorrect leading line chars");
    return sub_4460("invalid diff format; incorrect leading line chars", a2, v13, v14, v15);
  }
}



// Function: copy_stringlist @ 0x4460
long long copy_stringlist(long long a1, long long a2, long long a3, long long a4, long long a5)
{
  long long v9; // rbx
  const void *v10; // rsi
  size_t v11; // rdx

  if ( !a5 )
    return 1;
  v9 = 0;
  while ( 1 )
  {
    while ( 1 )
    {
      v10 = *(const void **)(a3 + 8 * v9);
      v11 = *(long long *)(a2 + 8 * v9);
      if ( v10 )
        break;
      *(long long *)(a3 + 8 * v9) = *(long long *)(a1 + 8 * v9);
      *(long long *)(a4 + 8 * v9++) = v11;
      if ( v9 == a5 )
        return 1;
    }
    if ( *(long long *)(a4 + 8 * v9) != v11 || memcmp(*(const void **)(a1 + 8 * v9), v10, v11) )
      break;
    if ( ++v9 == a5 )
      return 1;
  }
  return 0;
}



// Function: dotlines @ 0x4500
long long dotlines(_IO_FILE *a1, long long a2, int a3)
{
  long long v3; // r15
  long long v4; // r12
  long long v5; // rbx
  long long v6; // rdx
  char *v7; // rbp
  char *IO_write_ptr; // rax
  unsigned char v10; // [rsp+7h] [rbp-41h]

  v3 = a3;
  v4 = a2 + 16LL * a3;
  if ( *(long long *)(v4 + 16) < *(long long *)(v4 + 8) )
  {
    return 0;
  }
  else
  {
    v10 = 0;
    v5 = 0;
    do
    {
      v6 = 8 * v5;
      v7 = *(char **)(*(long long *)(a2 + 8 * v3 + 56) + 8 * v5);
      if ( *v7 == 46 )
      {
        IO_write_ptr = a1->_IO_write_ptr;
        if ( IO_write_ptr >= a1->_IO_write_end )
        {
          __overflow(a1, 46);
          v10 = 1;
          v6 = 8 * v5;
        }
        else
        {
          v10 = 1;
          a1->_IO_write_ptr = IO_write_ptr + 1;
          *IO_write_ptr = 46;
        }
      }
      ++v5;
      fwrite_unlocked(v7, 1u, *(long long *)(*(long long *)(a2 + 8 * v3 + 80) + v6), a1);
    }
    while ( *(long long *)(v4 + 16) - *(long long *)(v4 + 8) >= v5 );
  }
  return v10;
}



// Function: process_diff @ 0x46e0
long long process_diff(const char *a1, const char *a2, long long **a3, char **a4)
{
  char *v4; // r12
  long long *v5; // r15
  const char **v6; // rax
  const char *v7; // rbx
  int v8; // ebp
  unsigned long long st_blksize; // r13
  long long v10; // rbx
  char *i; // r14
  long long v12; // r12
  long long v13; // rax
  int v14; // eax
  unsigned long long v15; // rbx
  long long *v16; // rax
  int v17; // edx
  char *v18; // rax
  long long v19; // rcx
  char j; // dl
  char v21; // dl
  long long v22; // r12
  char *v23; // rax
  unsigned char v24; // al
  char *IO_write_ptr; // rdx
  int v26; // r12d
  char *v27; // rdx
  int k; // eax
  long long v29; // rsi
  char m; // al
  char v31; // al
  long long v32; // rbp
  long long v33; // r13
  long long v34; // rax
  long long v35; // rdx
  long long ii; // r13
  long long v37; // rbp
  long long v38; // r12
  long long v39; // rax
  long long v40; // rdx
  long long n; // r12
  char *v43; // rsi
  int v44; // eax
  long long v45; // rdx
  int v46; // eax
  long long v47; // rdi
  const char *v48; // rsi
  char *v49; // rax
  char *v50; // rax
  int *v51; // rax
  __pid_t pid; // [rsp+8h] [rbp-150h]
  int *pida; // [rsp+8h] [rbp-150h]
  int stat_loc[2]; // [rsp+20h] [rbp-138h] BYREF
  int pipedes[2]; // [rsp+28h] [rbp-130h] BYREF
  char *argv; // [rsp+30h] [rbp-128h] BYREF
  const char *v59; // [rsp+38h] [rbp-120h] BYREF
  char v60; // [rsp+40h] [rbp-118h] BYREF
  struct stat buf; // [rsp+80h] [rbp-D8h] BYREF
  unsigned long long v62; // [rsp+118h] [rbp-40h]

  v62 = __readfsqword(0x28u);
  argv = file;
  v6 = &v59;
  if ( byte_110F2 )
  {
    v59 = "-a";
    v6 = (const char **)&v60;
  }
  if ( byte_110F1 )
    *v6++ = "--strip-trailing-cr";
  v6[2] = a1;
  *v6 = "--horizon-lines=100";
  v7 = "--";
  v6[1] = "--";
  v6[3] = a2;
  v6[4] = 0;
  if ( pipe(pipedes) )
    goto LABEL_108;
  pid = fork();
  if ( !pid )
    goto LABEL_109;
  if ( pid == -1 )
  {
    sub_4330("fork");
    goto LABEL_102;
  }
  close(pipedes[1]);
  v8 = pipedes[0];
  if ( fstat(pipedes[0], &buf) )
  {
    sub_4330("fstat");
LABEL_108:
    sub_4330("pipe");
LABEL_109:
    close(pipedes[0]);
    if ( pipedes[1] != 1 )
    {
      dup2(pipedes[1], 1);
      close(pipedes[1]);
    }
    execvp(file, &argv);
    v51 = __errno_location();
    _exit((*v51 == 2) + 126);
  }
  st_blksize = buf.st_blksize;
  if ( buf.st_blksize <= 0 )
    st_blksize = 1;
  v10 = 0;
  for ( i = (char *)sub_7350(st_blksize); ; i = (char *)sub_73A0(i, st_blksize) )
  {
    v12 = st_blksize - v10;
    v13 = sub_7A10(v8, &i[v10]);
    v10 += v13;
    if ( v12 != v13 )
      break;
    if ( st_blksize > 0x3FFFFFFFFFFFFFFELL )
LABEL_95:
      sub_77D0();
    st_blksize *= 2LL;
  }
  if ( v13 == -1 )
    goto LABEL_98;
  if ( v10 && i[v10 - 1] != 10 )
    return sub_4300("invalid diff format; incomplete last line");
  if ( close(v8) )
  {
LABEL_113:
    sub_4330("close");
    return sub_4300("invalid diff format; incomplete last line");
  }
  if ( waitpid(pid, stat_loc, 0) < 0 )
  {
    sub_4330("waitpid");
    goto LABEL_113;
  }
  if ( (stat_loc[0] & 0x7F) != 0 )
  {
    v4 = file;
    LODWORD(v7) = 0x7FFFFFFF;
    v48 = "subsidiary program '%s' failed";
    goto LABEL_97;
  }
  v14 = stat_loc[0] >> 8;
  if ( (stat_loc[0] & 0xFE00) != 0 )
  {
    while ( 1 )
    {
      v4 = file;
      LODWORD(v7) = (unsigned char)v14;
      if ( (char)v14 == 126 )
      {
        v48 = "subsidiary program '%s' could not be invoked";
      }
      else
      {
LABEL_102:
        v48 = "subsidiary program '%s' failed (exit status %d)";
        if ( (int)v7 == 127 )
          v48 = "subsidiary program '%s' not found";
      }
LABEL_97:
      v49 = dcgettext(0, v48, 5);
      error(2, 0, v49, v4, (unsigned int)v7);
LABEL_98:
      v50 = dcgettext(0, "read failed", 5);
      LOBYTE(v14) = sub_4330(v50);
    }
  }
  v15 = (unsigned long long)&i[v10];
  pida = stat_loc;
  *a4 = i;
  while ( v15 > (unsigned long long)i )
  {
    v16 = (long long *)sub_7350(72);
    v16[5] = 0;
    v5 = v16;
    v16[4] = 0;
    v16[7] = 0;
    v16[6] = 0;
    v17 = (unsigned char)*i;
    v18 = i;
    if ( (char)v17 != 32 )
      goto LABEL_25;
    do
    {
      do
        v17 = (unsigned char)*++v18;
      while ( (char)v17 == 32 );
LABEL_25:
      ;
    }
    while ( (char)v17 == 9 );
    if ( (unsigned int)(v17 - 48) > 9 )
      goto LABEL_41;
    v19 = 0;
    do
    {
      ++v18;
      v19 = v17 - 48 + 10 * v19;
      v17 = (unsigned char)*v18;
    }
    while ( (unsigned int)(v17 - 48) <= 9 );
    *v5 = v19;
    for ( j = *v18; j == 32; j = *++v18 )
LABEL_30:
      ;
    if ( j == 9 )
      goto LABEL_30;
    if ( j == 44 )
    {
      v43 = v18 + 1;
      v44 = (unsigned char)v18[1];
      if ( (unsigned int)(v44 - 48) > 9 )
        goto LABEL_41;
      v45 = 0;
      do
      {
        ++v43;
        v45 = v44 - 48 + 10 * v45;
        v44 = (unsigned char)*v43;
      }
      while ( (unsigned int)(v44 - 48) <= 9 );
      v5[1] = v45;
      v18 = v43;
    }
    else
    {
      v5[1] = v19;
    }
    v21 = *v18;
    if ( *v18 == 9 || v21 == 32 )
    {
      do
      {
        do
          v21 = *++v18;
        while ( v21 == 32 );
      }
      while ( v21 == 9 );
    }
    switch ( v21 )
    {
      case 'c':
        v26 = 2;
        break;
      case 'd':
        v26 = 3;
        break;
      case 'a':
        v26 = 1;
        break;
      default:
        goto LABEL_41;
    }
    v27 = v18 + 1;
    for ( k = (unsigned char)v18[1]; (char)k == 32; k = (unsigned char)*++v27 )
LABEL_48:
      ;
    if ( (char)k == 9 )
      goto LABEL_48;
    if ( (unsigned int)(k - 48) > 9 )
      goto LABEL_41;
    v29 = 0;
    do
    {
      ++v27;
      v29 = k - 48 + 10 * v29;
      k = (unsigned char)*v27;
    }
    while ( (unsigned int)(k - 48) <= 9 );
    v5[2] = v29;
    for ( m = *v27; m == 32; m = *++v27 )
LABEL_54:
      ;
    if ( m == 9 )
      goto LABEL_54;
    if ( m == 44 )
    {
      v46 = (unsigned char)v27[1];
      if ( (unsigned int)(v46 - 48) > 9 )
        goto LABEL_41;
      ++v27;
      v47 = 0;
      do
      {
        ++v27;
        v47 = v46 - 48 + 10 * v47;
        v46 = (unsigned char)*v27;
      }
      while ( (unsigned int)(v46 - 48) <= 9 );
      v5[3] = v47;
      v31 = *v27;
      if ( *v27 == 9 || v31 == 32 )
      {
        do
        {
          do
            v31 = *++v27;
          while ( v31 == 32 );
        }
        while ( v31 == 9 );
      }
    }
    else
    {
      v5[3] = v29;
      v31 = *v27;
    }
    i = v27 + 1;
    if ( v31 != 10 )
      goto LABEL_106;
    if ( v26 == 2 )
    {
      v19 = *v5;
    }
    else
    {
      if ( v26 != 3 )
      {
        *v5 = v19 + 1;
LABEL_70:
        v37 = v5[3] - v5[2];
        v38 = v37 + 1;
        if ( (unsigned long long)(v37 + 1) > 0xFFFFFFFFFFFFFFELL )
          goto LABEL_95;
        v5[5] = sub_7350(8 * v38);
        v39 = sub_7350(8 * v38);
        v5[7] = v39;
        v40 = v39;
        if ( v37 != -1 )
        {
          for ( n = 0; ; ++n )
          {
            i = (char *)sub_4360(i, (long long *)(v5[5] + 8 * n), (long long *)(8 * n + v40), v15, 62);
            if ( v37 == n )
              break;
            v40 = v5[7];
          }
        }
        goto LABEL_75;
      }
      v5[2] = v29 + 1;
    }
    v32 = v5[1] - v19;
    v33 = v32 + 1;
    if ( (unsigned long long)(v32 + 1) > 0xFFFFFFFFFFFFFFELL )
      goto LABEL_95;
    v5[4] = sub_7350(8 * v33);
    v34 = sub_7350(8 * v33);
    v5[6] = v34;
    v35 = v34;
    if ( v32 != -1 )
    {
      for ( ii = 0; ; ++ii )
      {
        i = (char *)sub_4360(i, (long long *)(v5[4] + 8 * ii), (long long *)(8 * ii + v35), v15, 60);
        if ( v32 == ii )
          break;
        v35 = v5[6];
      }
    }
    if ( v26 == 2 )
    {
      if ( strncmp(i, "---\n", 4u) )
      {
        sub_4300("invalid diff format; invalid change separator");
LABEL_106:
        i = v27;
LABEL_41:
        v22 = qword_21140;
        v23 = dcgettext(0, "%s: diff failed: ", 5);
        __fprintf_chk(stderr, 1, v23, v22);
        while ( 1 )
        {
          v24 = *i;
          IO_write_ptr = stderr->_IO_write_ptr;
          if ( IO_write_ptr >= stderr->_IO_write_end )
          {
            __overflow(stderr, v24);
          }
          else
          {
            stderr->_IO_write_ptr = IO_write_ptr + 1;
            *IO_write_ptr = v24;
          }
          if ( *i++ == 10 )
            exit(2);
        }
      }
      i += 4;
      goto LABEL_70;
    }
LABEL_75:
    *(long long *)pida = v5;
    pida = (int *)(v5 + 8);
  }
  *(long long *)pida = 0;
  *a3 = v5;
  return *(long long *)stat_loc;
}


