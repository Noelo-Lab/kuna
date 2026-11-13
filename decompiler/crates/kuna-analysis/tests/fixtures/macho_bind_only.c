// A chained-fixup Mach-O whose only fixup is a BIND: `p` points at an undefined
// symbol resolved by dynamic lookup, and nothing else in the image holds a pointer.
extern int ext;
void *p = &ext;
int main(void) { return 0; }
