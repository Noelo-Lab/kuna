__attribute__((visibility("default"), noinline))
int audit_exported_function(int x) { return x + 7; }
__attribute__((visibility("default")))
volatile int audit_exported_data = 3;
__attribute__((visibility("default"), noinline))
int audit_caller(int x) { return audit_exported_function(x) + audit_exported_data; }
