// Source for the MSVC-mangled COFF fixture (multi-format loader PR-9).
// Built: clang -target x86_64-pc-windows-msvc -O1 -c msvc_mangled.cpp -o msvc_mangled.obj
// cl.exe is unavailable on Linux; clang's windows-msvc target emits the same
// `?`-prefixed MSVC mangling (verified `objdump -t`).
struct Bar {
    int foo(int x);
};
int Bar::foo(int x) { return x * 3 + 1; }       // ?foo@Bar@@QEAAHH@Z

namespace ns {
    int g(int a, int b) { return a * b + 7; }    // ?g@ns@@YAHHH@Z
}

int freefunc(int x) { return x + 42; }           // ?freefunc@@YAHH@Z
