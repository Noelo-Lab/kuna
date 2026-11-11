// GH #1401: the issue's fixture plus a __DATA segment whose zerofill __bss runs
// past its file content. __PAGEZERO declares memory and no access; __LINKEDIT's
// vmsize is rounded to the page by readprov_macho_arm64.py, as Apple's ld64 does.
.section __TEXT,__text,regular,pure_instructions
.p2align 2
.globl _main
_main:
  bl _actual_code
  ret
.section __TEXT,__const
.p2align 2
.globl _actual_code
_actual_code:
  mov w0, #42
  ret
.globl _instruction_like_data
_instruction_like_data:
  .long 0xd65f03c0
.section __DATA,__data
.p2align 3
.globl _initialized
_initialized:
  .quad 0x1122334455667788
.zerofill __DATA,__bss,_scratch,0x8000,3
