/* devirt_frame_x86_64: a Tigress-style in-function interpreter (the
   instruction set of devirt_interp_x86_64.c's tg_indexed) whose program reads
   the frame through a computed offset next to a direct write of a slot the
   read may address.

   tg_between   a direct write of st[2] between two indexed reads in a loop:

                    st[0..3] = a, a + 1, a + 2, a + 3;
                    s = 0;
                    for (i = 0; i < 3; i++) {
                        s += *(long *)((char *)&st[0] + i * 8);
                        st[2] = s;
                        s += *(long *)((char *)&st[0] + i * 8);
                    }
                    return s;

   `./devirt_frame_x86_64 A...` prints the result for a = argc (20 for a = 1,
   90 for a = 6).

   Build (gcc 11, Ubuntu 22.04 defaults):
       gcc -O0 -o devirt_frame_x86_64 devirt_frame_x86_64.c */
#include <stdio.h>

enum { T_ARG = 1, T_CONST, T_LOCAL, T_LOAD, T_STORE, T_ADD, T_MUL, T_OR, T_LT, T_INDEX, T_BRF, T_GOTO, T_RET };

/* st[0..3] at locals + 16..47, s at locals + 0, i at locals + 8. */
#define INIT                                                                  \
    T_ARG, T_LOCAL, 16, T_STORE,                                              \
    T_ARG, T_CONST, 1, 0, 0, 0, T_ADD, T_LOCAL, 24, T_STORE,                  \
    T_ARG, T_CONST, 2, 0, 0, 0, T_ADD, T_LOCAL, 32, T_STORE,                  \
    T_ARG, T_CONST, 3, 0, 0, 0, T_ADD, T_LOCAL, 40, T_STORE
#define INDEXED T_LOCAL, 16, T_LOCAL, 8, T_LOAD, T_CONST, 8, 0, 0, 0, T_MUL, T_INDEX, T_LOAD

unsigned char bt_code[] = {
    INIT,
    T_CONST, 0, 0, 0, 0, T_LOCAL, 0, T_STORE,
    T_CONST, 0, 0, 0, 0, T_LOCAL, 8, T_STORE,
    /* L: */ T_LOCAL, 8, T_LOAD, T_CONST, 3, 0, 0, 0, T_LT, T_BRF, 60,
    T_LOCAL, 0, T_LOAD, INDEXED, T_ADD, T_LOCAL, 0, T_STORE,
    T_LOCAL, 0, T_LOAD, T_LOCAL, 32, T_STORE,
    T_LOCAL, 0, T_LOAD, INDEXED, T_ADD, T_LOCAL, 0, T_STORE,
    T_LOCAL, 8, T_LOAD, T_CONST, 1, 0, 0, 0, T_ADD, T_LOCAL, 8, T_STORE,
    T_GOTO, (unsigned char)-71,
    /* E: */ T_LOCAL, 0, T_LOAD, T_RET,
};

union tg_node { long l; void *p; };

#define INTERPRETER(code)                                                                      \
    char locals[64];                                                                           \
    union tg_node stack[16], *sp = stack;                                                      \
    unsigned char *pc = code;                                                                  \
    for (;;)                                                                                   \
        switch (*pc) {                                                                         \
        case T_ARG: (sp + 1)->l = a; sp++; pc += 1; break;                                     \
        case T_CONST: (sp + 1)->l = *(int *)(pc + 1); sp++; pc += 5; break;                    \
        case T_LOCAL: (sp + 1)->p = locals + pc[1]; sp++; pc += 2; break;                      \
        case T_LOAD: sp->l = *(long *)sp->p; pc += 1; break;                                   \
        case T_STORE: *(long *)sp->p = (sp - 1)->l; sp -= 2; pc += 1; break;                   \
        case T_ADD: (sp - 1)->l = (sp - 1)->l + sp->l; sp--; pc += 1; break;                   \
        case T_MUL: (sp - 1)->l = (sp - 1)->l * sp->l; sp--; pc += 1; break;                   \
        case T_OR: (sp - 1)->l = (sp - 1)->l | sp->l; sp--; pc += 1; break;                    \
        case T_LT: (sp - 1)->l = (sp - 1)->l < sp->l; sp--; pc += 1; break;                    \
        case T_INDEX: (sp - 1)->p = (char *)(sp - 1)->p + sp->l; sp--; pc += 1; break;         \
        case T_BRF: pc += 2; if (!sp->l) pc += (signed char)pc[-1]; sp--; break;               \
        case T_GOTO: pc += 2 + (signed char)pc[1]; break;                                      \
        case T_RET: return sp->l;                                                              \
        }

long tg_between(long a) { INTERPRETER(bt_code) }

int main(int argc, char **argv)
{
    long a = argc;
    printf("%ld\n", tg_between(a));
    return 0;
}
