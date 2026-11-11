/* elf_gotdata fixtures: -fPIC shared objects that read three imported DATA
   symbols through R_*_GLOB_DAT GOT slots (undefined here), plus one global the
   object defines itself (GLOB_DAT to a defined, preemptible symbol). */
typedef struct _IO_FILE FILE;
extern FILE *stdout;
extern char **environ;
extern int imported_counter;
int exported_value = 7;
int get_counter(void) { return imported_counter; }
char **get_env(void) { return environ; }
FILE *get_out(void) { return stdout; }
int get_exported(void) { return exported_value; }
int *addr_exported(void) { return &exported_value; }
