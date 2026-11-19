
typedef unsigned int u32;
extern u32 _estack;
volatile u32 counter;
#define GPIOA_ODR (*(volatile u32 *)0x40020014)
#define USART1_DR (*(volatile u32 *)0x40011004)
#define USART1_SR (*(volatile u32 *)0x40011000)
void Default_Handler(void) { while (1) {} }
void SysTick_Handler(void) { counter++; }
void uart_put(char c) { while (!(USART1_SR & 0x80)) {} USART1_DR = c; }
void USART1_IRQHandler(void) { uart_put('x'); }
int main(void) { for (;;) { GPIOA_ODR ^= 0x20; for (volatile int i = 0; i < 100000; i++) {} if (counter > 10) uart_put('a'); } }
void Reset_Handler(void) { main(); }
__attribute__((section(".isr_vector"), used)) const void *vectors[16 + 40] = {
  (void *)0x20005000, Reset_Handler, Default_Handler, Default_Handler, Default_Handler, Default_Handler, Default_Handler,
  0, 0, 0, 0, Default_Handler, Default_Handler, 0, Default_Handler, SysTick_Handler,
  [16 ... 52] = Default_Handler, [16 + 37] = USART1_IRQHandler,
};
