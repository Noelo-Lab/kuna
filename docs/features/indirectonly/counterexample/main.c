#include <stdio.h>
void walk(void **items, int count);
void sink(void *p){ printf("sink %p\n", p); }
int main(void){ void *it[3] = {(void*)1,(void*)2,(void*)3}; walk(it,3); return 0; }
