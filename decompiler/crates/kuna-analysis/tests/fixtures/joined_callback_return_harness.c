#include <stdio.h>
#include <inttypes.h>
static int64_t scalar_op(int64_t a,int64_t b) { ++calls; return a^b; }
static void void_op(int64_t a,int64_t b) { (void)a; (void)b; ++calls; }
int main(void) {
  const int64_t values[] = {INT64_MIN,INT64_MIN+1,-4294967296LL,-65536,-17,-1,0,1,17,65536,4294967295LL,INT64_MAX-1,INT64_MAX};
  for (unsigned i=0;i<sizeof(values)/sizeof(*values);++i) {
    for (unsigned j=0;j<sizeof(values)/sizeof(*values);++j) {
      int64_t a=values[i],b=values[j];
      calls=0; int64_t x=first(make_pair,a,b); if(calls!=1)return 1;
      calls=0; int64_t y=second(make_pair,a,b); if(calls!=1)return 2;
      calls=0; int64_t z=both(make_pair,a,b); if(calls!=1)return 3;
      calls=0; Pair p=forward(make_pair,a,b); if(calls!=1)return 4;
      calls=0; int32_t n=narrow_second(make_pair,a,b); if(calls!=1)return 5;
      calls=0; int64_t s=scalar(scalar_op,a,b); if(calls!=1)return 6;
      calls=0; invoke_void(void_op,a,b); if(calls!=1)return 7;
      printf("%" PRIx64 " %" PRIx64 " %" PRIx64 " %" PRIx64 " %" PRIx64 " %" PRIx32 " %" PRIx64 "\n",(uint64_t)x,(uint64_t)y,(uint64_t)z,(uint64_t)p.first,(uint64_t)p.second,(uint32_t)n,(uint64_t)s);
    }
  }
  return 0;
}
