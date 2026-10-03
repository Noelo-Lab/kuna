#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

extern char *caller_first(void);
extern char *caller_adjacent(void);
extern char *caller_frame(void);
extern unsigned long caller_write_first(void);
extern unsigned long caller_write_adjacent(void);
extern int check_address(char *(*fn)(void), uintptr_t delta);
extern unsigned long write_payload(unsigned long (*fn)(void), const unsigned char *bytes);

__asm__(
".text\n.globl check_address\n.type check_address,@function\n"
"check_address:\n push %rbx\n sub $64,%rsp\n mov %rsp,%rbx\n"
" add %rsi,%rbx\n call *%rdi\n cmp %rbx,%rax\n sete %al\n"
" movzbl %al,%eax\n add $64,%rsp\n pop %rbx\n ret\n"
".size check_address,.-check_address\n"
".globl write_payload\n.type write_payload,@function\n"
"write_payload:\n sub $72,%rsp\n mov %rdi,%r11\n"
" mov 0(%rsi),%rax\n mov %rax,0(%rsp)\n"
" mov 8(%rsi),%rax\n mov %rax,8(%rsp)\n"
" mov 16(%rsi),%rax\n mov %rax,16(%rsp)\n"
" mov 24(%rsi),%rax\n mov %rax,24(%rsp)\n"
" mov 32(%rsi),%rax\n mov %rax,32(%rsp)\n"
" mov 40(%rsi),%rax\n mov %rax,40(%rsp)\n"
" mov 48(%rsi),%rax\n mov %rax,48(%rsp)\n"
" mov 56(%rsi),%rax\n mov %rax,56(%rsp)\n"
" call *%r11\n add $72,%rsp\n ret\n"
".size write_payload,.-write_payload\n");

int main(void) {
    char *sp;
    __asm__ volatile("mov %%rsp,%0" : "=r"(sp));
    if (caller_first() != sp || caller_adjacent() != sp + 11 || caller_frame() != sp + 11)
        abort();
    if (!check_address(caller_first, 0) || !check_address(caller_adjacent, 11)
        || !check_address(caller_frame, 11))
        abort();
    int pipefd[2];
    if (pipe(pipefd) || dup2(pipefd[1], 17) != 17)
        abort();
    unsigned long (*writers[])(void) = {caller_write_first, caller_write_adjacent};
    for (unsigned seed = 0; seed < 32; ++seed) {
        unsigned char bytes[64], actual[16];
        for (unsigned j = 0; j < sizeof bytes; ++j)
            bytes[j] = (unsigned char)(seed * 31 + j * 17);
        for (unsigned j = 0; j < 2; ++j) {
            if (write_payload(writers[j], bytes) != sizeof actual
                || read(pipefd[0], actual, sizeof actual) != sizeof actual
                || memcmp(actual, bytes + j * 11, sizeof actual))
                abort();
        }
    }
    puts("caller addresses and pipe payloads match");
    return 0;
}
