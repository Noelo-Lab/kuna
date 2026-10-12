#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

typedef struct Node { int value; } Node;
typedef struct Items {
    unsigned int count;
    unsigned int pad;
    unsigned long reserved;
    unsigned long *items;
} Items;

void cursor_walk(Items *array, int branch, void (*callback)(Node *));
static Items *current;
static Node *nodes;
static int visits;
static int mutate_count;

static void visit(Node *item) {
    printf(" %td:%d", item - nodes, item->value);
    item->value += 10;
    visits++;
    if (mutate_count) current->count = 0;
}

int main(void) {
    for (int branch = 0; branch < 2; branch++) {
        for (int mutation = 0; mutation < 2; mutation++) {
            for (int count = 0; count <= 3; count++) {
                for (int hole = -1; hole < 3; hole++) {
                    Node values[3] = {{20}, {30}, {40}};
                    unsigned long addresses[3] = {
                        (uintptr_t)&values[0], (uintptr_t)&values[1], (uintptr_t)&values[2]
                    };
                    if (hole >= 0) addresses[hole] = 0;
                    Items array = {(unsigned)count, 0, 0, count ? addresses : (unsigned long *)1};
                    nodes = values;
                    current = &array;
                    visits = 0;
                    mutate_count = mutation;
                    printf("%d,%d,%d,%d", branch, mutation, count, hole);
                    cursor_walk(&array, branch, visit);
                    printf(" =%d,%u,%d,%d,%d\n", visits, array.count,
                           values[0].value, values[1].value, values[2].value);
                }
            }
        }
    }
    visits = 0;
    cursor_walk(NULL, 0, visit);
    cursor_walk(NULL, 1, visit);
    printf("null=%d\n", visits);
    return 0;
}
