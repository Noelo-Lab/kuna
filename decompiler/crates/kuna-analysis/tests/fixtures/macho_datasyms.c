volatile int audit_global = 3;
const char *audit_message = "static marker";
__attribute__((noinline)) int audit_read(void) { return audit_global; }
int main(void) { return audit_read() + audit_message[0]; }
