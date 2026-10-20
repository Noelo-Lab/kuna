extern float sample(float,float);
extern volatile float seen;
int validation_main(void) {
 for(int i=0;i<1024;++i) {
  float a=(float)(i-512)/16.0f,b=(float)i/32.0f;
  if(sample(a,b)!=2.75f || seen!=a+b) return 1;
 }
 return 0;
}
