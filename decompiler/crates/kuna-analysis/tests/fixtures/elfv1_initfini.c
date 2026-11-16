/* A PPC64 ELFv1 PIE whose functions are reached only through function
   descriptors: e_entry, DT_INIT, DT_FINI, DT_INIT_ARRAY and DT_FINI_ARRAY. */
int counter;
void at_init(void) { counter += 1; }
void at_fini(void) { counter += 2; }
__attribute__((constructor)) static void ctor(void) { counter += 4; }
__attribute__((destructor)) static void dtor(void) { counter += 8; }
void start(void) { for (;;) counter += 16; }
