#include <stdbool.h>
volatile unsigned observed;
void barrier(void) { ++observed; }
bool observe(int value) { ++observed; return (value & 1) != 0; }
