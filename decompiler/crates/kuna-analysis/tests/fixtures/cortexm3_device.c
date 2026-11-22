/* Peripheral register access in the shape of a vendor HAL, at the device
   addresses of the ARMv7-M Peripheral (0x40000000) and System (0xE0000000)
   regions, which an M-profile memory map makes Device memory.
   clang --target=thumbv7m-none-eabi -mcpu=cortex-m3 -Os -ffreestanding -nostdlib -fno-inline \
     -fuse-ld=lld -Wl,-Ttext=0x8000000 -Wl,-e,Reset_Handler -Wl,-z,max-page-size=4 \
     cortexm3_device.c -o cortexm3_device.elf && llvm-strip --strip-debug cortexm3_device.elf */
typedef unsigned int u32;
#define REG(a) (*(volatile u32 *)(a))
#define RCC_CR 0x40021000u
#define USART_SR 0x40013800u
#define USART_DR 0x40013804u
#define DBG_CR 0xE0042004u
void clock_start(void) {
  REG(RCC_CR) = 1;
  while (!(REG(RCC_CR) & 2)) {}
  REG(RCC_CR) = 0x10001;
}
void uart_send2(u32 a, u32 b) { REG(USART_DR) = a; REG(USART_DR) = b; }
u32 uart_read2(void) { u32 lo = REG(USART_DR); u32 hi = REG(USART_DR); return lo | hi << 8; }
void uart_flush(void) { while (REG(USART_SR) & 0x20) { (void)REG(USART_DR); } }
void debug_freeze(void) { REG(DBG_CR) = 0; REG(DBG_CR) = 7; }
int main(void) {
  clock_start(); debug_freeze(); uart_send2('o', 'k'); uart_flush();
  for (;;) { if (uart_read2() == 0x0a0d) break; }
  return 0;
}
void Reset_Handler(void) { main(); for (;;) {} }
