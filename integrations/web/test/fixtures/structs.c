/* A linked list for the type definitions shown above a function: built without
 * debug info, so the decompiler works out the layout of struct item itself.
 *
 * Rebuild:  gcc -O0 -o structs.elf structs.c
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct item { int id; int count; long price; char name[16]; struct item *next; };

struct item *make_item(int id, const char *name, long price) {
    struct item *it = malloc(sizeof *it);
    it->id = id;
    it->count = 1;
    it->price = price;
    strncpy(it->name, name, sizeof it->name - 1);
    it->name[15] = 0;
    it->next = NULL;
    return it;
}

long total(struct item *list) {
    long sum = 0;
    for (struct item *it = list; it; it = it->next)
        sum += it->price * it->count;
    return sum;
}

int main(int argc, char **argv) {
    struct item *a = make_item(1, "apple", 3);
    a->next = make_item(2, "pear", 5);
    a->next->count = argc;
    printf("%ld\n", total(a));
    return 0;
}
