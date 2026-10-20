/* A base-standard function a hard-float Cortex-M4F image computes its doubles in
   through the AEABI helpers (tests/stages/kuna-armfloat-preset.xml).
   clang --target=thumbv7em-none-eabihf -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=hard -O2 -c armfloat_preset_m4f.c -o armfloat_preset_m4f.o */
__attribute__((noinline, pcs("aapcs"))) long long fixit(double a) { if (a < 0.0) return -(long long)(-a); return (long long)a; }
