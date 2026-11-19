//! (kuna) Compiler arithmetic runtime helpers as the C operators they implement.
//!
//! A 32-bit target with no instruction for an operation on `long long` (or, on
//! ARM, for any integer division) calls a runtime helper instead: GCC's libgcc
//! (`__udivdi3`, `__ashldi3`, ...), the ARM run-time ABI (`__aeabi_uldivmod`,
//! `__aeabi_idiv`, ...) or the MSVC CRT (`_alldiv`, `_allshl`, ...). Each one's
//! contract is documented (the libgcc manual's "Integer library routines", the
//! ARM "Run-time ABI for the Arm Architecture" section 4.2, Wine's ntdll
//! implementation of the MSVC helpers), so the call carries no information a C
//! operator does not. Without help it prints as an opaque call whose
//! eight-byte operands are split into four 32-bit words (`__udivdi3(a0,a1,a2,a3)`),
//! or, when the callee is an unresolved import, as an argument-less call whose
//! result nobody reads.
//!
//! Each helper is registered as a `<callfixup>`, the mechanism `kuna_msvcftol`
//! uses: the CALL is replaced with the helper's semantics in p-code, so the
//! arguments are ordinary register and stack reads and the result an ordinary
//! register write, and `a / b` falls out of the normal simplification. A helper
//! returning two results (`__aeabi_uldivmod` returns the quotient in r0:r1 and the
//! remainder in r2:r3) writes both; dead-code elimination keeps the one the
//! caller reads.
//!
//! Covered ABIs:
//! - x86-32: libgcc's cdecl helpers read their operands from the stack above the
//!   return address the CALL pushed, which the body pops first; the MSVC
//!   `_all*`/`_aull*` division and multiplication helpers are `__stdcall` and
//!   also pop their 16 argument bytes, the MSVC shift helpers take `EDX:EAX`
//!   and `CL`.
//! - 32-bit ARM, either byte order: AAPCS passes a `long long` in r0:r1 / r2:r3,
//!   low word in the lower register on little-endian and in the higher one on
//!   big-endian, and a shift count in r2.
//!
//! P-code shifts by at least the operand width yield zero (or the sign for
//! `s>>`), which is what `_allshl`/`_allshr` compute for counts of 64 and over;
//! the libgcc and AEABI helpers leave such counts undefined, as C does.

use kuna_base::error::KunaResult;

/// Prefix of every `<callfixup name=...>` this module registers; the installer
/// (`kuna_analysis::analyzers::callfixup`) skips them under `option arithhelpers off`.
pub const FIXUP_PREFIX: &str = "kuna_arithhelper_";

/// The helper calling convention of a language, or `None` when it has none here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperAbi {
    X86,
    Arm { big_endian: bool },
}

const X86_REGISTERS: [&str; 5] = ["EAX", "EDX", "ECX", "EBX", "ESP"];
const ARM_REGISTERS: [&str; 6] = ["r0", "r1", "r2", "r3", "lr", "cpsr"];

/// Pick the ABI from the registers the bodies name. Both are 32-bit targets: the
/// x86-64 language also resolves `EAX`..`ESP` as sub-registers, so the 4-byte
/// code space is what excludes it.
pub fn helper_abi(
    resolve_register: impl Fn(&[u8]) -> bool,
    code_space_addr_size: i32,
    big_endian: bool,
) -> Option<HelperAbi> {
    if code_space_addr_size != 4 {
        return None;
    }
    if X86_REGISTERS.iter().all(|r| resolve_register(r.as_bytes())) {
        return Some(HelperAbi::X86);
    }
    if ARM_REGISTERS.iter().all(|r| resolve_register(r.as_bytes())) {
        return Some(HelperAbi::Arm { big_endian });
    }
    None
}

struct Fixup {
    name: &'static str,
    targets: &'static [&'static str],
    body: String,
}

fn x86_fixups() -> Vec<Fixup> {
    let args = "ESP = ESP + 4; a:8 = *:8 ESP; b:8 = *:8 (ESP + 8);";
    let shift_args = "ESP = ESP + 4; a:8 = *:8 ESP; n:4 = *:4 (ESP + 8);";
    let ret = "EAX = r:4; EDX = r(4);";
    let edx_eax = "a:8 = (zext(EDX) << 32) | zext(EAX);";
    let cdecl = |name, targets, operands: &str, expr: &str| Fixup {
        name,
        targets,
        body: format!("{operands} r:8 = {expr}; {ret}"),
    };
    let stdcall = |name, targets, expr: &str| Fixup {
        name,
        targets,
        body: format!("{args} r:8 = {expr}; {ret} ESP = ESP + 16;"),
    };
    let divmod4 = |name, targets, div: &str, rem: &str| Fixup {
        name,
        targets,
        body: format!(
            "{args} p:4 = *:4 (ESP + 16); m:8 = a {rem} b; *:8 p = m; r:8 = a {div} b; {ret}"
        ),
    };
    let dvrm = |name, targets, div: &str, rem: &str| Fixup {
        name,
        targets,
        body: format!(
            "{args} r:8 = a {div} b; m:8 = a {rem} b; {ret} ECX = m:4; EBX = m(4); ESP = ESP + 16;"
        ),
    };
    let regshift = |name, targets, op: &str| Fixup {
        name,
        targets,
        body: format!("{edx_eax} r:8 = a {op} CL; {ret}"),
    };
    vec![
        cdecl("udiv64", &["__udivdi3"], args, "a / b"),
        cdecl("sdiv64", &["__divdi3"], args, "a s/ b"),
        cdecl("umod64", &["__umoddi3"], args, "a % b"),
        cdecl("smod64", &["__moddi3"], args, "a s% b"),
        cdecl("mul64", &["__muldi3"], args, "a * b"),
        cdecl("shl64", &["__ashldi3"], shift_args, "a << n"),
        cdecl("lshr64", &["__lshrdi3"], shift_args, "a >> n"),
        cdecl("ashr64", &["__ashrdi3"], shift_args, "a s>> n"),
        cdecl("neg64", &["__negdi2"], "ESP = ESP + 4; a:8 = *:8 ESP;", "-a"),
        divmod4("udivmod64", &["__udivmoddi4"], "/", "%"),
        divmod4("sdivmod64", &["__divmoddi4"], "s/", "s%"),
        stdcall("msvc_udiv64", &["__aulldiv"], "a / b"),
        stdcall("msvc_sdiv64", &["__alldiv"], "a s/ b"),
        stdcall("msvc_umod64", &["__aullrem"], "a % b"),
        stdcall("msvc_smod64", &["__allrem"], "a s% b"),
        stdcall("msvc_mul64", &["__allmul"], "a * b"),
        dvrm("msvc_udivrem64", &["__aulldvrm"], "/", "%"),
        dvrm("msvc_sdivrem64", &["__alldvrm"], "s/", "s%"),
        regshift("msvc_shl64", &["__allshl"], "<<"),
        regshift("msvc_lshr64", &["__aullshr"], ">>"),
        regshift("msvc_ashr64", &["__allshr"], "s>>"),
    ]
}

fn arm_fixups(big_endian: bool) -> Vec<Fixup> {
    let (lo, hi, blo, bhi) =
        if big_endian { ("r1", "r0", "r3", "r2") } else { ("r0", "r1", "r2", "r3") };
    let a = format!("a:8 = (zext({hi}) << 32) | zext({lo});");
    let b = format!("b:8 = (zext({bhi}) << 32) | zext({blo});");
    let ret = format!("{lo} = r:4; {hi} = r(4);");
    let rem = format!("{blo} = m:4; {bhi} = m(4);");
    let pair = |name, targets, expr: &str| Fixup {
        name,
        targets,
        body: format!("{a} {b} r:8 = {expr}; {ret}"),
    };
    let shift = |name, targets, op: &str| Fixup {
        name,
        targets,
        body: format!("{a} r:8 = a {op} r2; {ret}"),
    };
    let divmod = |name, targets, div: &str, md: &str| Fixup {
        name,
        targets,
        body: format!("{a} {b} r:8 = a {div} b; m:8 = a {md} b; {ret} {rem}"),
    };
    let word = |name, targets, expr: &str| Fixup {
        name,
        targets,
        body: format!("q:4 = {expr}; r0 = q;"),
    };
    let word_divmod = |name, targets, div: &str, md: &str| Fixup {
        name,
        targets,
        body: format!("q:4 = r0 {div} r1; m:4 = r0 {md} r1; r0 = q; r1 = m;"),
    };
    vec![
        divmod("udivmod64", &["__aeabi_uldivmod"], "/", "%"),
        divmod("sdivmod64", &["__aeabi_ldivmod"], "s/", "s%"),
        pair("udiv64", &["__udivdi3"], "a / b"),
        pair("sdiv64", &["__divdi3"], "a s/ b"),
        pair("umod64", &["__umoddi3"], "a % b"),
        pair("smod64", &["__moddi3"], "a s% b"),
        pair("mul64", &["__aeabi_lmul", "__muldi3"], "a * b"),
        shift("shl64", &["__aeabi_llsl", "__ashldi3"], "<<"),
        shift("lshr64", &["__aeabi_llsr", "__lshrdi3"], ">>"),
        shift("ashr64", &["__aeabi_lasr", "__ashrdi3"], "s>>"),
        Fixup { name: "neg64", targets: &["__negdi2"], body: format!("{a} r:8 = -a; {ret}") },
        word_divmod("udivmod32", &["__aeabi_uidivmod"], "/", "%"),
        word_divmod("sdivmod32", &["__aeabi_idivmod"], "s/", "s%"),
        word("udiv32", &["__aeabi_uidiv", "__udivsi3"], "r0 / r1"),
        word("sdiv32", &["__aeabi_idiv", "__divsi3"], "r0 s/ r1"),
        word("umod32", &["__umodsi3"], "r0 % r1"),
        word("smod32", &["__modsi3"], "r0 s% r1"),
    ]
}

fn fixups(abi: HelperAbi) -> Vec<Fixup> {
    match abi {
        HelperAbi::X86 => x86_fixups(),
        HelperAbi::Arm { big_endian } => arm_fixups(big_endian),
    }
}

/// The synthesized `<callfixup>` elements for `abi`, in the shape
/// `Architecture::decode_call_fixups` hands to `decode_inject`. Each helper is
/// also targeted with one more leading underscore: the installer retries a
/// function name with one and two underscores prepended, never removed, and
/// COFF and Mach-O spell the C name `__udivdi3` as `___udivdi3`.
pub fn payload_xml(abi: HelperAbi) -> Vec<String> {
    fixups(abi)
        .into_iter()
        .map(|f| {
            let targets: String = f
                .targets
                .iter()
                .flat_map(|t| [t.to_string(), format!("_{t}")])
                .map(|t| format!("<target name=\"{t}\"/>"))
                .collect();
            format!(
                "<callfixup name=\"{FIXUP_PREFIX}{}\">{targets}<pcode><body><![CDATA[{}]]></body></pcode></callfixup>",
                f.name, f.body
            )
        })
        .collect()
}

/// Parse each of [`payload_xml`]'s elements and hand it to `register`.
pub fn decode_payloads<F>(abi: HelperAbi, mut register: F) -> KunaResult<()>
where
    F: FnMut(&std::rc::Rc<kuna_base::xml::Element>) -> KunaResult<()>,
{
    use kuna_base::xml::DocumentStorage;
    for xml in payload_xml(abi) {
        let mut store = DocumentStorage::new();
        let root = store.parse_document(xml.as_bytes())?.get_root().clone();
        register(&root)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
