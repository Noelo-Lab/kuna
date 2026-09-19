// Function: main @ 0x26a0
int main(int argc,char **argv,char **envp)
{
  bool v1;
  char **v10;
  char *v11; // stack - 0x48
  char *v12; // stack - 0x40
  int v2;
  unsigned int v3; // eax
  int v4; // eax
  char *v5; // rax
  FILE *v6; // rax
  char *v7;
  unsigned long v8; // rax
  unsigned int v9; // eax
  
  sub_4350(*argv);
  setlocale(6,"");
  bindtextdomain("coreutils","/usr/local/share/locale");
  textdomain("coreutils");
  sub_7b90(sub_4100);
  dat_17130 = 0;
  dat_17131 = 0;
  dat_17132 = 0;
  dat_17133 = 0;
  dat_17120 = 0x4b;
  dat_17128 = "";
  dat_1711c = 0;
  dat_17118 = 0;
  dat_17114 = 0;
  v12 = NULL;
  v10 = argv;
  if (((2 <= argc) && (v7 = argv[1], *v7 == '-')) && ((unsigned int)((int)v7[1] - 0x30U) <= 9)) {
    v12 = &v7[1];
    v10 = &argv[1];
    argc -= 1;
    *v10 = *argv;
  }
  v11 = NULL;
  while (v2 = getopt_long(argc,v10,"0123456789cstuw:p:g:",(option *)0xbaa0,NULL), v2 != -1) {
    if (0x77 < v2) {
label_2acd:
      if ((unsigned int)(v2 - 0x30U) <= 9)
        error(0,0,dcgettext(NULL,"invalid option -- %c; -WIDTH is recognized only when it is the first\noption; use -w N instead",5),v2);
      sub_3d70(1); // no-return
    }
    if (v2 <= 0x62) {
      if (v2 == -0x83) {
        sub_6c80(stdout,"fmt","GNU coreutils",dat_c010,"Ross Paterson",0);
        exit(0); // no-return
      }
      if (v2 == -0x82)
        sub_3d70(0); // no-return
      goto label_2acd;
    }
    switch(v2) {
      case 99:
        dat_17133 = 1;
        break;
      default:
        goto label_2acd;
      case 0x67:
        v11 = optarg;
        break;
      case 0x70:
        dat_17118 = 0;
        v7 = optarg;
        if (*optarg == ' ') {
          do {
            dat_17118 = (1U - (int)optarg) + (int)v7;
            v7 = &v7[1];
          } while (*v7 == ' ');
        }
        dat_17128 = v7;
        dat_1711c = (int)strlen(v7);
        v5 = &v7[dat_1711c];
        if (v7 < v5) {
          do {
            if (v5[-1] != ' ') break;
            v5 = &v5[-1];
          } while (v7 != v5);
        }
        *v5 = '\0';
        dat_17114 = (int)v5 - (int)v7;
        break;
      case 0x73:
        dat_17131 = 1;
        break;
      case 0x74:
        dat_17132 = 1;
        break;
      case 0x75:
        dat_17130 = 1;
        break;
      case 0x77:
        v12 = optarg;
      
    }
  }
  if (v12) { // branch-flip
    v2 = sub_74a0(v12,0,0x9c4,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = v2;
    if (v11) // branch-flip
      dat_17110 = sub_74a0(v11,0,(long)v2,"",dcgettext(NULL,"invalid width",5),0);
    else {
label_2a19:
      dat_17110 = (dat_17120 * 0xbb) / 200;
    }
  }
  else {
    if (!v11) goto label_2a19;
    dat_17110 = sub_74a0(v11,0,0x4b,"",dcgettext(NULL,"invalid width",5),0);
    dat_17120 = dat_17110 + 10;
  }
  if (optind != argc) { // branch-flip
    v1 = 0;
    v9 = 1;
    v2 = optind;
    if (argc <= optind) {
      return (v9 ^ 1) & 0xff; // return-dupe
    }
    do {
      v7 = v10[v2];
      if (strcmp(v7,"-")) { // branch-flip
        v6 = fopen(v7,"r");
        if (v6) // branch-flip
          v9 &= sub_3700(v6,v7);
        else {
          v8 = sub_6030(4,v7);
          v7 = dcgettext(NULL,"cannot open %s for reading",5);
          v9 = 0;
          error(0,*__errno_location(),v7,v8);
        }
      }
      else {
        v3 = sub_3700(stdin,v7);
        v1 = 1;
        v9 &= v3;
      }
      v2 = optind + 1;
      optind = v2;
    } while (v2 < argc);
    if (!v1) {
      return (v9 ^ 1) & 0xff;
    }
  }
  else {
    v9 = sub_3700(stdin,"-");
  }
  v2 = sub_41f0(stdin);
  if (v2) {
    v7 = dcgettext(NULL,"closing standard input",5);
    error(1,*__errno_location(),"%s",v7);
    return v4;
  }
  return (v9 ^ 1) & 0xff;
}

