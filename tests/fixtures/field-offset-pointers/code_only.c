struct page { char pad[0x1000]; unsigned int id; unsigned int window; };

int update(struct page *c, unsigned int v)
{
    c->window += v;
    return c->id == v;
}
