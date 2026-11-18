__declspec(noinline) unsigned observe(unsigned char *p, unsigned n){ return p[0]+n; }
__declspec(noinline) unsigned sample_one(unsigned n){ unsigned char *p=(unsigned char*)__builtin_alloca(n); p[0]=7; return observe(p,n); }
int mainCRTStartup(void){ return sample_one(3); }
