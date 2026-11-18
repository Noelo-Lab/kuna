.text
.globl __chkstk
__chkstk:
  pop %ecx
  sub %eax,%esp
  jmp *%ecx
.globl __alloca_probe
__alloca_probe: jmp __chkstk
