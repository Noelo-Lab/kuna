int first(int a, int b) { return a + b; }
void second(int *p) { *p = 3; }
__asm__(".symver first,thing@VER_1");
__asm__(".symver second,thing@@VER_2");
void notify_pair(int a, int b) { (void)a; (void)b; }
__asm__(".symver notify_pair,notify@VER_1");
void pair(int a, int b) { (void)a; (void)b; }
__asm__(".symver pair,one_version@@VER_1");
