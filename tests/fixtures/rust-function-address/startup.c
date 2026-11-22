#include <stdint.h>
static unsigned hits;
void probe_main(void) __asm__("_ZN9r17_probe4main17h1111111111111111E");
int dispatch(void (*f)(void), long argc, char **argv, int flags) __asm__("_ZN3std2rt10lang_start17h2222222222222222E");
__attribute__((noinline)) void probe_main(void) { hits += 1; }
__attribute__((noinline)) int dispatch(void (*f)(void), long argc, char **argv, int flags) {
    if (f != probe_main || argc < 1 || argv == 0 || flags != 0) return 97;
    f();
    return hits == 1 ? 0 : 98;
}
void keyword_callback(void) __asm__("_ZN4type8callback17h3333333333333333E");
__attribute__((noinline)) void keyword_callback(void) { hits += 2; }
__attribute__((noinline)) uintptr_t keyword_address(void) { return (uintptr_t)keyword_callback; }
void const_callback(void) __asm__("_ZN5const8callback17h4444444444444444E");
__attribute__((noinline)) void const_callback(void) { hits += 3; }
__attribute__((noinline)) uintptr_t const_address(void) { return (uintptr_t)const_callback; }
__attribute__((noinline)) void ordinary_callback(void) { hits += 4; }
uintptr_t global_address(void) __asm__("_ZN9r17_probe14global_address17h5555555555555555E");
__attribute__((noinline)) uintptr_t global_address(void) { return (uintptr_t)ordinary_callback; }
int main(int argc, char **argv) { return dispatch(probe_main, argc, argv, 0); }
