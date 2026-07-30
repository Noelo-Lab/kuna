/* minimal stubs to link tr.o standalone (non-PIE) */
void abort(void){__builtin_unreachable();}
void __assert_fail(const char*a,const char*b,unsigned c,const char*d){}
int atexit(void(*f)(void)){return 0;}
char* bindtextdomain(const char*a,const char*b){return 0;}
int close(int a){return 0;}
void close_stdout(void){}
const unsigned short** __ctype_b_loc(void){return 0;}
char* dcgettext(const char*a,const char*b,int c){return (char*)b;}
int* __errno_location(void){static int e;return &e;}
void error(int a,int b,const char*c,...){}
void exit(int a){__builtin_unreachable();}
int fadvise(void*a,int b){return 0;}
int __fprintf_chk(void*a,int b,const char*c,...){return 0;}
int fputs_unlocked(const char*a,void*b){return 0;}
void free(void*a){}
unsigned long fwrite_unlocked(const void*a,unsigned long b,unsigned long c,void*d){return 0;}
int getopt_long(int a,char**b,const char*c,const void*d,int*e){return -1;}
int optind;
int __printf_chk(int a,const char*b,...){return 0;}
char* program_name;
long safe_read(int a,void*b,unsigned long c){return 0;}
char* setlocale(int a,const char*b){return 0;}
void set_program_name(const char*a){}
int __sprintf_chk(char*a,int b,unsigned long c,const char*d,...){return 0;}
void __stack_chk_fail(void){__builtin_unreachable();}
void* stderr; void* stdin; void* stdout;
char* stpcpy(char*a,const char*b){return a;}
unsigned long strlen(const char*a){return 0;}
int strncmp(const char*a,const char*b,unsigned long c){return 0;}
char* textdomain(const char*a){return 0;}
int tolower(int a){return a;}
int toupper(int a){return a;}
const char Version[]="0";
const char* quote(const char*a){return a;}
void version_etc(void*a,const char*b,const char*c,const char*d,...){}
void* xcalloc(unsigned long a,unsigned long b){return 0;}
void* xmalloc(unsigned long a){return 0;}
void* xnmalloc(unsigned long a,unsigned long b){return 0;}
int xstrtoumax(const char*a,char**b,int c,unsigned long*d,const char*e){return 0;}
/* tr.o has main? provide _start */
extern int main(int,char**);
void _start(void){}
