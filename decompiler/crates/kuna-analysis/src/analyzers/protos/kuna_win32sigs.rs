//! (kuna) `win32sigs` — built-in Win32 API prototypes, parked by ENTRY ADDRESS.
//!
//! [`super::LibProtoPass`] and [`super::kuna_libcsigs::LibcSigsPass`] carry libc.
//! Nothing in the tree carries a single Windows API signature, so on a PE every
//! `LoadLibraryExW` / `CreateFileW` / `WriteFile` call arrives at
//! `ActionDefaultParams` with an empty prototype and its arguments have to be
//! recovered from the call site alone. Where that recovery loses — the ordinary
//! case for an image that writes its outgoing slots well before the call — the
//! call renders `LoadLibraryExW()` and the argument stores are left behind as
//! mapped stack locals, which is the difference between 8 declarations and 47.
//!
//! ## Two halves, because either alone is inert
//!
//! **The table.** Ghidra applies its `.gdt` archives here; kuna cannot vendor
//! them, so this is the same deliberate stand-in [`super::LibProtoPass`] already
//! is, for the Win32 half. Ranking is measured, not intuited: an import
//! histogram over the 54 PE images of the repipe arena corpus admits a name at
//! **>= 5 images**. The resource/loader family below that bar is admitted
//! separately and named as such — it is the family the RE need was filed
//! against, and a resource loader is what a Windows reverser reads first.
//!
//! **The address key.** The by-name park ([`super::LibProtoPass`] via
//! `Architecture::set_function_prototype_pieces`) is a silent no-op on a PE
//! import: `pe_iat` registers TWO FunctionSymbols per import — the size-0 IAT
//! slot the engine constant-folds through, and the `FF 25` thunk veneer a direct
//! `call` targets — and the global by-name query answers with the slot, while
//! `ActionDefaultParams` reads `callee_proto_pieces(entry)` at the thunk. So this
//! pass emits into [`crate::pass::AnalysisOutput::prototypes_at`], keyed by every
//! address the import resolver names, and lands on both.
//!
//! ## Where the signatures come from, and the wrongness axis
//!
//! No Windows SDK header can be reduced on the build host, so each entry is
//! transcribed from the documented Win32 declaration and then **reduced to
//! [`Ty`]'s width-stable vocabulary**: `HANDLE`/`HMODULE`/`HWND`/`HRSRC`/
//! `LPVOID` are `void *`, `DWORD`/`UINT`/`LCID` are `unsigned int`, `BOOL`/`int`/
//! `LONG` are `int`, `SIZE_T` is pointer-width, `LPCSTR` is `char *`, `LPCWSTR` is
//! `wchar_t *`, `LPDWORD` is `unsigned int *`. A declaration with a slot that has
//! no honest spelling is REJECTED rather than approximated, which is why
//! `SetFilePointerEx` (a by-value `LARGE_INTEGER`), `RtlVirtualUnwind` and
//! `RtlLookupFunctionEntry` (by-value `DWORD64`) and `LCMapStringEx` are absent.
//!
//! The load-bearing field is the **arity**, not the types: the x86 PE default
//! model is `__stdcall` with `extrapop="unknown"`, so a locked N-parameter
//! prototype also tells the engine the callee pops `4 + 4N` bytes. Every name
//! here is a `WINAPI`/`APIENTRY` (callee-cleans) export for that reason — no CRT
//! name is admitted, `__cdecl` msvcrt spellings included, because those already
//! belong to the libc tables and would get the wrong cleanup here.
//!
//! Applied only to a name the image IMPORTS and does not itself define — the same
//! restriction, for the same reason, as [`super::kuna_libcsigs`].

use object::read::{Object, ObjectSymbol};
use object::SymbolKind;
use std::collections::HashSet;

use super::{build_pieces, Sig, Ty};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// Seed the built-in Win32 API signatures onto a PE's imported API names.
pub struct Win32SigsPass;

/// The built-in Win32 API prototype table.
///
/// Disjoint from both libc tables by construction (`table_is_disjoint_from_libc`)
/// and from the CRT entirely: every entry is a `WINAPI` export.
pub(super) const WIN32: &[(&str, Sig)] = &[
    // --- process / thread (kernel32) ---
    ("CreateThread", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Size, Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UIntPtr], vararg: -1 }),
    ("CreateToolhelp32Snapshot", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt, Ty::UInt], vararg: -1 }),
    ("ExitProcess", Sig { ret: Ty::Void, params: &[Ty::UInt], vararg: -1 }),
    ("GetCurrentProcess", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("GetCurrentProcessId", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetCurrentThread", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("GetCurrentThreadId", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetExitCodeProcess", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UIntPtr], vararg: -1 }),
    ("GetStartupInfoA", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetStartupInfoW", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("OpenProcess", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt, Ty::Int, Ty::UInt], vararg: -1 }),
    ("ResumeThread", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr], vararg: -1 }),
    ("Sleep", Sig { ret: Ty::Void, params: &[Ty::UInt], vararg: -1 }),
    ("SuspendThread", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr], vararg: -1 }),
    ("TerminateProcess", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("TerminateThread", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("WaitForSingleObject", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    // --- module / loader (kernel32) ---
    ("FreeLibrary", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetModuleFileNameA", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::UInt], vararg: -1 }),
    ("GetModuleFileNameW", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr, Ty::WCharPtr, Ty::UInt], vararg: -1 }),
    ("GetModuleHandleA", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("GetModuleHandleExW", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::WCharPtr, Ty::VoidPtr], vararg: -1 }),
    ("GetModuleHandleW", Sig { ret: Ty::VoidPtr, params: &[Ty::WCharPtr], vararg: -1 }),
    ("GetProcAddress", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::CharPtr], vararg: -1 }),
    ("LoadLibraryA", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr], vararg: -1 }),
    ("LoadLibraryExA", Sig { ret: Ty::VoidPtr, params: &[Ty::CharPtr, Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("LoadLibraryExW", Sig { ret: Ty::VoidPtr, params: &[Ty::WCharPtr, Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("LoadLibraryW", Sig { ret: Ty::VoidPtr, params: &[Ty::WCharPtr], vararg: -1 }),
    // --- resource loading (kernel32).  Below the >= 5-image bar; admitted
    //     because it is the family the RE need was filed against.
    ("EnumResourceNamesW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::WCharPtr, Ty::VoidPtr, Ty::Long], vararg: -1 }),
    ("EnumResourceTypesW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Long], vararg: -1 }),
    ("FindResourceA", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::CharPtr], vararg: -1 }),
    ("FindResourceW", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::WCharPtr, Ty::WCharPtr], vararg: -1 }),
    ("FreeResource", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("LoadResource", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("LockResource", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("SizeofResource", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    // --- heap / virtual memory (kernel32) ---
    ("GetProcessHeap", Sig { ret: Ty::VoidPtr, params: &[], vararg: -1 }),
    ("HeapAlloc", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::UInt, Ty::Size], vararg: -1 }),
    ("HeapCreate", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt, Ty::Size, Ty::Size], vararg: -1 }),
    ("HeapDestroy", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("HeapFree", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("HeapReAlloc", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::UInt, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    ("HeapSize", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("VirtualAlloc", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr, Ty::Size, Ty::UInt, Ty::UInt], vararg: -1 }),
    ("VirtualFree", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Size, Ty::UInt], vararg: -1 }),
    ("VirtualProtect", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::Size, Ty::UInt, Ty::UIntPtr], vararg: -1 }),
    ("VirtualQuery", Sig { ret: Ty::Size, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::Size], vararg: -1 }),
    // --- files / handles / console (kernel32) ---
    ("CloseHandle", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("CreateFileW", Sig { ret: Ty::VoidPtr, params: &[Ty::WCharPtr, Ty::UInt, Ty::UInt, Ty::VoidPtr, Ty::UInt, Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("DuplicateHandle", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::Int, Ty::UInt], vararg: -1 }),
    ("FindClose", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("FindFirstFileExW", Sig { ret: Ty::VoidPtr, params: &[Ty::WCharPtr, Ty::Int, Ty::VoidPtr, Ty::Int, Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("FindNextFileW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("FlushFileBuffers", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetConsoleMode", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UIntPtr], vararg: -1 }),
    ("GetConsoleOutputCP", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetFileSizeEx", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("GetFileType", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetStdHandle", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt], vararg: -1 }),
    ("ReadConsoleW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UIntPtr, Ty::VoidPtr], vararg: -1 }),
    ("ReadFile", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UIntPtr, Ty::VoidPtr], vararg: -1 }),
    ("SetFilePointer", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr, Ty::Int, Ty::IntPtr, Ty::UInt], vararg: -1 }),
    ("SetStdHandle", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("WriteConsoleW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UIntPtr, Ty::VoidPtr], vararg: -1 }),
    ("WriteFile", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UIntPtr, Ty::VoidPtr], vararg: -1 }),
    // --- environment / command line (kernel32) ---
    ("FreeEnvironmentStringsA", Sig { ret: Ty::Int, params: &[Ty::CharPtr], vararg: -1 }),
    ("FreeEnvironmentStringsW", Sig { ret: Ty::Int, params: &[Ty::WCharPtr], vararg: -1 }),
    ("GetCommandLineA", Sig { ret: Ty::CharPtr, params: &[], vararg: -1 }),
    ("GetCommandLineW", Sig { ret: Ty::WCharPtr, params: &[], vararg: -1 }),
    ("GetEnvironmentStrings", Sig { ret: Ty::CharPtr, params: &[], vararg: -1 }),
    ("GetEnvironmentStringsW", Sig { ret: Ty::WCharPtr, params: &[], vararg: -1 }),
    ("GetEnvironmentVariableW", Sig { ret: Ty::UInt, params: &[Ty::WCharPtr, Ty::WCharPtr, Ty::UInt], vararg: -1 }),
    ("SetEnvironmentVariableW", Sig { ret: Ty::Int, params: &[Ty::WCharPtr, Ty::WCharPtr], vararg: -1 }),
    // --- locale / code pages (kernel32) ---
    ("CompareStringW", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::WCharPtr, Ty::Int, Ty::WCharPtr, Ty::Int], vararg: -1 }),
    ("EnumSystemLocalesW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("GetACP", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetCPInfo", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("GetLocaleInfoW", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::WCharPtr, Ty::Int], vararg: -1 }),
    ("GetOEMCP", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetStringTypeA", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::CharPtr, Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("GetStringTypeW", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::WCharPtr, Ty::Int, Ty::VoidPtr], vararg: -1 }),
    ("GetUserDefaultLCID", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("IsValidCodePage", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("IsValidLocale", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt], vararg: -1 }),
    ("LCMapStringA", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::CharPtr, Ty::Int, Ty::CharPtr, Ty::Int], vararg: -1 }),
    ("LCMapStringW", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::WCharPtr, Ty::Int, Ty::WCharPtr, Ty::Int], vararg: -1 }),
    ("MultiByteToWideChar", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::CharPtr, Ty::Int, Ty::WCharPtr, Ty::Int], vararg: -1 }),
    ("WideCharToMultiByte", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::UInt, Ty::WCharPtr, Ty::Int, Ty::CharPtr, Ty::Int, Ty::CharPtr, Ty::IntPtr], vararg: -1 }),
    // --- synchronization / TLS (kernel32) ---
    ("AcquireSRWLockExclusive", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("DeleteCriticalSection", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("EnterCriticalSection", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("FlsAlloc", Sig { ret: Ty::UInt, params: &[Ty::VoidPtr], vararg: -1 }),
    ("FlsFree", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("FlsGetValue", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt], vararg: -1 }),
    ("FlsSetValue", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("InitializeCriticalSection", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("InitializeCriticalSectionAndSpinCount", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt], vararg: -1 }),
    ("InitializeCriticalSectionEx", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::UInt, Ty::UInt], vararg: -1 }),
    ("InitializeSListHead", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("LeaveCriticalSection", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("ReleaseSRWLockExclusive", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("SleepConditionVariableSRW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr, Ty::UInt, Ty::UInt], vararg: -1 }),
    ("TlsAlloc", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("TlsFree", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("TlsGetValue", Sig { ret: Ty::VoidPtr, params: &[Ty::UInt], vararg: -1 }),
    ("TlsSetValue", Sig { ret: Ty::Int, params: &[Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("WakeAllConditionVariable", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    // --- diagnostics / errors / timing (kernel32) ---
    ("CheckRemoteDebuggerPresent", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::IntPtr], vararg: -1 }),
    ("DecodePointer", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("EncodePointer", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetLastError", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("GetSystemTimeAsFileTime", Sig { ret: Ty::Void, params: &[Ty::VoidPtr], vararg: -1 }),
    ("GetTickCount", Sig { ret: Ty::UInt, params: &[], vararg: -1 }),
    ("IsDebuggerPresent", Sig { ret: Ty::Int, params: &[], vararg: -1 }),
    ("IsProcessorFeaturePresent", Sig { ret: Ty::Int, params: &[Ty::UInt], vararg: -1 }),
    ("OutputDebugStringA", Sig { ret: Ty::Void, params: &[Ty::CharPtr], vararg: -1 }),
    ("QueryPerformanceCounter", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("QueryPerformanceFrequency", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    ("RaiseException", Sig { ret: Ty::Void, params: &[Ty::UInt, Ty::UInt, Ty::UInt, Ty::VoidPtr], vararg: -1 }),
    ("SetLastError", Sig { ret: Ty::Void, params: &[Ty::UInt], vararg: -1 }),
    ("SetUnhandledExceptionFilter", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("UnhandledExceptionFilter", Sig { ret: Ty::Int, params: &[Ty::VoidPtr], vararg: -1 }),
    // --- thread context (kernel32) ---
    ("GetThreadContext", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    ("SetThreadContext", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::VoidPtr], vararg: -1 }),
    // --- user32 / gdi32 / advapi32 ---
    ("CryptReleaseContext", Sig { ret: Ty::Int, params: &[Ty::Size, Ty::UInt], vararg: -1 }),
    ("GetDC", Sig { ret: Ty::VoidPtr, params: &[Ty::VoidPtr], vararg: -1 }),
    ("MessageBoxA", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::CharPtr, Ty::CharPtr, Ty::UInt], vararg: -1 }),
    ("MessageBoxW", Sig { ret: Ty::Int, params: &[Ty::VoidPtr, Ty::WCharPtr, Ty::WCharPtr, Ty::UInt], vararg: -1 }),
];

/// Every address the import resolver names, for each name the image IMPORTS and
/// does not itself define.
///
/// One name yields SEVERAL addresses on a PE — the IAT slot and the `FF 25`
/// thunk veneer that jumps through it — and the prototype belongs on all of them:
/// a direct `call thunk` and a `call [slot]` are the same callee, and which one
/// `ActionDefaultParams` asks about depends on how the image spells the call.
fn imported_addrs_by_name(file: &object::File, bytes: &[u8]) -> Vec<(String, u64)> {
    let mut defined = HashSet::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() == SymbolKind::Text && !sym.is_undefined() {
            if let Ok(n) = sym.name() {
                defined.insert(n.to_string());
            }
        }
    }
    let mut out = Vec::new();
    for imp in crate::loader::format::resolve_imports(file, bytes) {
        let Ok(name) = String::from_utf8(imp.name) else { continue };
        if defined.contains(&name) {
            continue;
        }
        out.push((name, imp.addr));
    }
    out
}

impl AnalysisPass for Win32SigsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "win32sigs"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if !matches!(ctx.file.format(), object::BinaryFormat::Pe | object::BinaryFormat::Coff) {
            return out;
        }
        let types = ctx.arch.types();
        let (_addr_size, word_size) = ctx.arch.data_org();
        for (name, addr) in imported_addrs_by_name(ctx.file, ctx.bytes) {
            let Some((_, sig)) = WIN32.iter().find(|(n, _)| *n == name) else { continue };
            if let Ok(pieces) = build_pieces(&name, sig, types, word_size) {
                out.prototypes_at.push((addr, pieces));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_disjoint_from_libc() {
        // A name in two tables would be committed twice with two different
        // calling conventions -- the libc ones are __cdecl, these are __stdcall.
        for (name, _) in WIN32 {
            assert!(
                !super::super::LIBC.iter().any(|(n, _)| n == name),
                "{name} is already in the base libc table"
            );
            assert!(
                !super::super::kuna_libcsigs::LIBC_EXT.iter().any(|(n, _)| n == name),
                "{name} is already in the libcsigs table"
            );
        }
    }

    #[test]
    fn table_names_are_unique_and_sorted_within_groups() {
        let mut seen: HashSet<&str> = HashSet::new();
        for (name, _) in WIN32 {
            assert!(seen.insert(name), "{name} appears twice in WIN32");
        }
    }

    #[test]
    fn no_entry_is_variadic() {
        // No `WINAPI` export in this table is variadic. It matters more here than
        // in the libc tables: the x86 PE default model is __stdcall with
        // extrapop="unknown", so the fixed arity is also the callee's stack pop.
        for (name, sig) in WIN32 {
            assert_eq!(sig.vararg, -1, "{name}: this table carries no variadic export");
        }
    }

    #[test]
    fn the_resource_loader_family_carries_its_documented_arity() {
        // The witness of the RE need `resource-loader-c-retains`. Arity is the
        // load-bearing field: it is what makes the call stop rendering
        // `LoadLibraryExW()` and what the __stdcall model turns into `extrapop`.
        let get = |want: &str| &WIN32.iter().find(|(n, _)| *n == want).expect(want).1;
        assert_eq!(get("LoadLibraryExW").params.len(), 3, "lpLibFileName, hFile, dwFlags");
        assert!(matches!(get("LoadLibraryExW").params[0], Ty::WCharPtr), "the W form takes LPCWSTR");
        assert!(matches!(get("LoadLibraryExA").params[0], Ty::CharPtr), "the A form takes LPCSTR");
        assert_eq!(get("EnumResourceTypesW").params.len(), 3, "hModule, lpEnumFunc, lParam");
        assert_eq!(get("EnumResourceNamesW").params.len(), 4, "hModule, lpszType, lpEnumFunc, lParam");
        assert_eq!(get("FindResourceW").params.len(), 3, "hModule, lpName, lpType");
        assert_eq!(get("LoadResource").params.len(), 2, "hModule, hResInfo");
        assert_eq!(get("SizeofResource").params.len(), 2, "hModule, hResInfo");
        assert_eq!(get("FreeLibrary").params.len(), 1, "hLibModule");
    }

    #[test]
    fn no_crt_spelling_is_admitted() {
        // Every entry is a callee-cleans WINAPI export. A __cdecl msvcrt name
        // here would be given the wrong stack cleanup on i386, so the two worlds
        // are kept apart by construction rather than by review.
        for crt in ["malloc", "free", "memcpy", "memset", "printf", "exit", "_initterm", "wcslen"] {
            assert!(
                !WIN32.iter().any(|(n, _)| *n == crt),
                "{crt} is a CRT (__cdecl) name and belongs to the libc tables"
            );
        }
    }

    #[test]
    fn a_pe_import_yields_one_address_per_resolver_entry() {
        // The address key is the whole point: on a PE the resolver names both the
        // IAT slot and the `FF 25` thunk veneer, and the prototype has to land on
        // the thunk because that is the address a direct `call` resolves to.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/win32sigs_pe_i386.exe");
        let bytes = std::fs::read(path).expect("read win32sigs fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse win32sigs fixture");
        let pairs = imported_addrs_by_name(&file, &bytes);
        let addrs: Vec<u64> =
            pairs.iter().filter(|(n, _)| n == "LoadLibraryExW").map(|(_, a)| *a).collect();
        assert!(
            addrs.len() >= 2,
            "the resolver must name the IAT slot AND the thunk veneer: {addrs:x?}"
        );
    }

    #[test]
    fn an_elf_yields_nothing() {
        // PE/COFF only: an ELF that happens to define a matching spelling must be
        // untouched (the pass returns early on format).
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        assert_eq!(file.format(), object::BinaryFormat::Elf);
        let pairs = imported_addrs_by_name(&file, &bytes);
        assert!(
            !pairs.iter().any(|(n, _)| WIN32.iter().any(|(w, _)| w == n)),
            "no Win32 name should match an ELF import"
        );
    }
}
