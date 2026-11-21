struct st { int x; int y; };
int get(struct st *p) { return p->y * 3 + (p->y >> 2) + 1; }
