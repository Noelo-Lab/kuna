/* libcnames fixture: locals that meet Win32 calls through the import table.
 * Built for i386 and x86-64 with Ubuntu clang 14.0.0, llvm-dlltool 14 and
 * lld-link (LLD 20.1.8, run as the Rust toolchain's `rust-lld -flavor link`):
 *   llvm-dlltool -m i386 -k -d kernel32_i386.def -l kernel32_i386.lib
 *   llvm-dlltool -m i386:x86-64 -d kernel32_x86_64.def -l kernel32_x86_64.lib
 *   clang --target=i686-pc-windows-msvc -O2 -fno-builtin -c libcnames_win32.c -o i386.obj
 *   lld-link /entry:start /subsystem:console /nodefaultlib /safeseh:no i386.obj kernel32_i386.lib /out:libcnames_win32_i386.exe
 *   clang --target=x86_64-pc-windows-msvc -O2 -fno-builtin -c libcnames_win32.c -o x86_64.obj
 *   lld-link /entry:start /subsystem:console /nodefaultlib x86_64.obj kernel32_x86_64.lib /out:libcnames_win32_x86_64.exe
 * The .def files list the imports below (`LIBRARY kernel32.dll`, `EXPORTS`, one
 * name per line; the i386 one decorates each `@<4 * arity>`). No SDK header is
 * needed: the declarations are spelled out. */
typedef void *HANDLE;
typedef unsigned int DWORD;
typedef int BOOL;
typedef unsigned short WCHAR;
typedef __SIZE_TYPE__ SIZE_T;
#define IMPORT __declspec(dllimport) __stdcall

IMPORT HANDLE CreateFileW(const WCHAR *, DWORD, DWORD, void *, DWORD, DWORD, HANDLE);
IMPORT BOOL ReadFile(HANDLE, void *, DWORD, DWORD *, void *);
IMPORT BOOL WriteFile(HANDLE, const void *, DWORD, DWORD *, void *);
IMPORT BOOL CloseHandle(HANDLE);
IMPORT HANDLE GetProcessHeap(void);
IMPORT void *HeapAlloc(HANDLE, DWORD, SIZE_T);
IMPORT BOOL HeapFree(HANDLE, DWORD, void *);
IMPORT HANDLE LoadLibraryW(const WCHAR *);
IMPORT void *GetProcAddress(HANDLE, const char *);
IMPORT BOOL FreeLibrary(HANDLE);
IMPORT DWORD GetLastError(void);
IMPORT HANDLE GetStdHandle(DWORD);
IMPORT void ExitProcess(DWORD);

__declspec(noinline) int copy_to(const WCHAR *from, HANDLE out)
{
    HANDLE in = CreateFileW(from, 0x80000000, 1, 0, 3, 0x80, 0);
    char block[512];
    DWORD got, put;
    int total = 0;
    if (in == (HANDLE)-1)
        return -(int)GetLastError();
    while (ReadFile(in, block, sizeof block, &got, 0) && got) {
        WriteFile(out, block, got, &put, 0);
        total += put;
    }
    CloseHandle(in);
    return total;
}

__declspec(noinline) void *grab(SIZE_T size)
{
    HANDLE heap = GetProcessHeap();
    return HeapAlloc(heap, 8, size);
}

__declspec(noinline) void *resolve(const WCHAR *dll, const char *name)
{
    HANDLE module = LoadLibraryW(dll);
    void *proc;
    if (!module)
        return 0;
    proc = GetProcAddress(module, name);
    if (!proc)
        FreeLibrary(module);
    return proc;
}

void start(void)
{
    int copied = copy_to((const WCHAR *)L"a.txt", GetStdHandle((DWORD)-11));
    void *p = grab(64);
    HeapFree(GetProcessHeap(), 0, p);
    resolve((const WCHAR *)L"user32.dll", "MessageBoxW");
    ExitProcess(copied);
}
