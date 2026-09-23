// Function: main @ 0x4f20
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct option {
    char * name;
    int has_arg;
    int * flag;
    int val;
} option;

extern char g_41c29f;
extern char g_41e5b0;
extern struct struct_0 *g_424f40[2];
extern option g_425180;
extern long long g_4260a8;
extern unsigned int g_4260b0;
extern struct_1 *stdout;
extern unsigned int optind;
extern char optarg;
extern char g_426180;
extern unsigned long long g_426400;
extern char g_426408;
extern long long g_426410;
extern char g_426418;
extern unsigned long long g_426420;
extern unsigned long long g_426428;
extern unsigned long long g_426430;
extern char g_426438;
extern char g_426439;
extern char g_42643a;
extern unsigned long long g_426440;
extern unsigned long long g_426448;
extern unsigned long long g_426450;
extern unsigned long long g_426458;
extern void g_426460;
extern long long g_426468;
extern long long g_426470;
extern unsigned long long g_426478;
extern void g_426480;
extern long long g_426488;
extern char g_426490;
extern char g_426491;
extern char g_426492;
extern unsigned long long g_426498;
extern char g_4264a0;
extern char g_4264a1;
extern char g_4264c0;
extern char g_426500;
extern unsigned long long g_426540;
extern unsigned long long g_426548;
extern char g_426550;
extern char g_426551;
extern char g_426552;
extern char g_426553;
extern char g_426554;
extern unsigned int g_426558;
extern unsigned long long g_426560;
extern char g_426568;
extern unsigned long long g_426570;
extern char g_426578;
extern unsigned int g_42657c;
extern unsigned int g_426580;
extern char g_426584;
extern char g_426585;
extern char g_426586;
extern long long g_4265a0;
extern unsigned long long g_4265c0;
extern long long g_4265e0;
extern unsigned long long g_426600;
extern char g_426608;

unsigned int main(int a0, long long *a1)
{
    int iter;  // r14d
    unsigned long v19;  // rbp
    unsigned long v28;  // rax
    int v29;  // r10d
    long long v30;  // rax
    long long v31;  // rdx
    long long v32;  // r14
    unsigned long long *v33;  // r14
    unsigned long long *v34;  // rbx
    long long v35;  // rax
    long long v36;  // rdx
    unsigned long v37;  // rax
    long long *v20;  // rbx
    void* v38;  // rbp
    void* v39;  // rbp
    unsigned long v40;  // rax
    int v41;  // r14d
    unsigned long v42;  // rax
    long long v43;  // rdi
    long long v21;  // rdi
    char *v46;  // rax
    struct_0 **node;  // rbp
    char *v48;  // rsi
    char *v49;  // rax
    char *v50;  // rax
    char *ptr;  // rax
    char *v52;  // rbx
    long long v53;  // r13
    long long v54;  // r12
    long long v55;  // rbp
    long long v22;  // rdi
    long long v56;  // rax
    long long v57;  // rax
    char *v58;  // rax
    unsigned long v59;  // 4110
    long long v60;  // rsi
    unsigned long v61;  // rax
    unsigned long v62;  // rdi
    unsigned long v63;  // r11
    unsigned long long v64;  // rcx
    unsigned long v65;  // rdx
    long long v23;  // rsi
    unsigned long v66;  // rdi
    unsigned long v67;  // rax
    unsigned long v68;  // rdx
    long long v69;  // 4100
    unsigned int v70;  // eax
    unsigned long long v71;  // rax
    unsigned long v72;  // rdx
    unsigned long v73;  // rdx
    char *v74;  // rdi
    char v75;  // al
    long long v24;  // rdx
    char v76;  // al
    unsigned int v77;  // edx
    long long *v78;  // r12
    unsigned int v79;  // eax
    long long v80;  // r15
    long long *i;  // r13
    long long *v82;  // r14
    long long *v83;  // r14
    char *v84;  // rax
    char *v85;  // r14
    long long v25;  // rcx
    long long *v86;  // r12
    unsigned int v87;  // eax
    int v88;  // r12d
    int v89;  // eax
    unsigned long j;  // rax
    unsigned int v91;  // edx
    unsigned long v92;  // rax
    int v93;  // eax
    long long v26;  // r8
    long long v27;  // r9
    unsigned long long v0;  // [bp-0xc8]
    unsigned long long v1;  // [bp-0xc0]
    unsigned long long v2;  // [bp-0xb8]
    unsigned long long v3;  // [bp-0xb0]
    unsigned long long v4;  // [bp-0xa8]
    unsigned long long v5;  // [bp-0xa0]
    unsigned long long v6;  // [bp-0x98]
    unsigned long long v7;  // [bp-0x90]
    unsigned long v8;  // [bp-0x88]
    long long v9;  // [bp-0x80]
    char v10;  // [bp-0x74], Other Possible Types: int
    int v11;  // [bp-0x70], Other Possible Types: unsigned int
    long long *v12;  // [bp-0x68], Other Possible Types: unsigned int
    long long *v13;  // [bp-0x60]
    unsigned long v14;  // [bp-0x58]
    unsigned long v15;  // [bp-0x50]
    char *v16;  // [bp-0x48]

    iter = -0x1;
    v19 = a0;
    v20 = a1;
    v21 = *(a1);
    v14 = 0;
    v15 = 0;
    g_4260b0 = 2;
    sub_414480(v21);
    setlocale(6, &g_41e5b0);
    bindtextdomain("diffutils", "/usr/local/share/locale");
    textdomain("diffutils");
    sub_40fd50(0);
    g_426600 = &g_426500;
    g_4265c0 = &g_4264c0;
    re_set_syntax(330310);
    v28 = sub_410310(v22, v23, v24, v25, v26, v27);
    g_426180 = 0;
    g_426410 = v28;
    sub_416f50(v22, v23, v24, v25, v26, v27);
    v10 = 0;
    v8 = 0;
    *((char *)&v11) = 0;
    v9 = -0x1;
    while (1)
    {
        v29 = getopt_long(v19, v20, "0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ", &g_425180.name, NULL);
        if (v29 != -0x1)
        {
            switch (v29)
            {
            case 0: case 104: case 128: case 133:
                iter = v29;
                continue;
            case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57:
                if (iter - 48 <= 9)
                {
                    if (v9 - (v29 <= 0x33) <= 461168601842738789)
                    {
                        iter = v29;
                        v9 = v29 - 48 + v9 * 10;
                        break;
                    }
                    else
                    {
                        iter = v29;
                        v9 = 0x3fffffffffffffff;
                        break;
                    }
                }
                else
                {
                    iter = v29;
                    v9 = v29 - 48;
                    break;
                }
            case 66:
                g_426554 = 1;
                iter = v29;
                continue;
            case 67: case 85:
                if (*((long long *)&optarg))
                {
                    v11 = v29;
                    v30 = strtoimax(*((long long *)&optarg), &v16, 10);
                    if (*(v16) || !(v29 = v11, v30 >= 0))
                        sub_408e40("invalid context length '%s'", *((long long *)&optarg)); /* do not return */
                    v31 = 0x3fffffffffffffff;
                    if (v30 <= 0x3fffffffffffffff)
                        v31 = v30;
                    v32 = v31;
                }
                else
                {
                    v32 = 3;
                }
                iter = v29;
                sub_408f60((unsigned long long)((v29 == 0x55) + 2));
                if (g_426570 < v32)
                {
                    g_426570 = v32;
                    v11 = _INSERT(iter, 0, 1);
                    break;
                }
                else
                {
                    v11 = _INSERT(iter, 0, 1);
                    break;
                }
            case 68:
                iter = v29;
                sub_408f60(7);
                v37 = sub_416a40(strlen(*((long long *)&optarg)) * 7 + 97);
                v7 = *((long long *)&optarg);
                v6 = *((long long *)&optarg);
                v5 = *((long long *)&optarg);
                v4 = 0;
                v3 = *((long long *)&optarg);
                v2 = *((long long *)&optarg);
                v1 = 0;
                v0 = *((long long *)&optarg);
                __sprintf_chk(v37, 0x1);
                v13 = v20;
                v12 = v19;
                v38 = &g_426460;
                do
                {
                    v39 = v38 + 8;
                    sub_408f00(v38, v37, "-D");
                    v37 = v37 + strlen(v37) + 1;
                    v38 = v39;
                } while (v38 != &g_426480);
                v19 = v12;
                v20 = v13;
                continue;
            case 69:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = g_426558 | 1;
                    iter = v29;
                    break;
                }
                break;
            case 70:
                iter = v29;
                sub_408d60(&g_4265e0, *((long long *)&optarg));
                iter = iter;
                continue;
            case 72:
                g_426418 = 1;
                iter = v29;
                continue;
            case 73:
                iter = v29;
                sub_408d60(&g_4265a0, *((long long *)&optarg));
                iter = iter;
                continue;
            case 76:
                if (g_426540)
                {
                    if (!g_426548)
                    {
                        iter = v29;
                        g_426548 = *((long long *)&optarg);
                        break;
                    }
                    else
                    {
                        sub_40e7b0("too many file label options"); /* do not return */
                    }
                }
                else
                {
                    iter = v29;
                    g_426540 = *((long long *)&optarg);
                    break;
                }
            case 78:
                g_426586 = 1;
                iter = v29;
                continue;
            case 80:
                g_426585 = 1;
                iter = v29;
                continue;
            case 83:
                iter = v29;
                sub_408f00(&g_426488, *((long long *)&optarg), "-S");
                iter = iter;
                continue;
            case 84:
                g_426492 = 1;
                iter = v29;
                continue;
            case 87:
                iter = v29;
                v42 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (v42 <= 0 || !(v29 = iter, !*(v16)))
                    sub_408e40("invalid width '%s'", *((long long *)&optarg)); /* do not return */
                if (v42 != v8)
                {
                    if (!v8)
                    {
                        v8 = v42;
                        iter = v29;
                        break;
                    }
                    else
                    {
                        sub_40e7b0("conflicting width options"); /* do not return */
                    }
                }
                break;
            case 88:
                iter = v29;
                v29 = iter;
                if (sub_410bd0(sub_410680, g_426410, *((long long *)&optarg), (-(g_426551 < 1) & 0xfffffff0) + 268435472, 10))
                    sub_40e770(*((long long *)&optarg)); /* do not return */
                break;
            case 90:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = g_426558 | 2;
                    iter = v29;
                    break;
                }
                break;
            case 97:
                g_426568 = 1;
                iter = v29;
                break;
            case 98:
                iter = v29;
                if (g_426558 <= 3)
                {
                    g_426558 = 4;
                    iter = v29;
                    break;
                }
                break;
            case 99:
                iter = v29;
                v43 = 2;
LABEL_405968:
                sub_408f60(v43);
                if (g_426570 <= 2)
                {
                    g_426570 = 3;
                    break;
                }
                break;
            case 100:
                g_426408 = 1;
                iter = v29;
                continue;
            case 101:
                iter = v29;
                sub_408f60(4);
                iter = iter;
                continue;
            case 102:
                iter = v29;
                sub_408f60(5);
                iter = iter;
                continue;
            case 105:
                g_426552 = 1;
                iter = v29;
                continue;
            case 108:
                if (1)
                {
                    iter = v29;
                    g_426480 = 1;
                    signal(0x11, NULL);
                    iter = iter;
                    break;
                }
                else
                {
                    sub_408e40("pagination not supported on this host", 0); /* do not return */
                }
            case 110:
                iter = v29;
                sub_408f60(6);
                iter = iter;
                continue;
            case 112:
                v10 = v29;
                sub_408d60(&g_4265e0, "^[[:alpha:]$_]");
                iter = v10;
                v10 = 1;
                continue;
            case 113:
                g_4264a1 = 1;
                iter = v29;
                continue;
            case 114:
                g_426608 = 1;
                iter = v29;
                continue;
            case 115:
                g_426584 = 1;
                iter = v29;
                continue;
            case 116:
                g_4264a0 = 1;
                iter = v29;
                continue;
            case 117:
                iter = v29;
                v43 = 3;
                goto LABEL_405968;
            case 118:
                v53 = sub_414920("Len Tower");
                v54 = sub_414920("Richard Stallman");
                v55 = sub_414920("David Hayes");
                v56 = sub_414920("Mike Haertel");
                v57 = sub_414920("Paul Eggert");
                sub_416840(stdout, "diff", "GNU diffutils", g_4260a8, v57, v56, v55, v54, v53, 0);
                sub_409df0(v22, v23, v24, v25, v26, v27);
                return 0;
            case 119:
                g_426558 = 5;
                iter = v29;
                continue;
            case 120:
                iter = v29;
                sub_410680(g_426410, *((long long *)&optarg), (-(g_426551 < 1) & 0xfffffff0) + 268435472);
                iter = iter;
                continue;
            case 121:
                iter = v29;
                sub_408f60(8);
                iter = iter;
                continue;
            case 129:
                iter = v29;
                sub_408f00(&v14, *((long long *)&optarg), "--from-file");
                iter = iter;
                continue;
            case 130:
                __printf_chk(1, dcgettext(NULL, "Usage: %s [OPTION]... FILES\n", 5));
                __printf_chk(1, "%s\n\n", dcgettext(NULL, "Compare FILES line by line.", 5));
                v46 = dcgettext(NULL, "Mandatory arguments to long options are mandatory for short options too.\n", 5);
                node = &g_424f40[0];
                fputs_unlocked(v46, stdout);
                v48 = "    --normal                  output a normal diff (the default)";
                while (1)
                {
                    if (!*(v48))
                    {
                        v49 = &stdout->field_28->field_0;
                        if (v49 < *((long long *)&stdout[1].padding_0[0]))
                        {
                            stdout->field_28 = v49 + 1;
                            *(v49) = 10;
                        }
                        else
                        {
                            __overflow(stdout, 10);
                        }
                        v48 = &node[1]->field_0;
                        node += 1;
                        if (!v48)
                            break;
                    }
                    else
                    {
                        v50 = dcgettext(NULL, v48, 5);
                        while (1)
                        {
                            ptr = strchr(v50, 10);
                            if (!ptr)
                                break;
                            v52 = ptr + 1;
                            if ((unsigned int)(v52 - v50) > 0xfff)
                                __assert_fail(); /* do not return */
                            __printf_chk(1, "  %.*s");
                            v50 = v52;
                        }
                        node += 1;
                        __printf_chk(1, &(&g_41c29f)[2 * (*(v50) != 45 & _INSERT(0, 0, *(v50) != 32))]);
                        v48 = &*(node)->field_0;
                        if (!v48)
                            break;
                    }
                }
                sub_416920();
                sub_409df0(v22, v23, v24, v25, v26, v27);
                return 0;
            case 131:
                iter = v29;
                v35 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (*(v16) || v35 < 0)
                    sub_408e40("invalid horizon length '%s'", *((long long *)&optarg)); /* do not return */
                v36 = g_426560;
                if (g_426560 < v35)
                    v36 = v35;
                g_426560 = v36;
                break;
            case 132:
                g_426551 = 1;
                iter = v29;
                continue;
            case 134:
                g_426439 = 1;
                iter = v29;
                continue;
            case 135:
                v33 = &g_426440;
                iter = v29;
                sub_408f60(7);
                v12 = v20;
                do
                {
                    v34 = v33 + 1;
                    sub_408f00(v33, *((long long *)&optarg), "--line-format");
                    v33 = v34;
                } while (v33 != &g_426458);
                v20 = v12;
                continue;
            case 136:
                g_426550 = 1;
                iter = v29;
                continue;
            case 137:
                g_426551 = 0;
                iter = v29;
                continue;
            case 138:
                iter = v29;
                sub_408f60(1);
                iter = iter;
                continue;
            case 139:
                iter = v29;
                sub_408f60(8);
                g_42643a = 1;
                continue;
            case 140:
                g_426490 = 1;
                iter = v29;
                continue;
            case 141:
                g_426491 = 1;
                iter = v29;
                continue;
            case 142:
                g_426438 = 1;
                iter = v29;
                continue;
            case 143:
                iter = v29;
                v40 = strtoimax(*((long long *)&optarg), &v16, 10);
                if (v40 <= 0 || v40 > 0xfffffffffffffffc || !(v29 = iter, !*(v16)))
                    sub_408e40("invalid tabsize '%s'", *((long long *)&optarg)); /* do not return */
                if (v40 != g_426498)
                {
                    if (!g_426498)
                    {
                        g_426498 = v40;
                        iter = v29;
                        break;
                    }
                    else
                    {
                        sub_40e7b0("conflicting tabsize options"); /* do not return */
                    }
                }
                break;
            case 144:
                iter = v29;
                sub_408f00(&v15, *((long long *)&optarg), "--to-file");
                iter = iter;
                continue;
            case 145: case 146: case 147:
                iter = v29;
                sub_408f60(7);
                iter -= 145;
                sub_408f00(&(&g_426440)[iter], *((long long *)&optarg), (char *)(24 * iter + 4317056), &g_426440);
                continue;
            case 148: case 149: case 150: case 151:
                iter = v29;
                sub_408f60(7);
                iter -= 148;
                sub_408f00(&(&g_426460)[8 * iter], *((long long *)&optarg), (char *)(25 * iter + 0x41dfe0), &g_426460);
                continue;
            case 152:
                if (*((long long *)&optarg) && !(iter = v29, v29 = iter, !(int)(long long)strcmp(*((long long *)&optarg), "auto")))
                {
                    iter = v29;
                    if (!strcmp(*((long long *)&optarg), "always"))
                    {
                        g_42657c = 2;
                        v41 = iter;
                        iter = v41;
                        break;
                    }
                    else
                    {
                        iter = v41;
                        if (strcmp(*((long long *)&optarg), "never"))
                            sub_408e40("invalid color '%s'", *((long long *)&optarg)); /* do not return */
                        g_42657c = 0;
                        break;
                    }
                }
                else
                {
                    g_42657c = 1;
                    iter = v29;
                    break;
                }
            case 153:
                iter = v29;
                sub_40e7f0(*((long long *)&optarg));
                iter = iter;
                continue;
            case 154:
                g_426180 = 1;
                iter = v29;
                continue;
            default:
                sub_408e40(0, 0); /* do not return */
            }
        }
        else if (g_42657c == 1)
        {
            v58 = getenv("TERM");
            if (v58 && !strcmp(v58, "dumb"))
            {
                g_42657c = 0;
                break;
            }
        }
    }
    if (!g_426580)
    {
        if (v10)
        {
            sub_408f60(2);
            if (!(v9 - 0 >> 63 & 1))
            {
                if (g_426580 != 2 || (char)sub_410f10(2))
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                else
                    g_426400 = "%a %b %e %T %Y";
                v60 = g_426570;
                if (g_426580 - 2 <= 1 && (v9 > v60 || v9 != v60 && !(char)v11))
                {
                    g_426570 = v9;
                    v60 = v9;
                    goto LABEL_405b58;
                }
            }
            else
            {
                v59 = g_426580;
                g_426570 = 3;
                if ((unsigned int)v59 != 2)
                {
                    v60 = 3;
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                    goto LABEL_405b58;
                }
                else if ((char)sub_410f10(2))
                {
                    v60 = g_426570;
                    g_426400 = "%Y-%m-%d %H:%M:%S.%N %z";
                    goto LABEL_405b58;
                }
                else
                {
                    g_426400 = "%a %b %e %T %Y";
LABEL_405d54:
                    v60 = g_426570;
                    goto LABEL_405b58;
                }
            }
        }
        else
        {
            sub_408f60(1);
        }
    }
    if (g_426580 != 2 || (char)sub_410f10(2))
        v61 = "%Y-%m-%d %H:%M:%S.%N %z";
    else
        v61 = "%a %b %e %T %Y";
    g_426400 = v61;
    if (v9 - 0 >> 63 & 1)
        goto LABEL_405d54;
LABEL_405b58:
    if (!g_426498)
        g_426498 = 8;
    if (v8)
    {
        v62 = 1;
        if (g_4264a0)
            goto LABEL_405b8f;
        goto LABEL_405b88;
    }
    else if (!g_4264a0)
    {
        v8 = 130;
LABEL_405b88:
        v62 = g_426498;
LABEL_405b8f:
        v63 = v8;
        v64 = (v63 >> 1) + (v62 + 3 >> 1) + ((unsigned int)v62 + 3 & (unsigned int)v63 & 1);
        v65 = v64 % v62;
        v66 = v64 - v65;
        if (v66 > 3 && v66 < v63)
        {
            v67 = v65 - v64 + v63;
            v68 = v66 - 3;
            if (v67 > v68)
                v67 = v68;
            if (!v67)
                v66 = v63;
            v8 = v66;
        }
        else
        {
            v67 = 0;
        }
    }
    else
    {
        v8 = 67;
        v67 = 63;
    }
    g_426430 = v67;
    v69 = g_426560;
    g_426428 = v8;
    if (v69 < v60)
        g_426560 = v60;
    sub_408ea0(&g_4265e0);
    sub_408ea0(&g_4265a0);
    v70 = g_426580;
    if (g_426580 != 7)
    {
LABEL_405c36:
        v75 = v70 != 8 | g_426438;
        goto LABEL_405c42;
    }
    if (!g_426440)
        g_426440 = "%l\n";
    if (!g_426448)
        g_426448 = "%l\n";
    if (!g_426450)
        g_426450 = "%l\n";
    v71 = g_426478;
    if (!g_426468)
    {
        v72 = "%<";
        if (v71)
            v72 = v71;
        g_426468 = v72;
    }
    if (!g_426470)
    {
        v73 = "%>";
        if (v71)
            v73 = v71;
        g_426470 = v73;
    }
    if (!*((long long *)&g_426460))
    {
        *((char **)&g_426460) = "%=";
        v74 = "%=";
        if (v71)
            goto LABEL_405f6e;
    }
    else if (!(!v71))
    {
        goto LABEL_405e37;
    }
    g_426478 = sub_40fa30(g_426468, g_426470, &g_41e5b0);
    v70 = g_426580;
    if (g_426580 != 7)
        goto LABEL_405c36;
LABEL_405e37:
    v74 = *((long long *)&g_426460);
    v76 = 1;
    if (!*((char *)*((long long *)&g_426460)))
        goto LABEL_405e53;
LABEL_405f6e:
    v76 = 0;
    if (!strcmp(v74, "%="))
        v76 = !*((char *)g_426440);
LABEL_405e53:
    v75 = v76 & 1;
LABEL_405c42:
    g_426578 = v75;
    v77 = 1;
    if (!g_4265a0)
        v77 = g_426558;
    v78 = v20 + 1;
    g_426553 = (char)~(g_426554 | g_426552 | g_426490 | v77) & g_4264a1;
    v79 = optind;
    g_426553 = g_426553 & 1;
    if (v79 > 1)
    {
        v80 = 1;
        i = &v20[v79];
        v82 = v78;
        do
        {
            v83 = v82 + 1;
            v80 = v80 + sub_414d70(*(v82)) + 1;
            v82 = v83;
        } while (i != v82);
        v84 = sub_416a40(v80);
        v85 = v84;
        do
        {
            *(v84) = 32;
            v86 = v78 + 1;
            v84 = sub_414de0(v84 + 1, *(v78));
            v78 = v86;
        } while (i != v78);
    }
    else
    {
        v84 = sub_416a40(1);
        v85 = v84;
    }
    *(v84) = 0;
    g_426420 = v85;
    if (!v14)
    {
        j = optind;
        if (!v15)
        {
            v91 = (v19 & 0xffffffff) - optind;
            if (v91 != 2)
            {
                if (v91 > 1)
                    sub_408e40("extra operand '%s'", v20[2 + optind]); /* do not return */
                sub_408e40("missing operand after '%s'", *((long long *)((char *)&v20[v19] - 8))); /* do not return */
            }
            v92 = optind + 1;
            v88 = sub_408f90(0, *((long long *)((char *)&v20[v92] - 8)), v20[v92]);
        }
        else
        {
            for (v88 = 0; (unsigned int)j < (unsigned int)v19; j = optind)
            {
                v93 = sub_408f90(0, v20[j], v15);
                if (v88 < v93)
                    v88 = v93;
                optind = optind + 1;
            }
        }
    }
    else if (v15)
    {
        sub_40e7b0("--from-file and --to-file both specified"); /* do not return */
    }
    else
    {
        v87 = optind;
        v88 = 0;
        if ((unsigned int)v19 > optind)
        {
            do
            {
                v89 = sub_408f90(0, v14, v20[v87]);
                if (v88 < v89)
                    v88 = v89;
            } while ((v87 = optind + 1, optind = v87, v87 < (unsigned int)v19));
        }
    }
    sub_40e670();
    sub_409df0(v22, v23, v24, v25, v26, v27);
    exit(v88); /* do not return */
}



// Function: diff_2_files @ 0x6e20
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned int g_41c058[4];
extern unsigned long long g_4261a0;
extern char g_426408;
extern char g_426418;
extern char g_4264a1;
extern struct_0 *g_4264e0;
extern long long g_426540;
extern long long g_426550;
extern char g_426554;
extern char g_426578;
extern unsigned int g_426580;

long long diff_2_files(void* a0)
{
    void* idx;  // rbp
    unsigned long long ptr;  // r14
    unsigned long long v30;  // rax
    struct_0 *idx1;  // r15
    void* v121;  // rbp
    unsigned long v122;  // rbx
    unsigned long v123;  // rax
    unsigned long long v124;  // r13
    unsigned long v125;  // rax
    unsigned long long v127;  // r14
    struct_0 *v128;  // rax
    unsigned long iter;  // r14
    unsigned long v31;  // rbx
    unsigned long iter1;  // r13
    struct_0 *idx2;  // rdx
    unsigned long v132;  // rcx
    unsigned long v133;  // r9
    unsigned long iter2;  // rbx
    unsigned long long v135;  // r12
    unsigned long v136;  // rbp
    unsigned long long v137;  // r15
    struct_0 *v138;  // rax
    struct_0 *v139;  // rbp
    unsigned long v32;  // rdi
    struct_0 **v140;  // rax
    struct_0 *v141;  // rbp
    unsigned int v142;  // eax
    struct_0 *ptr1;  // r15
    struct_0 *v144;  // r15
    long long *v145;  // r15
    void* v146;  // rbx
    long long v147;  // rdi
    unsigned long long v33;  // rdx
    unsigned long long v34;  // rax
    unsigned long long v35;  // rax
    unsigned long v36;  // rdx
    void* ptr2;  // r12
    unsigned long long v38;  // rbx
    unsigned long i;  // rax
    unsigned long v22;  // rbx
    unsigned long v40;  // rax
    unsigned long v41;  // rax
    unsigned long j;  // rdx
    unsigned long v43;  // rdx
    void* *v44;  // r15
    void* v45;  // rcx
    void* v46;  // r10
    void* *v47;  // r9
    void* v48;  // rdi
    unsigned long v49;  // rax
    unsigned long long v23;  // rax
    void* v50;  // r8
    unsigned long v51;  // rdx
    unsigned long v52;  // r11
    unsigned long long v53;  // rdx
    unsigned long long v54;  // rsi
    unsigned long long v55;  // rdx
    unsigned long long v56;  // rdx
    unsigned long v57;  // rdx
    void* v58;  // rdx
    long long v59;  // r11
    unsigned long long v24;  // rdi
    long long v60;  // rax
    char *v61;  // r9
    long long v62;  // rsi
    long long v63;  // rcx
    unsigned long long v64;  // r8
    void* v65;  // rcx
    unsigned long v25;  // r13
    unsigned long v26;  // rbx
    void* v84;  // rax
    void* v85;  // rsi
    unsigned long v86;  // r8
    unsigned long long v87;  // rcx
    unsigned long long v88;  // rcx
    unsigned long v89;  // rdx
    unsigned int v27;  // r12d
    long long v90;  // rbx
    unsigned long long v91;  // rax
    unsigned long v92;  // rdx
    unsigned long long v93;  // rax
    long long v94;  // rbx
    long long v95;  // rbx
    unsigned long long m;  // rbx
    unsigned long long v97;  // rcx
    unsigned long long *v98;  // rdi
    void* k;  // rsi
    unsigned long long v28;  // r13
    void* l;  // rsi
    unsigned long long v101;  // rcx
    unsigned long v102;  // rsi
    unsigned long long node;  // rcx
    long long v104;  // r11
    unsigned long v105;  // rdi
    long long i1;  // r10
    unsigned long v107;  // r8
    long long v108;  // rax
    long long i5;  // rdx
    unsigned long long v29;  // rax
    long long v110;  // rdx
    long long v111;  // rax
    long long v112;  // rax
    long long v113;  // rdx
    long long v114;  // rax
    void* i4;  // r9
    void* v116;  // rsi
    unsigned long v117;  // r8
    unsigned long v118;  // rbx
    unsigned long n;  // r12
    char *v0;  // [bp-0xf0], Other Possible Types: void*, unsigned long long
    struct_0 *v1;  // [bp-0xe8], Other Possible Types: void*, void* *, unsigned long
    void* v2;  // [bp-0xe0]
    void* v3;  // [bp-0xd8], Other Possible Types: unsigned long
    unsigned long v4;  // [bp-0xd0]
    void* v5;  // [bp-0xc8]
    unsigned long v6;  // [bp-0xc0]
    char v7;  // [bp-0xb8]
    char v8;  // [bp-0xb0]
    char v9;  // [bp-0xa8]
    char v10;  // [bp-0xa0]
    unsigned long long v11;  // [bp-0x98]
    unsigned long long v12;  // [bp-0x90]
    unsigned long long v13;  // [bp-0x88]
    unsigned long long v14;  // [bp-0x80]
    char v15;  // [bp-0x78]
    unsigned long long v16;  // [bp-0x70]
    void* v17;  // [bp-0x68]
    unsigned long long v18;  // [bp-0x50]

    idx = a0;
    if (!(char)sub_40bbc0())
    {
        v2 = sub_40fab0((long long)idx[504] + (long long)idx[200] + 4);
        *((void* *)&idx[280]) = v2 + 1;
        v29 = (long long)idx[200] + (long long)idx[504];
        *((void* *)&idx[584]) = v2 + (long long)idx[200] + 3;
        v30 = sub_416a40(v29 * 16);
        v31 = (long long)idx[200];
        v32 = (long long)idx[296];
        *((unsigned long long *)&idx[0x100]) = v30;
        v33 = v31 * 8;
        v34 = v30 + v33;
        *((unsigned long long *)&idx[264]) = v34;
        v35 = v34 + v33;
        v36 = (long long)idx[504];
        *((unsigned long long *)&idx[560]) = v35;
        *((unsigned long long *)&idx[568]) = v35 + v36 * 8;
        ptr2 = sub_40fab0(v32 * 16);
        (&v17)[2] = ptr2;
        v38 = ptr2 + (long long)idx[296] * 8;
        v18 = v38;
        if ((long long)idx[200] > 0)
        {
            i = 0;
            do
            {
                v40 = i + 1;
                *((unsigned long long *)((char *)ptr2 + 8 * *((long long *)((long long)idx[248] + i * 8)))) = *((long long *)((char *)ptr2 + 8 * *((long long *)((long long)idx[248] + i * 8)))) + 1;
                i = v40;
            } while (i < (long long)idx[200]);
        }
        v41 = (long long)idx[504];
        if (v41 > 0)
        {
            j = 0;
            do
            {
                v43 = j + 1;
                *((unsigned long long *)(v38 + *((long long *)((long long)idx[552] + j * 8)) * 8)) = *((long long *)(v38 + *((long long *)((long long)idx[552] + j * 8)) * 8)) + 1;
                v41 = (long long)idx[504];
                j = v43;
            } while (j < v41);
        }
        v44 = &v17;
        v45 = idx + 248;
        v46 = (char *)&v17 - 16;
        v47 = &v17;
        v48 = sub_40fab0(v41 + (long long)idx[200]);
        v49 = (long long)idx[200];
        v50 = v48;
        (&v17)[1] = v48 + v49;
        while (1)
        {
            v51 = v49;
            v52 = *((long long *)v45);
            v53 = v51 >> 8;
            if (v51 & 0xffffffffffffff00)
            {
                v54 = 5;
                do
                {
                    v54 *= 2;
                    v56 = v53 >> 2;
                    v53 = v56;
                } while (v55 & 0xfffffffffffffffc);
                goto LABEL_407184;
            }
            else
            {
                if (!v49)
                {
LABEL_4071bb:
                    v45 += 304;
                    v47 += 1;
                    if (idx + 856 == v45)
                        break;
                    v38 = *((long long *)v46);
                    v49 = *((long long *)((char *)v45 - 48));
                    v46 -= 8;
                    v50 = *(v47);
                }
                else
                {
                    v54 = 5;
LABEL_407184:
                    v57 = 0;
                    while (1)
                    {
                        if (!*((long long *)(v52 + v57 * 8)))
                        {
LABEL_40718d:
                            v57 += 1;
                            if (v57 == v49)
                                break;
                        }
                        else
                        {
                            if (!*((long long *)(v38 + *((long long *)(v52 + v57 * 8)) * 8)))
                            {
                                *((char *)v50 + v57) = 1;
                                goto LABEL_40718d;
                            }
                            if (*((long long *)(v38 + *((long long *)(v52 + v57 * 8)) * 8)) <= v54)
                                goto LABEL_40718d;
                            *((char *)v50 + v57) = 2;
                            v57 += 1;
                            if (v57 == v49)
                                break;
                        }
                    }
                    goto LABEL_4071bb;
                }
            }
        }
        v1 = &v17;
        v58 = v48;
        v0 = 0;
    }
    else if (!((long long)idx[64] > 0 & (long long)idx[64] != (long long)idx[368]) || (long long)idx[368] <= 0 || *((int *)idx) >= 0 && ((short)(int)idx[40] & 0xf000) != 0x8000 || (int)idx[304] >= 0 && ((short)(int)idx[344] & 0xf000) != 0x8000)
    {
        ptr = (long long)idx[160];
        if (*((int *)idx) != (int)idx[304])
        {
            v22 = sub_417190(8, sub_417190((long long)idx[72], (long long)idx[376], 0x7ffffffffffffffe), 0x7ffffffffffffffe);
            v23 = sub_416a90(ptr, v22);
            v24 = (long long)idx[464];
            v25 = v22;
            *((unsigned long long *)&idx[160]) = v23;
            *((unsigned long long *)&idx[464]) = sub_416a90(v24, v25);
            while (1)
            {
                if (*((int *)idx) >= 0)
                    sub_40b8a0(idx, v25 - (long long)idx[176]);
                if ((int)idx[304] >= 0)
                    sub_40b8a0(idx + 304, v25 - (long long)idx[480]);
                v26 = (long long)idx[176];
                if (v26 != (long long)idx[480] || (v0 = (unsigned long long)(long long)idx[160], v27 = (unsigned int)(int)(long long)memcmp((long long)idx[160], (long long)idx[464], v26), (int)(long long)memcmp((long long)idx[160], (long long)idx[464], v26)))
                    break;
                if (v26 == v25)
                {
                    *((unsigned long *)&idx[480]) = 0;
                    *((unsigned long *)&idx[176]) = 0;
                }
                else
                {
                    v28 = (long long)idx[464];
                    ptr = v0;
                    goto LABEL_406fc4;
                }
            }
            goto LABEL_406f97;
        }
        else
        {
            v28 = (long long)idx[464];
            v27 = 0;
            goto LABEL_406fc4;
        }
    }
    else
    {
LABEL_406f97:
        v27 = 1;
        sub_406d90(1, idx);
        ptr = (long long)idx[160];
        v28 = (long long)idx[464];
        goto LABEL_406fc4;
    }
    while (1)
    {
        v3 = v58 - 1;
        v60 = 0;
        if (*((long long *)((char *)idx + v0 + 200)) > 0)
            break;
LABEL_40722b:
        v0 += 304;
        v1 += 1;
        if (v0 == 608)
        {
            v84 = idx + 200;
            v85 = v48;
            while (1)
            {
                v86 = *((long long *)v84);
                v87 = 0;
                v88 = 0;
                if (v86 > 0)
                {
                    v89 = 0;
                    while (1)
                    {
                        if (!g_426408 && *((char *)v85 + v89))
                        {
                            *((char *)((long long)v84[80] + v89)) = 1;
                            v89 += 1;
                            v88 = v87;
                            if (v86 == v89)
                                break;
                        }
                        else
                        {
                            *((long long *)((long long)v84[56] + v87 * 8)) = *((long long *)((long long)v84[48] + v89 * 8));
                            v88 = v87 + 1;
                            *((unsigned long *)((long long)v84[64] + v87 * 8)) = v89;
                            v89 += 1;
                            v87 = v88;
                            if (v86 == v89)
                                break;
                        }
                    }
                }
                *((unsigned long long *)&v84[72]) = v88;
                v84 += 304;
                v44 += 1;
                if (idx + 808 == v84)
                    break;
                v85 = *(v44);
            }
            free(v48);
            free(ptr2);
            v90 = (long long)idx[576] + (long long)idx[272] + 3;
            v11 = (long long)idx[0x100];
            v12 = (long long)idx[560];
            v91 = sub_416a40(v90 * 16);
            v92 = (long long)idx[576] * 8 + 8;
            v13 = v91 + v92;
            v14 = v91 + v92 + v90 * 8;
            v15 = g_426418;
            if (v90)
            {
                v93 = 1;
                do
                {
                    v93 *= 2;
                    v95 = v90 >> 2;
                    v90 = v95;
                } while (v94 >> 2);
                if (v93 < 0x1000)
                    v93 = 0x1000;
            }
            else
            {
                v93 = 0x1000;
            }
            v16 = v93;
            m = 0;
            v97 = 38;
            v98 = &g_4261a0;
            for (k = idx; v97; k += 8)
            {
                v97 -= 1;
                *(v98) = *((long long *)k);
                v98 += 1;
            }
            l = idx + 304;
            for (v101 = 38; v101; l += 8)
            {
                v101 -= 1;
                *(v98) = *((long long *)l);
                v98 += 1;
            }
            sub_406360(0, (long long)idx[272]);
            free(0xfffffffffffffff8 - (long long)idx[576] * 8 + v13);
            do
            {
                v102 = *((long long *)((char *)idx + m + 280));
                node = 0;
                v104 = 0;
                v105 = *((long long *)((char *)idx + m + 248));
                i1 = *((long long *)((char *)idx + m + 200));
                v107 = *((long long *)((char *)idx + -(m) + 584));
                while (i1 > v104)
                {
LABEL_407618:
                    if (*((char *)(v102 + v104)))
                    {
                        do
                        {
LABEL_407623:
                            v108 = v104;
                            do
                            {
                                v108 += 1;
                            } while (*((char *)(v102 + v108)));
                            for (; *((char *)(v107 + node)); node += 1);
                            i5 = v104;
                            do
                            {
                                v110 = i5;
                                v111 = v108;
                                v112 = v111;
                                v113 = v110;
                                v108 = v111;
                                i5 = v110;
                                if (v113)
                                {
                                    do
                                    {
                                        v108 = v112;
                                        i5 = v113;
                                        if (*((long long *)(v105 + i5 * 8 - 8)) != *((long long *)(v105 + v108 * 8 - 8)))
                                            break;
                                        v113 -= 1;
                                        v112 -= 1;
                                        *((char *)(v102 + v113)) = 1;
                                        *((char *)(v102 + v112)) = 0;
                                        if (*((char *)(v102 + v113 - 2)))
                                        {
                                            do
                                            {
                                                v113 -= 1;
                                            } while (*((char *)(v102 + v113 - 1)));
                                        }
                                        do
                                        {
                                            node -= 1;
                                        } while (*((char *)(v107 + node)));
                                        v108 = v112;
                                        i5 = v113;
                                    } while (i5);
                                }
                                if (!*((char *)(v107 + node - 1)))
                                    v104 = i1;
                                else
                                    v104 = v108;
                                if (v108 != i1)
                                {
                                    do
                                    {
                                        if (*((long long *)(v105 + i5 * 8)) != *((long long *)(v105 + v108 * 8)))
                                            break;
                                        i5 += 1;
                                        v114 = v108 + 1;
                                        *((char *)(v102 + i5 - 1)) = 0;
                                        *((char *)(v102 + v108)) = 1;
                                        if (*((char *)(v102 + v114)))
                                        {
                                            do
                                            {
                                                v114 += 1;
                                            } while (*((char *)(v102 + v114)));
                                            node += 1;
                                            if (!*((char *)(v107 + node)))
                                                continue;
                                        }
                                        do
                                        {
                                            v104 = v114;
                                        } while ((node += 1, *((char *)(v107 + node))));
                                    } while ((v108 = v114, v108 != i1));
                                }
                            } while (v111 - v110 != v108 - i5);
                            if (v108 <= v104)
                                v104 = v108;
                            i4 = v102 + v108 - 1;
                            do
                            {
                                *((char *)i4 - v108 + i5) = 1;
                                *((char *)i4) = 0;
                                do
                                {
                                    node -= 1;
                                } while (*((char *)(v107 + node)));
                                i4 -= 1;
                            } while (v102 - 1 + v104 != i4);
                            if (i1 > v104)
                                goto LABEL_407618;
                        } while (i1 != v104);
                        goto LABEL_40776b;
                    }
                    else
                    {
                        do
                        {
                            node += 1;
                        } while (*((char *)(v107 + node - 1)));
                        v104 += 1;
                    }
                }
                if (i1 != v104)
                    goto LABEL_407623;
LABEL_40776b:
                m += 304;
            } while (m != 608);
            v116 = (long long)idx[280];
            v117 = (long long)idx[584];
            v118 = (long long)idx[200];
            n = (long long)idx[504];
            if (g_426580 != 4)
            {
                if ((n & v118) < 0)
                    goto LABEL_407b50;
                v0 = idx;
                idx1 = NULL;
                v121 = v116;
                do
                {
                    do
                    {
                        v122 = v118;
                        if (*((char *)v121 + v122 - 1) || *((char *)(v117 + n - 1)))
                        {
                            if (*((char *)v121 + v118 - 1))
                            {
                                v123 = v118;
                                do
                                {
                                    v123 -= 1;
                                } while (*((char *)v121 + v123 - 1));
                                v124 = v118 - v123;
                                v118 = v123;
                            }
                            else
                            {
                                v124 = 0;
                            }
                            v122 = v118;
                            if (*((char *)(v117 + n - 1)))
                            {
                                v125 = n;
                                do
                                {
                                    v125 -= 1;
                                } while (*((char *)(v117 + v125 - 1)));
                                v127 = n - v125;
                                n = v125;
                            }
                            else
                            {
                                v127 = 0;
                            }
                            v1 = v117;
                            v128 = sub_416a40(48);
                            v117 = v1;
                            v128->field_0 = idx1;
                            idx1 = v128;
                            idx1->field_18 = v122;
                            idx1->field_20 = n;
                            idx1->field_8 = v127;
                            idx1->field_10 = v124;
                        }
                        n -= 1;
                        v118 = v122 - 1;
                    } while (!(v122 - 1 >> 63 & 1));
                } while (n >= 0);
                idx = v0;
                goto LABEL_4077e6;
            }
            else if (v118 > 0 || n > 0)
            {
                v0 = idx;
                iter = 0;
                iter1 = 0;
                idx2 = NULL;
                v132 = v118;
                v133 = n;
                do
                {
                    do
                    {
                        if (*((char *)v116 + iter1) || *((char *)(v117 + iter)))
                        {
                            iter2 = iter1;
                            if (*((char *)v116 + iter1))
                            {
                                do
                                {
                                    iter2 += 1;
                                } while (*((char *)v116 + iter2));
                                v135 = iter2 - iter1;
                            }
                            else
                            {
                                v135 = 0;
                            }
                            v136 = iter;
                            if (*((char *)(v117 + iter)))
                            {
                                do
                                {
                                    v136 += 1;
                                } while (*((char *)(v117 + v136)));
                                v137 = v136 - iter;
                            }
                            else
                            {
                                v137 = 0;
                            }
                            v6 = v117;
                            v5 = v116;
                            v4 = v133;
                            v3 = v132;
                            v1 = idx2;
                            v138 = sub_416a40(48);
                            v117 = v6;
                            v138->field_18 = iter1;
                            v116 = v5;
                            iter1 = iter2;
                            v138->field_20 = iter;
                            v133 = v4;
                            iter = v136;
                            v138->field_0 = v1;
                            v132 = v3;
                            idx2 = v138;
                            idx2->field_8 = v137;
                            idx2->field_10 = v135;
                        }
                        iter1 += 1;
                        iter += 1;
                    } while (iter1 < v132);
                } while (iter < v133);
                idx = v0;
                idx1 = idx2;
LABEL_4077e6:
                if (!(g_426554 || g_4264e0))
                {
LABEL_407c25:
                    v27 = idx1;
LABEL_40786c:
                    if (g_4264a1)
                    {
                        if (v27)
                            sub_406d90(1, idx);
                        goto LABEL_40788b;
                    }
                    else
                    {
                        if (v27)
                            goto LABEL_4079c0;
                        goto LABEL_407b7c;
                    }
                }
                else if (idx1)
                {
                    v1 = idx;
                    v139 = idx1;
                    v0 = &v7;
                    do
                    {
                        v140 = sub_40f060(v139);
                    } while ((v141 = *(v140), *(v140) = NULL, v142 = (unsigned int)(unsigned long long)sub_40f670(v139, v0, &v8, &v9, &v10), *(v140) = v141, v141 && (v139 = v141, !v142)));
                    idx = v1;
                    v27 = v142;
                    goto LABEL_40786c;
                }
            }
            else
            {
LABEL_407b50:
                if (!g_426554 && (idx1 = g_4264e0, !g_4264e0))
                    goto LABEL_407c25;
                idx1 = NULL;
                v27 = 0;
                if (g_4264a1)
                    goto LABEL_40788b;
LABEL_407b7c:
                v27 = 0;
                if (!g_426578)
                {
LABEL_4079c0:
                    v147 = g_426540;
                    if (!g_426540)
                        v147 = (long long)idx[8];
                    sub_40e800(v147);
                    if (g_426580 > 8)
                        abort(); /* do not return */
                    goto *((void *)((long long)(g_41c058[g_426580] + (char *)&g_41c058[0])));
                }
LABEL_40788b:
                free((long long)idx[0x100]);
                free(v2);
                free((long long)idx[248]);
                free((long long)idx[184] + (long long)idx[192] * 8);
                free((long long)idx[552]);
                free((long long)idx[488] + (long long)idx[496] * 8);
                if (idx1)
                {
                    do
                    {
                        ptr1 = idx1;
                        v144 = ptr1->field_0;
                        free(ptr1);
                    } while ((idx1 = v144, ptr1->field_0));
                }
                if (g_426580 - 4 <= 1)
                {
                    v145 = &g_426540;
                    v146 = idx + 288;
                    do
                    {
                        if (*((char *)v146))
                        {
                            dcgettext(NULL, "No newline at end of file", 5);
                            error(0, 0, "%s: %s\n");
                            v27 = 2;
                        }
                    } while ((v145 += 8, v146 += 304, v145 != &g_426550));
                }
                ptr = (long long)idx[160];
                v28 = (long long)idx[464];
            }
LABEL_406fc4:
            if (ptr != v28)
            {
                free(ptr);
                free((long long)idx[464]);
            }
            else
            {
                free(v28);
            }
            return v27;
        }
        v58 = *(v1);
    }
    while (1)
    {
LABEL_407211:
        v61 = v58 + v60;
        if (*(v61) != 2)
            break;
        v60 += 1;
        *(v61) = 0;
        if (v59 <= v60)
            goto LABEL_40722b;
    }
    if (!*(v61))
    {
LABEL_407208:
        v60 += 1;
        if (v59 <= v60)
            goto LABEL_40722b;
        else
            goto LABEL_407211;
    }
    v62 = 0;
    v63 = v60;
    while (1)
    {
        v63 += 1;
        if (v59 <= v63 || !*((char *)v58 + v63))
            break;
        v62 += *((char *)v58 + v63) == 2;
    }
    if (v60 >= v63)
    {
        v64 = v63 - v60;
        goto LABEL_407208;
    }
    do
    {
        if (*((char *)v58 + v63 - 1) != 2 && v63 - v60 < v62 * 4)
        {
            if (v60 < v63)
            {
                v65 = v58 + v63 - 1;
                do
                {
                    if (*((char *)v65) == 2)
                        *((char *)v65) = 0;
                } while ((v65 -= 1, v3 + v60 != v65));
            }
            goto LABEL_407208;
        }
    } while ((v63 = (long long)(v63 - 1), v62 = (long long)(v62 - 1), *((char *)((char *)v58 + v63)) = 0, v60 != v63));
    v64 = 0;
    goto LABEL_407208;
}



// Function: print_context_label @ 0x7cf0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_426188;
extern long long g_426400;

void print_context_label(char *a0, int *a1, char *a2, char *a3)
{
    char *v2;  // rax
    char v0[56];  // [bp-0x68]

    sub_40f510(0);
    if (a3)
    {
        __fprintf_chk(*((unsigned int *)&g_426188), 0x1, "%s %s", a0, a3);
    }
    else
    {
        if (!localtime(a1 + 26) || !sub_414460(v0, 43, g_426400))
            __sprintf_chk(v0, 0x1);
        __fprintf_chk(*((unsigned int *)&g_426188), 0x1, "%s %s\t%s", a0, a2, v0);
    }
    sub_40f510(3);
    v2 = g_426188[1].field_0;
    if (v2 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v2 + 1;
        *(v2) = 10;
    }
    else
    {
        __overflow();
    }
    return;
}



// Function: find_function @ 0x7e50
extern unsigned long long g_426170;
extern long long g_426178;

unsigned long long find_function(unsigned long a0, long long a1)
{
    long long v1;  // rbp
    long long idx;  // rbx
    unsigned long long v3;  // r12
    unsigned long long v4;  // r12

    v1 = g_426178;
    idx = a1;
    g_426178 = a1;
    while (1)
    {
        idx -= 1;
        if (v1 > idx)
            break;
        v3 = *((long long *)(a0 + idx * 8));
        if (re_search() >= 0)
        {
            g_426170 = idx;
            return v3;
        }
    }
    v4 = 0;
    if (g_426170 != 0x7fffffffffffffff)
        v4 = *((long long *)(a0 + g_426170 * 8));
    return v4;
}



// Function: print_context_function @ 0x7f00
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    char padding_0[1];
    char field_1;
} struct_0;

long long print_context_function(FILE *idx, char *a1)
{
    char *v1;  // rbx
    char *v2;  // rax
    char v11;  // al
    int v3;  // ecx
    char *v4;  // rdx
    char v5;  // al
    char *v6;  // rdx
    struct_0 *v7;  // rsi
    int iter;  // edx
    struct_0 *v9;  // rsi
    void* node;  // rsi

    v1 = a1;
    v2 = *((long long *)&idx[40]);
    if (*((long long *)&idx[48]) > v2)
    {
        *((char **)&idx[40]) = v2 + 1;
        *(v2) = 32;
    }
    else
    {
        __overflow();
    }
    v3 = 0;
    v4 = v1;
    while (1)
    {
        v5 = *(v4);
        if (v5 > 13)
        {
            if (v5 != 32)
                break;
            v6 = v4 + 1;
        }
        else if (v5 <= 8 || !(v6 = v4 + 1, v5 != 10))
        {
            break;
        }
        v4 = v6;
        v3 += 1;
    }
    v7 = &v1[v3 + 1];
    iter = v3;
    if (v5 != 10)
    {
        do
        {
            v9 = v7;
            iter += 1;
        } while (iter <= v3 + 39 && (v7 = v9 + 1, v9->padding_0 != 10));
    }
    if (v3 < iter)
    {
        node = &v1[iter] - 1;
        do
        {
            v11 = *((char *)node);
            if (v11 <= 13)
            {
                if (v11 <= 8)
                    break;
            }
            else
            {
                if (v11 != 32)
                    break;
            }
        } while ((iter = (int)(iter - 1), node -= 1, v3 != iter));
    }
    return fwrite_unlocked(v4, 1, iter - v3, idx);
}



// Function: print_context_number_range @ 0x7fe0
extern unsigned long long g_426188;

void print_context_number_range(void)
{
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_40f5e0();
    if (v1 > v0)
    {
        __fprintf_chk(g_426188, 0x1, "%ld,%ld", v0, v1);
        return;
    }
    __fprintf_chk(g_426188, 0x1, "%ld", v1);
    return;
}



// Function: pr_context_hunk @ 0x8070
typedef struct struct_0 {
    struct struct_0 *field_0;
    long long field_8;
    long long field_10;
    char padding_18[8];
    long long field_20;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    unsigned long long field_28;
} struct_1;

extern struct_1 *g_426188;
extern unsigned int g_426258;
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_426388;
extern unsigned long long g_4263a0;
extern unsigned long long g_426520;
extern unsigned long long g_426570;

int pr_context_hunk(void)
{
    struct_0 *v6;  // rdi
    struct_0 *v7;  // rbx
    long long v16;  // rdi
    unsigned long v17;  // r13
    long long v18;  // rsi
    long long v19;  // rdx
    long long v20;  // rcx
    long long v21;  // r8
    long long v22;  // r9
    struct_1 *idx;  // rbp
    unsigned long v24;  // rax
    unsigned long v25;  // rax
    unsigned int v8;  // eax
    long long k;  // r15
    struct_0 *v27;  // r14
    long long v28;  // rdi
    unsigned long long v29;  // r13
    unsigned long v30;  // rax
    unsigned long v31;  // rax
    unsigned long l;  // r15
    long long v33;  // rdi
    unsigned long long v34;  // r13
    unsigned long v35;  // rax
    unsigned int v9;  // r12d
    unsigned long v10;  // rcx
    long long v11;  // rdx
    long long v12;  // rdx
    long long v13;  // rdx
    long long v14;  // r8
    long long v15;  // rdx
    long long v0;  // [bp-0x60], Other Possible Types: unsigned long
    long long i;  // [bp-0x58]
    unsigned long v2;  // [bp-0x50]
    long long v3;  // [bp-0x48]
    long long j;  // [bp-0x48]

    v7 = v6;
    v8 = sub_40f670();
    if (!v8)
        return;
    v9 = v8;
    v10 = -(g_426288);
    v11 = v0 - g_426570;
    if (v11 < v10)
        v11 = v10;
    v12 = v2 - g_426570;
    v0 = v11;
    if (v12 >= v10)
        v10 = v12;
    v2 = v10;
    v13 = g_426270 - 1;
    v14 = g_426270 - g_426570;
    if (i < v14)
        v13 = g_426570 + i;
    i = v13;
    v15 = g_4263a0 - 1;
    v16 = g_4263a0 - g_426570;
    if (v3 < v16)
        v15 = g_426570 + v3;
    j = v15;
    if (g_426520)
    {
        v17 = sub_407e50(*((long long *)&g_426258), v11);
        sub_40e830(v6, v18, v19, v20, v21, v22);
        idx = g_426188;
        fputs_unlocked("***************", g_426188);
        if (v17)
            sub_407f00(g_426188, v17);
    }
    else
    {
        sub_40e830(v16, v11, v15, v3, v14);
        idx = g_426188;
        fputs_unlocked("***************", g_426188);
    }
    v24 = idx->field_28;
    if (v24 < *((long long *)&idx[1].padding_0[0]))
    {
        idx->field_28 = v24 + 1;
        *((char *)v24) = 10;
    }
    else
    {
        __overflow(idx, 10);
    }
    sub_40f510(4);
    fputs_unlocked("*** ", idx);
    sub_407fe0();
    fputs_unlocked(" ****", idx);
    sub_40f510(3);
    v25 = idx->field_28;
    if (v25 < *((long long *)&idx[1].padding_0[0]))
    {
        idx->field_28 = v25 + 1;
        *((char *)v25) = 10;
    }
    else
    {
        __overflow(idx, 10);
    }
    if ((char)v9 & 1)
    {
        k = v0;
        if (k <= i)
        {
            v27 = v7;
            do
            {
                sub_40f510(2);
                if (v27)
                {
                    while (v27->field_10 + v27->padding_18 <= k)
                    {
                        if (!v27->field_0)
                            goto LABEL_408440;
                    }
                    if (v27->padding_18 <= k)
                    {
                        v28 = "-";
                        if (NULL < v27->field_8)
                        {
                            v28 = "!";
                            goto LABEL_408274;
                        }
                        else
                        {
                            goto LABEL_408274;
                        }
                    }
                }
LABEL_408440:
                v28 = " ";
LABEL_408274:
                v29 = k * 8;
                sub_40f330(v28, *((long long *)&g_426258) + v29, 1);
                sub_40f510(3);
                if (*((char *)(*((long long *)(*((long long *)&g_426258) + v29 + 8)) - 1)) == 10)
                {
                    v30 = idx->field_28;
                    if (v30 < *((long long *)&idx[1].padding_0[0]))
                    {
                        idx->field_28 = v30 + 1;
                        *((char *)v30) = 10;
                    }
                    else
                    {
                        __overflow(idx, 10);
                    }
                }
                k += 1;
            } while (i >= k);
        }
    }
    sub_40f510(4);
    fputs_unlocked("--- ", idx);
    sub_407fe0();
    fputs_unlocked(" ----", idx);
    sub_40f510(3);
    v31 = idx->field_28;
    if (v31 < *((long long *)&idx[1].padding_0[0]))
    {
        idx->field_28 = v31 + 1;
        *((char *)v31) = 10;
    }
    else
    {
        __overflow(idx, 10);
    }
    if (!((char)v9 & 2))
        return;
    l = v2;
    if (l > j)
        return;
    do
    {
        sub_40f510(1);
        if (v7)
        {
            while (v7->field_8 + v7->field_20 <= l)
            {
                if (!v7->field_0)
                    goto LABEL_408430;
            }
            if (v7->field_20 <= l)
            {
                v33 = "+";
                if (NULL < v7->field_10)
                {
                    v33 = "!";
                    goto LABEL_40839d;
                }
                else
                {
                    goto LABEL_40839d;
                }
            }
        }
LABEL_408430:
        v33 = " ";
LABEL_40839d:
        v34 = l * 8;
        sub_40f330(v33, g_426388 + v34, 1);
        sub_40f510(3);
        if (*((char *)(*((long long *)(g_426388 + v34 + 8)) - 1)) == 10)
        {
            v35 = idx->field_28;
            if (v35 < *((long long *)&idx[1].padding_0[0]))
            {
                idx->field_28 = v35 + 1;
                *((char *)v35) = 10;
            }
            else
            {
                __overflow(idx, 10);
            }
        }
        l += 1;
    } while (j >= l);
    return;
}



// Function: print_unidiff_number_range @ 0x84f0
extern unsigned long long g_426188;

void print_unidiff_number_range(void)
{
    char *v3;  // rdx
    long long v0;  // [bp-0x20]
    long long v1;  // [bp-0x18]

    sub_40f5e0();
    if (v1 > v0)
    {
        __fprintf_chk(g_426188, 0x1, "%ld,%ld", v0, v1 - v0 + 1);
        return;
    }
    v3 = "%ld,0";
    if (v1 >= v0)
        v3 = "%ld";
    __fprintf_chk(g_426188, 0x1, v3);
    return;
}



// Function: pr_unidiff_hunk @ 0x8590
typedef struct struct_3 {
    struct struct_2 *field_0;
    void* field_8;
} struct_3;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    long long field_18;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    unsigned long long field_28;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

extern struct_1 *g_426188;
extern unsigned int g_426258;
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_426388;
extern unsigned long long g_4263a0;
extern char g_426491;
extern char g_426492;
extern unsigned long long g_426520;
extern unsigned long long g_426570;

void pr_unidiff_hunk(struct_0 *a0, unsigned long a1, unsigned long a2, unsigned long a3, unsigned long a4, unsigned long a5, unsigned long a6)
{
    struct_0 *index;  // r14
    long long v8;  // rdx
    long long v17;  // rsi
    long long v18;  // rdx
    long long v19;  // rcx
    long long v20;  // r8
    long long v21;  // r9
    struct_1 *idx;  // r15
    unsigned long v23;  // rax
    long long v24;  // r13
    struct_2 **v25;  // rbp
    unsigned long v26;  // rax
    unsigned long v9;  // rcx
    char v27;  // sil
    unsigned long long v28;  // rbx
    unsigned long long iter;  // r12
    unsigned long long v30;  // rbx
    unsigned long v31;  // rax
    struct_3 *v32;  // rbp
    unsigned long v33;  // rax
    unsigned long v34;  // rax
    unsigned long long v35;  // rbx
    unsigned long long node;  // r12
    long long v10;  // rdx
    unsigned long long v37;  // rbx
    unsigned long v38;  // rax
    struct_3 *v39;  // rbp
    unsigned long v40;  // rax
    unsigned long v41;  // rax
    long long v11;  // rdx
    long long v12;  // r8
    long long v13;  // rdx
    long long v14;  // rdi
    unsigned long v15;  // rbp
    long long v16;  // rdi
    unsigned long v0;  // [bp-0x78]
    unsigned long v1;  // [bp-0x70]
    long long v2;  // [bp-0x60], Other Possible Types: unsigned long
    long long v3;  // [bp-0x58]
    unsigned long v4;  // [bp-0x50]
    long long v5;  // [bp-0x48]
    long long v6;  // [bp-0x48]

    index = a0;
    if (!sub_40f670())
        return;
    v8 = v2 - g_426570;
    v9 = -(g_426288);
    if (v8 < v9)
        v8 = v9;
    v10 = v4 - g_426570;
    v2 = v8;
    if (v10 >= v9)
        v9 = v10;
    v4 = v9;
    v11 = g_426270 - 1;
    v12 = g_426270 - g_426570;
    if (v3 < v12)
        v11 = g_426570 + v3;
    v3 = v11;
    v13 = g_4263a0 - 1;
    v14 = g_4263a0 - g_426570;
    if (v5 < v14)
        v13 = g_426570 + v5;
    v6 = v13;
    if (g_426520)
    {
        v15 = sub_407e50(*((long long *)&g_426258), v8);
        sub_40e830(v16, v17, v18, v19, v20, v21);
        idx = g_426188;
        sub_40f510(4);
        fputs_unlocked("@@ -", g_426188);
        sub_4084f0();
        fputs_unlocked(" +", g_426188);
        sub_4084f0();
        fputs_unlocked(" @@", g_426188);
        sub_40f510(3);
        if (v15)
            sub_407f00(g_426188, v15);
    }
    else
    {
        sub_40e830(v14, v8, v13, v5, v12);
        idx = g_426188;
        sub_40f510(4);
        fputs_unlocked("@@ -", g_426188);
        sub_4084f0();
        fputs_unlocked(" +", g_426188);
        sub_4084f0();
        fputs_unlocked(" @@", g_426188);
        sub_40f510(3);
    }
    v23 = idx->field_28;
    if (v23 < *((long long *)&idx[1].padding_0[0]))
    {
        idx->field_28 = v23 + 1;
        *((char *)v23) = 10;
    }
    else
    {
        __overflow(idx, 10);
    }
    v24 = v2;
    for (v0 = v4; v3 >= v24 || v6 >= v0; index = index->field_0)
    {
        while (!index || index->field_18 > v24)
        {
            v25 = *((long long *)&g_426258) + v24 * 8;
            if (!g_426491 || *(v25)->field_0 != 10)
            {
                v26 = idx->field_28;
                v27 = -(g_426492 < 1);
                if (v26 < *((long long *)&idx[1].padding_0[0]))
                {
                    idx->field_28 = v26 + 1;
                    *((char *)v26) = (v27 & 23) + 9;
                }
                else
                {
                    __overflow(idx);
                }
            }
            v24 += 1;
            sub_40f500(0, v25);
            v0 += 1;
            if (v3 < v24 && v6 < v0)
                return;
        }
        v28 = index->field_10;
        v1 = v28 - 1;
        if (v28)
        {
            iter = v24 * 8;
            while (1)
            {
                v30 = v28;
                sub_40f510(2);
                v31 = idx->field_28;
                v32 = *((long long *)&g_426258) + iter;
                if (v31 < *((long long *)&idx[1].padding_0[0]))
                {
                    idx->field_28 = v31 + 1;
                    *((char *)v31) = 45;
                }
                else
                {
                    __overflow(idx, 45);
                }
                if (g_426492 && (!g_426491 || v32->field_0->field_0 != 10))
                {
                    v33 = idx->field_28;
                    if (v33 < *((long long *)&idx[1].padding_0[0]))
                    {
                        idx->field_28 = v33 + 1;
                        *((char *)v33) = 9;
                    }
                    else
                    {
                        __overflow(idx, 9);
                    }
                }
                sub_40f330(0, v32, 1);
                sub_40f510(3);
                if (*((char *)v32->field_8 - 1) == 10)
                {
                    v34 = idx->field_28;
                    if (v34 < *((long long *)&idx[1].padding_0[0]))
                    {
                        iter += 8;
                        idx->field_28 = v34 + 1;
                        *((char *)v34) = 10;
                        v28 = v30 - 1;
                        if (v30 == 1)
                            break;
                        else
                            continue;
                    }
                    else
                    {
                        __overflow(idx, 10);
                    }
                }
                iter += 8;
                v28 = v30 - 1;
                if (v30 == 1)
                    break;
            }
            v24 = v24 + v1 + 1;
        }
        v35 = index->field_8;
        v1 = v35 - 1;
        if (v35)
        {
            node = v0 * 8;
            while (1)
            {
                v37 = v35;
                sub_40f510(1);
                v38 = idx->field_28;
                v39 = g_426388 + node;
                if (v38 < *((long long *)&idx[1].padding_0[0]))
                {
                    idx->field_28 = v38 + 1;
                    *((char *)v38) = 43;
                }
                else
                {
                    __overflow(idx, 43);
                }
                if (g_426492 && (!g_426491 || v39->field_0->field_0 != 10))
                {
                    v40 = idx->field_28;
                    if (v40 < *((long long *)&idx[1].padding_0[0]))
                    {
                        idx->field_28 = v40 + 1;
                        *((char *)v40) = 9;
                    }
                    else
                    {
                        __overflow(idx, 9);
                    }
                }
                sub_40f330(0, v39, 1);
                sub_40f510(3);
                if (*((char *)v39->field_8 - 1) == 10)
                {
                    v41 = idx->field_28;
                    if (v41 < *((long long *)&idx[1].padding_0[0]))
                    {
                        node += 8;
                        idx->field_28 = v41 + 1;
                        *((char *)v41) = 10;
                        v35 = v37 - 1;
                        if (v37 == 1)
                            break;
                        else
                            continue;
                    }
                    else
                    {
                        __overflow(idx, 10);
                    }
                }
                node += 8;
                v35 = v37 - 1;
                if (v37 == 1)
                    break;
            }
            v0 = v0 + v1 + 1;
        }
    }
    return;
}



// Function: find_hunk @ 0x8b10
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
    char field_28;
} struct_0;

extern long long g_426570;

struct_0 * find_hunk(struct_0 *idx)
{
    long long v1;  // r11
    unsigned long v2;  // rdx
    unsigned long v3;  // rax
    struct_0 *v4;  // r8
    unsigned long v5;  // rsi
    long long v6;  // r9
    long long v7;  // rcx
    unsigned long v8;  // rdx

    v1 = g_426570;
    v2 = idx->field_18;
    v3 = idx->field_20;
    while (1)
    {
        v4 = idx;
        idx = idx->field_0;
        if (!idx)
            return v4;
        v5 = idx->field_18;
        v6 = g_426570 * 2 + 1;
        if (idx->field_28)
            v6 = v1;
        v7 = idx->field_18 - (v2 + v4->field_10);
        v8 = idx->field_20;
        if (v7 != idx->field_20 - (v3 + v4->field_8))
        {
            abort(); /* do not return */
        }
        else if (v7 < v6)
        {
            v2 = v5;
            v3 = v8;
        }
        else
        {
            return v4;
        }
    }
}



// Function: print_context_header @ 0x8b80
extern char *g_426540;
extern char *g_426548;

void print_context_header(int *a0, char **a1, char a2)
{
    int *v1;  // rbp

    v1 = a0 + 76;
    if (a2)
    {
        sub_407cf0("---", a0, *(a1), g_426540);
        sub_407cf0("+++", v1, a1[1], g_426548);
    }
    else
    {
        sub_407cf0("***", a0, *(a1), g_426540);
        sub_407cf0("---", v1, a1[1], g_426548);
    }
    return;
}



// Function: print_context_script @ 0x8c10
typedef struct struct_0 {
    struct struct_0 *field_0;
    char padding_8[32];
    char field_28;
} struct_0;

extern unsigned long long g_426170;
extern unsigned long long g_426178;
extern unsigned long long g_426288;
extern unsigned long long g_4264e0;
extern char g_426554;

long long print_context_script(struct_0 *a0, char a1)
{
    unsigned long v6;  // fs
    struct_0 *v7;  // rax
    struct_0 *v8;  // rax
    struct_0 *i;  // rbx
    struct_0 *v10;  // r14
    unsigned int v11;  // eax
    char v0;  // [bp-0x60]
    char v1;  // [bp-0x58]
    char v2;  // [bp-0x50]
    char v3;  // [bp-0x48]
    unsigned long v4;  // [bp-0x40]

    v4 = *((long long *)(40 + v6));
    if (!(g_426554 || g_4264e0))
    {
        v7 = a0;
        if (a0)
        {
            do
            {
                v7->field_28 = 0;
                v8 = v7->field_0;
            } while (v8 && (v8->field_28 = 0, v7 = (struct_0 *)v8->field_0, v8->field_0));
        }
    }
    else if (a0)
    {
        i = a0;
        do
        {
            v10 = i;
            i = i->field_0;
            v10->field_0 = NULL;
            v11 = sub_40f670(v10, &v0, &v1, &v2, &v3);
            v10->field_0 = i;
            v10->field_28 = !v11;
        } while (i);
    }
    g_426178 = -(g_426288);
    g_426170 = 0x7fffffffffffffff;
    if (a1)
    {
        if (v4 != *((long long *)(40 + v6)))
            __stack_chk_fail(); /* do not return */
    }
    else
    {
        if (v4 != *((long long *)(40 + v6)))
            __stack_chk_fail(); /* do not return */
    }
    return sub_40f080(a0, sub_408b10);
}



// Function: add_regexp @ 0x8d60
typedef struct struct_0 {
    long long field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    char field_18;
    char padding_19[7];
    long long field_20;
} struct_0;

long long add_regexp(struct_0 *idx, unsigned long a1)
{
    unsigned long len;  // rbp
    long long v2;  // r14
    unsigned long long v3;  // r13
    unsigned long v4;  // rsi
    long long v5;  // rax
    unsigned long i;  // rdx
    unsigned long v7;  // r13

    len = strlen(a1);
    if (re_compile_pattern(a1, len, idx->field_20))
        return (unsigned long long)error(2, 0, "%s: %s");
    v2 = idx->field_0;
    v3 = idx->field_8;
    v4 = idx->field_10;
    idx->field_18 = v2;
    v5 = v2;
    i = ((v2) * 2 & 2) + v3 + len;
    idx->field_8 = i;
    if (i >= v4)
    {
        if (!v4)
            v4 = 1;
        do
        {
            v4 *= 2;
        } while (i >= v4);
        idx->field_10 = v4;
        v5 = sub_416a90(v2);
        idx->field_0 = v5;
    }
    if (v2)
    {
        *((char *)(v5 + v3)) = 92;
        v7 = v3 + 2;
        *((char *)(v5 + v3 + 1)) = 124;
        v3 = v7;
    }
    return memcpy(v5 + v3, a1, len + 1);
}



// Function: try_help @ 0x8e40
void try_help(char *a0, unsigned long a1)
{
    unsigned long long v2;  // r12
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    if (a0)
        error(0, 0, dcgettext(NULL, a0, 5));
    error(2, 0, dcgettext(NULL, "Try '%s --help' for more information.", 5));
}



// Function: compare_files @ 0x8f90
typedef struct struct_1 {
    char field_0;
    char padding_1[303];
    unsigned int field_130;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[36];
    unsigned int field_28;
    char padding_2c[20];
    unsigned long long field_40;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern char g_426550;
extern char g_426553;
extern char g_426578;
extern unsigned int g_426580;
extern char g_426584;
extern char g_426585;
extern char g_426586;
extern char g_426608;

unsigned long long compare_files(long long *a0, unsigned long a1, unsigned long a2)
{
    unsigned long v12;  // r13
    unsigned long v13;  // r12
    unsigned long v22;  // rax
    void* iter;  // r15
    int v24;  // r13d
    long long v25;  // rax
    long long v26;  // rax
    struct_0 *node;  // rax
    struct_1 *v28;  // rsi
    unsigned int v29;  // r15d
    unsigned int v30;  // edx
    unsigned int v31;  // esi
    char v14;  // dl
    unsigned int v32;  // r9d
    int v33;  // edx
    unsigned long long v34;  // rcx
    void* iter1;  // rdi
    int *err;  // rbx
    int v37;  // edx
    unsigned long long v38;  // rcx
    void* iter2;  // rdi
    unsigned long long v40;  // rcx
    void* n;  // rdi
    unsigned long long v15;  // rcx
    unsigned int v42;  // eax
    unsigned long v43;  // rax
    unsigned long long idx;  // rax
    long long v45;  // rax
    unsigned long long v46;  // rax
    unsigned int v48;  // edx
    unsigned int v49;  // r10d
    unsigned int v50;  // edi
    unsigned int v51;  // eax
    unsigned long long *j;  // rdi
    unsigned int v52;  // r9d
    unsigned int v53;  // ecx
    char v54;  // dil
    char v55;  // dl
    unsigned int v56;  // eax
    unsigned int v57;  // ebx
    unsigned int v58;  // eax
    unsigned int v59;  // edi
    unsigned int v60;  // ebx
    unsigned int v61;  // edi
    unsigned long long v17;  // rcx
    unsigned int v62;  // edi
    void* ptr;  // rbp
    void* ptr1;  // r12
    unsigned long long *i;  // rdi
    unsigned long long v19;  // rcx
    unsigned long long *k;  // rdi
    unsigned long v21;  // r14
    unsigned long v2;  // [bp-0x2c0]
    unsigned int v3;  // [bp-0x2b8], Other Possible Types: unsigned long, unsigned long long, int
    unsigned long v4;  // [bp-0x2b0]
    int v5;  // [bp-0x2a8], Other Possible Types: char
    uint128_t v6;  // [bp-0x118]
    uint128_t v7;  // [bp-0x108]
    int v8;  // [bp-0xf8]
    int v9;  // [bp-0xe8]
    long long *v10;  // [bp-0x48]

    v12 = a2;
    v13 = a1;
    v14 = !a2;
    if (a1 && !v14)
    {
        v15 = 76;
        v10 = a0;
        for (j = &v5; v15; j += 1)
        {
            v15 -= 1;
            *(j) = 0;
        }
        *((unsigned int *)&v5) = 0xfffffffe;
        goto LABEL_409062;
    }
    if (g_426585 == 1 && !v14)
    {
        v17 = 76;
        v10 = a0;
        for (i = &v5; v17; i += 1)
        {
            v17 -= 1;
            *(i) = 0;
        }
        if (v13)
        {
            *((unsigned int *)&v5) = 0xfffffffe;
            *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
LABEL_409020:
            v13 = v12;
            if (a0)
                goto LABEL_409072;
            goto LABEL_409028;
        }
        else
        {
            *((unsigned int *)&v5) = 0xffffffff;
        }
LABEL_409698:
        *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
        if (v12)
            goto LABEL_409020;
        goto LABEL_409688;
    }
    else
    {
        if (!g_426586)
        {
            sub_40e660();
            return 1;
        }
        v19 = 76;
        v10 = a0;
        for (k = &v5; v19; k += 1)
        {
            v19 -= 1;
            *(k) = 0;
        }
        if (v13)
        {
            *((unsigned int *)&v5) = 0xfffffffe;
            if (v12)
            {
LABEL_409062:
                *((unsigned int *)&(&v5)[304]) = 0xfffffffe;
            }
            else
            {
                *((unsigned int *)&(&v5)[304]) = 0xffffffff;
                v12 = v13;
            }
        }
        else
        {
            *((unsigned int *)&v5) = 0xffffffff;
            if (v12)
                goto LABEL_409698;
            *((unsigned int *)&(&v5)[304]) = 0xffffffff;
LABEL_409688:
            v13 = 0;
        }
        if (a0)
        {
LABEL_409072:
            v22 = sub_410d80(a0[1], v13, 0);
            *((unsigned long *)&(&v5)[8]) = v22;
            v21 = v22;
            v2 = sub_410d80(a0[39], v12, 0);
            v12 = v2;
LABEL_4090a1:
            *((unsigned long *)&(&v5)[312]) = v12;
            iter = &v5 - 104;
            v24 = 0;
            while (1)
            {
                if (*((int *)((char *)iter - 104)) != 0xffffffff)
                {
                    if (v24 && !strcmp(*((unsigned long long *)(&v5 + 312)), *((unsigned long long *)(&v5 + 8))))
                    {
                        memcpy(&v5 - 320, &v5 - 16, 16);
                        *((unsigned int *)&(&v5)[304]) = *((unsigned int *)&v5);
                        *((uint128_t *)&(&v5)[336]) = *((uint128_t *)(&v5 + 32));
                        *((uint128_t *)&(&v5)[352]) = *((uint128_t *)(&v5 + 48));
                        *((uint128_t *)&(&v5)[368]) = *((uint128_t *)(&v5 + 64));
                        *((uint128_t *)&(&v5)[384]) = *((uint128_t *)(&v5 + 80));
                        v6 = *((uint128_t *)(&v5 + 96));
                        v7 = *((uint128_t *)(&v5 + 112));
                        v8 = (int)*((uint128_t *)(&v5 + 128));
                        v9 = (int)*((uint128_t *)(&v5 + 144));
                    }
                    else
                    {
                        v3 = *((long long *)((char *)iter - 96));
                        if (!strcmp(*((long long *)((char *)iter - 96)), "-"))
                        {
                            *((unsigned int *)((char *)iter - 104)) = 0;
                            v3 = iter - 88;
                            isatty(0);
                            if (fstat(0, v3))
                            {
LABEL_409390:
                                *((unsigned int *)((char *)iter - 104)) = 0xfffffffd - *(__errno_location());
                            }
                            else
                            {
                                if (((short)*((int *)((char *)iter - 64)) & 0xf000) == 0x8000)
                                {
                                    v25 = lseek(0, 0, 1);
                                    if (v25 >= 0)
                                    {
                                        *((long long *)((char *)iter - 40)) = *((long long *)((char *)iter - 40)) - v25;
                                        v26 = *((long long *)((char *)iter - 40));
                                        if (*((long long *)((char *)iter - 40)) < 0)
                                            v26 = 0;
                                        *((long long *)((char *)iter - 40)) = v26;
                                    }
                                    else
                                    {
                                        *((unsigned int *)((char *)iter - 104)) = 0xfffffffd - *(__errno_location());
                                    }
                                }
                                sub_410eb0(iter);
                            }
                        }
                        else if (g_426550)
                        {
                            if (lstat(v3, v3))
                                goto LABEL_409390;
                        }
                        else
                        {
                            if (stat(v3, v3))
                                goto LABEL_409390;
                        }
                    }
                }
                iter += 304;
                if (v24 == 1)
                    break;
                v24 = 1;
            }
            node = &v5;
            v28 = &v5;
            v29 = 0;
            while (1)
            {
                v29 += 1;
                if (!g_426586 && v29 == 1)
                {
                    if (g_426585)
                        goto LABEL_4092e0;
                    goto LABEL_40932a;
                }
LABEL_4092e0:
                v30 = node->field_0;
                if (v30 != 0xfffffffe)
                {
                    if ((v30 == 0xfffffffb || v30 == 0xfffffff4) && !a0 && !(v28->field_130 + 2 & 0xfffffffd))
                        goto LABEL_40931a;
                    goto LABEL_4091b7;
                }
                else if (((unsigned short)node->field_28 & 0xf000) != 0x8000 || (unsigned short)node->field_28 & 0x1ff || node->field_40)
                {
LABEL_4091b7:
                    if (v29 == 2)
                        break;
                    else
                        goto LABEL_40932a;
                }
                else
                {
LABEL_40931a:
                    node->field_0 = 0xffffffff;
                    if (v29 == 2)
                        break;
LABEL_40932a:
                    node = &node[4].padding_4[12];
                    v28 = (char *)v28 - 304;
                }
            }
            v31 = *((unsigned int *)&v5);
            v32 = *((unsigned int *)(&v5 + 304));
            if (v31 != 0xffffffff)
            {
                v33 = 0xfffffffd - v31;
                if (v32 != 0xffffffff)
                {
                    if (v33 < 0)
                        goto LABEL_4093d7;
                    goto LABEL_4091ef;
                }
                v34 = 18;
                for (iter1 = &v5 - 320; v34; iter1 += 8)
                {
                    v34 -= 1;
                    *((unsigned long long *)iter1) = 0;
                }
                *((unsigned int *)&(&v5)[344]) = *((unsigned int *)(&v5 + 40));
                if (v33 < 0)
                    goto LABEL_4093e5;
LABEL_4091ef:
                v3 = v33;
                err = __errno_location();
                *(err) = v3;
                sub_40e490(*((unsigned long long *)(&v5 + 8)));
                v37 = 0xfffffffd - *((unsigned int *)(&v5 + 304));
                if ((unsigned int)(0xfffffffd - *((unsigned int *)(&v5 + 304))) >= 0)
                    goto LABEL_409219;
            }
            else
            {
                v38 = 18;
                for (iter2 = &v5 - 16; v38; iter2 += 8)
                {
                    v38 -= 1;
                    *((unsigned long long *)iter2) = 0;
                }
                *((unsigned int *)&(&v5)[40]) = *((unsigned int *)(&v5 + 344));
                if (v32 != 0xffffffff)
                {
LABEL_4093d7:
                    if ((unsigned int)(0xfffffffd - v32) < 0)
                        goto LABEL_4093e5;
                    v3 = 0xfffffffd - v32;
                    v37 = v3;
                    err = __errno_location();
LABEL_409219:
                    *(err) = v37;
                    sub_40e490(*((unsigned long long *)(&v5 + 312)));
                }
                else
                {
                    v40 = 18;
                    for (n = &v5 - 320; v40; n += 8)
                    {
                        v40 -= 1;
                        *((unsigned long long *)n) = 0;
                    }
                    *((unsigned int *)&(&v5)[344]) = *((unsigned int *)(&v5 + 40));
LABEL_4093e5:
                    if (!a0 && (v42 = *((unsigned int *)((void*)&v5 + 40)) & 0xf000, (char)(char)(v42 == 0x4000) != (char)(char)((*((unsigned int *)((void*)&v5 + 344)) & 0xf000) == 0x4000)))
                    {
                        v43 = v42 == 0x4000;
                        idx = v43 * 19;
                        v4 = *((long long *)((char *)&v5 + 16 * idx - 8));
                        v45 = sub_40fb40(*((long long *)((char *)&v5 + 16 * idx - 8)));
                        v46 = (int)(1 - v43) * 304;
                        v3 = v46;
                        v21 = sub_40a660(*((long long *)((char *)&v5 + v46 - 8)), v45);
                        *((unsigned long *)((char *)&v5 + v3 - 8)) = v21;
                        if (!strcmp(v4, "-"))
                            sub_40e7b0("cannot compare '-' to a directory"); /* do not return */
                        v31 = *((unsigned int *)&v5);
                        if (!(!g_426550 ? stat(v21, &v5 - 16 + v3) : lstat(v21, &v5 - 16 + v3)))
                            goto LABEL_4094c7;
                        sub_40e490(v21);
                    }
                    else
                    {
LABEL_4094c7:
                        v48 = *((unsigned int *)(&v5 + 304));
                        if (v31 != 0xffffffff)
                        {
                            v49 = *((unsigned int *)(&v5 + 40));
                            v50 = *((unsigned int *)(&v5 + 344));
                            if (v48 == 0xffffffff)
                            {
LABEL_4096c0:
                                v53 = 0;
                                v51 = v49 & 0xf000;
                            }
                            else if (*((unsigned long long *)(&v5 + 24)) != *((unsigned long long *)(&v5 + 328)) || *((unsigned long long *)(&v5 + 16)) != *((unsigned long long *)(&v5 + 320)))
                            {
                                v51 = v49 & 0xf000;
                                v52 = v50 & 0xf000;
                                if (v51 != 0x6000)
                                {
                                    v53 = 0;
                                    if (v51 == 0x2000)
                                    {
                                        if (v52 == 0x2000)
                                            goto LABEL_40990d;
                                        goto LABEL_409536;
                                    }
                                }
                                else
                                {
                                    if (v52 == 0x6000)
                                    {
LABEL_40990d:
                                        if (*((unsigned long long *)(&v5 + 360)) == *((unsigned long long *)(&v5 + 56)))
                                            goto LABEL_40984b;
                                    }
LABEL_409536:
                                    if (v51 == 0x4000 || !(v53 = 0, v52 != 0x4000))
                                        goto LABEL_409940;
                                    goto LABEL_40955c;
                                }
                            }
                            else
                            {
LABEL_40984b:
                                if (v49 != v50)
                                    goto LABEL_4096c0;
                                if (*((unsigned long long *)(&v5 + 32)) != *((unsigned long long *)(&v5 + 336)) || *((unsigned int *)(&v5 + 44)) != *((unsigned int *)(&v5 + 348)) || *((unsigned int *)(&v5 + 48)) != *((unsigned int *)(&v5 + 352)) || *((unsigned long long *)(&v5 + 64)) != *((unsigned long long *)(&v5 + 368)) || *((unsigned long long *)(&v5 + 104)) != *((unsigned long long *)((void*)&v6 + 8)) || *((unsigned long long *)(&v5 + 120)) != *((unsigned long long *)((void*)&v7 + 8)))
                                {
                                    v53 = 0;
                                    v51 = v50 & 0xf000;
                                }
                                else
                                {
                                    if (g_426578)
                                    {
LABEL_409730:
                                        if (g_426584 && ((unsigned short)*((unsigned int *)(&v5 + 40)) & 0xf000) != 0x4000)
                                            sub_40e660();
                                        free(v21);
                                        free(v2);
                                        return 0;
                                    }
                                    v53 = 1;
                                    v51 = v50 & 0xf000;
                                }
                            }
                        }
                        else
                        {
                            if (v48 == 0xffffffff)
                                goto LABEL_409730;
                            v50 = *((unsigned int *)(&v5 + 344));
                            v53 = 0;
                            v51 = *((unsigned int *)(&v5 + 40)) & 0xf000;
                        }
                        v52 = v50 & 0xf000;
                        v54 = v52 == 0x4000;
                        if (v51 == 0x4000)
                        {
                            if (v54)
                            {
                                if (g_426580 == 7)
                                    sub_40e7b0("-D option not supported with directories"); /* do not return */
                                if (!a0 || g_426608)
                                    goto LABEL_409717;
                                sub_40e660("Common subdirectories: %s and %s\n", *((unsigned long long *)(&v5 + 8)), *((unsigned long long *)(&v5 + 312)));
                                goto LABEL_409730;
                            }
LABEL_409931:
                            if (v31 == 0xffffffff)
                                goto LABEL_409949;
LABEL_409940:
                            if (v48 != 0xffffffff)
                                goto LABEL_4098a0;
LABEL_409949:
                            if (g_426608 && (g_426586 || g_426585 && v31 == 0xffffffff))
                            {
LABEL_409717:
                                v60 = sub_40a230(&v5, compare_files);
LABEL_409728:
                                if (!v60)
                                    goto LABEL_409730;
                                v29 = v60;
                            }
                            else
                            {
                                if (!a0)
                                    __assert_fail(); /* do not return */
LABEL_409598:
                                v29 = 1;
                                sub_40e660("Only in %s: %s\n", a0[1 + 38 * (v31 == 0xffffffff)], v13);
                            }
                        }
                        else
                        {
                            if (v54)
                                goto LABEL_409931;
LABEL_40955c:
                            if (a0 && ((v51 & 0xffffdfff) != 0x8000 || (v52 & 0xffffdfff) != 0x8000))
                            {
                                if (v31 == 0xffffffff || v48 == 0xffffffff)
                                    goto LABEL_409598;
                                goto LABEL_4098a0;
                            }
                            v55 = v52 == 0xa000;
                            if (!(v51 != 0xa000 && !v55))
                            {
                                if (!g_426550)
                                    __assert_fail(); /* do not return */
                                if (v51 != 0xa000 || !v55)
                                {
LABEL_4098a0:
                                    v29 = 1;
                                    sub_410c90(&v5 - 320);
                                    sub_410c90();
                                    sub_40e4c0();
                                }
                                else
                                {
                                    ptr = sub_416f20(*((unsigned long long *)(&v5 + 8)));
                                    if (ptr)
                                    {
                                        ptr1 = sub_416f20(*((unsigned long long *)(&v5 + 312)));
                                        if (!ptr1)
                                            goto LABEL_409d6e;
                                        if (!strcmp(ptr, ptr1))
                                        {
                                            free(ptr);
                                            free(ptr1);
                                            goto LABEL_409730;
                                        }
                                        else
                                        {
                                            v29 = 1;
                                            sub_40e660("Symbolic links %s and %s differ\n", *((unsigned long long *)(&v5 + 8)), *((unsigned long long *)(&v5 + 312)));
                                            free(ptr);
                                            free(ptr1);
                                        }
                                    }
                                    else
                                    {
                                        v24 = 0;
LABEL_409d6e:
                                        sub_40e490(*((long long *)((char *)&v5 + 304 * v24 - 8)));
                                        free(ptr);
                                        free(NULL);
                                    }
                                }
                            }
                            else if (!(!(v51 == 0x8000 & g_426553) || v52 != 0x8000 || !(*((unsigned long long *)(&v5 + 64)) > 0 & *((unsigned long long *)(&v5 + 64)) != *((unsigned long long *)(&v5 + 368))) || *((unsigned long long *)(&v5 + 368)) <= 0))
                            {
                                v29 = 1;
                                sub_40e660("Files %s and %s differ\n");
                            }
                            else if (v31 != 0xfffffffe || !(v3 = v53, v56 = (unsigned int)(int)(long long)open(*((unsigned long long *)((void*)&v5 + 8)), 0, 0), v53 = v3, *((unsigned int *)&v5) = v56, v56 < 0))
                            {
                                v57 = 0;
                                if (*((unsigned int *)(&v5 + 304)) != 0xfffffffe)
                                    goto LABEL_409b2f;
                                goto LABEL_409a17;
                            }
                            else
                            {
                                sub_40e490(*((unsigned long long *)(&v5 + 8)));
                                v53 = v3;
                                if (*((unsigned int *)(&v5 + 304)) != 0xfffffffe)
                                {
LABEL_409a32:
                                    v59 = *((unsigned int *)&v5);
                                    if (v59 < 0)
                                    {
                                        v61 = *((unsigned int *)(&v5 + 304));
                                        if (v61 >= 0)
                                            goto LABEL_409a4d;
                                    }
                                    else
                                    {
                                        v60 = 2;
LABEL_409b75:
                                        if (!close(v59))
                                        {
LABEL_409b41:
                                            v62 = *((unsigned int *)(&v5 + 304));
                                            if (v62 < 0 || v62 == *((unsigned int *)&v5) || !close(v62))
                                                goto LABEL_409728;
LABEL_409a5a:
                                            sub_40e490(*((unsigned long long *)(&v5 + 312)));
                                        }
                                        else
                                        {
                                            sub_40e490(*((unsigned long long *)(&v5 + 8)));
                                            v61 = *((unsigned int *)(&v5 + 304));
                                            if (v61 >= 0 && *((unsigned int *)&v5) != v61)
                                            {
LABEL_409a4d:
                                                if (close(v61))
                                                    goto LABEL_409a5a;
                                            }
                                        }
                                    }
                                }
                                else
                                {
                                    v57 = 2;
LABEL_409a17:
                                    if (v53)
                                    {
                                        *((unsigned int *)&(&v5)[304]) = *((unsigned int *)&v5);
                                        goto LABEL_409a2a;
                                    }
                                    else
                                    {
                                        v58 = open(*((unsigned long long *)(&v5 + 312)), 0, 0);
                                        *((unsigned int *)&(&v5)[304]) = v58;
                                        if (v58 >= 0)
                                        {
LABEL_409a2a:
                                            if (!v57)
                                            {
LABEL_409b2f:
                                                v59 = *((unsigned int *)&v5);
                                                v60 = sub_406e20(&v5);
                                                if (v59 < 0)
                                                    goto LABEL_409b41;
                                                goto LABEL_409b75;
                                            }
                                        }
                                        else
                                        {
                                            sub_40e490(*((unsigned long long *)(&v5 + 312)));
                                            goto LABEL_409a32;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if (fflush_unlocked(stdout))
                sub_40e770(dcgettext(NULL, "standard output", 5)); /* do not return */
            free(v21);
            free(v2);
            return v29;
        }
LABEL_409028:
        *((unsigned long *)&(&v5)[8]) = v13;
        v21 = 0;
        v2 = 0;
        goto LABEL_4090a1;
    }
}



// Function: dir_loop @ 0x9e40
typedef struct struct_1 {
    char padding_0[16];
    unsigned long long field_10;
    unsigned long long field_18;
    char padding_20[8];
    unsigned int field_28;
    char padding_2c[12];
    unsigned long long field_38;
    char padding_40[544];
    struct struct_0 *field_260;
} struct_1;

typedef struct struct_0 {
    char padding_0[608];
    struct struct_0 *field_260;
} struct_0;

struct_0 * dir_loop(struct_1 *a0, int a1)
{
    struct_0 *iter;  // rax
    unsigned long long v2;  // rcx
    struct_1 *v3;  // rsi
    unsigned long long v4;  // rdx

    iter = a0->field_260;
    if (!a0->field_260)
        return NULL;
    v2 = a1 * 304;
    v3 = &a0->padding_0[v2];
    while (1)
    {
        v4 = &iter->padding_0[v2];
        if (*((long long *)(v4 + 24)) == v3->field_18 && *((long long *)(v4 + 16)) == v3->field_10)
            return 0x1;
        switch (*((int *)&iter->padding_0[40 + v2]) & 0xf000)
        {
        case 24576:
            if (((unsigned short)v3->field_28 & 0xf000) == 0x6000)
            {
                if (*((long long *)&iter->padding_0[56 + v2]) == v3->field_38)
                    return 0x1;
                break;
            }
            break;
        case 8192:
            if ((v3->field_28 & 0xf000) == 0x2000)
                goto LABEL_409ef1;
            else
                goto LABEL_409e8a;
        default:
LABEL_409e8a:
            iter = iter->field_260;
            if (!iter)
                return iter;
            continue;
        }
LABEL_409ef1:
        if (*((long long *)&iter->padding_0[56 + v2]) == v3->field_38)
            return 0x1;
        iter = iter->field_260;
        if (!iter)
            return iter;
    }
}



// Function: compare_names @ 0x9f10
extern char g_426551;
extern char g_4266e8;

int compare_names(char *a0, char *a1)
{
    unsigned int *err;  // rax
    unsigned long v2;  // 4112
    int v3;  // eax

    if (g_4266e8)
    {
        err = __errno_location();
        v2 = g_426551;
        *(err) = 0;
        if ((char)v2)
            return strcasecmp(a0, a1);
        v3 = strcoll(a0, a1);
        if (v3)
            return v3;
    }
    return strcmp(a0, a1);
}



// Function: dir_read @ 0x9f90
typedef struct DIR {
} DIR;

typedef struct dirent {
    unsigned long d_ino;
    unsigned short d_reclen;
    char d_type;
    char d_namelen;
    char d_name[1];
} dirent;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    char *field_8;
} struct_0;

extern long long g_426410;

unsigned long long dir_read(struct_0 *a0, void* idx)
{
    unsigned long v6;  // 4143
    unsigned long long v7;  // r14
    void* v16;  // rax
    void* v17;  // rbx
    long long v18;  // rbp
    void* iter;  // rbx
    void* v20;  // rax
    unsigned long v8;  // rax
    unsigned long v9;  // r15
    dirent *count;  // rbx
    dirent *ptr;  // r13
    unsigned long v12;  // rax
    unsigned long long v13;  // rbx
    unsigned long v14;  // rax
    long long v15;  // r12
    unsigned long len;  // [bp-0x60]
    DIR *ptr1;  // [bp-0x58]
    unsigned int *err;  // [bp-0x50]
    unsigned long long v3;  // [bp-0x48]
    unsigned long long v4;  // [bp-0x40]

    v6 = a0->field_0;
    *((unsigned long long **)&idx[8]) = NULL;
    *((unsigned long *)&idx[16]) = 0;
    if ((unsigned int)v6 != 0xffffffff)
    {
        ptr1 = opendir(a0->field_8);
        if (!ptr1)
            return 0;
        v7 = 0x200;
        v8 = sub_416a40(0x200);
        *((unsigned long *)&idx[16]) = v8;
        v9 = v8;
        v3 = 0;
        err = __errno_location();
        v4 = 0;
        while (1)
        {
            *(err) = 0;
            count = readdir(ptr1);
            if (!count)
                break;
            ptr = (char *)&count[1].d_ino + 6;
            len = strlen(ptr);
            if ((*((char *)&count[1].d_ino + 6) != 46 || *((char *)&count[1].d_ino + 7) && (*((char *)&count[1].d_ino + 7) != 46 || (char)count[1].d_reclen)) && !(char)sub_4104d0(g_426410, ptr))
            {
                v12 = len + 1;
                v13 = v3 + v12;
                len = v12;
                if (v7 < v13)
                {
                    do
                    {
                        if (v7 > 0x3ffffffffffffffe)
                            sub_416ec0(); /* do not return */
                    } while ((v7 *= 2, v14 = (unsigned long)(unsigned long long)sub_416a90(v9, v7), *((unsigned long *)&idx[16]) = v14, v9 = v14, v7 < v13));
                }
                memcpy(v3 + v9, ptr, len);
                v4 += 1;
                v3 = v13;
            }
        }
        if (*(err))
        {
            closedir(ptr1);
            *(err) = *(err);
            return 0;
        }
        if (closedir(ptr1))
            return 0;
        if (v4 > 0xffffffffffffffd)
            sub_416ec0(); /* do not return */
        v15 = v4 * 8 + 8;
        v16 = sub_416a40(v15);
        *((unsigned long long *)idx) = v4;
        *((void* *)&idx[8]) = v16;
        v17 = v16;
        v18 = v15 - 8;
        if (v4)
        {
            iter = v17;
            do
            {
                *((unsigned long *)iter) = v9;
                iter += 8;
                v9 = v9 + strlen(v9) + 1;
            } while (iter != v18 + v17);
        }
        else
        {
            iter = v17 + v18;
        }
    }
    else
    {
        v20 = sub_416a40(8);
        *((unsigned long long *)idx) = 0;
        *((void* *)&idx[8]) = v20;
        iter = v20;
    }
    *((unsigned long *)iter) = 0;
    return 1;
}



// Function: compare_names_for_qsort @ 0xa1c0
extern char g_426551;
extern char g_4266e8;

int compare_names_for_qsort(char **a0, char **a1)
{
    char *v2;  // r12
    unsigned int *err;  // rax
    unsigned long v4;  // 4112
    char *v5;  // rdi
    int v6;  // eax
    char v0[8];  // [bp+0x0]

    v2 = *(a1);
    if (g_4266e8)
    {
        err = __errno_location();
        v4 = g_426551;
        v5 = *(a0);
        *(err) = 0;
        v6 = (!(char)v4 ? (int)strcoll(v5, v2) : (int)strcasecmp(v5, v2));
        if (v6)
            return v6;
    }
    return strcmp(v0, v2);
}



// Function: diff_dirs @ 0xa230
typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[288];
    unsigned int field_130;
    char padding_134[4];
    long long field_138;
    char padding_140[288];
    unsigned long long field_260;
} struct_0;

extern char *g_426488;
extern char g_426551;
extern long long g_426620;
extern char g_4266e8;

int diff_dirs(struct_0 *a0, unsigned long long *a1)
{
    unsigned int v10;  // eax
    unsigned int v11;  // eax
    unsigned long long idx;  // rax
    struct_1 *ptr;  // r14
    unsigned long v14;  // rbp
    unsigned long v15;  // r15
    unsigned long v16;  // r13
    unsigned long node;  // r12
    unsigned int v18;  // eax
    int v19;  // eax
    int v0;  // [bp-0x8c]
    void* iter;  // [bp-0x88]
    void* iter1;  // [bp-0x80]
    unsigned long v3;  // [bp-0x78]
    void* v4;  // [bp-0x70]
    void* v5;  // [bp-0x68]
    char v6;  // [bp-0x60]
    void* v7;  // [bp-0x58]
    void* v8;  // [bp-0x50]

    v0 = 0;
    if ((a0->field_0 == 0xffffffff || (char)sub_409e40(a0, 0)) && (a0->field_130 == 0xffffffff || (char)sub_409e40(a0, 1)))
    {
        error(0, 0, dcgettext(NULL, "%s: recursive directory loop", 5));
        return 2;
    }
    if (!(char)sub_409f90(a0, &v3))
    {
        sub_40e490(a0->field_8);
        v0 = 2;
    }
    if (!(char)sub_409f90(&a0->field_130, &v6))
    {
        sub_40e490(a0->field_138);
        v0 = 2;
        if (!v0)
        {
LABEL_40a36a:
            g_4266e8 = 1;
            iter = v4;
            iter1 = v7;
            if (_setjmp(&g_426620))
                g_4266e8 = 0;
            qsort(iter, v3, 8, sub_40a1c0);
            qsort(iter1, v6, 8, sub_40a1c0);
            if (g_426488 && !a0->field_260)
            {
                for (; *((long long *)iter) && sub_409f10(*((long long *)iter), g_426488) < 0; iter += 8);
                for (; *((long long *)iter1) && sub_409f10(*((long long *)iter1), g_426488) < 0; iter1 += 8);
            }
            while (*((long long *)iter) || *((long long *)iter1))
            {
                if (!*((long long *)iter))
                {
                    iter1 += 8;
                    goto LABEL_40a465;
                }
                if (*((long long *)iter1))
                {
                    v10 = sub_409f10(*((long long *)iter), *((long long *)iter1));
                    if (!v10)
                    {
                        if (g_426551)
                        {
                            v11 = strcmp(*((long long *)iter), *((long long *)iter1));
                            if (v11)
                            {
                                idx = v11 >> 31;
                                ptr = (&iter)[1 + -1 * idx];
                                v14 = ptr->field_8;
                                v15 = &ptr->field_8;
                                v16 = *((long long *)(&iter)[idx]);
                                node = v15;
                                if (ptr->field_8)
                                {
                                    do
                                    {
                                        if (sub_409f10(v14, v16))
                                            break;
                                        v18 = strcmp(v14, v16);
                                        if (v18 >= 0 && !v18)
                                        {
                                            memmove(v15, ptr, node - (char *)ptr);
                                            *((unsigned long *)&ptr->padding_0[0]) = v16;
                                            break;
                                        }
                                    } while ((v14 = (unsigned long)*((long long *)(node + 8)), node = (unsigned long)(node + 8), v14));
                                }
                            }
                        }
                        iter1 += 8;
                        goto LABEL_40a454;
                    }
                    if (v10 < 0)
                        goto LABEL_40a4ab;
                    iter1 += 8;
LABEL_40a465:
                    v19 = a1(a0);
                    if (v0 >= v19)
                        continue;
                    v0 = v19;
                }
                else
                {
LABEL_40a4ab:
LABEL_40a454:
                    iter += 8;
                    goto LABEL_40a465;
                }
            }
        }
    }
    else if (!v0)
    {
        goto LABEL_40a36a;
    }
    free(v4);
    free(v5);
    free(v7);
    free(v8);
    return v0;
}



// Function: find_dir_file_pathname @ 0xa660
typedef struct struct_0 {
    char padding_0[8];
    char *field_8;
} struct_0;

extern char g_426551;
extern long long g_426620;
extern char g_4266e8;

unsigned long long find_dir_file_pathname(long long a0, char *a1)
{
    struct_0 *iter;  // rbp
    char *v8;  // rbx
    char *v9;  // r12
    unsigned long long v10;  // rax
    char *v0;  // [bp-0x190]
    char v1;  // [bp-0x178]
    struct_0 *v2;  // [bp-0x170]
    void* v3;  // [bp-0x168]
    struct_0 v4;  // [bp-0x158]
    long long v5;  // [bp-0x150]

    v0 = a1;
    v2 = NULL;
    v3 = NULL;
    if (g_426551)
    {
        v5 = a0;
        v4 = (struct_0)0;
        if ((char)sub_409f90(&v4, &v1))
        {
            g_4266e8 = 1;
            if (!_setjmp(&g_426620))
            {
                iter = v2;
                v8 = iter->padding_0;
                if (iter->padding_0)
                {
                    do
                    {
                        v9 = a1;
                        if (sub_409f10(v8, v9))
                            continue;
                        if (strcmp(v8, v9))
                        {
                            if (v9 != v0)
                                v8 = v0;
                            v0 = v8;
                        }
                        else
                        {
                            v0 = v8;
                            break;
                        }
                    } while ((v8 = iter->field_8, iter += 8, v8));
                }
            }
            else
            {
                v0 = a1;
            }
        }
    }
    v10 = sub_410d80(a0, v0, 0);
    free(v2);
    free(v3);
    return v10;
}



// Function: print_ed_hunk @ 0xa7c0
typedef struct struct_2 {
    char field_0;
    char field_1;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern char g_41e550;
extern long long g_41e5b0;
extern struct_0 *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426388;

void print_ed_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // ebx
    unsigned long v7;  // rsi
    char *v8;  // rax
    char *v9;  // rax
    long long v10;  // rbx
    struct_2 **v11;  // rsi
    long long v0;  // [bp-0x50]
    long long v1;  // [bp-0x48]
    long long v2;  // [bp-0x40]
    long long v3;  // [bp-0x38]

    v5 = sub_40f670();
    if (!v5)
        return;
    v6 = v5;
    sub_40e830();
    sub_40f610(44, &g_4261a0, v0, v1);
    v7 = (&g_41e550)[v6];
    v8 = g_426188[1].field_0;
    if (v8 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v8 + 1;
        *(v8) = v7;
    }
    else
    {
        __overflow(g_426188, v7, &g_41e550);
    }
    v9 = g_426188[1].field_0;
    if (v9 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v9 + 1;
        *(v9) = 10;
    }
    else
    {
        __overflow(g_426188, 10);
    }
    if (v6 == 1)
        return;
    v10 = v2;
    if (v3 >= v10)
    {
        while (1)
        {
            v11 = g_426388 + v10 * 8;
            v10 += 1;
            if (*(v11)->field_0 != 46 || *(v11)->field_1 != 10)
            {
                sub_40f500(&g_41e5b0);
                if (v10 > v3)
                    break;
            }
            else
            {
                fputs_unlocked("..\n.\ns/.//\n", g_426188);
                if (v3 < v10)
                    return;
                fputs_unlocked("a\n", g_426188);
            }
        }
    }
    fputs_unlocked(".\n", g_426188);
    return;
}



// Function: pr_forward_ed_hunk @ 0xa960
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern char g_41e550;
extern long long g_41e5b0;
extern struct_0 *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426388;

void pr_forward_ed_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // ebx
    unsigned long v7;  // rdx
    char *v8;  // rax
    char *v9;  // rax
    long long v10;  // rbx
    long long v11;  // rbx
    long long v0;  // [bp-0x40]
    long long v1;  // [bp-0x38]
    long long v2;  // [bp-0x30]
    long long i;  // [bp-0x28]

    v5 = sub_40f670();
    if (!v5)
        return;
    v6 = v5;
    sub_40e830();
    v7 = (&g_41e550)[v6];
    v8 = g_426188[1].field_0;
    if (v8 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v8 + 1;
        *(v8) = v7;
    }
    else
    {
        __overflow(g_426188, (char)v7, v7);
    }
    sub_40f610(32, &g_4261a0, v0, v1);
    v9 = g_426188[1].field_0;
    if (v9 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v9 + 1;
        *(v9) = 10;
    }
    else
    {
        __overflow(g_426188, 10);
    }
    if (v6 == 1)
        return;
    v10 = v2;
    if (v10 <= i)
    {
        do
        {
            v11 = v10 + 1;
            sub_40f500(&g_41e5b0, g_426388 + v10 * 8);
            v10 = v11;
        } while (i >= v10);
    }
    fputs_unlocked(".\n", g_426188);
    return;
}



// Function: print_rcs_hunk @ 0xaab0
extern long long g_41e5b0;
extern unsigned long long g_426188;
extern long long g_4261a0;
extern long long g_4262d0;
extern unsigned long long g_426388;

void print_rcs_hunk(void)
{
    unsigned int v9;  // eax
    unsigned int v10;  // ebx
    unsigned long long v11;  // r8
    unsigned long long v12;  // r8
    long long v13;  // rbx
    long long v14;  // rbx
    long long v0;  // [bp-0x60]
    long long v1;  // [bp-0x58]
    long long v2;  // [bp-0x50]
    long long i;  // [bp-0x48]
    long long v4;  // [bp-0x40]
    long long v5;  // [bp-0x38]
    long long v6;  // [bp-0x30]
    long long v7;  // [bp-0x28]

    v9 = sub_40f670();
    if (!v9)
        return;
    v10 = v9;
    sub_40e830();
    sub_40f5e0(&g_4261a0, v0, v1, &v4, &v5);
    if ((char)v10 & 1)
    {
        v11 = 1;
        if (v4 <= v5)
            v11 = v5 - v4 + 1;
        __fprintf_chk(g_426188, 0x1, "d%ld %ld\n", v4, v11);
    }
    if (!((char)v10 & 2))
        return;
    sub_40f5e0(&g_4262d0, v2, i, &v6, &v7);
    v12 = 1;
    if (v6 <= v7)
        v12 = v7 - v6 + 1;
    __fprintf_chk(g_426188, 0x1, "a%ld %ld\n", v5, v12);
    v13 = v2;
    if (v13 > i)
        return;
    do
    {
        v14 = v13 + 1;
        sub_40f500(&g_41e5b0, g_426388 + v13 * 8);
        v13 = v14;
    } while (i >= v13);
    return;
}



// Function: print_ed_script @ 0xac20
void print_ed_script(unsigned long a0)
{
    sub_40f080();
    return;
}



// Function: pr_forward_ed_script @ 0xac40
void pr_forward_ed_script(unsigned long a0)
{
    sub_40f080();
    return;
}



// Function: print_rcs_script @ 0xac60
void print_rcs_script(unsigned long a0)
{
    sub_40f080();
    return;
}



// Function: groups_letter_value @ 0xac80

unsigned long long groups_letter_value(long long *a0, char a1)
{
    long long *iter;  // rdx

    iter = a0;
    switch (a1)
    {
    case 69:
        iter += 3;
        break;
    case 70:
        goto LABEL_40acf4;
    case 76:
        iter += 3;
        goto LABEL_40ad0c;
    case 77:
        goto LABEL_40ad2c;
    case 78:
        iter += 3;
        goto LABEL_40ad44;
    default:
        switch (a1)
        {
        case 101:
            return sub_40f5d0(*(iter), iter[1]) - 1;
        case 102:
LABEL_40acf4:
            return sub_40f5d0();
        case 108:
LABEL_40ad0c:
            return sub_40f5d0(*(iter), iter[2]) - 1;
        case 109:
LABEL_40ad2c:
            return sub_40f5d0();
        case 110:
LABEL_40ad44:
            return iter[2] - iter[1];
        default:
            return 0xffffffffffffffff;
        }
    }
}



// Function: do_printf_spec @ 0xad70
typedef struct struct_0 {
    char padding_0[1];
    char field_1;
    char field_2;
    char field_3;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_2 *field_28;
} struct_1;

typedef struct struct_2 {
    char field_0;
} struct_2;

struct_0 * do_printf_spec(struct_1 *idx, struct_0 *a1, unsigned long long a2, unsigned long a3, long long *a4)
{
    struct_0 *iter;  // rbx
    unsigned long v9;  // r12
    struct_0 *v18;  // rcx
    unsigned int v19;  // eax
    struct_0 *v20;  // rcx
    char *v21;  // rax
    unsigned long v22;  // rax
    unsigned long v23;  // rax
    unsigned long v24;  // rdi
    unsigned long n;  // rdx
    unsigned long v26;  // rsi
    unsigned long v27;  // rax
    unsigned int v10;  // eax
    char *i;  // r8
    unsigned long long v29;  // rdi
    unsigned long long v30;  // rbx
    unsigned long long v11;  // rax
    char v12;  // dl
    unsigned long v13;  // cc_dep1
    unsigned int v14;  // eax
    unsigned int v15;  // edx
    struct_0 *v16;  // r15
    unsigned int v17;  // eax
    char v0;  // [bp-0x2068]
    char v1;  // [bp-0x1068]
    unsigned long v2;  // [bp-0x188]
    char v3;  // [bp-0x68]
    unsigned long v4;  // [bp-0x60]
    char *v5;  // [bp-0x58], Other Possible Types: unsigned long
    unsigned long v6;  // [bp-0x50]

    iter = &a1->field_1;
    do
    {
        v9 = (unsigned long long)iter->padding_0;
        iter = &iter->field_1;
        if ((char)v9 > 48)
        {
            if ((char)v9 - 48 > 9)
                goto LABEL_40add0;
            goto LABEL_40ae30;
        }
    } while ((v2 = 0xfffedf7fffffffff, !(((char)v9 & 254 | *((char *)((char *)&v2 + ((long long)(v9 & 63) >> 3))) >> (unsigned long long)((char)v9 & 63 & 7) & 1) & 1)));
    if ((char)v9 - 48 > 9)
    {
        if ((char)v9 == 46)
            goto LABEL_40ae48;
        goto LABEL_40add0;
    }
    do
    {
LABEL_40ae30:
        iter = &iter->field_1;
        v9 = (char)iter->padding_0;
    } while (v10 - 48 <= 9);
    if ((char)v9 == 46)
    {
        do
        {
LABEL_40ae48:
            iter = &iter->field_1;
            v9 = (char)iter->padding_0;
        } while (v14 - 48 <= 9);
        v11 = v9 - 88 & 0xffffffff;
        v12 = (char)iter->padding_0;
        v13 = (char)v11;
        if ((char)v11 > 32)
            return NULL;
LABEL_40ade0:
        v2 = 4303360001;
        if (!(((char)v13 & 254 | *((char *)&v2 + ((long long)(v11 & 63) >> 3)) >> ((char)v11 & 63 & 7) & 1) & 1))
        {
            if ((char)v11 != 11)
            {
                return NULL;
            }
            else if (v12 == 39)
            {
                v15 = iter->field_1;
                if ((char)v15 == 39)
                    return NULL;
                if ((char)v15 == 92)
                {
                    v17 = iter->field_2;
                    v18 = &iter->field_3;
                    if ((char)v17 != 39)
                    {
                        v15 = 0;
                        while (1)
                        {
                            v19 = v17 - 48;
                            if (v19 > 7)
                                return NULL;
                            v16 = &v18->field_1;
                            v15 = v19 + v15 * 8;
                            v17 = *(&v16->padding_0[0] - 1);
                            v20 = v18;
                            if ((char)v17 == 39)
                                break;
                            v18 = v16;
                        }
                    }
                    else
                    {
                        v15 = 0;
                        v20 = &iter->field_2;
                        v16 = v18;
                    }
                    if (v20 - iter - 3 > 2)
                        return NULL;
                }
                else if (!(char)v15)
                {
                    return NULL;
                }
                else
                {
                    v16 = &iter->field_3;
                    if (iter->field_2 != 39)
                        return NULL;
                }
                if (!idx)
                    return v16;
                v21 = &idx->field_28->field_0;
                if (v21 >= *((long long *)&idx[1].padding_0[0]))
                {
                    __overflow(idx, (char)v15);
                    return v16;
                }
                idx->field_28 = v21 + 1;
                *(v21) = v15;
                return v16;
            }
        }
        else
        {
            if (!a2)
            {
                v22 = sub_40ac80(a4, v12);
                if (v22 >= 0)
                    goto LABEL_40aee9;
            }
            else if (v12 == 110)
            {
                v22 = sub_40f5d0();
LABEL_40aee9:
                if (!idx)
                    return &iter->field_1;
                v23 = &iter->field_1 - a1;
                v24 = v23 + 1;
                n = v23 - 2;
                v26 = v23 - 1;
                if (v24 <= 4000)
                {
                    v27 = v23 + 55;
                    i = &(&v3)[-1 * (v27 & 0xfffffffffffff000)];
                    if (&v3 != i)
                    {
                        do
                        { } while (&v0 != i);
                    }
                    v29 = (unsigned int)v27 & 0xfffffff0 & 0xfff;
                    /* unsupported instruction */ = (int)&(&v1)[-1 * v29];
                    if (v29)
                        *((long long *)(/* unsupported instruction */ + v29 - 8)) = *((long long *)(/* unsupported instruction */ + v29 - 8));
                    v30 = (/* unsupported instruction */ + 15 & 0xfffffffffffffff0) + 31 & 0xffffffffffffffe0;
                }
                else
                {
                    v4 = n;
                    v5 = v22;
                    v6 = v26;
                    v22 = v5;
                    v30 = sub_416f00(v24, v26, n);
                    v26 = v6;
                    n = v4;
                }
                v4 = v22;
                v5 = v30 + v26;
                v6 = n;
                memcpy(v30, a1, n);
                *((char *)(v30 + v6)) = 108;
                *(v5) = v9;
                v5[1] = 0;
                __fprintf_chk(*((unsigned int *)&idx), 0x1, v30);
                sub_4121c0(v30);
                return &iter->field_1;
            }
        }
        return NULL;
    }
LABEL_40add0:
    v11 = v9 - 88 & 0xffffffff;
    v12 = (char)iter->padding_0;
    v13 = (char)v11;
    if ((char)v11 > 32)
        return NULL;
    goto LABEL_40ade0;
}



// Function: print_ifdef_lines @ 0xb070
typedef struct struct_0 {
    char padding_0[184];
    unsigned long long field_b8;
} struct_0;

typedef struct struct_2 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    struct struct_1 *field_10;
} struct_2;

typedef struct struct_1 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
} struct_1;

typedef struct struct_3 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_3;

extern char g_4264a0;

char * print_ifdef_lines(struct_3 *index, char *a1, struct_2 *idx)
{
    unsigned long idx1;  // r15
    unsigned long long v6;  // r14
    char v15;  // al
    char *v16;  // r13
    struct_1 *v7;  // rax
    unsigned long v8;  // rdi
    unsigned long v9;  // rdx
    char *v10;  // rax
    char *v11;  // rbp
    char *v12;  // r9
    char v13;  // bl
    char v14;  // sil
    struct_1 *idx2;  // [bp-0x58]
    char *v1;  // [bp-0x50]
    char *v2;  // [bp-0x48]
    struct_0 *v3;  // [bp-0x40]

    idx1 = idx->field_8;
    v3 = idx->field_0;
    v6 = idx->field_0->field_b8;
    v7 = idx->field_10;
    idx2 = v7;
    if (!index)
        return v7;
    if (!g_4264a0)
    {
        v7 = a1;
        if (v7->field_0 == 37)
        {
            v7 = v7->field_1;
            if (*((char *)&v7) == 108)
            {
                v7 = a1;
                if (v7->field_2 == 10 && !v7->field_3)
                {
                    if (idx1 >= idx2)
                        return v7;
                    v8 = *((long long *)(v6 + idx1 * 8));
                    v9 = *((long long *)(v6 + idx2 * 8)) + (*((char *)(*((long long *)(v6 + idx2 * 8)) - 1)) != 10) - *((long long *)(v6 + idx1 * 8));
                    return fwrite_unlocked(v8, 1, v9, index);
                }
            }
            else if (*((char *)&v7) == 76)
            {
                v7 = a1;
                if (!v7->field_2)
                {
                    v9 = *((long long *)(v6 + idx2 * 8)) - *((long long *)(v6 + idx1 * 8));
                    v8 = *((long long *)(v6 + idx1 * 8));
                    return fwrite_unlocked(*((long long *)(v6 + idx1 * 8)), 1, *((long long *)(v6 + idx2 * 8)) - *((long long *)(v6 + idx1 * 8)), index);
                }
            }
        }
    }
    if (idx1 >= idx2)
        return v7;
    v10 = a1 + 1;
    v1 = v10;
    do
    {
        v11 = a1;
        v12 = v1;
        v13 = *(v11);
        if (v13)
        {
            while (1)
            {
                if (v13 != 37)
                {
                    v14 = v13;
                    v11 = v12;
                    goto LABEL_40b0f6;
                }
                else
                {
                    v15 = v11[1];
                    v16 = v11 + 2;
                    if (v15 != 76)
                    {
                        if (v15 == 108)
                            goto LABEL_40b1d1;
                        if (v15 != 37)
                        {
                            v2 = v12;
                            v10 = &sub_40ad70(index, v11, v3, idx1, NULL)->padding_0[0];
                            v11 = v10;
                            if (!v10)
                            {
                                v11 = v2;
                                v10 = &index->field_28->field_0;
                                v14 = 37;
                                if (index->field_28 >= *((long long *)&index[1].padding_0[0]))
                                {
LABEL_40b178:
                                    v10 = __overflow(index, v14);
                                    v13 = *(v11);
                                    v12 = v11 + 1;
                                    if (!v13)
                                        break;
                                    else
                                        continue;
                                }
LABEL_40b102:
                                index->field_28 = v10 + 1;
                                *(v10) = v13;
                                goto LABEL_40b10d;
                            }
                        }
                        else
                        {
                            v11 = v16;
                            v14 = 37;
                        }
LABEL_40b0f6:
                        v10 = &index->field_28->field_0;
                        if (index->field_28 >= *((long long *)&index[1].padding_0[0]))
                            goto LABEL_40b178;
                        goto LABEL_40b102;
                    }
                    else
                    {
LABEL_40b1d1:
                        v11 = v16;
                        v10 = sub_40f0e0(*((long long *)(v6 + idx1 * 8)));
LABEL_40b10d:
                        v13 = *(v11);
                        v12 = v11 + 1;
                        if (!*(v11))
                            break;
                    }
                }
            }
        }
    } while ((idx1 = (unsigned long)(idx1 + 1), idx2 != idx1));
    return v10;
}



// Function: format_group @ 0xb2a0
typedef struct struct_2 {
    char field_0;
    char field_1;
    char field_2;
} struct_2;

typedef struct struct_0 {
    char padding_0[40];
    struct struct_1 *field_28;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned long long g_426440;
extern unsigned long long g_426448;
extern unsigned long long g_426450;

struct_2 * format_group(struct_0 *index, struct_2 *a1, char a2, unsigned long a3)
{
    char v9;  // bl
    struct_2 *v10;  // r12
    struct_2 *v19;  // rax
    unsigned long v20;  // rax
    struct_0 *v21;  // rdi
    struct_0 *v22;  // r12
    struct_2 *v23;  // rax
    struct_2 *v24;  // rax
    unsigned long v25;  // rax
    char v11;  // cl
    struct_2 *v12;  // r10
    unsigned long v13;  // rdx
    unsigned long v14;  // rsi
    unsigned long long v15;  // rcx
    unsigned int v16;  // esi
    unsigned int *err;  // rax
    unsigned long long idx;  // rcx
    unsigned long v0;  // [bp-0x90]
    char *v1;  // [bp-0x88], Other Possible Types: struct_2 *
    struct_2 *v2;  // [bp-0x80], Other Possible Types: unsigned int *, unsigned long long
    struct_2 *v3;  // [bp-0x78]
    unsigned long long v4;  // [bp-0x70]
    char v5;  // [bp-0x60]
    unsigned long v6;  // [bp-0x58]
    unsigned long v7;  // [bp-0x50]

    v9 = a1->field_0;
    if (v9 == a2)
    {
        return a1;
    }
    else if (v9)
    {
        v0 = a3 + 24;
        while (1)
        {
            v10 = &a1->field_1;
            if (v9 != 37)
            {
LABEL_40b2ee:
                if (index)
                {
                    v25 = index->field_28;
                    if (v25 < *((long long *)&index[1].padding_0[0]))
                    {
                        index->field_28 = v25 + 1;
                        *((char *)v25) = v9;
                        goto LABEL_40b30b;
                    }
                    else
                    {
                        __overflow(index, v9);
                        v9 = v10->field_0;
                        a1 = v10;
                        goto LABEL_40b313;
                    }
                }
                else
                {
LABEL_40b30b:
                    v9 = v10->field_0;
                    a1 = v10;
                    goto LABEL_40b313;
                }
            }
            else
            {
                v11 = a1->field_1;
                v12 = &a1->field_2;
                switch (v11)
                {
                case 37:
                    v10 = v12;
                    v9 = v11;
                    goto LABEL_40b2ee;
                case 40:
                    v15 = 0;
                    v1 = &v6;
                    while (1)
                    {
                        v16 = v12->field_0;
                        if (v16 - 48 <= 9)
                        {
                            v4 = v15;
                            v3 = v12;
                            err = __errno_location();
                            *(err) = 0;
                            v2 = err;
                            idx = v4;
                            *((unsigned long long *)&v1[8 * idx]) = strtoimax(v3, &v5, 10);
                            if (*(v2))
                                break;
                            v19 = (struct_2 *)v5;
                        }
                        else
                        {
                            v2 = v15;
                            v3 = v12;
                            v20 = sub_40ac80(a3, v16);
                            idx = v2;
                            *((unsigned long *)&v1[8 * idx]) = v20;
                            if (v20 < NULL)
                                break;
                            v19 = &v3->field_1;
                        }
                        v12 = &v19->field_1;
                        if (v19->field_0 != *((char *)(idx + 4317588)))
                            break;
                        if (idx == 1)
                        {
                            v21 = NULL;
                            v22 = NULL;
                            if (v6 == v7)
                                v21 = index;
                            if (v6 != v7)
                                v22 = index;
                            v23 = format_group(v21, v12, 58, a3);
                            if (!v23->field_0)
                                return v23;
                            v24 = format_group(v22, &v23->field_1, 41, a3);
                            if (!v24->field_0)
                                return v24;
                            v9 = v24->field_1;
                            a1 = &v24->field_1;
                            goto LABEL_40b313;
                            goto LABEL_40b313;
                        }
                        else
                        {
                            v15 = 1;
                        }
                    }
                case 60:
                    v2 = a1;
                    v14 = g_426448;
                    v13 = a3;
                    v1 = v12;
                    break;
                case 61:
                    v2 = a1;
                    v14 = g_426440;
                    v13 = a3;
                    v1 = v12;
                    break;
                case 62:
                    v2 = a1;
                    v13 = v0;
                    v1 = v12;
                    v14 = g_426450;
                    break;
                default:
                    a1 = sub_40ad70(index, a1, 0, 0, a3);
                    if (a1)
                    {
                        v9 = a1->field_0;
                        goto LABEL_40b313;
                    }
                }
                sub_40b070(index, v14, v13);
                v9 = v2->field_2;
                a1 = v1;
LABEL_40b313:
                if (!v9)
                {
                    return a1;
                }
                else if (a2 == v9)
                {
                    return a1;
                }
            }
        }
    }
    else
    {
        return a1;
    }
}



// Function: print_ifdef_hunk @ 0xb560
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_426188;
extern char g_4261a0;
extern unsigned long long g_4262d0;
extern struct_0 g_426460;
extern unsigned long long g_4266f0;
extern unsigned long long g_4266f8;

void print_ifdef_hunk(void)
{
    unsigned long long idx;  // rbx
    long long v12;  // rcx
    long long v13;  // rsi
    unsigned long v14;  // r8
    long long v0;  // [bp-0x88]
    unsigned long v1;  // [bp-0x80]
    long long v2;  // [bp-0x78]
    unsigned long v3;  // [bp-0x70]
    unsigned long v4;  // [bp-0x68]
    long long v5;  // [bp-0x60]
    long long v6;  // [bp-0x58], Other Possible Types: unsigned long
    unsigned long long v7;  // [bp-0x50]
    long long v8;  // [bp-0x48]
    long long v9;  // [bp-0x40], Other Possible Types: unsigned long

    idx = sub_40f670() & 0xffffffff;
    if (!(unsigned int)idx)
        return;
    sub_40e830();
    v12 = v0;
    v13 = v2;
    if (g_4266f8 < v12 || g_4266f0 < v13)
    {
        v9 = v13;
        v6 = v12;
        v8 = g_4266f0;
        v4 = &g_4261a0;
        v5 = g_4266f8;
        v7 = &g_4262d0;
        sub_40b2a0(g_426188, g_426460.field_0, 0, &v4);
        v12 = v0;
        v13 = v2;
    }
    v5 = v12;
    v14 = (&g_426460.field_0)[idx];
    v8 = v13;
    v4 = &g_4261a0;
    v6 = v1 + 1;
    g_4266f8 = v6;
    v7 = &g_4262d0;
    v9 = v3 + 1;
    g_4266f0 = v9;
    sub_40b2a0(g_426188, v14, 0, &v4);
    return;
}



// Function: print_ifdef_script @ 0xb6c0
typedef struct struct_2 {
    char field_0;
    char field_1;
    char field_2;
} struct_2;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_426188;
extern char g_4261a0;
extern char g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_4262d0;
extern char g_4263a0;
extern struct_2 *g_426460;
extern unsigned long long g_4266f0;
extern unsigned long long g_4266f8;

void print_ifdef_script(void)
{
    unsigned long v0;  // [bp-0x48]
    unsigned long v1;  // [bp-0x40]
    unsigned long v2;  // [bp-0x38]
    unsigned long long v3;  // [bp-0x30]
    unsigned long v4;  // [bp-0x28]
    unsigned long v5;  // [bp-0x20]

    g_4266f0 = -(g_426288);
    g_4266f8 = g_4266f0;
    sub_40f080();
    if (*((long long *)&g_426270) <= g_4266f8 && *((long long *)&g_4263a0) <= g_4266f0)
        return;
    sub_40e830();
    v0 = &g_4261a0;
    v1 = g_4266f8;
    v2 = *((long long *)&g_426270);
    v3 = &g_4262d0;
    v4 = g_4266f0;
    v5 = *((long long *)&g_4263a0);
    sub_40b2a0(g_426188, g_426460, 0, &v0);
    return;
}



// Function: prepare_text @ 0xb7b0
typedef struct struct_0 {
    char padding_0[160];
    void* field_a0;
    char padding_a8[8];
    unsigned long long field_b0;
    char padding_b8[104];
    char field_120;
} struct_0;

extern char g_426490;

void* prepare_text(struct_0 *idx)
{
    void* node;  // r13
    unsigned long long iter;  // r12
    void* v3;  // rbp
    void* ptr;  // rax
    void* iter1;  // rdx
    void* v6;  // rax

    node = idx->field_a0;
    if (!node)
        return v6;
    iter = idx->field_b0;
    if (g_426490)
    {
        v3 = node + iter;
        *((char *)v3) = 13;
        ptr = rawmemchr(node, 13);
        if (v3 != ptr)
        {
            iter1 = ptr;
            while (1)
            {
                if (*((char *)ptr) != 13 || (char)ptr[1] != 10)
                {
                    v6 = ptr + 1;
                    iter1 += 1;
                    *((char *)iter1 - 1) = *((char *)ptr);
                    ptr = v6;
                    if (v3 == ptr)
                        break;
                }
                else
                {
                    iter1 += 1;
                    ptr += 2;
                    *((char *)iter1 - 1) = 10;
                    if (v3 == ptr)
                        break;
                }
            }
        }
        else
        {
            iter1 = v3;
            v6 = ptr;
        }
        iter -= v3 - iter1;
    }
    if (iter)
    {
        v6 = node + iter;
        if (*((char *)node + iter - 1) != 10)
        {
            *((char *)v6) = 10;
            iter += 1;
            idx->field_120 = 1;
            node += iter;
        }
        else
        {
            node = v6;
        }
    }
    *((unsigned long *)node) = 0;
    idx->field_b0 = iter;
    return v6;
}



// Function: sip @ 0xb920
typedef struct struct_0 {
    int field_0;
    char padding_4[68];
    long long field_48;
    char padding_50[80];
    void* field_a0;
    long long field_a8;
    char field_b0;
    char padding_b1[112];
    char field_121;
} struct_0;

unsigned int sip(struct_0 *idx, char a1)
{
    long long v1;  // rax
    void* ptr;  // rax

    if (idx->field_0 >= 0)
    {
        v1 = sub_417190(8, idx->field_48, 0x7fffffffffffffef);
        idx->field_a8 = v1;
        idx->field_a0 = sub_416a40(v1);
        if (!a1)
        {
            sub_40b8a0(idx, idx->field_a8);
            ptr = memchr(idx->field_a0, 0, *((long long *)&idx->field_b0));
            return _INSERT(ptr, 0, ptr);
        }
    }
    else
    {
        idx->field_a8 = 8;
        idx->field_a0 = sub_416a40(8);
    }
    *((unsigned long *)&idx->field_b0) = 0;
    idx->field_121 = 0;
    return 0;
}



// Function: slurp @ 0xb9e0
typedef struct struct_0 {
    unsigned int field_0;
    char padding_4[4];
    long long field_8;
    char padding_10[24];
    unsigned int field_28;
    char padding_2c[116];
    long long field_a0;
    long long field_a8;
    long long field_b0;
    char padding_b8[105];
    char field_121;
} struct_0;

unsigned long long slurp(struct_0 *idx)
{
    unsigned long long v3;  // rax
    unsigned long long v4;  // r12
    long long v13;  // rdi
    unsigned long long v14;  // rax
    unsigned long long i;  // r12
    long long v16;  // r12
    long long v17;  // rax
    long long v18;  // rdi
    unsigned long long v19;  // rsi
    unsigned long long v20;  // rax
    unsigned long long v5;  // rbx
    unsigned long long v6;  // rbp
    unsigned long long v7;  // rsi
    unsigned long long v8;  // rax
    unsigned long long v9;  // rax
    unsigned long long v10;  // rdx
    unsigned long long v11;  // rsi
    unsigned long long v12;  // rsi
    unsigned long long v0;  // [bp-0x18]
    unsigned long long v1;  // [bp-0x8]

    v3 = idx->field_0;
    if ((unsigned int)v3 < 0)
        return v3;
    v1 = v4;
    v0 = v5;
    if (((unsigned short)idx->field_28 & 0xf000) == 0x8000)
    {
        v6 = *((long long *)&idx->padding_2c[20]);
        v7 = (v6 & 0xfffffffffffffff8) + 16;
        if (v6 > v7)
            sub_416ec0(); /* do not return */
        if (v7 > 0x7ffffffffffffffe)
            sub_416ec0(); /* do not return */
        if (idx->field_a8 >= v7)
        {
            v8 = idx->field_b0;
            if (v8 <= v6)
                goto LABEL_40bb82;
        }
        else
        {
            idx->field_a8 = v7;
            idx->field_a0 = sub_416a90(idx->field_a0);
            v8 = idx->field_b0;
            if (v8 <= v6)
            {
LABEL_40bb82:
                sub_40b8a0(idx, v6 - v8 + 1);
                v8 = idx->field_b0;
                if (v8 <= v6)
                    return v8;
            }
        }
    }
    else
    {
        v8 = idx->field_b0;
    }
    v9 = sub_40b8a0(idx, idx->field_a8 - v8);
    v10 = idx->field_b0;
    if (!v10)
        return v9;
    v11 = idx->field_a8;
    if (v10 != v11)
    {
        v18 = idx->field_a0;
        v19 = v10 + 16 & 0xfffffffffffffff8;
        idx->field_a8 = v19;
        v20 = sub_416a90(v18, v19);
        idx->field_a0 = v20;
        return v20;
    }
    while (1)
    {
        do
        {
            if (v11 > 0x3ffffffffffffff7)
                sub_416ec0(); /* do not return */
            v12 = v11 * 2;
            v13 = idx->field_a0;
            idx->field_a8 = v12;
            v14 = sub_416a90(v13, v12);
            v11 = idx->field_a8;
            v10 = idx->field_b0;
            idx->field_a0 = v14;
            v16 = v11 - v10;
        } while (i == v10);
        if (!idx->field_121)
        {
            v17 = sub_4170e0(idx->field_0, v14 + v10, v16);
            if (v17 == -0x1)
                sub_40e770(idx->field_8); /* do not return */
            v11 = idx->field_a8;
            v10 = idx->field_b0 + v17;
            idx->field_b0 = v10;
            idx->field_121 = v16 > v17;
        }
        if (v10 != v11)
        {
            v18 = idx->field_a0;
            v19 = v10 + 16 & 0xfffffffffffffff8;
            idx->field_a8 = v19;
            v20 = sub_416a90(v18, v19);
            idx->field_a0 = v20;
            return v20;
        }
    }
}



// Function: read_files @ 0xbbc0
typedef struct struct_0 {
    int field_0;
    char padding_4[68];
    long long field_48;
    char padding_50[80];
    void* field_a0;
    long long field_a8;
    char field_b0;
    char padding_b1[112];
    char field_121;
} struct_0;

extern char g_41e1a0;
extern unsigned long long g_426498;
extern unsigned long long g_426520;
extern char g_426552;
extern char g_426558;
extern unsigned long long g_426560;
extern char g_426568;
extern char g_426570;
extern char g_426578;
extern unsigned int g_426580;
extern unsigned long long g_426700;
extern unsigned long long g_426708;
extern unsigned long long g_426710;
extern unsigned long long g_426718;
extern unsigned long long g_426720;

long long read_files(struct_0 *idx, char a1)
{
    char v23;  // bl
    void* v33;  // r14
    void* iter;  // rdi
    unsigned long v124;  // rax
    unsigned long long v125;  // r13
    unsigned long long v126;  // rcx
    unsigned long long *v128;  // rax
    unsigned long long i0;  // r13
    unsigned long long *v131;  // rax
    void* iter1;  // rax
    unsigned long v133;  // r14
    unsigned long v134;  // rax
    unsigned long long *v136;  // rax
    long long v137;  // rbp
    unsigned long long v138;  // rbx
    unsigned long long v139;  // r12
    long long v140;  // rcx
    unsigned long v141;  // rax
    void* *v142;  // rdx
    void* v35;  // rdi
    long long v143;  // r15
    struct_0 *v144;  // rax
    struct_0 *v145;  // rax
    struct_0 *idx1;  // rax
    void* v36;  // r8
    void* node;  // rdx
    unsigned int v38;  // r9d
    unsigned long long v39;  // r10
    unsigned long long v40;  // rcx
    struct_0 *idx2;  // r15
    void* iter2;  // rcx
    struct_0 *v25;  // rdx
    void* v43;  // rbx
    void* v44;  // r9
    void* j;  // rdx
    unsigned long v46;  // r9
    struct_0 *v47;  // rdx
    unsigned long v48;  // 4121
    unsigned long long v49;  // rdx
    unsigned long long n;  // r9
    void* v51;  // rax
    unsigned long long v52;  // rax
    struct_0 *v26;  // rbx
    void* v53;  // rax
    unsigned long long v54;  // rax
    unsigned long long v55;  // r10
    unsigned long long v56;  // rax
    unsigned long long v57;  // r12
    unsigned long long v58;  // rsi
    unsigned long long v59;  // rsi
    void* v60;  // rcx
    unsigned long long v61;  // r9
    unsigned long long v62;  // r15
    struct_0 *v27;  // rcx
    void* v63;  // r8
    unsigned long long v64;  // r10
    void* v65;  // r11
    void* v66;  // rax
    void* i;  // rbx
    unsigned long long v68;  // rdi
    unsigned long long v69;  // r13
    unsigned long long v70;  // r15
    unsigned long long v71;  // r12
    void* v72;  // r14
    unsigned long long v28;  // rsi
    unsigned long long v73;  // r12
    unsigned long long v74;  // rbp
    unsigned long long v75;  // rbp
    unsigned long long v76;  // rbx
    unsigned long long v77;  // rbp
    unsigned long long l;  // rbp
    unsigned long long v79;  // rdi
    unsigned long long v80;  // rsi
    unsigned long long v81;  // rax
    unsigned long long v82;  // rax
    void* v29;  // r13
    unsigned long long v83;  // rax
    unsigned long long v84;  // rbx
    unsigned long long v85;  // rcx
    unsigned long long v86;  // rax
    unsigned long long k;  // rax
    unsigned long long v88;  // rax
    unsigned long long v89;  // rax
    struct_0 *index;  // rsi
    unsigned long long v91;  // rax
    long long v92;  // r12
    char v30;  // al
    unsigned long long v93;  // rbx
    unsigned long long v94;  // rax
    unsigned long v95;  // rax
    unsigned long long v96;  // rdx
    long long v97;  // rdi
    unsigned int v98;  // ecx
    unsigned long long m;  // rax
    struct_0 *v100;  // rax
    struct_0 *v101;  // rbx
    void* v102;  // r14
    unsigned long long v31;  // r11
    unsigned int v103;  // r15d
    unsigned long v104;  // rbp
    void* v105;  // r13
    unsigned long long *v106;  // rax
    unsigned long long v107;  // rbx
    unsigned long long *v108;  // rax
    void* v109;  // r13
    unsigned long v110;  // r12
    unsigned long long *v111;  // rsi
    void* v112;  // rbp
    struct_0 *v32;  // rbp
    struct_0 *v113;  // rax
    long long v114;  // rax
    unsigned long long v115;  // rbp
    long long v116;  // rbx
    long long v117;  // rax
    unsigned long long v118;  // r14
    unsigned long long v119;  // rbx
    char v120;  // r13b
    unsigned long long v121;  // rbp
    unsigned long long *v122;  // rax
    void* v0;  // [bp-0xe0], Other Possible Types: unsigned long, unsigned long long
    void* v1;  // [bp-0xd8], Other Possible Types: long long, unsigned long long
    unsigned long long *v2;  // [bp-0xd0], Other Possible Types: void*, unsigned int
    char v3;  // [bp-0xc8], Other Possible Types: unsigned long long
    void* v4;  // [bp-0xc0], Other Possible Types: unsigned long
    unsigned long v5;  // [bp-0xb8]
    unsigned int v6;  // [bp-0xb0], Other Possible Types: unsigned long long
    void* v7;  // [bp-0xa8]
    unsigned long v8;  // [bp-0xa0]
    long long v9;  // [bp-0x98]
    long long v10;  // [bp-0x90]
    unsigned long long v11;  // [bp-0x88]
    void* v12;  // [bp-0x80]
    void* v13;  // [bp-0x78]
    unsigned long long v14;  // [bp-0x70]
    unsigned long long v15;  // [bp-0x68]
    struct_0 *v16;  // [bp-0x60]
    unsigned long long v17;  // [bp-0x58]
    unsigned int v18;  // [bp-0x48]
    char v19;  // [bp-0x42]
    char v20;  // [bp-0x41]
    struct_0 *v21;  // [bp-0x40]

    v23 = g_426568 | a1;
    v20 = (char)sub_40b920(idx, v23) | a1;
    if (idx->field_0 == *((int *)&idx[1].padding_4[10]))
    {
        v25 = idx;
        *((void* *)((char *)&v25[1].field_a8 + 6)) = v25->field_a0;
        *((long long *)&v25[1].padding_b1[5]) = v25->field_a8;
        *((long long *)&v25[1].padding_b1[13]) = *((long long *)&v25->field_b0);
        if (v20)
            return v20;
    }
    else
    {
        v20 |= (char)sub_40b920(&idx[1].padding_4[10], v23 | v20);
        if (v20)
            return v20;
    }
    v26 = idx;
    sub_40b9e0(v26);
    sub_40b7b0(v26);
    if (v26->field_0 == *((int *)&v26[1].padding_4[10]))
    {
        v27 = idx;
        v28 = *((long long *)&v27->field_b0);
        v29 = v27->field_a0;
        *((long long *)&v27[1].padding_b1[5]) = v27->field_a8;
        v30 = v27->padding_b1[111];
        v31 = v28;
        *((void* *)((char *)&v27[1].field_a8 + 6)) = v29;
        *((unsigned long long *)&v27[1].padding_b1[13]) = v28;
        v27[2].padding_4[8] = v30;
LABEL_40bcf5:
        iter1 = v29 + v31;
        v35 = v29 + v28;
        v33 = v29;
        v36 = iter1;
        node = iter1;
        v38 = g_426580 - 4;
    }
    else
    {
        v32 = &v26[1].padding_4[10];
        sub_40b9e0(v32);
        sub_40b7b0(v32);
        v33 = v26->field_a0;
        v29 = *((long long *)((char *)&v26[1].field_a8 + 6));
        v28 = *((long long *)&v26->field_b0);
        v31 = *((long long *)&v26[1].padding_b1[13]);
        if (v33 == v29)
            goto LABEL_40bcf5;
        v35 = v33 + v28;
        v36 = v29 + v31;
        if (v28 >= v31)
            *((char *)v36) = ~(*((char *)v33 + v31));
        else
            *((char *)v35) = ~(*((char *)v29 + v28));
        node = v29;
        iter1 = v33;
        if (*((long long *)v29) == *((long long *)v33))
        {
            do
            {
                iter1 += 8;
                node += 8;
            } while (*((long long *)iter1) == *((long long *)node));
        }
        if (*((char *)node) == *((char *)iter1))
        {
            do
            {
                iter1 += 1;
                node += 1;
            } while (*((char *)iter1) == *((char *)node));
        }
        v38 = g_426580 - 4;
        if (v38 > 1 && iter1 > v28 - idx->padding_b1[111] + v33 != node > v31 - idx[2].padding_4[8] + v29)
        {
            iter1 -= 1;
            node -= 1;
        }
    }
    v39 = g_426560;
    v40 = g_426560;
    if (iter1 != v33)
    {
        while (1)
        {
            if (*((char *)iter1 - 1) != 10)
            {
                iter1 -= 1;
                node -= 1;
                if (iter1 == v33)
                    break;
            }
            else if (!v40 || !(iter1 -= 1, v40 -= 1, node -= 1, iter1 != v33))
            {
                break;
            }
        }
    }
    idx2 = idx;
    *((void* *)&idx2->padding_b1[47]) = iter1;
    *((void* *)&idx2[1].padding_b1[61]) = node;
    if (v38 <= 1 || (iter2 = v36, v43 = v35, idx2->padding_b1[111] == idx2[2].padding_4[8]))
    {
        v44 = v28 - v31 + iter1;
        if (v31 > v28)
            v44 = iter1;
        j = v35;
        for (iter2 = v36; j != v44 && *((char *)j - 1) == *((char *)iter2 - 1); j -= 1)
        {
            iter2 -= 1;
        }
        if (j != v33 && *((char *)j - 1) != 10)
        {
            v39 = g_426560 + 1;
        }
        else if (iter2 != v29)
        {
            v39 = g_426560 + (*((char *)iter2 - 1) != 10);
        }
        v46 = v39 - 1;
        v43 = j;
        if (v39 && j != v35)
        {
            while (1)
            {
                v43 += 1;
                if (*((char *)v43 - 1) != 10)
                    continue;
                v46 -= 1;
                if (!(v46 & v43 != v35))
                    break;
            }
            iter2 += v43 - j;
        }
    }
    v47 = idx;
    v48 = g_426578;
    *((void* *)&v47->padding_b1[63]) = v43;
    *((void* *)&v47[1].padding_b1[77]) = iter2;
    if (!(char)v48 || g_426520 || (v49 = (unsigned long long)*((long long *)&g_426570), *((long long *)&g_426570) > 0x1ffffffffffffffe || *((long long *)&g_426570) >= v28))
    {
        v59 = v28 >> 5;
        v55 = 0xffffffffffffffff;
        if (!(v28 & 0xffffffffffffffe0))
            v59 = 1;
        if (v59 > 542551296285575042)
            v59 = 542551296285575042;
        n = 0;
        v58 = v59 + 5;
        v57 = v58;
    }
    else
    {
        n = 1;
        v51 = v43 - iter1;
        v52 = v51 >> 5;
        if (!(v51 & 0xffffffffffffffe0))
            v52 = 1;
        if (v52 > 542551296285575042)
            v52 = 542551296285575042;
        v53 = v35 - v43;
        v54 = v53 >> 5;
        if (!(v53 & 0xffffffffffffffe0))
            v54 = 1;
        if (v54 > 542551296285575042)
            v54 = 542551296285575042;
        v55 = 0;
        v56 = v54 + 5;
        if (*((long long *)&g_426570) > 0)
        {
            do
            {
                n *= 2;
            } while (*((long long *)&g_426570) >= n);
            v55 = n - 1;
        }
        if (v56 < *((long long *)&g_426570))
            v49 = v56;
        v57 = v52 + 5 + n + v49;
        v58 = v57;
    }
    v3 = v55;
    v2 = v36;
    v1 = n;
    v0 = iter2;
    v60 = v0;
    v61 = v1;
    v62 = sub_416a40(v58 * 8);
    v63 = v2;
    v64 = v3;
    v65 = *((long long *)&idx->padding_b1[47]);
    if (g_426578 && v43 == v65 && (v66 = (void*)*((long long *)&idx[1].padding_b1[61]), v60 == v66))
    {
        v65 = v33;
        goto LABEL_40cbe8;
    }
    i = v33;
    if (v33 != v65)
    {
        v0 = v29;
        v68 = v62;
        v69 = v57;
        v70 = v64;
        v1 = v33;
        v71 = 0;
        v72 = v65;
        do
        {
            v73 = v71 + 1;
            v74 = v71 & v70;
            if (v74 == v69)
            {
                if (v69 <= 0x7fffffffffffffe)
                {
                    v69 *= 2;
                    v4 = v63;
                    v3 = v61;
                    v2 = v60;
                    v63 = v4;
                    v61 = v3;
                    v60 = v2;
                    v68 = sub_416a90();
                }
                else
                {
                    sub_416ec0(); /* do not return */
                }
            }
            *((void* *)(v68 + v74 * 8)) = i;
            do
            {
                i += 1;
            } while (*((char *)i - 1) != 10);
            v71 = v73;
        } while (i != v72);
        v65 = v72;
        v75 = v73;
        v64 = v70;
        v57 = v69;
        v33 = v1;
        v62 = v68;
        v29 = v0;
        v76 = *((long long *)&g_426570);
        v66 = *((long long *)&idx[1].padding_b1[61]);
        if (v61)
            goto LABEL_40bfa9;
        v61 = v75;
        v77 = v75;
        goto LABEL_40bfb3;
    }
    else
    {
        v66 = *((long long *)&idx[1].padding_b1[61]);
LABEL_40cbe8:
        v76 = *((long long *)&g_426570);
        if (!v61)
        {
            l = 0;
            v79 = 32;
            v80 = v60 - v66;
        }
        else
        {
            v75 = 0;
LABEL_40bfa9:
            if (v76 <= v75)
            {
                v77 = v76;
                v61 = v75;
            }
            else
            {
                v77 = v75;
                v61 = v75;
            }
LABEL_40bfb3:
            v79 = 32;
            v80 = v60 - v66;
            if (v61 > 9)
                v79 = (v65 - v33) / (v61 - 1);
        }
    }
    v1 = v64;
    v0 = v61;
    v81 = (v63 - v60) / v79;
    if (!v81)
        v81 = 1;
    if (v81 > 542551296285575042)
        v81 = 542551296285575042;
    v82 = v81 + 5;
    if (v82 <= v76)
        v76 = v82;
    v83 = v80 / v79;
    if (!v83)
        v83 = 1;
    if (v83 > 542551296285575042)
        v83 = 542551296285575042;
    v84 = v76 + l + v83 + 5;
    v20 = v84 < l | v84 > 0xffffffffffffffe;
    if (v84 >= l && v84 <= 0xffffffffffffffe)
    {
        v85 = sub_416a40(v84 * 8);
        if (l != v0)
        {
            if (l > 0)
            {
                v86 = v0 - *((long long *)&g_426570);
                k = v86;
                do
                {
                    *((long long *)(v85 + -(v86) * 8 + k * 8)) = *((long long *)(v62 + (v1 & k) * 8));
                    k += 1;
                } while (l + v86 != k);
                v88 = 0;
                do
                {
                    *((long long *)(v62 + v88 * 8)) = *((long long *)(v85 + v88 * 8));
                    v88 += 1;
                } while (l != v88);
LABEL_40c0d0:
                v89 = 0;
                do
                {
                    *((unsigned long *)(v85 + v89 * 8)) = *((long long *)(v62 + v89 * 8)) + v29 - v33;
                    v89 += 1;
                } while (l > v89);
            }
        }
        else
        {
            if (l)
                goto LABEL_40c0d0;
        }
        index = idx;
        v91 = l * 8;
        v92 = v57 - l;
        v93 = v84 - l;
        *((unsigned long long *)&index[1].padding_b1[21]) = v91 + v85;
        v94 = -(l);
        *((unsigned long long *)&index->padding_b1[7]) = v62 + v91;
        *((unsigned long long *)&index[1].padding_b1[29]) = v94;
        *((unsigned long long *)&index->padding_b1[15]) = v94;
        *((long long *)&index->padding_b1[39]) = v92;
        *((unsigned long long *)&index[1].padding_b1[53]) = v93;
        *((unsigned long long *)&index[1].padding_b1[69]) = v0;
        *((unsigned long long *)&index->padding_b1[55]) = v0;
        g_426700 = v92 + v93 + 1;
        if (g_426700 <= 0x3fffffffffffffe)
        {
            v95 = sub_416a40(g_426700 * 32);
            g_426708 = 1;
            g_426710 = v95;
            v96 = (g_426700 * 6148914691236517206 >> 64) - ((long long)(g_426700) >> 63);
            if (v96 <= 0x200)
            {
                g_426718 = 509;
                v97 = 0xff0;
            }
            else
            {
                v98 = 9;
                do
                {
                    v98 += 1;
                    m = 1 << ((char)v98 & 63);
                } while (m < v96);
                g_426718 = m - (&g_41e1a0)[v98];
                if (g_426718 > 0xffffffffffffffe)
                    sub_416ec0(); /* do not return */
                v97 = g_426718 * 8 + 8;
            }
            g_426720 = sub_40fab0(v97) + 8;
            v100 = idx;
            v16 = &v100->field_a0;
            v21 = &v100[2].padding_b1[11];
LABEL_40c1f0:
            v101 = v16;
            v102 = *((long long *)&v101->padding_4[60]);
            v8 = *((long long *)&v101->padding_4[20]);
            v17 = *((long long *)&v101->padding_4[28]);
            v10 = *((long long *)&v101->padding_4[52]);
            v9 = sub_416a40(*((long long *)&v101->padding_4[52]) * 8);
            v3 = g_426552;
            v5 = g_426710;
            v13 = *((long long *)&v101->padding_50[0]);
            v4 = g_426708;
            v11 = g_426700;
            v12 = *((long long *)&v101->padding_4[12]) + *((long long *)&v101->field_0);
            v19 = *((int *)&g_426558) | g_426552;
            v14 = *((long long *)&v101->padding_4[28]) * 8;
            v15 = -(v14);
            if (v102 < *((long long *)&v101->padding_50[0]))
            {
                v103 = *((int *)&g_426558);
                v104 = *((char *)v102);
                v105 = v102 + 1;
                v1 = 0;
                v18 = *((int *)&g_426558) & 1;
                if (*((int *)&g_426558) != 4)
                {
                    if (*((int *)&g_426558) <= 4)
                        goto LABEL_40c2d6;
                    goto LABEL_40c6b0;
                }
LABEL_40c4c5:
                if ((char)v104 == 10)
                    goto LABEL_40c8d1;
                v107 = 0;
                v0 = v102;
                v2 = v103;
                v133 = *((long long *)__ctype_b_loc());
                do
                {
                    v109 = v105;
                    if (!(*((char *)(*((long long *)__ctype_b_loc()) + v104 * 2 + 1)) & 32))
                        continue;
                    v134 = *((char *)v105);
                    v109 = v105 + 1;
                    if ((char)v134 == 10)
                        break;
                    do
                    {
                        if (!(*((char *)(*((long long *)__ctype_b_loc()) + v134 * 2 + 1)) & 32))
                        {
                            v104 = v134;
                            v105 = v109;
                            v107 = __ROL__(v107, 7) + 32;
                            goto LABEL_40c4f0;
                        }
                    } while ((v134 = (unsigned long)(unsigned long long)(char)*((char *)v109), v109 += 1, (char)v134 != 10));
LABEL_40c4f0:
                    if (v3)
                    {
                        v136 = __ctype_tolower_loc();
                        v104 = *((int *)(*(v136) + v104 * 4));
                    }
                    v105 = v109 + 1;
                    v107 = __ROL__(v107, 7) + v104;
                    v104 = *((char *)v105);
                    v109 = v105;
                } while ((char)v104 != 10);
                v102 = v0;
                v103 = v2;
                v110 = v109 - v102 - 1;
                while (1)
                {
LABEL_40c3d0:
                    v2 = g_426720 + v107 % g_426718 * 8;
                    if (v12 == v109 && v16->padding_50[48] && g_426580 - 4 > 1)
                    {
                        v111 = g_426720 - 8;
                        if (v103 > 1)
                            v111 = v2;
                        v2 = v111;
                    }
                    v6 = *(v2);
                    v0 = v6;
                    if (v6)
                    {
                        v7 = v109;
                        while (1)
                        {
                            while (1)
                            {
                                v112 = v0 * 32 + v5;
                                if (v107 == (long long)v112[8])
                                    break;
LABEL_40c420:
                                v0 = *((long long *)v112);
                                if (!*((long long *)v112))
                                    goto LABEL_40c5a0;
                            }
                            if ((long long)v112[24] != v110)
                            {
                                if (!v103)
                                    goto LABEL_40c420;
                                goto LABEL_40c459;
                            }
                            if (!memcmp((long long)v112[16], v102, v110))
                                break;
                            if (v19)
                            {
LABEL_40c459:
                                if (!(char)sub_40ec00((long long)v112[16], v102))
                                    break;
                                goto LABEL_40c420;
                            }
                            v0 = *((long long *)v112);
                            if (!*((long long *)v112))
                            {
LABEL_40c5a0:
                                v109 = v7;
                                goto LABEL_40c5a5;
                            }
                        }
                        v109 = v7;
                        if (v1 == v10)
                            goto LABEL_40c600;
                    }
                    else
                    {
LABEL_40c5a5:
                        if (v11 != v4)
                        {
LABEL_40c5b9:
                            v4 += 1;
                            v0 = v4;
                            v113 = v0 * 32 + v5;
                            *((unsigned long long *)&v113->field_0) = v6;
                            *((unsigned long long *)&v113->padding_4[4]) = v107;
                            *((void* *)&v113->padding_4[12]) = v102;
                            *((unsigned long *)&v113->padding_4[20]) = v110;
                            *(v2) = v0;
                            if (v1 == v10)
                            {
LABEL_40c600:
                                v114 = v1;
                                if (v114 > 3074457345618258601 || !(v115 = v17, v116 = (long long)(v114 * 2 - v115), v116 <= 0xffffffffffffffe && v114 - v115 <= 0xffffffffffffffe))
                                    break;
                                v9 = sub_416a90(v9, v116 * 8);
                                v10 = v116;
                                v8 = sub_416a90(v8 + v14, (v116 - v115) * 8) + v15;
                            }
                        }
                        else
                        {
                            if (v11 > 0x1fffffffffffffe)
                                break;
                            v11 *= 2;
                            v5 = sub_416a90(v5, v11 * 32);
                            v6 = *(v2);
                            goto LABEL_40c5b9;
                        }
                    }
                    v117 = v1;
                    *((void* *)(v8 + v117 * 8)) = v102;
                    *((unsigned long long *)(v9 + v117 * 8)) = v0;
                    v1 = v117 + 1;
                    if (v13 <= v109)
                        goto LABEL_40c8e7;
                    v102 = v109;
                    v104 = *((char *)v102);
                    v105 = v102 + 1;
                    if (v103 == 4)
                        goto LABEL_40c4c5;
                    if (v103 <= 4)
                    {
LABEL_40c2d6:
                        if (v103 - 1 > 2)
                            goto LABEL_40c810;
                        if ((char)v104 != 10)
                        {
                            v2 = v102;
                            v6 = v103;
                            v118 = 0;
                            v119 = (char)v104;
                            v0 = g_426498;
                            v121 = 0;
                            do
                            {
                                if ((char)v103 & 2 && (v122 = (unsigned long long *)(unsigned long long)__ctype_b_loc(), *((char *)(*(v122) + (unsigned long long)(char)v119 * 2 + 1)) & 32))
                                {
                                    iter = v105;
                                    do
                                    {
                                        iter += 1;
                                        if (*((char *)iter) == 10)
                                        {
                                            v107 = v118;
                                            v103 = v6;
                                            v109 = iter;
                                            v110 = iter - v2 - 1;
                                            goto LABEL_40c3d0;
                                        }
                                    } while (*((char *)(*(v122) + v124 * 2 + 1)) & 32);
                                }
                                if (v18)
                                {
                                    switch (v120)
                                    {
                                    case 9:
                                        v119 = 32;
                                        v126 = v0 - v121 % v0;
                                        v125 = v126;
                                        if (__CFADD__(v121, v125) & 1)
                                        {
                                            v121 = 0;
                                            break;
                                        }
                                        else
                                        {
                                            v121 += v126;
                                            break;
                                        }
                                        continue;
                                    case 13:
                                        v119 = 13;
                                        v125 = 1;
                                        v121 = 0;
                                        continue;
                                    case 8:
                                        v119 = 8;
                                        v121 = v121 - 1 + (v121 < 1);
                                        goto LABEL_40c6a0;
                                    default:
                                        v121 += 1;
                                        v125 = 1;
                                        continue;
                                    }
                                }
LABEL_40c6a0:
                                v125 = 1;
                                if (v3)
                                {
                                    v128 = __ctype_tolower_loc();
                                    v119 = *((char *)(*(v128) + v119 * 4));
                                }
                                do
                                {
                                    v118 = __ROL__(v118, 7) + v119;
                                    v125 -= 1;
                                } while (i0 != 1);
                                v105 += 1;
                                v119 = *((char *)v105);
                            } while (*((char *)v105) != 10);
                            v109 = v105;
                            v107 = v118;
                            v102 = v2;
                            v103 = v6;
                            v110 = v109 - v102 - 1;
                            continue;
                        }
                    }
                    else
                    {
LABEL_40c6b0:
                        if (v103 != 5)
                        {
LABEL_40c810:
                            v109 = v105;
                            if (!v3)
                            {
                                v107 = 0;
                                v110 = 0;
                                if ((char)v104 != 10)
                                {
                                    do
                                    {
                                        v109 += 1;
                                        v107 = __ROL__(v107, 7) + v104;
                                        v104 = *((char *)v109 - 1);
                                    } while ((char)v104 != 10);
                                    v110 = v109 - v102 - 1;
                                    continue;
                                }
                            }
                            else
                            {
                                v105 = v109;
                                if ((char)v104 != 10)
                                {
                                    v131 = __ctype_tolower_loc();
                                    v107 = 0;
                                    do
                                    {
                                        v109 += 1;
                                        v107 = __ROL__(v107, 7) + *((int *)(*(v131) + v104 * 4));
                                        v104 = *((char *)v109 - 1);
                                    } while ((char)v104 != 10);
                                    v110 = v109 - v102 - 1;
                                    continue;
                                }
                            }
                        }
                        else if ((char)v104 != 10)
                        {
                            v106 = __ctype_b_loc();
                            v0 = v102;
                            v107 = 0;
                            do
                            {
                                if (!(*((char *)(*(v106) + v104 * 2 + 1)) & 32))
                                {
                                    if (v3)
                                    {
                                        v108 = __ctype_tolower_loc();
                                        v104 = *((int *)(*(v108) + v104 * 4));
                                    }
                                    v107 = __ROL__(v107, 7) + v104;
                                }
                            } while ((v104 = (unsigned long)(unsigned long long)(char)*((char *)v105), v109 = v105 + 1, v105 = v109, (char)v104 != 10));
                            v102 = v0;
                            v110 = v109 - v102 - 1;
                            continue;
                        }
                    }
LABEL_40c8d1:
                    v110 = 0;
                    v107 = 0;
                }
            }
            else
            {
                v1 = 0;
                v109 = v102;
LABEL_40c8e7:
                v137 = v1;
                v138 = 0;
                v139 = v17;
                *((long long *)&v16->padding_4[36]) = v137;
                v140 = v10;
                v141 = v8;
                while (1)
                {
                    if (v140 != v137)
                    {
                        v142 = v141 + v137 * 8;
                        *(v142) = v109;
                        if (v12 == v109)
                            goto LABEL_40c9c2;
                        goto LABEL_40c928;
                    }
                    if (v140 > 3074457345618258601 || !(v143 = (long long)(v140 * 2 - v139), v143 <= 0xffffffffffffffe && v140 - v139 <= 0xffffffffffffffe))
                        break;
                    v141 = sub_416a90(v141 + v14, (v143 - v139) * 8) + v15;
                    v140 = v143;
                    v142 = v141 + v137 * 8;
                    *(v142) = v109;
                    if (v12 == v109)
                    {
LABEL_40c9c2:
                        v8 = v141;
                        v1 = v137;
                        v10 = v140;
                        if (v16->padding_50[48] && g_426580 - 4 > 1)
                        {
                            *(v142) = v12 - 1;
                            goto LABEL_40c9fc;
                        }
                    }
LABEL_40c928:
                    if (*((long long *)&g_426570) > v138 || !g_426578)
                    {
                        v137 += 1;
                        do
                        {
                            v109 += 1;
                        } while (*((char *)v109 - 1) != 10);
                        v138 += 1;
                    }
                    else
                    {
                        v8 = v141;
                        v1 = v137;
                        v10 = v140;
LABEL_40c9fc:
                        v144 = v16;
                        *((unsigned long *)&v144->padding_4[20]) = v8;
                        v145 = &v144[1].padding_4[10];
                        g_426700 = v11;
                        *((long long *)((char *)v145 - 0x100)) = v1;
                        g_426708 = v4;
                        *((long long *)((char *)v145 - 248)) = v10;
                        *((long long *)((char *)v145 - 216)) = v9;
                        v16 = v145;
                        g_426710 = v5;
                        if (v21 == v16)
                        {
                            idx1 = idx;
                            *((unsigned long *)&idx1[2].padding_4[16]) = v4;
                            *((unsigned long *)&idx1[1].padding_4[2]) = v4;
                            free(v5);
                            free(g_426720 - 8);
                            return v20;
                        }
                        goto LABEL_40c1f0;
                    }
                }
            }
        }
    }
    sub_416ec0(); /* do not return */
}



// Function: print_normal_hunk @ 0xcd40
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern char g_41e550;
extern struct_0 *g_426188;
extern long long g_4261a0;
extern unsigned long long g_426258;
extern long long g_4262d0;
extern unsigned long long g_426388;

void print_normal_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // r12d
    unsigned long long v15;  // rbx
    unsigned long long v16;  // rbx
    unsigned long v17;  // rax
    unsigned long v7;  // rsi
    unsigned long v8;  // rax
    unsigned long v9;  // rax
    long long iter;  // rbp
    unsigned long long v11;  // rbx
    unsigned long long v12;  // rbx
    unsigned long v13;  // rax
    long long v14;  // rbp
    long long v0;  // [bp-0x50]
    long long v1;  // [bp-0x48]
    long long v2;  // [bp-0x40]
    long long v3;  // [bp-0x38]

    v5 = sub_40f670();
    if (!v5)
        return;
    v6 = v5;
    sub_40e830();
    sub_40f510(4);
    sub_40f610(44, &g_4261a0, v0, v1);
    v7 = (&g_41e550)[v6];
    v8 = g_426188[1].field_0;
    if (v8 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v8 + 1;
        *((char *)v8) = v7;
    }
    else
    {
        __overflow(g_426188, v7, &g_41e550);
    }
    sub_40f610(44, &g_4262d0, v2, v3);
    sub_40f510(3);
    v9 = g_426188[1].field_0;
    if (v9 < g_426188[1].field_8)
    {
        g_426188[1].field_0 = v9 + 1;
        *((char *)v9) = 10;
    }
    else
    {
        __overflow(g_426188, 10);
    }
    if ((char)v6 & 1)
    {
        iter = v0;
        if (iter <= v1)
        {
            v11 = iter * 8;
            while (1)
            {
                sub_40f510(2);
                v12 = v11 + 8;
                sub_40f330("<", g_426258 + v11, 1);
                sub_40f510(3);
                if (*((char *)(*((long long *)(g_426258 + v12)) - 1)) == 10)
                {
                    v13 = g_426188[1].field_0;
                    if (v13 < g_426188[1].field_8)
                    {
                        iter += 1;
                        g_426188[1].field_0 = v13 + 1;
                        *((char *)v13) = 10;
                        v11 = v12;
                        if (v1 < iter)
                            break;
                        else
                            continue;
                    }
                    else
                    {
                        __overflow(g_426188, 10);
                    }
                }
                iter += 1;
                v11 = v12;
                if (v1 < iter)
                    break;
            }
        }
    }
    if (v6 == 3)
    {
        fputs_unlocked("---\n", g_426188);
    }
    else if (!((char)v6 & 2))
    {
        return;
    }
    v14 = v2;
    if (v14 > v3)
        return;
    v15 = v14 * 8;
    do
    {
        sub_40f510(1);
        v16 = v15 + 8;
        sub_40f330(">", g_426388 + v15, 1);
        sub_40f510(3);
        if (*((char *)(*((long long *)(g_426388 + v16)) - 1)) == 10)
        {
            v17 = g_426188[1].field_0;
            if (v17 < g_426188[1].field_8)
            {
                g_426188[1].field_0 = v17 + 1;
                *((char *)v17) = 10;
            }
            else
            {
                __overflow(g_426188, 10);
            }
        }
    } while ((v14 = (long long)(v14 + 1), v15 = v16, v3 >= v14));
    return;
}



// Function: print_normal_script @ 0xcff0
void print_normal_script(unsigned long a0)
{
    sub_40f080();
    return;
}



// Function: tab_from_to @ 0xd010
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

unsigned long long tab_from_to(unsigned long long a0, unsigned long long a1)
{
    unsigned long long v1;  // rax
    char *v2;  // rdx
    unsigned long long v3;  // rbp
    char *v4;  // rax

    if (!g_4264a0)
    {
        v1 = g_426498 + a0 - a0 % g_426498;
        if (v1 <= a1)
        {
            do
            {
                v2 = g_426188[1].field_0;
                a0 = v1;
                if (v2 < g_426188[1].field_8)
                {
                    g_426188[1].field_0 = v2 + 1;
                    *(v2) = 9;
                }
                else
                {
                    __overflow(g_426188, 9);
                }
            } while ((v1 = g_426498 + a0, a1 >= v1));
        }
    }
    v3 = a0 + 1;
    if (a1 <= a0)
        return a1;
    while (1)
    {
        v4 = g_426188[1].field_0;
        if (v4 < g_426188[1].field_8)
        {
            g_426188[1].field_0 = v4 + 1;
            *(v4) = 32;
        }
        else
        {
            __overflow(g_426188, 32);
        }
        if (a1 == v3)
            break;
        v3 += 1;
    }
    return a1;
}



// Function: print_half_line @ 0xd0d0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

typedef struct FILE {
} FILE;

extern FILE *stdout;
extern struct_1 *g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

unsigned long long print_half_line(struct_0 **a0, unsigned long long a1, unsigned long long a2)
{
    unsigned long long iter;  // r10
    char *v7;  // rbp
    unsigned long v16;  // rax
    unsigned long v17;  // rax
    unsigned long v18;  // rax
    unsigned long long v19;  // rdx
    unsigned long v20;  // rax
    unsigned long v21;  // rax
    unsigned long v22;  // rax
    unsigned int v23;  // eax
    char *v8;  // r14
    unsigned long long node;  // r12
    char v10;  // bl
    char *v11;  // r9
    unsigned long long v12;  // rcx
    unsigned long long v13;  // rbx
    unsigned long v14;  // rax
    unsigned long v15;  // rax
    char *count;  // [bp-0x70], Other Possible Types: unsigned long, unsigned long long
    unsigned long long v1;  // [bp-0x68]
    unsigned long long v2;  // [bp-0x60]
    char v3;  // [bp-0x4c], Other Possible Types: unsigned int
    unsigned long v4;  // [bp-0x48]

    iter = 0;
    v7 = &*(a0)->field_0;
    v8 = &a0[1]->field_0;
    v4 = 0;
    if (v7 < v8)
    {
        node = 0;
        while (1)
        {
            v10 = *(v7);
            v11 = v7 + 1;
            if (v10 <= 95)
            {
                if (v10 <= 7)
                    goto LABEL_40d1c0;
                switch (v10)
                {
                case 8:
                    if (node)
                    {
                        node -= 1;
                        if (a2 > node)
                        {
                            if (node < iter)
                            {
                                v17 = g_426188->field_28;
                                if (v17 < *((long long *)&g_426188[1].padding_0[0]))
                                {
                                    iter = node;
                                    g_426188->field_28 = v17 + 1;
                                    *((char *)v17) = 8;
                                    break;
                                }
                                else
                                {
                                    __overflow(g_426188, 8);
                                    iter = node;
                                    break;
                                }
                            }
                            else if (node > iter)
                            {
                                do
                                {
                                    v18 = g_426188->field_28;
                                    if (v18 < *((long long *)&g_426188[1].padding_0[0]))
                                    {
                                        g_426188->field_28 = v18 + 1;
                                        *((char *)v18) = 32;
                                    }
                                    else
                                    {
                                        v1 = iter;
                                        count = v11;
                                        __overflow(g_426188, 32);
                                        v11 = count;
                                        iter = v1;
                                    }
                                } while ((iter += 1, node != iter));
                                v11 = v11;
                                break;
                            }
                        }
                    }
                    break;
                case 9:
                    v12 = g_426498 - node % g_426498;
                    if (iter == node)
                    {
                        v13 = v12 + iter;
                        if (g_4264a0)
                        {
                            if (a2 <= v13)
                                v13 = a2;
                            if (v13 > iter)
                            {
                                do
                                {
                                    v15 = g_426188->field_28;
                                    if (v15 < *((long long *)&g_426188[1].padding_0[0]))
                                    {
                                        g_426188->field_28 = v15 + 1;
                                        *((char *)v15) = 32;
                                    }
                                    else
                                    {
                                        v2 = iter;
                                        v1 = v12;
                                        count = v11;
                                        __overflow(g_426188, 32);
                                        v11 = count;
                                        v12 = v1;
                                        iter = v2;
                                    }
                                } while ((iter += 1, iter != v13));
                            }
                        }
                        else if (a2 > v13)
                        {
                            v14 = g_426188->field_28;
                            if (v14 < *((long long *)&g_426188[1].padding_0[0]))
                            {
                                iter = v13;
                                g_426188->field_28 = v14 + 1;
                                *((char *)v14) = 9;
                            }
                            else
                            {
                                v1 = v12;
                                count = v11;
                                __overflow(g_426188, 9);
                                v11 = count;
                                v12 = v1;
                                iter = v13;
                            }
                        }
                    }
                    node += v12;
                    break;
                case 10:
                    goto LABEL_40d198;
                case 11: case 12:
LABEL_40d1fd:
                    if (a2 > node)
                    {
                        v22 = g_426188->field_28;
                        if (v22 < *((long long *)&g_426188[1].padding_0[0]))
                        {
                            g_426188->field_28 = v22 + 1;
                            *((char *)v22) = v10;
                            break;
                        }
                        else
                        {
                            count = iter;
                            __overflow(g_426188, v10);
                            iter = count;
                            break;
                        }
                    }
                    break;
                case 13:
                    v16 = g_426188->field_28;
                    if (v16 < *((long long *)&g_426188[1].padding_0[0]))
                    {
                        g_426188->field_28 = v16 + 1;
                        *((char *)v16) = 13;
                    }
                    else
                    {
                        count = v11;
                        __overflow(g_426188, 13);
                        v11 = count;
                    }
                    count = v11;
                    node = 0;
                    sub_40d010(0, a1);
                    v11 = count;
                    iter = 0;
                    break;
                case 32: case 33: case 34: case 35: case 37: case 38: case 39: case 40: case 41: case 42: case 43: case 44: case 45: case 46: case 47: case 48: case 49: case 50: case 51: case 52: case 53: case 54: case 55: case 56: case 57: case 58: case 59: case 60: case 61: case 62: case 63: case 65: case 66: case 67: case 68: case 69: case 70: case 71: case 72: case 73: case 74: case 75: case 76: case 77: case 78: case 79: case 80: case 81: case 82: case 83: case 84: case 85: case 86: case 87: case 88: case 89: case 90: case 91: case 92: case 93: case 94: case 95:
LABEL_40d15f:
                    v19 = node + 1;
                    if (a2 > node)
                    {
                        v20 = g_426188->field_28;
                        if (v20 < *((long long *)&g_426188[1].padding_0[0]))
                        {
                            v7 = v11;
                            iter = v19;
                            node = v19;
                            g_426188->field_28 = v20 + 1;
                            *((char *)v20) = v10;
                            if (v8 <= v7)
                                goto LABEL_40d198;
                            else
                                continue;
                        }
                        else
                        {
                            v1 = v19;
                            count = v11;
                            __overflow(g_426188, v10);
                            iter = v1;
                            v11 = count;
                            node = iter;
                            break;
                        }
                    }
                    else
                    {
                        node = v19;
                        break;
                    }
                default:
LABEL_40d1c0:
                    v1 = iter;
                    count = v11;
                    v21 = sub_417200(&v3, v7, v8 - v7, &v4);
                    v11 = count;
                    iter = v1;
                    if (v21 - 1 > 0xfffffffffffffffc)
                        goto LABEL_40d1fd;
                    count = v21;
                    v23 = wcwidth(v3);
                    if (v23 > 0)
                        node += v23;
                    iter = v1;
                    if (a2 >= node)
                    {
                        fwrite_unlocked(v7, 1, count, stdout);
                        iter = node;
                    }
                    v11 = &v7[count];
                    break;
                }
                v7 = v11;
                if (v8 <= v7)
                    break;
            }
            else
            {
                if (v10 - 97 <= 29)
                    goto LABEL_40d15f;
                else
                    goto LABEL_40d1c0;
            }
        }
    }
LABEL_40d198:
    return iter;
}



// Function: print_1sdiff_line @ 0xd500
typedef struct struct_5 {
    char padding_0[8];
    void* field_8;
} struct_5;

typedef struct struct_1 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_1;

typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_3 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_3;

extern struct_3 *g_426188;
extern struct_0 *g_426428;
extern unsigned long long g_426430;

char * print_1sdiff_line(struct_5 *a0, char a1, struct_1 *a2)
{
    char *v4;  // rax
    unsigned int v5;  // r15d
    char *v6;  // rbp
    unsigned long v7;  // r15
    unsigned long long v8;  // rax
    char *v0;  // [bp-0x48]
    char v1;  // [bp-0x3a]
    char v2;  // [bp-0x39]

    v4 = g_426428;
    v2 = a1;
    v0 = g_426428;
    switch (a1)
    {
    case 60:
        sub_40f510(2);
        goto LABEL_40d67a;
    case 62:
        sub_40f510(1);
LABEL_40d67a:
        v1 = 1;
        if (a0)
        {
LABEL_40d556:
            v5 = _INSERT(v7, 0, *((char *)a0->field_8 - 1) == 10);
            v4 = sub_40d0d0(a0, 0, g_426430);
            v6 = v4;
            if (a1 != 32)
                goto LABEL_40d5fe;
            break;
        }
        else
        {
            v5 = 0;
            v6 = sub_40d010(0, &v0[g_426430] - 1 >> 1) + 1;
LABEL_40d6a5:
            v4 = &g_426188->field_28->field_0;
            if (v4 < *((long long *)&g_426188[1].padding_0[0]))
            {
                g_426188->field_28 = v4 + 1;
                *(v4) = v2;
                break;
            }
            else
            {
                v4 = __overflow(g_426188);
                break;
            }
        }
    default:
        v1 = 0;
        if (a0)
            goto LABEL_40d556;
        v5 = 0;
        v6 = NULL;
        if (a1 != 32)
        {
LABEL_40d5fe:
            v6 = sub_40d010(v6, &v0[g_426430] - 1 >> 1) + 1;
            if (a1 == 124 && *((char *)(a2->field_8 - 1)) == 10 != (char)v5)
            {
                v2 = ((char)-((char)v5 < 1) & 45) + 47;
                goto LABEL_40d6a5;
            }
        }
        break;
    }
    if (a2)
    {
        v5 |= _INSERT(a2->field_8, 0, *((char *)(a2->field_8 - 1)) == 10);
        v4 = &a2->field_0->field_0;
        if (*(v4) != 10)
        {
            v8 = sub_40d010(v6, v0);
            v4 = sub_40d0d0(a2, v8, g_426430);
        }
    }
    if ((char)v5)
    {
        v4 = &g_426188->field_28->field_0;
        if (v4 < *((long long *)&g_426188[1].padding_0[0]))
        {
            g_426188->field_28 = v4 + 1;
            *(v4) = 10;
        }
        else
        {
            v4 = __overflow(g_426188, 10);
        }
    }
    if (v1)
        return sub_40f510();
    return v4;
}



// Function: print_sdiff_common_lines @ 0xd700
extern unsigned long long g_426188;
extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern char g_426438;
extern char g_426439;
extern char g_42643a;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

char * print_sdiff_common_lines(unsigned long long a0, unsigned long long i)
{
    unsigned long long j;  // rbp
    unsigned long long v2;  // rbx
    char v3;  // r15b
    char *v4;  // rax

    if (!g_426438 && (j = g_426730, v2 = g_426728, v3 = (char)(char)(g_426728 != i), g_426730 != a0 || v3))
    {
        if (g_42643a)
            v4 = __fprintf_chk(g_426188, 0x1, "i%ld,%ld\n", a0 - g_426730, i - g_426728);
        if (!g_426439)
        {
            if (g_426730 != a0 && v3)
            {
                do
                {
                    v2 += 1;
                    j += 1;
                    v4 = sub_40d500(g_426258 + j * 8 - 8, 32, g_426388 + v2 * 8 - 8);
                    if (a0 == j)
                        goto LABEL_40d7c8;
                } while (i != v2);
            }
            else
            {
LABEL_40d7c8:
                if (i != v2)
                {
                    do
                    {
                        v4 = sub_40d500(NULL, 41, g_426388 + v2 * 8);
                        v2 += 1;
                    } while (i != v2);
                    if (a0 == j)
                    {
                        g_426730 = a0;
                        g_426728 = i;
                        return v4;
                    }
                }
            }
        }
        for (; a0 != j; j += 1)
        {
            v4 = sub_40d500(g_426258 + j * 8, 40, NULL);
        }
    }
    g_426730 = a0;
    g_426728 = i;
    return v4;
}



// Function: print_sdiff_hunk @ 0xd860
extern unsigned long long g_426188;
extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern char g_42643a;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

void print_sdiff_hunk(void)
{
    unsigned int v5;  // eax
    unsigned int v6;  // r12d
    unsigned long v15;  // rbx
    unsigned long v16;  // rbx
    unsigned long k;  // rcx
    unsigned long v8;  // rbx
    unsigned long v9;  // rdx
    unsigned long v10;  // rbp
    unsigned long v11;  // rbp
    unsigned long v12;  // rbx
    unsigned long v13;  // rbx
    unsigned long v14;  // rbx
    unsigned long v0;  // [bp-0x40]
    unsigned long j;  // [bp-0x38]
    unsigned long v2;  // [bp-0x30]
    unsigned long i;  // [bp-0x28]

    v5 = sub_40f670();
    if (!v5)
        return;
    v6 = v5;
    sub_40d700(v0, v2);
    if (g_42643a)
        __fprintf_chk(g_426188, 0x1, "c%ld,%ld\n", j - v0 + 1, i - v2 + 1);
    if (v6 == 3)
    {
        k = i;
        v8 = v0;
        v9 = j;
        if (v8 <= v9)
        {
            for (v10 = v2; k >= v10; k = i)
            {
                v11 = v10 + 1;
                v12 = v8 + 1;
                sub_40d500(g_426258 + v8 * 8, 124, g_426388 + v10 * 8);
                if (j >= v12)
                {
                    v10 = v11;
                    v8 = v12;
                }
                else
                {
                    v10 = v11;
                    v8 = v12;
                    k = i;
                    break;
                }
            }
        }
        v0 = v8;
        g_426730 = v8;
        v2 = v10;
        g_426728 = v10;
        v6 = (v8 <= v9) + (v10 <= k) * 2;
    }
    if ((char)v6 & 2)
    {
        v13 = v2;
        v14 = v13;
        if (v14 <= i)
        {
            do
            {
                v14 = v13 + 1;
                sub_40d500(NULL, 62, g_426388 + v13 * 8);
                v13 = v14;
            } while (i >= v13);
        }
        g_426728 = v14;
    }
    if (!((char)v6 & 1))
        return;
    v15 = v0;
    v16 = v15;
    if (j >= v16)
    {
        do
        {
            v16 = v15 + 1;
            sub_40d500(g_426258 + v15 * 8, 60, NULL);
            v15 = v16;
        } while (j >= v15);
    }
    g_426730 = v16;
    return;
}



// Function: print_sdiff_script @ 0xda50
extern unsigned long long g_426270;
extern unsigned long long g_426288;
extern unsigned long long g_4263a0;
extern unsigned long long g_426728;
extern unsigned long long g_426730;

char * print_sdiff_script(long long a0)
{
    sub_40e830();
    g_426728 = -(g_426288);
    g_426730 = g_426728;
    sub_40f080(a0, sub_40f060, sub_40d860);
    return sub_40d700(g_426270, g_4263a0);
}



// Function: sighandler @ 0xdaa0
extern unsigned int g_426774;

long long sighandler(unsigned int a0)
{
    unsigned long v1;  // rax

    v1 = g_426774;
    if (!(unsigned int)v1)
        g_426774 = a0;
    return v1;
}



// Function: get_funky_string @ 0xdac0
typedef struct struct_0 {
    char field_0;
    char field_1;
    char field_2;
    char field_3;
    char field_4;
} struct_0;


unsigned int get_funky_string(void* *a0, struct_0 **a1, unsigned int a2, unsigned long long *a3)
{
    unsigned int v1;  // eax
    struct_0 *node;  // rcx
    unsigned long long iter;  // rdi
    unsigned int v4;  // edx
    unsigned long long v5;  // r8
    void* v7;  // r9

    v1 = a2;
    node = *(a1);
    iter = 1;
    v4 = node->field_0;
    v5 = 0;
    v7 = *(a0) - 0;
    if ((char)v4 != 92)
        if ((char)v4 <= 92)
            goto LABEL_0x40dafc;
        else
            goto LABEL_0x40db70;
}



// Function: check_color_output @ 0xdd30
typedef struct FILE {
} FILE;

typedef struct sigaction {
} sigaction;

typedef struct struct_0 {
    char padding_0[32];
    struct struct_0 *field_20;
} struct_0;

typedef struct struct_1 {
    char padding_0[8];
    unsigned long long field_8;
    char padding_10[8];
    unsigned long long field_18;
    struct struct_0 *field_20;
} struct_1;

typedef struct struct_3 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_3;

extern int g_41e524;
extern int g_41e550;
extern struct_3 g_425920;
extern char g_426020;
extern char g_426180;
extern FILE *g_426188;
extern unsigned int g_42657c;
extern struct_0 *g_426748;
extern struct_0 *g_426750;
extern char g_426758;
extern uint128_t g_426780;
extern uint128_t g_426790;
extern uint128_t g_4267a0;
extern uint128_t g_4267b0;
extern uint128_t g_4267c0;
extern uint128_t g_4267d0;
extern uint128_t g_4267e0;
extern uint128_t g_4267f0;

void check_color_output(char a0)
{
    unsigned int v18;  // eax
    unsigned int v19;  // ebp
    struct_0 *v29;  // rbx
    struct_0 *ptr;  // rbx
    struct_0 *v31;  // rbx
    int v32;  // r15d
    int *iter;  // rbx
    int *node;  // rbp
    int v35;  // ebp
    sigaction v36;  // rax
    unsigned int v20;  // eax
    char *iter1;  // rbx
    char v22;  // al
    unsigned long long index;  // r15
    char *v24;  // rsi
    unsigned long long *v25;  // rcx
    struct_1 *idx;  // r12
    char *v0;  // [bp-0x108]
    char *v1;  // [bp-0x100]
    char *iter2;  // [bp-0xf8], Other Possible Types: unsigned long
    void* v3;  // [bp-0xf0]
    sigaction v4;  // [bp-0xe8]
    int v5;  // [bp-0xe0]
    int v6;  // [bp-0xd0]
    int v7;  // [bp-0xc0]
    int v8;  // [bp-0xb0]
    int v9;  // [bp-0xa0]
    int v10;  // [bp-0x90]
    int v11;  // [bp-0x80]
    int v12;  // [bp-0x70]
    unsigned int v13;  // [bp-0x60]
    char v14;  // [bp-0x43], Other Possible Types: unsigned short
    char v15;  // [bp-0x42]
    char v16;  // [bp-0x41]

    if (!g_426188)
        return;
    v18 = g_42657c;
    if (!g_42657c)
        return;
    if (g_426180)
    {
        v19 = 1;
        goto LABEL_40dd84;
    }
    if (a0)
    {
        if (g_42657c != 2)
        {
            g_426758 = 0;
            return;
        }
        v19 = 0;
    }
    else
    {
        v20 = isatty(fileno(g_426188));
        v18 = g_42657c;
        v19 = v20;
LABEL_40dd84:
        if (v18 != 2)
        {
            if (v18 != 1 || !v19)
                goto LABEL_40df11;
            v19 = 1;
        }
    }
    iter1 = g_426748;
    g_426758 = 1;
    iter2 = g_426748;
    if (g_426748 && (char)g_426748->field_0)
    {
        v16 = 0;
        v14 = 0x3f3f;
        v3 = sub_416ea0(g_426748);
        while (1)
        {
            v22 = *(iter1);
            switch (v22)
            {
            case 42:
                idx = sub_416a40(40);
                v1 = &iter2;
                v0 = &v3;
                idx->field_20 = g_426750;
                g_426750 = idx;
                idx->field_8 = v3;
                iter2 = iter1 + 1;
                if ((char)sub_40dac0(&v3, &iter2, 1, idx) && !(iter2 = (char *)(iter2 + 1), *((char *)iter2) != 61 || (idx->field_18 = (unsigned long long)v3, !(char)sub_40dac0(v0, v1, 0, idx + 16))))
                    goto LABEL_40e14b;
LABEL_40deca:
                error(0, 0, dcgettext(NULL, "unparsable value for --palette", 5));
                free(v3);
                v29 = g_426750;
                if (g_426750)
                {
                    do
                    {
                        ptr = v29;
                        v31 = ptr->field_20;
                        free(ptr);
                        v29 = v31;
                    } while (ptr->field_20);
                }
                break;
            case 58:
                iter1 += 1;
                iter2 = iter1;
                break;
            case 0:
                goto LABEL_40df18;
            default:
                v14 = v22;
                iter2 = iter1 + 1;
                if (!iter1[1] || (v15 = iter1[1], iter2 = iter1 + 3, iter1[2] != 61))
                    goto LABEL_40deca;
                index = 0;
                v24 = "lc";
                while (strcmp(&v14, v24))
                {
                    index += 1;
                    v24 = *((long long *)&g_425920.padding_0[8 * index]);
                    if (!v24)
                        goto LABEL_40dea6;
                }
                v25 = &(&g_426020)[16 * (unsigned int)index];
                v25[1] = v3;
                if (!(char)sub_40dac0(&v3, &iter2, 0, v25))
                {
LABEL_40dea6:
                    error(0, 0, dcgettext(NULL, "unrecognized prefix: %s", 5));
                    goto LABEL_40deca;
                }
                else
                {
LABEL_40e14b:
                    iter1 = iter2;
                    break;
                }
            }
        }
LABEL_40df11:
        g_426758 = 0;
    }
LABEL_40df18:
    if (!v19)
        return;
    v32 = 20;
    iter = &g_41e524;
    sigemptyset(&g_426780);
    node = iter;
    while (1)
    {
        sigaction(v32, NULL, &v4);
        if (v4 != 1)
            sigaddset(&g_426780, v32);
        if (node == &g_41e550)
            break;
        v32 = *(node);
        node += 1;
    }
    v13 = 0x10000000;
    v35 = 20;
    v5 = (int)g_426780;
    v6 = (int)g_426790;
    v7 = (int)g_4267a0;
    v8 = (int)g_4267b0;
    v9 = (int)g_4267c0;
    v10 = (int)g_4267d0;
    v11 = (int)g_4267e0;
    v12 = (int)g_4267f0;
    while (1)
    {
        if (sigismember(&g_426780, v35))
        {
            v36 = (sigaction)sub_40daa0;
            if (v35 == 20)
                v36 = (sigaction)sub_40e1a0;
            v4 = v36;
            sigaction(v35, &v4, NULL);
        }
        if (iter == &g_41e550)
            break;
        v35 = *(iter);
        iter += 1;
    }
    return;
}



// Function: put_indicator @ 0xe180
typedef struct struct_0 {
    char field_0;
    char padding_1[7];
    void* field_8;
} struct_0;

typedef struct FILE {
} FILE;

extern FILE *g_426188;

long long put_indicator(struct_0 *a0)
{
    return fwrite_unlocked(a0->field_8, *((long long *)&a0->field_0), 1, g_426188);
}



// Function: process_signals @ 0xe1c0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned int g_426010;
extern struct_0 g_426020;
extern struct_0 g_426030;
extern struct_0 g_426050;
extern FILE *stdout;
extern char g_426758;
extern unsigned int g_426770;
extern unsigned int g_426774;
extern int g_426780;

void process_signals(void)
{
    unsigned int v2;  // r15d
    int v0[34];  // [bp-0xc8]

    while (g_426774 || g_426770)
    {
        if (g_426758 && g_426010 != 3)
        {
            sub_40e180(&g_426020.field_0);
            sub_40e180(&g_426050.field_0);
            sub_40e180(&g_426030.field_0);
            g_426010 = 3;
        }
        fflush_unlocked(stdout);
        sigprocmask(0, &g_426780, v0);
        v2 = g_426774;
        if (g_426770)
        {
            v2 = 19;
            g_426770 = g_426770 - 1;
        }
        else
        {
            signal(g_426774, NULL);
        }
        raise(v2);
        sigprocmask(2, v0, NULL);
    }
    return;
}



// Function: c_escape @ 0xe2e0

void* c_escape(void* a0)
{
    char v1;  // al
    void* node;  // r12
    unsigned long long iter;  // rsi
    char v4;  // dil
    void* idx;  // rdx
    void* v6;  // rax
    void* iter1;  // rdx
    char v8;  // al

    v1 = *((char *)a0);
    node = a0;
    if (!*((char *)a0))
        return node;
    iter = 0;
    v4 = 0;
    while (1)
    {
        idx = a0;
        if (v1 == 32)
        {
            v4 = 1;
            goto LABEL_40e308;
        }
        if (v1 == 0x22)
        {
LABEL_40e304:
            iter += 1;
            goto LABEL_40e308;
        }
        else if (v1 <= 0x22)
        {
            if (v1 - 7 <= 6)
                goto LABEL_40e304;
            if (v1 <= 31)
                iter += 3;
LABEL_40e308:
            v1 = (char)idx[1];
            a0 = idx + 1;
            if (!(char)idx[1])
                break;
        }
        else if (v1 != 92)
        {
            v1 = (char)idx[1];
            a0 = idx + 1;
            if (!(char)idx[1])
                break;
        }
    }
    if (!iter && !v4)
        return node;
    v6 = sub_416a40(iter + a0 - node + 3);
    *((char *)v6) = 0x22;
    iter1 = v6 + 1;
    v8 = *((char *)node);
    if (v8)
    {
        do
        {
            if (v8 <= 0x22)
            {
                if (v8 <= 6)
                    goto LABEL_40e3a8;
                switch (v8)
                {
                case 7:
                    *((char *)iter1) = 92;
                    *((char *)&iter1[1]) = 97;
                    continue;
                case 9:
                    v8 = 116;
                    break;
                case 10:
                    *((char *)iter1) = 92;
                    *((char *)&iter1[1]) = 110;
                    continue;
                case 11:
                    *((char *)iter1) = 92;
                    *((char *)&iter1[1]) = 118;
                    continue;
                case 12:
                    *((char *)iter1) = 92;
                    *((char *)&iter1[1]) = 0x66;
                    continue;
                case 13:
                    v8 = 114;
                    break;
                case 8:
                    v8 = 98;
                case 34:
LABEL_40e416:
                    *((char *)iter1) = 92;
                    *((char *)&iter1[1]) = v8;
                    continue;
                default:
LABEL_40e3a8:
                    if (v8 <= 31)
                    {
                        *((char *)iter1) = 92;
                        iter1 += 4;
                        *((char *)iter1 - 3) = (v8 >> 6) + 48;
                        *((char *)iter1 - 1) = (v8 & 7) + 48;
                        *((char *)iter1 - 2) = ((char)(v8) >> 3 & 7) + 48;
                        continue;
                    }
                }
            }
            else
            {
                if (v8 == 92)
                    goto LABEL_40e416;
                else
                    goto LABEL_40e484;
            }
LABEL_40e484:
            *((char *)iter1) = v8;
            iter1 += 1;
        } while ((v8 = (char)node[1], node += 1, v8));
    }
    *((unsigned short *)iter1) = 0x22;
    return v6;
}



// Function: perror_with_name @ 0xe490
void perror_with_name(unsigned long a0)
{
    int *err;  // rax

    err = __errno_location();
    error(0, *(err), "%s");
    return;
}



// Function: message5 @ 0xe4c0
typedef struct struct_0 {
    char field_0;
} struct_0;

typedef struct struct_1 {
    char padding_0[40];
    struct struct_0 *field_28;
} struct_1;

extern unsigned long long g_41e5b0;
extern struct_0 *g_4260a0;
extern struct_1 *stdout;
extern char g_42643a;
extern char g_426480;

int message5(void* a0, unsigned long a1, unsigned long a2, unsigned long long a3, unsigned long long a4)
{
    long long v6;  // r13
    unsigned long long v7;  // rbx
    unsigned long v16;  // fs
    char *v17;  // rax
    void* v8;  // rdi
    unsigned long v9;  // rax
    unsigned long long v10;  // rbx
    void* v11;  // r14
    void* v12;  // rcx
    unsigned long long v13;  // rbx
    void* dst;  // rax
    void* *v15;  // rax
    int v0;  // [bp-0x88]
    unsigned long long v1;  // [bp-0x70]
    unsigned long long v2;  // [bp-0x68]
    int v3;  // [bp-0x58]
    unsigned long v4;  // [bp-0x30]

    if (g_426480)
    {
        *((unsigned long *)&(&v0)[8]) = a1;
        v6 = 8;
        if (!a3)
            a3 = &g_41e5b0;
        *((unsigned long *)&(&v0)[16]) = a2;
        if (!a4)
            a4 = &g_41e5b0;
        v1 = a3;
        v7 = 0;
        v2 = a4;
        v8 = a0;
        while (1)
        {
            v9 = strlen(v8) + 1;
            *((unsigned long *)((char *)&v3 + v7)) = v9;
            v7 += 8;
            v6 += v9;
            if (v7 == 40)
                break;
            v8 = *((long long *)((char *)&v0 + v7));
        }
        v10 = 0;
        v11 = sub_416a40(v6);
        v12 = v11 + 8;
        while (1)
        {
            v13 = v10 + 8;
            dst = memcpy(v12, a0, *((long long *)((char *)&v3 + v10)));
            v12 = dst + *((long long *)((char *)&v3 + v10));
            if (v13 == 40)
                break;
            a0 = *((long long *)((char *)&v0 + v13));
            v10 = v13;
        }
        v15 = g_4260a0;
        g_4260a0 = v11;
        *(v15) = v11;
        *((unsigned long long *)v11) = 0;
        return v4 - *((long long *)(40 + v16));
    }
    else
    {
        if (!g_42643a)
            return __printf_chk(1, dcgettext(NULL, a0, 5));
        v17 = &stdout->field_28->field_0;
        if (v17 < *((long long *)&stdout[1].padding_0[0]))
        {
            stdout->field_28 = v17 + 1;
            *(v17) = 32;
        }
        else
        {
            __overflow(stdout, 32);
        }
        return __printf_chk(1, dcgettext(NULL, a0, 5));
    }
}



// Function: message @ 0xe660
int message(void* a0, unsigned int a1, unsigned int a2)
{
    return sub_40e4c0(a0, a1, a2, 0, 0);
}



// Function: print_message_queue @ 0xe670
typedef struct struct_0 {
    struct struct_0 *field_0;
} struct_0;

extern struct_0 *g_426800;

void print_message_queue(void)
{
    struct_0 **i;  // rbp
    struct_0 **ptr;  // r13
    char *iter;  // r15
    unsigned long v10;  // rbx
    unsigned long v11;  // rbx
    unsigned long long v0;  // [bp-0x80]
    unsigned long long v1;  // [bp-0x78]
    char *v2;  // [bp-0x70]
    char v3;  // [bp-0x68]
    unsigned long long v4;  // [bp-0x50]
    unsigned long long v5;  // [bp-0x48]

    i = g_426800;
    v2 = &v3;
    if (!g_426800)
        return;
    do
    {
        ptr = i;
        iter = v2;
        i = *(i);
        v10 = ptr + 1;
        v11 = v10;
        do
        {
            iter += 8;
            v11 = v11 + strlen(v11) + 1;
            *((unsigned long *)&iter) = v11;
        } while (&v5 != iter);
        v1 = v5;
        v0 = v4;
        __printf_chk(1, dcgettext(NULL, v10, 5));
        free(ptr);
    } while (i);
    return;
}



// Function: pfatal_with_name @ 0xe770
void pfatal_with_name(unsigned long a0)
{
    unsigned long long v2;  // r12
    int *err;  // rax
    unsigned long long v0;  // [bp-0x8]

    v0 = v2;
    err = __errno_location();
    sub_40e670();
    error(2, *(err), "%s");
}



// Function: fatal @ 0xe7b0
void fatal(char *a0)
{
    sub_40e670();
    dcgettext(NULL, a0, 5);
    error(2, 0, "%s");
}



// Function: set_color_palette @ 0xe7f0
extern unsigned long long g_426748;

void set_color_palette(unsigned long a0)
{
    g_426748 = a0;
    return;
}



// Function: setup_output @ 0xe800
extern unsigned long long g_426188;
extern char g_426759;
extern unsigned long long g_426760;
extern unsigned long long g_426768;

void setup_output(unsigned long a0, unsigned long a1, char a2)
{
    g_426768 = a0;
    g_426760 = a1;
    g_426759 = a2;
    g_426188 = 0;
    return;
}



// Function: begin_output @ 0xe830
extern char g_41e558;
extern unsigned long long stdout;
extern unsigned long long g_426188;
extern int g_4261a0;
extern long long g_426420;
extern char g_426480;
extern unsigned int g_426580;
extern int g_426740;
extern char g_426759;
extern void g_426760;
extern void* g_426768;

void begin_output(void)
{
    void* v9;  // rbp
    unsigned int *err;  // rax
    int v0;  // [bp-0x60]
    int v1;  // [bp-0x5c]
    void* v2;  // [bp-0x58]
    unsigned long v3;  // [bp-0x50]
    unsigned long v4;  // [bp-0x48]
    unsigned long long v5;  // [bp-0x40]
    void* v6;  // [bp-0x38]
    unsigned long long v7;  // [bp-0x30]

    if (g_426188)
        return;
    v2 = sub_40e2e0(g_426768);
    v3 = sub_40e2e0(*((long long *)&g_426760));
    v9 = sub_417020("diff%s %s %s", g_426420, v2, v3);
    if (!g_426480)
    {
        g_426188 = stdout;
        sub_40dd30(0);
        if (g_426759)
            __printf_chk(1, "%s\n", v9);
    }
    else if (!fflush_unlocked(stdout))
    {
        v6 = v9;
        v4 = &g_41e558;
        v5 = "-h";
        v7 = 0;
        if (pipe(&v0))
            sub_40e770("pipe"); /* do not return */
        g_426740 = fork();
        if (g_426740 < 0)
            sub_40e770("fork"); /* do not return */
        if (!g_426740)
        {
            close(v1);
            if (v0)
            {
                if (dup2(v0, 0) >= 0)
                    close(v0);
                else
                    sub_40e770("dup2"); /* do not return */
            }
            execv(&g_41e558, &v4);
            err = __errno_location();
            _exit((unsigned int)((*(err) == 2) + 126)); /* do not return */
        }
        close(v0);
        g_426188 = fdopen(v1, "w");
        if (!g_426188)
            sub_40e770("fdopen"); /* do not return */
        sub_40dd30(1);
    }
    else
    {
        sub_40e770(dcgettext(NULL, "write failed", 5)); /* do not return */
    }
    free(v9);
    if (g_426580 == 2)
    {
        sub_408b80(&g_4261a0, &v2, 0);
    }
    else if (g_426580 == 3)
    {
        sub_408b80(&g_4261a0, &v2, 1);
    }
    if (v2 != g_426768)
        free(v2);
    if (v3 != *((long long *)&g_426760))
    {
        free(v3);
        return;
    }
    return;
}



// Function: finish_output @ 0xead0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern char stdout;
extern struct_0 *g_426188;
extern int g_426740;

void finish_output(void)
{
    char v2;  // al
    unsigned int v3;  // ebx
    char *v4;  // rsi
    unsigned int v0;  // [bp-0x14]

    if (!g_426188)
    {
        g_426188 = 0;
        return;
    }
    else if (g_426188 == *((long long *)&stdout))
    {
        g_426188 = 0;
        return;
    }
    else if ((char)g_426188->field_0 & 32)
    {
        sub_40e7b0("write failed"); /* do not return */
    }
    else if (fclose(g_426188))
    {
        sub_40e770(dcgettext(NULL, "write failed", 5)); /* do not return */
    }
    else if (waitpid(g_426740, &v0, 0) >= 0)
    {
        v2 = v0;
        if (!(v2 & 127))
        {
            v3 = v2;
            if (!v2)
            {
                g_426188 = 0;
                return;
            }
            v4 = "subsidiary program '%s' could not be invoked";
            if (v3 != 126)
            {
                v4 = "subsidiary program '%s' failed (exit status %d)";
                if (v3 == 127)
                    v4 = "subsidiary program '%s' not found";
            }
        }
        else
        {
            v4 = "subsidiary program '%s' failed";
        }
        error(2, 0, dcgettext(NULL, v4, 5));
    }
    else
    {
        sub_40e770("waitpid"); /* do not return */
    }
}



// Function: lines_differ @ 0xec00
extern unsigned long long g_426498;
extern char g_426552;
extern unsigned int g_426558;

unsigned int lines_differ(void* a0, void* a1)
{
    void* v6;  // r15
    unsigned long long iter;  // r12
    unsigned long v15;  // rdx
    void* v16;  // r9
    unsigned long long iter1;  // r9
    unsigned long v18;  // rax
    void* v19;  // rbp
    unsigned long v20;  // rax
    unsigned long v21;  // rdx
    unsigned short v22;  // r11w
    unsigned long v23;  // r9
    char v24;  // dl
    char v8;  // cl
    void* iter2;  // rsi
    unsigned long long *v26;  // rax
    unsigned long long v27;  // rax
    unsigned long v9;  // rbx
    void* node;  // rbp
    unsigned long v11;  // r11
    unsigned long v12;  // rax
    unsigned long v13;  // rdx
    void* v14;  // r11
    unsigned int v0;  // [bp-0x5c]
    unsigned int v1;  // [bp-0x50]
    char v2;  // [bp-0x49]
    void* v3;  // [bp-0x48]
    void* v4;  // [bp-0x40]

    v6 = a0;
    iter = 0;
    v4 = a1 + 1;
    v0 = g_426558;
    v2 = g_426552;
    v3 = v6 + 1;
    v1 = g_426558 - 2;
    while (1)
    {
        v8 = *((char *)v6);
        v9 = *((char *)a1);
        iter2 = v6 + 1;
        node = a1 + 1;
        if (v8 != (char)v9)
            break;
LABEL_40ed3a:
        a1 = node;
        v6 = iter2;
        if ((char)v9 == 10)
            return 0;
        v27 = 1;
        if ((char)v9 == 9)
            v27 = g_426498 - iter % g_426498;
        iter += v27;
    }
    if (v0 == 4)
    {
        v8 = v8;
        iter2 = iter2;
        v20 = *((long long *)__ctype_b_loc());
        v21 = v8;
        v22 = *((short *)(v20 + (char)v9 * 2)) & 0x2000;
        if (!(*((char *)(v20 + v21 * 2 + 1)) & 32))
        {
            if (!v22)
                goto LABEL_40eec1;
        }
        else
        {
            for (v8 = v21; v8 != 10; iter2 += 1)
            {
                v23 = *((char *)iter2);
                v8 = v23;
                if (!(*((char *)(v20 + v23 * 2 + 1)) & 32))
                {
                    v8 = 32;
                    break;
                }
            }
            if (!v22)
                goto LABEL_40ef65;
        }
        while ((char)v9 != 10)
        {
            v9 = *((char *)node);
            if (*((char *)(v20 + v9 * 2 + 1)) & 32)
            {
                node += 1;
            }
            else
            {
                v9 = 32;
                goto LABEL_40ef65;
            }
        }
LABEL_40ef65:
        if ((char)v9 != v8)
        {
LABEL_40eec1:
            v24 = (char)v9 == 32 & v8 != 10;
            if ((char)v9 == 32 & v8 != 10)
            {
                if (iter2 > v3)
                {
                    v9 = 32;
                    v6 = iter2 - 1;
                    a1 = node;
                    if (*((char *)(v20 + *((char *)iter2 - 2) * 2 + 1)) & 32)
                        continue;
                }
                else
                {
                    v9 = 32;
                }
            }
            else
            {
                v24 = (char)v9 != 10;
            }
            if (v8 == 32 && v24)
            {
                if (node > v4)
                {
                    a1 = node - 1;
                    v6 = iter2;
                    if (*((char *)(v20 + *((char *)node - 2) * 2 + 1)) & 32)
                        continue;
                }
                else
                {
                    v8 = 32;
                }
            }
        }
    }
    else if (v0 <= 4)
    {
        if (v0 == 1)
        {
LABEL_40edf0:
            if (v8 != 32 || (char)v9 != 9)
            {
                if (v8 == 9 && (char)v9 == 32)
                {
                    v8 = 9;
                    goto LABEL_40ee11;
                }
            }
            else
            {
LABEL_40ee11:
                iter1 = iter;
                while (1)
                {
                    switch (v8)
                    {
                    case 32:
                        iter1 += 1;
                        break;
                    case 9:
                        iter1 = iter1 + g_426498 - iter1 % g_426498;
                        break;
                    default:
                        while (1)
                        {
                            if ((char)v9 != 32)
                            {
                                if ((char)v9 != 9)
                                    break;
                                iter = g_426498 + iter - iter % g_426498;
                            }
                            else
                            {
                                iter += 1;
                            }
                            v9 = *((char *)node);
                            node += 1;
                        }
                    }
                    v8 = *((char *)iter2);
                    iter2 += 1;
                }
                if (iter1 != iter)
                    return 1;
            }
        }
        else if (v1 <= 1)
        {
            v8 = v8;
            iter2 = iter2;
            v11 = v8;
            v12 = *((long long *)__ctype_b_loc());
            v8 = v11;
            if (*((char *)(v12 + v11 * 2 + 1)) & 32 && *((char *)(v12 + (char)v9 * 2 + 1)) & 32)
            {
                if (v8 != 10 && !(v13 = (unsigned long)(unsigned long long)(char)(char)v6[1], v14 = iter2, (char)v13 == 10))
                {
                    do
                    {
                        if (!(*((char *)(v12 + v13 * 2 + 1)) & 32))
                            goto LABEL_40ed08;
                    } while ((v13 = (unsigned long)(unsigned long long)(char)(char)v14[1], v14 += 1, (char)v13 != 10));
                }
                else if ((char)v9 == 10)
                {
                    return 0;
                }
                else if ((char)a1[1] == 10)
                {
                    return 0;
                }
                else
                {
                    v16 = node;
                    while (*((char *)(v12 + v15 * 2 + 1)) & 32)
                    {
                        v16 += 1;
                        if ((char)v16[1] == 10)
                            return 0;
                    }
                }
            }
            else
            {
                if (v0 != 2)
                    goto LABEL_40edf0;
            }
        }
    }
    else
    {
        if (v0 == 5)
        {
            v8 = v8;
            iter2 = iter2;
            for (v18 = *((long long *)__ctype_b_loc()); *((char *)(v18 + v8 * 2 + 1)) & 32 && v8 != 10; iter2 += 1)
            {
                v8 = *((char *)iter2);
            }
            v19 = node;
            if (*((char *)(v18 + (char)v9 * 2 + 1)) & 32)
            {
                do
                {
                    node = v19;
                } while ((char)v9 != 10 && (v19 = node + 1, v9 = (unsigned long)(unsigned long long)(char)*((char *)node), node = v19, *((char *)(v18 + v9 * 2 + 1)) & 32));
            }
        }
    }
LABEL_40ed08:
    if (v2)
    {
        v26 = __ctype_tolower_loc();
        v8 = *((char *)(*(v26) + v8 * 4));
        v9 = *((char *)(*(v26) + v9 * 4));
    }
    if (v8 != (char)v9)
        return 1;
    goto LABEL_40ed3a;
}



// Function: print_script @ 0xf080
void print_script(unsigned long long a0)
{
    if (!a0)
        goto LABEL_0x40f0d0;
    else
        goto LABEL_0x40f089;
}



// Function: output_1_line @ 0xf0e0
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern struct_0 *g_426188;
extern unsigned long long g_426498;
extern char g_4264a0;

long long output_1_line(char *a0, char *a1, char *a2, unsigned long long a3)
{
    char *iter;  // r15
    unsigned long v6;  // rbp
    unsigned long long v15;  // rcx
    unsigned long long v16;  // rcx
    unsigned long v17;  // rax
    unsigned long long *v18;  // rax
    unsigned long v19;  // rax
    unsigned long v20;  // rax
    unsigned long v7;  // rax
    unsigned long v8;  // rbx
    unsigned long count;  // rax
    unsigned long v10;  // rax
    unsigned long v11;  // rax
    unsigned long long node;  // r13
    unsigned long long v13;  // r14
    unsigned int v14;  // esi
    char v0;  // [bp-0x58], Other Possible Types: unsigned long long
    unsigned int v1;  // [bp-0x4c]
    unsigned long v2;  // [bp-0x48]
    unsigned long long v3;  // [bp-0x40]

    iter = a0;
    v3 = a3;
    if (!g_4264a0)
    {
        v6 = a1 - a0;
        if (a1 == a0)
            return v7;
        while (1)
        {
            v8 = 0x400;
            if (v6 <= 0x400)
                v8 = v6;
            count = fwrite_unlocked(iter, 1, v8, g_426188);
            if (v8 > count)
                return count;
            v6 -= count;
            iter = &iter[count];
            v10 = (unsigned long long)sub_40e1c0();
            if (!v6)
                return v10;
        }
    }
    else
    {
        v11 = g_426498;
        node = 0;
        v13 = 1;
        v2 = g_426498;
        if (a0 >= a1)
            return g_426498;
        while (1)
        {
            do
            {
                v14 = *(iter);
                iter += 1;
                if ((char)v14 == 9)
                {
LABEL_40f23a:
                    v15 = v2 - node % v2;
                    node += v15;
                    do
                    {
                        v16 = v15;
                        v11 = g_426188[1].field_0;
                        if (v11 < g_426188[1].field_8)
                        {
                            g_426188[1].field_0 = v11 + 1;
                            *((char *)v11) = 32;
                        }
                        else
                        {
                            v0 = v16;
                            v11 = __overflow(g_426188, 32);
                            v16 = v0;
                        }
                    } while ((v15 = v16 - 1, v16 != 1));
                    goto LABEL_40f20a;
                }
LABEL_40f1b2:
                if ((char)v14 != 13)
                {
                    if ((char)v14 != 8)
                    {
                        v1 = v14;
                        v0 = v14;
                        v18 = __ctype_b_loc();
                        v19 = v0;
                        v20 = *((short *)(*(v18) + v19 * 2));
                        node += _INSERT(v20, 0, (unsigned short)v20 >> 14) & 1;
                        v11 = g_426188[1].field_0;
                        if (v11 < g_426188[1].field_8)
                        {
                            g_426188[1].field_0 = v11 + 1;
                            *((char *)v11) = v19;
                            goto LABEL_40f20a;
                        }
                    }
                    else
                    {
                        if (!node)
                            goto LABEL_40f300;
                        v11 = g_426188[1].field_0;
                        node -= 1;
                        if (v11 < g_426188[1].field_8)
                        {
                            g_426188[1].field_0 = v11 + 1;
                            *((char *)v11) = 8;
                            goto LABEL_40f20a;
                        }
                    }
                    v11 = __overflow(g_426188);
LABEL_40f20a:
                    if (a1 == iter)
                        return v11;
                }
                else
                {
                    v11 = g_426188[1].field_0;
                    if (v11 < g_426188[1].field_8)
                    {
                        g_426188[1].field_0 = v11 + 1;
                        *((char *)v11) = 13;
                    }
                    else
                    {
                        v11 = __overflow(g_426188);
                    }
                    if (a2 && a1 > iter)
                    {
                        node = 0;
                        if (*(iter) != 10)
                        {
                            __fprintf_chk(*((unsigned int *)&g_426188), 0x1, a2);
                            continue;
                        }
                    }
LABEL_40f300:
                    node = 0;
                    goto LABEL_40f20a;
                }
            } while ((v13 += 1, v11 = v17, v13 != 0x400));
            v11 = (unsigned long long)sub_40e1c0();
            v14 = *(iter);
            iter += 1;
            v13 = 0;
            if ((char)v14 == 9)
                goto LABEL_40f23a;
            else
                goto LABEL_40f1b2;
        }
    }
}



// Function: print_1_line_nl @ 0xf330
typedef struct struct_0 {
    struct struct_1 *field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

extern unsigned int g_426010;
extern struct_0 g_426020;
extern struct_0 g_426030;
extern struct_0 g_426050;
extern unsigned long long g_426188;
extern char g_426491;
extern char g_426492;
extern char g_426758;

long long print_1_line_nl(char *a0, struct_0 *a1, char a2)
{
    char *v1;  // r14
    void* v2;  // r12
    char *v3;  // r13
    char *v4;  // r13
    char *v5;  // rdx
    long long v6;  // rax
    char *v7;  // rax

    v1 = &a1->field_0->field_0;
    v2 = a1->field_8;
    if (a0)
    {
        v3 = NULL;
        if (*(a0))
        {
            v3 = "%s ";
            v4 = "%s\t";
            if (g_426492)
                v3 = v4;
            v5 = v3;
            if (g_426491)
            {
                v5 = v3;
                if (*(v1) == 10)
                    v5 = "%s";
            }
            __fprintf_chk(g_426188, 0x1, v5);
        }
        if (!a2)
        {
            v6 = sub_40f0e0(v1, v2, v3, a0);
            if (!*(a0))
                return v6;
        }
        else
        {
            v6 = sub_40f0e0(v1, v2 - (*((char *)v2 - 1) == 10), v3, a0);
            if (!*(a0))
                return v6;
        }
    }
    else
    {
        v6 = (!a2 ? sub_40f0e0(v1, v2, NULL, 0) : sub_40f0e0(v1, v2 - (*((char *)v2 - 1) == 10), NULL, 0));
    }
    if (*((char *)v2 - 1) == 10)
        return v6;
    if (g_426758 && g_426010 != 3)
    {
        sub_40e180(&g_426020.field_0);
        sub_40e180(&g_426050.field_0);
        sub_40e180(&g_426030.field_0);
        g_426010 = 3;
    }
    v7 = dcgettext(NULL, "No newline at end of file", 5);
    return __fprintf_chk(g_426188, 0x1, "\n\\ %s\n", v7);
}



// Function: print_1_line @ 0xf500
typedef struct struct_0 {
    struct struct_1 *field_0;
    void* field_8;
} struct_0;

typedef struct struct_1 {
    char field_0;
} struct_1;

long long print_1_line(char *a0, struct_0 *a1)
{
    return sub_40f330(a0, a1, 0);
}



// Function: print_number_range @ 0xf610
typedef struct struct_0 {
    char padding_0[232];
    unsigned long long field_e8;
} struct_0;

extern unsigned long long g_426188;

int print_number_range(char a0, struct_0 *idx, unsigned long a2, unsigned long a3)
{
    long long v1;  // rcx
    long long v2;  // r9

    v1 = idx->field_e8 + a2 + 1;
    v2 = idx->field_e8 + a3 + 1;
    if (v1 >= v2)
        return __fprintf_chk(g_426188, 0x1, "%ld", v2);
    return __fprintf_chk(g_426188, 0x1, "%ld%c%ld", v1, a0, v2);
}



// Function: analyze_hunk @ 0xf670
typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
    unsigned long long field_10;
    unsigned long long field_18;
    unsigned long long field_20;
} struct_0;

extern unsigned long long g_426258;
extern unsigned long long g_426388;
extern long long g_4264c0;
extern unsigned long long g_4264e0;
extern char g_426554;
extern unsigned int g_426558;

unsigned int analyze_hunk(struct_0 *index, unsigned long long *a1, unsigned long long *a2, unsigned long long *a3, unsigned long long *a4)
{
    char v13;  // al
    unsigned int v14;  // ebp
    unsigned long v23;  // r8
    unsigned long long *v24;  // rax
    unsigned long v25;  // rdx
    unsigned long v26;  // r12
    char *v27;  // rsi
    unsigned long long v28;  // r13
    unsigned long v29;  // r8
    unsigned long long *v30;  // rax
    unsigned long v31;  // rdx
    unsigned int v32;  // r8d
    unsigned long long v15;  // r14
    struct_0 *idx;  // r12
    unsigned long idx1;  // rbx
    unsigned long idx2;  // r15
    unsigned long long v19;  // r12
    struct_0 *v20;  // r14
    char *v21;  // rsi
    unsigned long long v22;  // r13
    unsigned long v0;  // [bp-0x98]
    unsigned long v1;  // [bp-0x90]
    unsigned long v2;  // [bp-0x88]
    char *iter;  // [bp-0x80]
    struct_0 *v4;  // [bp-0x78], Other Possible Types: char
    char v5;  // [bp-0x6a]
    char v6;  // [bp-0x69]
    unsigned int v7;  // [bp-0x68]
    int v8;  // [bp-0x64]
    unsigned long long v9;  // [bp-0x60]
    unsigned long long v10;  // [bp-0x58]
    unsigned long v11;  // [bp-0x50]

    v13 = g_426554;
    if (!g_426554)
    {
        v8 = -0x1;
        v7 = 0;
        v13 = g_4264e0;
        v14 = 0;
    }
    else if (g_426558 > 1)
    {
        v8 = 0;
        v14 = 1;
        v7 = 3 < g_426558;
    }
    else
    {
        v8 = 0;
        v14 = 0;
        v7 = 0;
    }
    v10 = 0;
    v9 = 0;
    v15 = g_426258;
    *(a1) = index->field_18;
    v11 = g_426388;
    idx = index;
    *(a3) = index->field_20;
    do
    {
        idx1 = idx->field_18;
        idx2 = idx->field_20;
        v10 += idx->field_8;
        v9 += idx->field_10;
        v1 = idx->field_10 + idx1 - 1;
        v0 = idx2 + idx->field_8 - 1;
        v4 = idx1 <= v1 & v13;
        if (idx1 <= v1 & v13)
        {
            v19 = v15;
            v20 = idx;
            do
            {
                v21 = *((long long *)(v19 + idx1 * 8));
                v22 = *((long long *)(v19 + idx1 * 8 + 8)) + (*((char *)(*((long long *)(v19 + idx1 * 8 + 8)) - 1)) != 10) - 1;
                v23 = v22 - v21;
                if (v14)
                {
                    v5 = *(v21);
                    if (*(v21) != 10)
                    {
                        iter = v21;
                        v2 = v23;
                        v24 = __ctype_b_loc();
                        v25 = v5;
                        do
                        {
                            if (!(*((char *)(*(v24) + v25 * 2 + 1)) & 32))
                            {
                                v23 = v22 - iter;
                                if (!v7)
                                    v23 = v2;
                                goto LABEL_40f7b2;
                            }
                        } while ((v25 = (unsigned long)(unsigned long long)iter[1], iter += 1, (char)v25 != 10));
                        v23 = v22 - iter;
                    }
                }
LABEL_40f7b2:
                if (v8 == v23)
                {
                    v13 = v4;
                }
                else if (g_4264e0)
                {
                    v13 = ~((unsigned int)re_search(&g_4264c0)) >> 31;
                }
                else
                {
                    v15 = v19;
                    idx = v20;
                    v13 = 0;
                }
            } while ((idx1 = (unsigned long)(idx1 + 1), v1 >= idx1 && v13));
            idx = v20;
            v15 = v19;
            idx2 = idx->field_20;
            v6 = idx2 <= v0 & v13;
        }
        else
        {
            v6 = idx2 <= v0 & v13;
        }
        if (v6)
        {
            v4 = idx;
            v26 = v11;
            do
            {
                v27 = *((long long *)(v26 + idx2 * 8));
                v28 = *((long long *)(v26 + idx2 * 8 + 8)) + (*((char *)(*((long long *)(v26 + idx2 * 8 + 8)) - 1)) != 10) - 1;
                v29 = v28 - v27;
                if (v14)
                {
                    v5 = *(v27);
                    if (*(v27) != 10)
                    {
                        iter = v27;
                        v2 = v29;
                        v30 = __ctype_b_loc();
                        v31 = v5;
                        do
                        {
                            if (!(*((char *)(*(v30) + v31 * 2 + 1)) & 32))
                            {
                                v29 = v28 - iter;
                                if (!v7)
                                    v29 = v2;
                                goto LABEL_40f935;
                            }
                        } while ((v31 = (unsigned long)(unsigned long long)iter[1], iter += 1, (char)v31 != 10));
                        v29 = v28 - iter;
                    }
                }
LABEL_40f935:
                if (v8 == v29)
                {
                    v13 = v6;
                }
                else if (g_4264e0)
                {
                    v13 = ~((unsigned int)re_search(&g_4264c0)) >> 31;
                }
                else
                {
                    idx = v4;
                    v13 = 0;
                }
            } while ((idx2 = (unsigned long)(idx2 + 1), v0 >= idx2 && v13));
            idx = v4;
        }
    } while ((idx = (struct_0 *)idx->field_0, idx));
    v32 = 0;
    *(a2) = v1;
    *(a4) = v0;
    if (!v13)
        v32 = (v10) * 2 | v9;
    return v32;
}



// Function: concat @ 0xfa30
char * concat(char *a0, char *a1, char *a2)
{
    unsigned long len;  // rax
    unsigned long len1;  // rax
    char *v4;  // rax
    char *v0;  // [bp-0x38]

    len = strlen(a0);
    len1 = strlen(a1);
    v4 = sub_416a40(len + len1 + strlen(a2) + 1);
    v0 = a2;
    __sprintf_chk(v4, 0x1);
    return v4;
}



// Function: zalloc @ 0xfab0
void* zalloc(unsigned long a0)
{
    return memset(sub_416a40(), 0, a0);
}



// Function: debug_script @ 0xfad0
typedef struct FILE {
} FILE;

typedef struct struct_0 {
    struct struct_0 *field_0;
    unsigned long long field_8;
} struct_0;

extern FILE *stdout;
extern FILE *stderr;

int debug_script(struct_0 *a0)
{
    struct_0 *v2;  // rbx
    struct_0 *v3;  // rbx
    unsigned long v0;  // [bp-0x28]

    v2 = a0;
    fflush_unlocked(stdout);
    if (v2)
    {
        do
        {
            v3 = v2;
            v0 = v3->field_8;
            __fprintf_chk(*((unsigned int *)&stderr), 0x1, "%3ld %3ld delete %ld insert %ld\n");
            v2 = v3->field_0;
        } while (v3->field_0);
    }
    return fflush_unlocked(stderr);
}


