struct bits { unsigned lo : 3, hi : 5, wide : 16, top : 8; };
struct three { int head; unsigned f1 : 4, f2 : 12, f3 : 16; };

unsigned get_wide(struct bits *p) { return p->wide; }
unsigned get_top(struct bits *p) { return p->top; }
void set_hi(struct bits *p, unsigned v) { p->hi = v; }
unsigned sum3(struct three *t) { return t->f1 + t->f2 + t->f3; }
