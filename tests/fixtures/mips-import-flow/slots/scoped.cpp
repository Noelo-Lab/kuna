namespace pkg { namespace detail {
extern void send(int *) __asm__("ns_send");
} }
typedef void nothing;
typedef nothing result;
extern "C" result report(const char *) __asm__("perror");
extern void pass(int *);
extern "C" void take_ref(int &) __asm__("take");
extern "C" void take_move(int &&) __asm__("move_int");

extern "C" void proof_namespace(int *p) { pkg::detail::send(p); }
extern "C" void proof_alias(const char *p) { report(p); }
extern "C" void proof_pointer(int *p) { pass(p); }
extern "C" void proof_reference(int &x) { take_ref(x); }
extern "C" void proof_rvalue(int &x) { take_move(static_cast<int &&>(x)); }

struct Api { static void flush() __asm__("bzero"); };
extern "C" void proof_member() { Api::flush(); }
