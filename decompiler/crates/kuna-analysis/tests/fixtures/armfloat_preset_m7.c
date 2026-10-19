/* Cortex-M7 shapes for the armfloat preset (tests/stages/kuna-armfloat-preset.xml).
   arm-none-eabi-gcc -mcpu=cortex-m7 -mthumb -mfpu=fpv5-d16 -mfloat-abi=hard -O2 -c armfloat_preset_m7.c -o armfloat_preset_m7.o */
#define NI __attribute__((noinline))
NI int first(int x) { return x * 3 + 1; }
NI unsigned neg_trunc(float x) { return (unsigned)(0.0 - x); }
NI long long to_ticks(int base, float scale, int n) { double d = (double)n * 1.5; return (long long)d + (int)scale + base; }
