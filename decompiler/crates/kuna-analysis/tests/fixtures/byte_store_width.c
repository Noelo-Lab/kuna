struct node { unsigned char preceding[24]; unsigned int word; };
extern void *get_mem(unsigned long size);
static int count;

__attribute__((noinline)) struct node *alloc_node(void) { return get_mem(sizeof(struct node)); }

int fill(void)
{
    struct node *n = alloc_node();
    if (!n) return 1;
    if (!n->word) count++;
    n->word = 3;
    return 0;
}

int fill_text(void)
{
    struct node *n = alloc_node();
    if (!n) return 1;
    __builtin_memcpy(&n->word, "abc", 4);
    return 0;
}
