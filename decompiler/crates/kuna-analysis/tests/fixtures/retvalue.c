#include <stdio.h>
#include <stdlib.h>

struct cell { struct cell *next; int key; };
struct cell *free_list, *table[16];
int gi, gj;

__attribute__((noinline)) struct cell *make_cell(int key)
{
    struct cell *c = malloc(sizeof *c);
    if (c) {
        c->next = free_list;
        c->key = key;
    }
    return c;
}

__attribute__((noinline)) char *skip_blanks(char *p)
{
    while (*p == ' ')
        p++;
    return p;
}

__attribute__((noinline)) struct cell *lookup(int key)
{
    struct cell *c = table[key & 15];
    while (c && c->key != key)
        c = c->next;
    return c;
}

__attribute__((noinline)) int reset(void)
{
    gi = 0;
    gj = 0;
    return 0;
}

__attribute__((noinline)) struct cell *push_cell(int key)
{
    struct cell *c = malloc(sizeof *c);
    if (c) {
        c->next = free_list;
        c->key = key;
        free_list = c;
    }
    return c;
}

int main(int argc, char **argv)
{
    struct cell *c = make_cell(argc);
    char *s = skip_blanks(argv[0]);
    reset();
    push_cell(argc);
    struct cell *l = lookup(argc);
    printf("%d %c %p\n", c ? c->key : 0, *s, (void *)l);
    return 0;
}
