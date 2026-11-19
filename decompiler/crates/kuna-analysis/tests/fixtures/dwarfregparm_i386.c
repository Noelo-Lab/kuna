
__attribute__((noinline, regparm(3))) int combine(int a, int b, int c) { return a * 7 + b * 3 - c; }
int main(int argc, char **argv) { return combine(argc, argc + 1, (int)(long)argv); }
