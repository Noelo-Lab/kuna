#include <unistd.h>
extern unsigned long call_write(unsigned long, unsigned long, unsigned long, unsigned long);
int main(void) { return call_write(1, 2, 3, 4) + write(1, "", 0); }
