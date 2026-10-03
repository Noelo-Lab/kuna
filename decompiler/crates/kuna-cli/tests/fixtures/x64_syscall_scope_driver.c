#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>

uintptr_t scope_sp;
unsigned long native_metadata8(void), native_metadata40(void);
unsigned long scope_invoke(unsigned long (*fn)(void));
__asm__(
  ".text\n.globl scope_invoke\n.type scope_invoke,@function\n"
  "scope_invoke:\n push %rbx\n mov %rdi,%rbx\n sub $128,%rsp\n"
  "mov %rsp,scope_sp(%rip)\n call *%rbx\n add $128,%rsp\n pop %rbx\n ret\n"
  ".size scope_invoke,.-scope_invoke\n");

static void check(unsigned long (*fn)(void), uintptr_t offset) {
  int p[2];
  uintptr_t observed = 0;
  if (pipe(p) || dup2(p[1], 17) < 0) abort();
  close(p[1]);
  if (scope_invoke(fn) != 8) abort();
  close(17);
  if (read(p[0], &observed, 8) != 8 || observed != scope_sp + offset - 8) abort();
  close(p[0]);
}

int main(void) {
  for (int i = 0; i < 16; ++i) {
    check(native_metadata8, 8);
    check(metadata8, 8);
    check(native_metadata40, 40);
    check(metadata40, 40);
  }
  return 0;
}
