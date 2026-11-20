typedef long long i64;
void ext(void);
void keep(int x) { int y = x; ext(); i64 t = y; (void)t; }
int drop(int x) { volatile i64 t = x; ext(); (void)t; return 0; }
