extern unsigned observed;
#ifdef REGENERATED
extern float _sample(float, unsigned);
#else
extern float _sample(void);
#endif
int validation_main(void) {
 unsigned state=0x12345678;
 for(unsigned i=0;i<1024;++i) {
  state=state*1664525u+1013904223u;
  /* Use finite values so NaN quieting cannot obscure storage errors. */
  union {unsigned bits; float value;} input={state&0xfeffffffu},output;
  unsigned control=state^0xa531769bu;
#ifdef REGENERATED
  output.value=_sample(input.value,control);
#else
  __asm__ volatile("pushl %1; call _sample; addl $4, %%esp"
   : "=t"(output.value) : "m"(input.value),"D"(control)
   : "eax","ecx","edx","cc","memory");
#endif
  unsigned record[4]={input.bits,control,output.bits,observed};
  int written;
  __asm__ volatile("int $0x80" : "=a"(written)
   : "0"(4),"b"(1),"c"(record),"d"(sizeof record) : "cc","memory");
  if(written!=sizeof record||output.bits!=input.bits||observed!=control) return 1;
 }
 return 0;
}
