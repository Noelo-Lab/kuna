struct view { unsigned char preceding[24]; unsigned int word; };
extern void *lookup(void *ctx, unsigned int idx);
extern void *get_mem(unsigned long size);
extern void use(struct view *v);

__attribute__((noinline)) struct view *alloc_node(unsigned int idx, void *ctx)
{
    struct view *v = get_mem(sizeof *v);
    if (v) { v->word = idx; *(void **)v->preceding = ctx; }
    return v;
}

__attribute__((noinline)) struct view *alloc_sub(void *owner, unsigned int a, unsigned int b)
{
    struct view *v = get_mem(sizeof *v);
    if (v) { v->word = a + b; *(void **)v->preceding = owner; }
    return v;
}

int add_node(void *ctx, unsigned int idx)
{
    if (lookup(ctx, idx)) return 0;
    struct view *n = alloc_node(idx, ctx);
    if (!n) return 1;
    n->word = 3;
    return 0;
}

int add_sub(void *ctx, unsigned int idx, void *owner, unsigned int a)
{
    if (lookup(ctx, idx)) return 0;
    struct view *s = alloc_sub(owner, a, idx);
    if (!s) return 1;
    use(s);
    return 0;
}

int add_default(void *ctx, unsigned int idx)
{
    struct view *z = alloc_node(0, ctx);
    if (!z) return 1;
    if (lookup(ctx, idx)) return 0;
    struct view *n = alloc_node(idx, ctx);
    if (!n) return 1;
    use(z); use(n);
    return 0;
}
