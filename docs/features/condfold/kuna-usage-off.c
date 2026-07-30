void usage(int4 status)

{
  uint1 *v1;
  uint1 *v10;
  int8 v11; // fs_offset
  bool v12; // cf
  bool v13;
  char *v14; // stack - 0xa0
  char *v15; // stack - 0x98
  char *v16; // stack - 0x90
  char *v17; // stack - 0x88
  char *v18; // stack - 0x80
  char *v19; // stack - 0x78
  uint1 *v2;
  char *v20; // stack - 0x70
  char *v21; // stack - 0x68
  char *v22; // stack - 0x60
  char *v23; // stack - 0x58
  char *v24; // stack - 0x50
  unsigned long v25; // stack - 0x48
  unsigned long v26; // stack - 0x40
  unsigned long v27; // stack - 0x30
  unsigned long v3;
  char *v4; // rax
  int8 v5;
  void *v6; // rdx
  void *v7;
  unsigned long v8; // stack - 0xa8
  uint1 *v9;
  
  v3 = program_name;
  v27 = *(void *)(v11 + 0x28);
  __printf_chk(1,dcgettext(0,"Usage: %s [ignored command line arguments]\n  or:  %s OPTION\n",5),v3,v3);
  __printf_chk(1,0x4769,dcgettext(0,"Exit with a status code indicating success.",5));
  fputs_unlocked(dcgettext(0,"      --help     display this help and exit\n",5),stdout);
  fputs_unlocked(dcgettext(0,"      --version  output version information and exit\n",5),stdout);
  __printf_chk(1,dcgettext(0,"\nNOTE: your shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell\'s documentation\nfor details about the options it supports.\n",5),0x4764);
  v10 = (uint1 *)0x47e8;
  v25 = 0;
  v26 = 0;
  v8 = 0x476e;
  v19 = "sha256sum";
  v15 = "coreutils";
  v14 = "test invocation";
  v21 = "sha384sum";
  v16 = "Multi-call invocation";
  v23 = "sha512sum";
  v17 = "sha224sum";
  v18 = "sha2 utilities";
  v20 = "sha2 utilities";
  v22 = "sha2 utilities";
  v24 = "sha2 utilities";
  v7 = &v8;
  while( true ) {
    v6 = (void *)((int8)v7 + 0x10);
    v12 = 0;
    if (v10 == (uint1 *)0x0) break;
    v5 = 5;
    v13 = 0;
    v9 = (uint1 *)0x4764;
    do {
      if (v5 == 0) break;
      v5 = v5 + -1;
      v2 = &v10[1];
      v1 = &v9[1];
      v12 = *v9 < *v10;
      v13 = *v9 == *v10;
      v9 = v1;
      v10 = v2;
    } while (v13);
    if ((!v12 && !v13) == v12) break;
    v10 = *(uint1 **)((int8)v7 + 0x20);
    v7 = (void *)v6;
  }
  v5 = *(int8 *)((int8)v7 + 0x18);
                    /* WARNING: branchflip: flipped negated guard for linearity (positive condition, if/else arms swapped) */
  if (v5 != 0) {
    __printf_chk(1,dcgettext(0,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,(char *)0x0);
    if (v4 != (char *)0x0) {
      if (strncmp(v4,"en_",3) != 0) {
label_18e8:
        fputs_unlocked(dcgettext(0,"Report any translation bugs to <https://translationproject.org/team/>\n",5),stdout);
      }
    }
    __printf_chk(1,dcgettext(0,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",0x4764);
    v4 = "";
    if (v5 == 0x4764) {
      v4 = " invocation";
    }
  }
  else {
    __printf_chk(1,dcgettext(0,"\n%s online help: <%s>\n",5),"GNU coreutils","https://www.gnu.org/software/coreutils/");
    v4 = setlocale(5,(char *)0x0);
    if (v4 != (char *)0x0) {
      if (strncmp(v4,"en_",3) != 0) {
        v5 = 0x4764;
        goto label_18e8;
      }
    }
    v5 = 0x4764;
    v4 = " invocation";
    __printf_chk(1,dcgettext(0,"Full documentation <%s%s>\n",5),"https://www.gnu.org/software/coreutils/",0x4764);
  }
  __printf_chk(1,dcgettext(0,"or available locally via: info \'(coreutils) %s%s\'\n",5),v5,v4);
                    /* WARNING: Subroutine does not return */
  exit(status);
}
