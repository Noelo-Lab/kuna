//! (kuna) `entrymainproto` — give the function the MSVC CRT startup calls with
//! argc/argv/envp the prototype that call site establishes (P1 program prep).
//!
//! kuna recovers a callee's parameters from the callee's own body: a register
//! the ABI reserves for argument `i` that is READ before it is written is
//! parameter `i`. That is why `sub_140004e8c(int a0)` comes out right on a
//! stripped PE — its body reads `ecx`. It is also why `main` comes out
//! `void(void)`: a `main` that ignores `argc`/`argv` never reads `rcx`/`rdx`/`r8`,
//! so there is nothing for body-driven recovery to find, and the agent reading
//! the output sees a callee declared to take nothing being called with three
//! arguments a few lines up in its caller.
//!
//! The entry point is the one place where the arguments are visible *without*
//! the body, because on PE the C runtime startup is INSIDE the image and kuna
//! already decompiles it correctly. MSVC's `__scrt_common_main_seh` fetches each
//! argument through a named CRT accessor immediately before the call:
//!
//! ```text
//!   call _get_initial_narrow_environment ; mov rdi,rax    -> envp
//!   call __p___argv                      ; mov rbx,[rax]  -> argv
//!   call __p___argc                      ; mov ecx,[rax]  -> argc
//!   mov  r8,rdi ; mov rdx,rbx
//!   call <main>
//! ```
//!
//! In a UCRT image those names are called from the startup and from nowhere else,
//! so the window between the first accessor call and the next direct call to a
//! non-accessor names the program's entry function (`main`, `wmain`, or a
//! `WinMain` reached the same way). MinGW's msvcrt shim calls two of them as
//! well, which is what [`REJECTORS`] is for.
//!
//! ## What is asserted, and what is not
//!
//! The parameters are typed at the width the call site establishes — the 4-byte
//! `argc` slot and the two pointer-width slots — and named after the accessor
//! that produced each. They are deliberately NOT typed `int` / `char **`: that
//! would assert the C library's declaration of `main`, which this pass has no
//! evidence for (the same shape carries `wmain`'s `wchar_t **`, and a hand-rolled
//! entry point need not be `main` at all). The names carry the meaning; the types
//! stay at the level of what was observed, which is also how every other kuna
//! prototype recovered from code reads.
//!
//! The prototype is parked by NAME on the callee, through the ordinary
//! `AnalysisOutput::prototypes` seam the libc-signature pass uses, so the commit
//! boundary applies it exactly like any other recovered signature.
//!
//! ## Guards
//!
//! * PE only, and the reason is the evidence rather than the symptom. The symptom
//!   is general: a stripped ELF whose `main` ignores its arguments comes out
//!   `sub_1149(void)` too (`gcc -O1 int main(int,char**){puts("hi");return 42;}`,
//!   stripped). But on ELF the CRT lives in libc — `_start` hands `main` to
//!   `__libc_start_main` and the argument passing happens in another image — so
//!   there is no call site in the object to read. kuna's ELF `main` oracle finds
//!   the address; asserting three slots there would be quoting the C convention,
//!   not observing a caller, which is a weaker claim than this pass makes.
//! * The callee must carry **no function symbol**. A named `main` — from
//!   `.symtab`, an export, a PDB, or DWARF — already has, or will get, a better
//!   signature from that source, and this pass must not overwrite it.
//! * At least the `argc` and `argv` accessors must appear in the window; `envp`
//!   adds its slot only when its own accessor is there, so a startup that passes
//!   two arguments is described with two.
//! * The callee must be inside an executable section and must not be one of the
//!   accessors itself.
//! * A call to msvcrt's `__getmainargs` shim family inside the window abandons
//!   the cluster: MinGW reaches the same three values through OUT pointers and
//!   its shim calls `__p___argc`/`__p___argv` too, so without this the scan
//!   matches inside it.
//!
//! ## The measured cost
//!
//! Declaring `argc` makes the first ABI argument register live at the entry
//! function's own entry, so a call there to an import kuna has no prototype for
//! now finds a value in it and renders `IsDebuggerPresent(CONCAT44(dat_c,argc))`
//! where it used to render `IsDebuggerPresent()`. That is the standing behaviour
//! at any unprototyped callee reached with a live argument register, and the real
//! answer is a Win32 prototype table beside the libc ones, not withholding the
//! entry prototype. Measured over 139 PE crackmes: the scan locates a candidate
//! on 37 images, the guards reject 7, and of the 30 that fire 4 gain one such
//! argument.
//!
//! One interaction is worth stating, because it is silent. The prototype is
//! parked by NAME, and the name is minted here, at LOAD, from the naming policy
//! in force then (`sub_<addr>`). `--option namestyle` is applied AFTER the load,
//! so under a non-default naming policy the commit registers the callee under a
//! different spelling and the park finds nothing. That is a no-op, never a wrong
//! prototype — the alternative, minting a `sub_<addr>` name that contradicts the
//! policy the user asked for, is worse.
//!
//! Default-**on**; the facts are computed at load and COMMITTED only when the
//! gate is on, so `--option entrymainproto off` restores the `void(void)` form
//! exactly.

use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::type_metatype;
use kuna_decomp::fspec::PrototypePieces;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

use super::{executable_sections, in_executable_section};

/// (kuna) The PE CRT entry-function prototype pass (`entrymainproto`).
///
/// Registered like every other analysis pass and computed at LOAD; the commit
/// boundary applies its one prototype only when `--option entrymainproto on`
/// (the default) let this pass's output through the gate, so `off` renders the
/// `void(void)` form exactly.
pub struct EntryMainProtoPass;

impl AnalysisPass for EntryMainProtoPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "entrymainproto"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        if let Some((addr, pieces)) = entry_main_prototype(ctx) {
            // The prototype is parked BY NAME, so the entry function has to be a
            // registered function for it to land. It usually already is (it is a
            // direct CALL target from code the walk reaches), but on an obfuscated
            // image whose prologue no oracle recognizes it is not, and the park
            // would be a silent no-op. Emitting the address as an entry is the same
            // evidence the prototype rests on -- the C runtime calls it -- so the
            // two travel together.
            out.entries.push(addr);
            out.prototypes.push(pieces);
        }
        out
    }
}

/// The MSVC CRT accessors `__scrt_common_main_seh` reads each `main` argument
/// through, mapped to the parameter slot each one feeds.
///
/// `_get_initial_wide_environment` / `__p___wargv` are the `wmain` spellings of
/// the same three fetches; the slot layout is identical.
///
/// The list is deliberately confined to the **UCRT** spellings, the ones
/// `__scrt_common_main_seh` uses. The msvcrt-era environment accessors belong to
/// a different idiom and are handled by [`REJECTORS`] instead.
const ACCESSORS: &[(&str, Slot)] = &[
    ("__p___argc", Slot::Argc),
    ("__p___argv", Slot::Argv),
    ("__p___wargv", Slot::Argv),
    ("_get_initial_narrow_environment", Slot::Envp),
    ("_get_initial_wide_environment", Slot::Envp),
];

/// Names whose presence in the window means this is NOT the UCRT startup, and the
/// cluster is abandoned.
///
/// MinGW links against msvcrt and reaches the same three values through
/// `__getmainargs`, which fills them via OUT pointers. Its shim calls
/// `__p___argc` and `__p___argv` too, so the accessor test alone matches inside
/// it — and the next direct call there is `_set_new_mode`, not `main`. Seven
/// images in the crackmes corpus have that shape. The unnamed-callee guard below
/// happens to reject all seven (every candidate it produces is a named import),
/// but that is luck rather than reasoning: on an image where the following call
/// landed on a local function the match would stand. Naming the shim's own
/// accessors and bailing on them is the guard that actually holds.
const REJECTORS: &[&str] = &["__p__environ", "__p__wenviron", "__getmainargs", "__wgetmainargs"];

/// Which `main` parameter an accessor call supplies.
#[derive(Copy, Clone, PartialEq, Eq)]
enum Slot {
    Argc,
    Argv,
    Envp,
}

/// How far past the last accessor call the `call main` may sit. The MSVC
/// sequence is three calls plus their register moves — 29 bytes on the witness;
/// the bound is generous enough for an inlined `__scrt_initialize_onexit_tables`
/// tail without reaching a second, unrelated call cluster.
const WINDOW: u64 = 96;

/// The entry function's address and the prototype recovered for it, or `None`
/// when the image is not a PE, carries no MSVC CRT startup, or names its entry
/// function already.
///
/// An image that matches nothing contributes nothing, so the commit is
/// byte-identical to before.
fn entry_main_prototype(ctx: &AnalysisCtx) -> Option<(u64, PrototypePieces)> {
    if ctx.file.format() != object::BinaryFormat::Pe {
        return None;
    }
    let execs = executable_sections(ctx.file);
    let (accessors, rejectors) = accessor_addrs(ctx);
    if accessors.is_empty() {
        return None;
    }
    let (main, slots) = find_main_call(&execs, &accessors, &rejectors)?;
    if !in_executable_section(&execs, main) {
        return None;
    }
    // A named entry function has a better signature coming from whatever named it.
    if super::existing_function_addrs(ctx.file, ctx.bytes).binary_search(&main).is_ok() {
        return None;
    }
    Some((main, prototype_for(ctx.arch, main, &slots)?))
}

/// `(vma, slot)` for every CRT argument accessor the image imports, and the bare
/// VMAs of the [`REJECTORS`], at every address a direct CALL can reach either by
/// (the import thunk and the IAT slot both carry the name).
fn accessor_addrs(ctx: &AnalysisCtx) -> (Vec<(u64, Slot)>, Vec<u64>) {
    let mut out: Vec<(u64, Slot)> = Vec::new();
    let mut reject: Vec<u64> = Vec::new();
    for sym in crate::loader::format::resolve_imports(ctx.file, ctx.bytes) {
        let name = String::from_utf8_lossy(&sym.name).into_owned();
        if let Some((_, slot)) = ACCESSORS.iter().find(|(n, _)| *n == name) {
            out.push((sym.addr, *slot));
        } else if REJECTORS.contains(&name.as_str()) {
            reject.push(sym.addr);
        }
    }
    out.sort_by_key(|(a, _)| *a);
    out.dedup_by_key(|(a, _)| *a);
    reject.sort_unstable();
    reject.dedup();
    (out, reject)
}

/// Walk the executable bytes for the accessor cluster and return the callee of
/// the first direct CALL after it, with the slots the cluster established.
///
/// This is a byte scan for `E8 rel32`, not a disassembly: the CRT startup is
/// ordinary compiler output with no overlapping encodings, and a false `E8`
/// inside an immediate cannot pass the test below — it would have to point at a
/// CRT accessor by accident and then be followed by a call to mapped code.
fn find_main_call(
    execs: &[(u64, u64, Vec<u8>)],
    accessors: &[(u64, Slot)],
    rejectors: &[u64],
) -> Option<(u64, Vec<Slot>)> {
    let slot_at = |t: u64| accessors.iter().find(|(a, _)| *a == t).map(|(_, s)| *s);
    for (lo, hi, data) in execs {
        let mut i = 0usize;
        let mut seen: Vec<Slot> = Vec::new();
        let mut last_accessor: Option<u64> = None;
        while i + 5 <= data.len() {
            if data[i] != 0xE8 {
                i += 1;
                continue;
            }
            let vma = lo + i as u64;
            let disp = i32::from_le_bytes([data[i + 1], data[i + 2], data[i + 3], data[i + 4]]);
            let target = (vma + 5).wrapping_add(disp as i64 as u64);
            match slot_at(target) {
                Some(slot) => {
                    if last_accessor.is_none_or(|prev| vma - prev <= WINDOW) {
                        if !seen.contains(&slot) {
                            seen.push(slot);
                        }
                    } else {
                        seen = vec![slot];
                    }
                    last_accessor = Some(vma);
                }
                None if rejectors.binary_search(&target).is_ok() => {
                    seen.clear();
                    last_accessor = None;
                }
                None => {
                    let in_window = last_accessor.is_some_and(|prev| vma - prev <= WINDOW);
                    if in_window
                        && seen.contains(&Slot::Argc)
                        && seen.contains(&Slot::Argv)
                        && target >= *lo
                        && target < *hi
                    {
                        return Some((target, seen));
                    }
                    if !in_window {
                        seen.clear();
                        last_accessor = None;
                    }
                }
            }
            i += 5;
        }
    }
    None
}

/// Build the `PrototypePieces` for the entry function at `main`.
///
/// Slots are emitted in ABI order (`argc`, `argv`, `envp`) regardless of the
/// order the CRT fetched them in, and only for the ones actually observed. The
/// return type is the 4-byte integer the startup reads back into its exit-code
/// register.
fn prototype_for(arch: &Architecture, main: u64, slots: &[Slot]) -> Option<PrototypePieces> {
    let space = arch.manage().get_default_code_space()?;
    let addr = kuna_base::address::Address::new(std::rc::Rc::clone(space), main);
    let name = arch.name_function(&addr);

    let types = arch.types();
    let uint4 = types.get_base(4, type_metatype::TYPE_UINT).ok()?;
    let ptr_size = space.get_addr_size() as kuna_base::types::int4;
    let uintp = types.get_base(ptr_size, type_metatype::TYPE_UINT).ok()?;

    let mut intypes = Vec::new();
    let mut innames = Vec::new();
    for (slot, slot_name, ty) in [
        (Slot::Argc, "argc", &uint4),
        (Slot::Argv, "argv", &uintp),
        (Slot::Envp, "envp", &uintp),
    ] {
        if slots.contains(&slot) {
            intypes.push(std::rc::Rc::clone(ty));
            innames.push(slot_name.to_string());
        }
    }
    if intypes.len() < 2 {
        return None;
    }
    Some(PrototypePieces {
        name,
        outtype: Some(uint4),
        intypes,
        innames,
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    })
}
