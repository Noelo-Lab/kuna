const char audit_label[] = "AUDIT_STATIC_LABEL";
const char *volatile audit_label_ptr = audit_label;
__attribute__((noinline)) int audit_target(int value) { return value + 11; }
int (*volatile audit_func_ptr)(int) = audit_target;
__attribute__((noinline)) const char *audit_direct_label(void) { return audit_label; }
__attribute__((noinline)) int audit_direct_call(int value) { return audit_target(value); }
__attribute__((noinline)) int audit_indirect_call(int value) { return audit_func_ptr(value); }
int main(void) { return audit_direct_call(1) + audit_indirect_call(2) + audit_direct_label()[0]; }
extern void audit_imported(void);
void (*volatile audit_import_ptr)(void) = audit_imported;
