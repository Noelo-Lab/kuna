int total;
int __cdecl count(int c);
int __cdecl tally(int c) { total += c; return total; }
int __cdecl peek(int *p) { return *p + total; }
int __stdcall bump(int *p) { *p += total; return *p; }
extern "C" int __cdecl start() { return count(5); }
