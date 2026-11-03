#include <stdint.h>
#include <stdbool.h>
struct Arrays { int prefix; int values[12]; long *pointer; };
__attribute__((noinline)) int array_field(struct Arrays*p,long i){return p->values[i];}
__attribute__((noinline)) long pointer_field(struct Arrays*p,long i){return p->pointer[i];}
__attribute__((noinline)) int nested_array(int(*p)[12],long i,long j){return p[i][j];}
__attribute__((noinline)) double float_load(double*p,long i){return p[i];}
__attribute__((noinline)) void float_store(double*p,long i,double value){p[i]=value;}
__attribute__((noinline)) int*pointer_load(int**p,long i){return p[i];}
__attribute__((noinline)) void pointer_store(int**p,long i,int*value){p[i]=value;}
__attribute__((noinline)) bool bool_load(bool*p,long i){return p[i];}
__attribute__((noinline)) void bool_store(bool*p,long i,bool value){p[i]=value;}
int main(void){return 0;}
