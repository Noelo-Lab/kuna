extern float sample(float,float);
int validation_main(void) {
 for(int i=0;i<1024;++i) {
  float a=(float)(i-512)/16.0f,b=(float)i/32.0f;
  float first=a+3.25f,second=b-0.5f;
  if(sample(a,b)!=first+second) return 1;
 }
 return 0;
}
