/* Fresh record-store fixture. Build with Clang --target=i686-w64-windows-gnu -O0 -shared -nostdlib -Wl,--entry,0. */

typedef __UINTPTR_TYPE__ uintptr_t;
typedef struct Record {unsigned int length;void *pointer;} Record;
#ifdef _WIN32
#define API __declspec(dllexport) __attribute__((noinline))
#else
#define API __attribute__((visibility("default"),noinline))
#endif
API unsigned int consume(Record *r){return r->length+((uintptr_t)r->pointer&255);}
API __attribute__((naked)) unsigned int emit(char *payload){
#ifdef _WIN32
__asm__("pushl %ebp\nmovl %esp,%ebp\nsubl $8,%esp\nmovl 8(%ebp),%eax\nmovl $5,-8(%ebp)\nmovl %eax,-4(%ebp)\nleal -8(%ebp),%eax\npushl %eax\ncalll _consume\naddl $4,%esp\nmovl %ebp,%esp\npopl %ebp\nretl\n");
#else
__asm__("pushq %rbp\nmovq %rsp,%rbp\nsubq $16,%rsp\nmovl $5,-16(%rbp)\nmovq %rdi,-8(%rbp)\nleaq -16(%rbp),%rdi\ncall consume\nmovq %rbp,%rsp\npopq %rbp\nretq\n");
#endif
}
