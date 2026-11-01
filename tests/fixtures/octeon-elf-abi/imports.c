/* Original unresolved PIC call for loader/import recovery. */
typedef unsigned long long U;
extern U external_value(U value);
U imported_call(U value) { return external_value(value) + value; }
