// GH #1406: code in __TEXT,__const, a section an Apple toolchain links with no
// instruction attribute although its segment is executable.
.section __TEXT,__text,regular,pure_instructions
.p2align 2
.globl _main
_main: bl _middle
  bl _quiet
  ret
.globl _goal
_goal: mov w0, #42
  ret
.globl _direct_goal
_direct_goal: bl _goal
  ret
.globl _unrelated
_unrelated: mov w0, #9
  ret
.section __TEXT,__const
.p2align 2
.globl _middle
_middle: bl _goal
  ret
.globl _quiet
_quiet: nop
  ret
.globl _orphan
_orphan: bl _goal
  ret
.section __TEXT,__cstring,cstring_literals
.globl _greeting
_greeting: .asciz "hello"
.section __DATA,__data
.globl _data
_data: .long 42
