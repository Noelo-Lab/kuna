
/* Startup in the shape of a vendor HAL: SystemInit, NVIC and SysTick setup,
   written against the fixed System Control Space addresses.
   clang --target=thumbv7m-none-eabi -mcpu=cortex-m3 -Os -ffreestanding -nostdlib -fno-inline \
     -fuse-ld=lld -Wl,-T,cortexm3_raw.ld cortexm3_scs.c -o cortexm3_scs.elf
   llvm-objcopy -O binary cortexm3_scs.elf cortexm3_scs.bin */
typedef unsigned int u32;
typedef unsigned char u8;
#define REG(a) (*(volatile u32 *)(a))
#define REG8(a) (*(volatile u8 *)(a))
volatile u32 ticks;
void Default_Handler(void) { while (1) {} }
void SysTick_Handler(void) { ticks++; }
void SystemInit(void) { REG(0xE000ED88) |= 0xFu << 20; REG(0xE000ED08) = 0x08000000; }
void nvic_enable(u32 irq) { REG(0xE000E100 + 4 * (irq >> 5)) = 1u << (irq & 31); }
void nvic_priority(u32 irq, u32 prio) { REG8(0xE000E400 + irq) = prio << 4; }
u32 systick_config(u32 t) {
  if (t - 1 > 0xFFFFFF) return 1;
  REG(0xE000E014) = t - 1; REG8(0xE000ED23) = 0xF0; REG(0xE000E018) = 0; REG(0xE000E010) = 7;
  return 0;
}
void system_reset(void) { REG(0xE000ED0C) = 0x05FA0004; for (;;) {} }
void trace_enable(void) { REG(0xE000EDFC) |= 1u << 24; REG(0xE0001004) = 0; REG(0xE0001000) |= 1; REG(0xE000EF00) = 37; }
int main(void) {
  systick_config(72000); nvic_priority(37, 5); nvic_enable(37); REG(0xE000E100) = 1u << 6; trace_enable();
  for (;;) { if (ticks > 1000) system_reset(); }
}
void Reset_Handler(void) { SystemInit(); main(); }
__attribute__((section(".isr_vector"), used)) const void *vectors[16 + 40] = {
  (void *)0x20005000, Reset_Handler, Default_Handler, Default_Handler, Default_Handler, Default_Handler, Default_Handler,
  0, 0, 0, 0, Default_Handler, Default_Handler, 0, Default_Handler, SysTick_Handler,
  [16 ... 55] = Default_Handler,
};
