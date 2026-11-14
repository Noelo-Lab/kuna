__attribute__((visibility("hidden"), noinline)) int audit_target(int x) { return x + 7; }
__attribute__((noinline)) int audit_export(int x) { return x * 3; }
int audit_data = 5;
int (*audit_slot)(int) = audit_target;
int (*audit_export_slot)(int) = audit_export;
int *audit_data_ptr(void) { return &audit_data; }
int audit_call(int x) { return audit_export(x); }
