struct near { char pad[0xff8]; unsigned int id; unsigned int window; };
struct page { char pad[0x1000]; unsigned int id; unsigned int window; };
struct conn { char pad[0x48a0]; unsigned int id; unsigned int window; unsigned char blocked; };

char table_data[256] = {1, 2, 3};
char table_bss[256];

int near_leaf(struct near *c, unsigned int v)
{
    c->window += v;
    return c->id == v;
}

int page_leaf(struct page *c, unsigned int v)
{
    c->window += v;
    return c->id == v;
}

int conn_leaf(struct conn *c, unsigned int v)
{
    c->window += v;
    if (c->blocked == 1)
        c->blocked = 0;
    return c->id == v;
}

int get_data(long i) { return table_data[i]; }

int put_bss(long i, char v)
{
    table_bss[i] = v;
    return 0;
}

static const char code_table[256] __attribute__((section(".text"))) = {4, 5, 6};
static char local_table[256];

int get_code(long i) { return code_table[i]; }

int get_local(long i)
{
    local_table[i]++;
    return local_table[i + 1];
}
