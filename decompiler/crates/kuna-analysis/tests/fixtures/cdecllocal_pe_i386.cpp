int __cdecl tally(int c);
int __cdecl peek(int *p);
int __stdcall bump(int *p);
int __cdecl count(int c) {
    int x = 0x1234;
    tally(c);
    peek(&x);
    bump(&x);
    return x;
}
