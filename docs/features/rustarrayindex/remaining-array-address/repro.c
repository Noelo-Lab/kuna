#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#define OMIT_MAIN
#include "/home/mahaloz/kwt/angr-rust-array-index/decompiler/crates/kuna-analysis/tests/fixtures/array_index.c"
int main(int argc,char**argv){if(argc!=2)return 2;struct ArrayFields value={0};struct ArrayFields*p=&value;long i=strtol(argv[1],0,10);printf("%lu\n",(unsigned long)((uintptr_t)array_address_s(p,i)-(uintptr_t)p));return 0;}
