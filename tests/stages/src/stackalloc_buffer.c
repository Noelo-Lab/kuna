/* Clang: --target=i686-w64-windows-gnu -O0 -shared -nostdlib -Wl,--entry,0 */
#define API __declspec(dllexport) __attribute__((noinline))
API int observe(char *data,int n){data[n-1]=9;return (unsigned char)data[0]+(unsigned char)data[n-1];}
API __attribute__((naked)) int allocate(int n){
 __asm__("pushl %ebp\nmovl %esp,%ebp\npushl %esi\nmovl 8(%ebp),%eax\naddl $15,%eax\nandl $-16,%eax\nsubl %eax,%esp\nmovl %esp,%esi\nmovb $7,(%esi)\npushl 8(%ebp)\npushl %esi\ncalll _observe\naddl $8,%esp\nmovl -4(%ebp),%esi\nmovl %ebp,%esp\npopl %ebp\nretl\n");
}
