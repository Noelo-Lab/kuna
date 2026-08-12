int g_counter;
extern int ext_helper(int);

int alpha(int x) { return x * 3 + 1; }
int beta(int x) { return alpha(x) + g_counter; }
int gamma_(int x) { return ext_helper(x) + beta(x); }
