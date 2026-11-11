int audit_target(int v) { return v + 11; }
static int local_target(int v) { return v * 2; }
int (*exported_tab[])(int) = { audit_target, local_target };
const char *msg = "hello";
int call_it(int i) { return exported_tab[i](i); }
