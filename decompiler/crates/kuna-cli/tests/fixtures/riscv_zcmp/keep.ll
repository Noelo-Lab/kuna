target triple = "riscv32-unknown-elf"
declare i32 @ext(i32)
define i32 @keep(i32 %a, i32 %b) optsize {
  %x = call i32 @ext(i32 %a)
  %y = call i32 @ext(i32 %b)
  %z = call i32 @ext(i32 %x)
  ret i32 %z
}
