#!/usr/bin/env python3
"""Regenerate the WIN32_NAMES rows of kuna-analysis/src/analyzers/protos/kuna_libcnames.rs.

usage: python3 gen_win32_names.py MINGW_INCLUDE > rows.rs

MINGW_INCLUDE is the x86_64-w64-mingw32/include directory of mingw-w64 8.0.0
(Ubuntu 22.04 `mingw-w64-x86-64-dev` and `mingw-w64-common`; `apt-get download`
both and `dpkg-deb -x` them into one tree). Needs clang.

Parameter names are the mingw-w64 declarations, which carry the Windows SDK
names, read with _WIN32_WINNT=0x0A00 so the Vista-and-later entry points are
declared. mingw-w64 leaves the OLE Automation BSTR family unnamed; SDK below
supplies the names the SDK documentation gives them. Return names are the RET
table below: only a result that is one kind of thing and that the SDK names
as a parameter of another entry point (`hModule`, `hHeap`, `lpMem`).
"""
import json, os, re, subprocess, sys, tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
WIN32 = os.path.join(HERE, '../../../decompiler/crates/kuna-analysis/src/analyzers/protos/kuna_win32sigs.rs')
SIG = re.compile(r'\("(\w+)", Sig \{ ret: [^,]+, params: &\[([^\]]*)\], vararg: (-?\d+) \}\)')
HEADERS = ['winsock2.h', 'windows.h', 'tlhelp32.h', 'oleauto.h', 'wincrypt.h']
SDK = {
    'SysAllocString': ['psz'],
    'SysAllocStringLen': ['strIn', 'ui'],
    'SysFreeString': ['bstrString'],
    'SysReAllocString': ['pbstr', 'psz'],
    'SysReAllocStringLen': ['pbstr', 'psz', 'len'],
    'SysStringLen': ['pbstr'],
}
RET = {
    'hThread': 'CreateThread GetCurrentThread',
    'hSnapshot': 'CreateToolhelp32Snapshot',
    'hProcess': 'GetCurrentProcess OpenProcess',
    'dwProcessId': 'GetCurrentProcessId',
    'dwThreadId': 'GetCurrentThreadId',
    'hModule': 'GetModuleHandleA GetModuleHandleW LoadLibraryA LoadLibraryW LoadLibraryExA LoadLibraryExW',
    'hResInfo': 'FindResourceA FindResourceW',
    'hResData': 'LoadResource',
    'hHeap': 'GetProcessHeap HeapCreate',
    'lpMem': 'HeapAlloc HeapReAlloc',
    'lpAddress': 'VirtualAlloc',
    'hFile': 'CreateFileW',
    'hFindFile': 'FindFirstFileExW',
    'penv': 'GetEnvironmentStrings GetEnvironmentStringsW',
    'CodePage': 'GetACP GetOEMCP',
    'Locale': 'GetUserDefaultLCID',
    'dwTlsIndex': 'TlsAlloc',
    'lpTlsValue': 'TlsGetValue',
    'dwFlsIndex': 'FlsAlloc',
    'lpFlsData': 'FlsGetValue',
    'dwErrCode': 'GetLastError',
    'iError': 'WSAGetLastError',
    'bstrString': 'SysAllocString SysAllocStringByteLen SysAllocStringLen',
}


def arities():
    src = open(WIN32).read()
    table = src[src.index('pub(super) const WIN32'):]
    table = table[:table.index('];')]
    return {m.group(1): len(re.findall(r'Ty::', m.group(2))) for m in SIG.finditer(table)}


def declarations(include):
    with tempfile.NamedTemporaryFile('w', suffix='.c') as f:
        f.write(''.join('#include <%s>\n' % h for h in HEADERS))
        f.flush()
        ast = json.loads(subprocess.run(['clang', '--target=x86_64-w64-mingw32', '-D_WIN32_WINNT=0x0A00',
                                         '-isystem', include, '-fsyntax-only', '-Xclang', '-ast-dump=json', f.name],
                                        capture_output=True, check=True).stdout)
    best = {}
    for d in ast.get('inner', []):
        if d.get('kind') != 'FunctionDecl':
            continue
        ps = [p.get('name', '') for p in d.get('inner', []) if p.get('kind') == 'ParmVarDecl']
        score = (all(ps), sum(1 for p in ps if p))
        if d['name'] not in best or score > best[d['name']][0]:
            best[d['name']] = (score, ps)
    return {k: v[1] for k, v in best.items()}


def main():
    decl = declarations(sys.argv[1])
    ret = {fn: name for name, fns in RET.items() for fn in fns.split()}
    for fn, n in sorted(arities().items()):
        names = SDK.get(fn) or decl.get(fn) or []
        names = (names + [''] * n)[:n]
        if any(names) or fn in ret:
            r = 'Some("%s")' % ret[fn] if fn in ret else 'None'
            print('    ("%s", LibraryNames { params: &[%s], ret: %s }),' % (fn, ', '.join('"%s"' % p for p in names), r))


if __name__ == '__main__':
    main()
