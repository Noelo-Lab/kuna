#define main fixture_main
#include "constant.c"
#undef main
#include <stdio.h>
int main(void){int32_t a[9]={0};uint8_t*b[9]={0};printf("%td %td %td %td\n",(char*)integer_eight(&a)-(char*)a,(char*)integer_end(&a)-(char*)a,(char*)pointer_eight(&b)-(char*)b,(char*)pointer_end(&b)-(char*)b);}
