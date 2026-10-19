#include <stdint.h>
#include <string.h>
extern float sample(int);
#ifdef REGENERATED
float values[3];
#endif
int main(void) {
  const uint32_t words[3] = {0x42640000,0x40414243,0x42700000};
#ifdef REGENERATED
  memcpy(values,words,sizeof words);
#endif
  for (int i=0;i<4096;++i) {
    int choice=i&1;
    float value=sample(choice);
    uint32_t actual;
    memcpy(&actual,&value,sizeof actual);
    if (actual!=words[choice]) return 1;
  }
  return 0;
}
