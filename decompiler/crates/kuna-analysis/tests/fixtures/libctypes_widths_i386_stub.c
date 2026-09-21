struct timespec;
struct statfs;
struct obstack;
int clock_getres(int c, struct timespec *t) { (void)c; (void)t; return 0; }
int statfs(const char *p, struct statfs *b) { (void)p; (void)b; return 0; }
int _obstack_begin(struct obstack *h, int s, int a, void *(*f)(long), void (*g)(void *))
{ (void)h; (void)s; (void)a; (void)f; (void)g; return 1; }
