struct st { float f; int z; };
int get(struct st *p) { return p->z * 3 + (p->z >> 2) + 1; }
int other(struct st *p, int k) { return p->z * k + (int)(p->f * 7.0f) - k; }
