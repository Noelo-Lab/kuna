extern int optional(int) __attribute__((weak));
#ifdef TYPE_HINT
__asm__(".type optional,@function");
#endif
int guard(int value){return optional?optional(value):value+7;}
int offset_guard(void){return (unsigned long)optional+0x8000UL==0x8000UL;}
int present(int value){return value+11;}
int defined_guard(int value){return present?present(value):value+7;}
