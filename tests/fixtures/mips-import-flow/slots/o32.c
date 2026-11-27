extern void report_error(const char *) __asm__("perror");
extern void clear(void *, unsigned) __asm__("bzero");
extern void take_one(void *);
enum wide { large = 0x123456789ULL };
extern void wide_arg(int, enum wide, int);

#ifdef STRIPPED
extern int release_value(int, int) __asm__("free");
void discard_result(int x) { release_value(x + 1, 12345); }
int add_result(int x) { return release_value(x + 1, 12345) + 1; }
extern void trace(void *, int, int, int, void *) __asm__("free");
void send_return(void *p, int a, int b, int c) {
    trace(p, a, b, c, __builtin_return_address(0));
}
#else
extern void packed(int, int, int);
void send_pack(void) { packed(1, 2, 3); }
extern void free(void *);
void forward_pointer(void *p) { free(p); }
#endif
void forward_message(const char *p) { report_error(p); }
void forward_one(void *p) { take_one(p); }
void forward_two(void *p, unsigned n) { clear(p, n); }
void send_enum(void) { wide_arg(6, large, 19); }
