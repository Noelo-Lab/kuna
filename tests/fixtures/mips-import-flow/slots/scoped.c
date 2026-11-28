extern void notify(void) __asm__("free");
extern void perror(const char *);
void no_inputs(void) { notify(); }
void forward_alias(const char *p) { perror(p); }
extern void pass(int *) __asm__("_Z4passPi");
extern void take(int *);
extern void move_int(int *);
void forward_pointer(int *p) { pass(p); }
void forward_reference(int *p) { take(p); }
void forward_rvalue(int *p) { move_int(p); }

extern void flush_member(void) __asm__("bzero");
void call_member(void) { flush_member(); }

extern void ns_send(int *);
void forward_namespace(int *p) { ns_send(p); }
