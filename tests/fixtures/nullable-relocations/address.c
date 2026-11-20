extern int optional(int) __attribute__((weak));
__asm__(".type optional,@function");
unsigned long address_value(void){return (unsigned long)optional;}
unsigned long address_offset(void){return (unsigned long)optional+0x8000UL;}
unsigned long address_negative(void){return (unsigned long)optional-0x8000UL;}
unsigned long unrelated_literal(void){return 0x402000UL;}
