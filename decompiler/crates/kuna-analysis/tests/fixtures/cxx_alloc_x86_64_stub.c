/* The C++ allocation operators cxx_alloc_x86_64 imports, so its calls to them go
 * through the PLT the way a call into libstdc++ does. Only linked against.
 * Build: zig cc -target x86_64-linux-gnu.2.38 -shared -fPIC -o libcxx_alloc_x86_64_stub.so cxx_alloc_x86_64_stub.c */
void *_Znwm(unsigned long n) { return (void *)n; }
void *_Znam(unsigned long n) { return (void *)n; }
void _ZdlPvm(void *p, unsigned long n) { (void)p; (void)n; }
void _ZdaPv(void *p) { (void)p; }
