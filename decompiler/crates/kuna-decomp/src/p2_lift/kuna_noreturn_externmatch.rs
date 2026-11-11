//! (kuna) Name-matched no-return for unresolved externs, part of `noreturn_known`
//! (angr port: `test_decompiling_incorrect_duplication_chcon_main`;
//! angr CLE/CFGFast extern no-return tagging; S2 flow follow).
//!
//! ## The gap
//!
//! On a normal dynamically-linked ELF, the `noreturn_known` analysis pass
//! (`kuna-analysis` `loader/noreturn.rs`, the kuna analog of Ghidra's
//! `NoReturnFunctionAnalyzer`) marks `exit`/`abort`/`__stack_chk_fail`/… no-return
//! by scanning the symbol table and emitting a `NoReturnFact` keyed by the
//! function's **address**. The commit sets the `FunctionSymbol` proto's
//! no-return flag, so `FlowInfo::queryCall` plants an `artificialHalt(noreturn)`
//! after the call and flow never runs into the bytes past it.
//!
//! In an **ET_REL `.o`** that breaks down: `__stack_chk_fail` is an *undefined*
//! extern — its `.symtab` entry has `address()==0` and there is no PLT stub — so
//! the address-keyed scan emits no fact for it. The `relocobjects` loader
//! (default-on, DIV-7) binds the extern to a *synthetic* call target and installs
//! a `FunctionSymbol` named `__stack_chk_fail` there (the call site **prints**
//! `__stack_chk_fail()`), but no no-return flag is ever set on that synthetic
//! target. Flow then runs off the end of the function into the inter-function
//! alignment padding (`00 00 …` decoded as `add byte ptr [rax], al`), and the
//! decompiler emits dozens of garbage `*v = *v + c;` lines after the call — the
//! `chcon.o::main` symptom angr's `test_decompiling_incorrect_duplication_chcon_main`
//! does not have.
//!
//! ## What this does
//!
//! Under `noreturn_known`, the `FlowEnvironment::query_call_no_return`
//! hook (`infra/decompile_drive.rs`) additionally consults the callee **name** at
//! the direct-call entry: if the name (after stripping leading `_`, honoring the
//! upstream global/`std` namespace guard) matches the same vendored ELF
//! known-no-return list `noreturn_known` uses, the hook reports no-return. flow.rs
//! ORs this query with the proto flag at the artificial-halt site
//! (`let no_return = is_no_return || query_call_no_return(entry)`), so a name match
//! plants the halt and the trailing padding is never decoded — closing the gap by
//! name where the address-keyed scan could not.
//!
//! On a normal ELF the proto flag is already set by `noreturn_known`, so the OR is
//! a no-op (byte-identical output); the only behavioral changes are on ET_REL
//! undefined externs the address-keyed scan missed and on the proxy stubs below.
//! The list is `include_str!`'d from the canonical
//! `kuna-analysis/data/ElfFunctionsThatDoNotReturn` (a build-time string include,
//! *not* a crate dependency) so it can never drift from `noreturn_known`.
//!
//! ## Proxy stubs
//!
//! A patch or analysis-proxy stub often carries a runtime name behind a prefix
//! (`stub___stack_chk_fail`). The same match also treats a callee as no-return when
//! its name is `<prefix>__<entry>` for an exact list entry holding an `_` of its
//! own ([`is_proxy_noreturn_name`]) *and* its body in the image is a stub: the size
//! its symbol states is exactly one mapped instruction, an unconditional return or
//! jump ([`is_stub_body`]). A callee whose symbol states no size, or whose body
//! runs on past that instruction (an entry jump into the rest of its own body),
//! keeps the exact match alone. The name alone is not enough, since C also spells
//! ordinary functions `<object>__<verb>`: a `session__pthread_exit` with a real
//! body returns, and so do `machine__exit` and `ui__abort`, whose single-word
//! entries never take the prefix.
//!
//! A clean 0/675 datatest ablation (DIV-13: the corpus is either real ELFs, where
//! `noreturn_known` already set the proto flag and this is a no-op, or bytechunks
//! with no name-matched returning extern) and a ~23% speed *win* on the target
//! (less dead padding to decompile). Proxy stubs aside, it carries no risk class
//! beyond `noreturn_known` itself: it applies the *same* vendored name list and the
//! *same* global/`std` namespace guard, just at the flow query boundary to reach the
//! ET_REL extern that the address-keyed scan structurally misses, and it is
//! gated by that pass's flag.

use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use crate::architecture::Architecture;

/// Marshaling element `<noreturn_externmatch>` (kuna). ElementIds live in the
/// 4000+ range; 4103 (`noreturn_extern`, PR #90) is the previous max.
pub const ELEM_NORETURN_EXTERNMATCH: ElementId = ElementId::new("noreturn_externmatch", 4104);

/// The known-no-return name list, `include_str!`'d from the canonical vendored
/// file the `noreturn_known` analysis pass uses
/// (`kuna-analysis/data/ElfFunctionsThatDoNotReturn`, itself vendored verbatim
/// from Ghidra `Ghidra/Features/Base/data/ElfFunctionsThatDoNotReturn`). A
/// build-time string include — kuna-decomp does *not* depend on kuna-analysis;
/// this keeps a single source of truth so the two matchers can never drift.
const ELF_NORETURN_LIST: &str =
    include_str!("../../../kuna-analysis/data/ElfFunctionsThatDoNotReturn");

/// Strip **all** leading underscores (faithful to
/// `NoReturnFunctionAnalyzer.java:84-91` — a loop, not a single strip: so
/// `__stack_chk_fail` -> `stack_chk_fail` and `_exit` -> `exit`).
fn strip_leading_underscores(s: &str) -> &str {
    s.trim_start_matches('_')
}

/// Split a (possibly namespaced) symbol name into `(base, namespace)`. kuna
/// installs `::`-split namespaces, so a demangled `Menu::exit` has base `exit` in
/// namespace `Menu`; a flat C import `exit` has no namespace.
fn base_and_namespace(name: &str) -> (&str, Option<&str>) {
    match name.rfind("::") {
        Some(i) => (&name[i + 2..], Some(&name[..i])),
        None => (name, None),
    }
}

/// `true` if `name` matches the vendored ELF known-no-return list, mirroring the
/// `noreturn_known` matcher (`loader/noreturn.rs::name_matches`): the namespace
/// guard (only global / `std`, never a C++ class method like `Menu::_exit`),
/// leading-`_` strip, then exact or trailing-`*` wildcard-prefix match. A proxy
/// spelling `<prefix>__<entry>` is not matched here: [`is_proxy_noreturn_name`]
/// takes it, and only for a callee whose body passes [`is_stub_body`].
///
/// The parse is recomputed per call (the list is ~20 short lines; this is invoked
/// only from the gated `query_call_no_return` hook on direct CALLs). It mirrors
/// `parse_list`: skip `#`/blank lines, strip leading `_`, a trailing `*` makes a
/// wildcard prefix entry (the shipped ELF list has no wildcards, but the parse is
/// faithful).
pub fn is_known_noreturn_name(name: &str) -> bool {
    let (base, ns) = base_and_namespace(name);
    // Namespace guard: skip class methods. Global (no namespace) is allowed; the
    // sole allowed non-global namespace is exactly `std`.
    if let Some(ns) = ns {
        if ns != "std" {
            return false;
        }
    }
    let stripped = strip_leading_underscores(base);
    if stripped.is_empty() {
        return false;
    }
    for line in ELF_NORETURN_LIST.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let entry = strip_leading_underscores(line);
        if let Some(prefix) = entry.strip_suffix('*') {
            if stripped.starts_with(prefix) {
                return true;
            }
        } else if entry == stripped {
            return true;
        }
    }
    false
}

/// `true` if `name` is a proxy spelling `<prefix>__<entry>`: the text after the last
/// `__` is an exact list entry holding an `_` of its own, under the same namespace
/// guard as [`is_known_noreturn_name`].
pub fn is_proxy_noreturn_name(name: &str) -> bool {
    let (base, ns) = base_and_namespace(name);
    if ns.is_some_and(|ns| ns != "std") {
        return false;
    }
    let Some((_, proxied)) = strip_leading_underscores(base).rsplit_once("__") else {
        return false;
    };
    proxied.contains('_')
        && ELF_NORETURN_LIST
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .any(|line| strip_leading_underscores(line) == proxied)
}

/// `true` if the callee at `entry` is a whole stub body in the image: its symbol
/// states a size, and those bytes, all mapped, decode as exactly one instruction
/// that is an unconditional return or jump with no control transfer before it.
pub fn is_stub_body(arch: &Architecture, entry: &Address) -> bool {
    let code = arch.manage().get_default_code_space().map(|space| space.get_index());
    if entry.get_space().map(|space| space.get_index()) != code {
        return false;
    }
    let Ok(at) = arch.func_sizes.binary_search_by_key(&entry.get_offset(), |&(addr, _)| addr) else {
        return false;
    };
    let translate = arch.translate();
    let Some(image) = translate.loader_rc().try_borrow().ok().and_then(|loader| loader.shared_bytes()) else {
        return false;
    };
    let mut ops = EntryOps::default();
    let len = translate.one_instruction_checked(&mut ops, entry, image.as_ref());
    let Some((&(last, relative), rest)) = ops.0.split_last() else {
        return false;
    };
    len.is_ok_and(|len| u64::try_from(len) == Ok(arch.func_sizes[at].1))
        && matches!(last, OpCode::CPUI_RETURN | OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND)
        && !relative
        && !rest.iter().any(|&(op, _)| is_transfer(op))
}

fn is_transfer(op: OpCode) -> bool {
    matches!(
        op,
        OpCode::CPUI_BRANCH
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_RETURN
    )
}

/// Each op of one instruction, with whether its first input is a constant (a
/// p-code-relative branch).
#[derive(Default)]
struct EntryOps(Vec<(OpCode, bool)>);

impl PcodeEmit for EntryOps {
    fn dump(&mut self, _addr: &Address, opc: OpCode, _outvar: Option<&VarnodeData>, vars: &[VarnodeData]) {
        let relative = vars
            .first()
            .and_then(|v| v.space.as_ref())
            .is_some_and(|s| s.get_type() == spacetype::IPTR_CONSTANT);
        self.0.push((opc, relative));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::error::KunaResult;
    use kuna_sleigh::globalcontext::ContextInternal;
    use kuna_sleigh::loadimage::{ImageBytes, LoadImage};
    use kuna_sleigh::sleigh::Sleigh;
    use kuna_sleigh::translate::Translate;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn matches_canary_and_abort_family() {
        // The chcon.o::main target and its neighbours in the vendored list.
        assert!(is_known_noreturn_name("__stack_chk_fail"));
        assert!(is_known_noreturn_name("stack_chk_fail"));
        assert!(is_known_noreturn_name("abort"));
        assert!(is_known_noreturn_name("exit"));
        assert!(is_known_noreturn_name("_exit"));
        assert!(is_known_noreturn_name("__assert_fail"));
    }

    #[test]
    fn namespace_guard_rejects_class_methods_but_allows_std() {
        assert!(!is_known_noreturn_name("Menu::exit"));
        assert!(!is_known_noreturn_name("Menu::_exit"));
        // The guard allows the `std` namespace; `exit` is in the vendored list.
        assert!(is_known_noreturn_name("std::exit"));
    }

    /// (kuna DIV-78, GH-273) The libstdc++ `std::__throw_*` family reaches this
    /// matcher in its DEMANGLED form (`query_call` resolves the display name), so
    /// the plain half of the list entry is what fires here; the mangled half is
    /// what the analysis-tier scan matches pre-demangle.
    #[test]
    fn matches_cxx_throw_family_demangled_and_mangled() {
        for want in [
            "std::__throw_length_error",
            "std::__throw_out_of_range",
            "std::__throw_bad_alloc",
            "std::__throw_bad_function_call",
            "__throw_logic_error",
            "_ZSt20__throw_length_errorPKc",
            "_ZSt24__throw_out_of_range_fmtPKcz",
        ] {
            assert!(is_known_noreturn_name(want), "{want} must be no-return");
        }
        // The namespace guard still rejects a same-named user class method.
        assert!(!is_known_noreturn_name("Buf::__throw_length_error"));
        // A user function that merely mentions `throw` is untouched.
        assert!(!is_known_noreturn_name("throw_it"));
        assert!(!is_known_noreturn_name("rethrow"));
    }

    #[test]
    fn proxy_names_take_multi_word_entries_only() {
        for want in [
            "stub___stack_chk_fail",
            "stub__stack_chk_fail",
            "__wrap___stack_chk_fail",
            "__GI___assert_fail",
            "session__pthread_exit",
            "std::stub___stack_chk_fail",
        ] {
            assert!(is_proxy_noreturn_name(want), "{want} is a proxy spelling");
            assert!(!is_known_noreturn_name(want), "{want} needs the stub check");
        }
        for keep in [
            "machine__exit",
            "ui__abort",
            "__stack_chk_fail",
            "mystack_chk_fail_count",
            "handle_exit",
            "stub__stack_chk_fail_count",
            "Menu::stub___stack_chk_fail",
        ] {
            assert!(!is_proxy_noreturn_name(keep), "{keep} is not a proxy spelling");
        }
    }

    /// Bytes mapped at 0x1000, read as the loaders do: zero-filled past the end.
    #[derive(Debug)]
    struct Mapped(Vec<u8>);

    impl ImageBytes for Mapped {
        fn fill_span(&self, dst: &mut [u8], start: u64) -> usize {
            if !self.mapped_covers(start, start + 1) {
                return dst.len();
            }
            for (at, byte) in (start..).zip(dst.iter_mut()) {
                *byte = self.0.get((at - 0x1000) as usize).copied().unwrap_or(0);
            }
            0
        }

        fn mapped_covers(&self, lo: u64, hi: u64) -> bool {
            lo >= 0x1000 && hi <= 0x1000 + self.0.len() as u64
        }
    }

    struct StubImage(Arc<Mapped>);

    impl LoadImage for StubImage {
        fn get_file_name(&self) -> &str {
            "stub"
        }
        fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
            self.0.fill_span(ptr, addr.get_offset());
            Ok(())
        }
        fn get_arch_type(&self) -> Vec<u8> {
            Vec::new()
        }
        fn adjust_vma(&mut self, _adjust: i64) {}
        fn shared_bytes(&self) -> Option<Arc<dyn ImageBytes>> {
            Some(Arc::clone(&self.0) as Arc<dyn ImageBytes>)
        }
    }

    /// [`is_stub_body`] for an x86-64 function of `size` bytes at 0x1000 over `bytes`.
    fn stub_body(bytes: &[u8], size: u64) -> bool {
        let sla = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../specs/Ghidra/Processors/x86/data/languages/x86-64.sla");
        let sla = std::fs::read(&sla).expect("x86-64.sla is built");
        let image = StubImage(Arc::new(Mapped(bytes.to_vec())));
        let mut sleigh = Sleigh::new(Box::new(image), Box::new(ContextInternal::new()));
        sleigh.initialize_from_sla(&sla).expect("x86-64.sla initializes");
        for (name, val) in [("addrsize", 2), ("opsize", 1), ("longMode", 1)] {
            sleigh.set_context_default(name, val);
        }
        let mut arch = Architecture::new("x86:LE:64:default", sleigh);
        arch.func_sizes = Rc::new(vec![(0x1000, size)]);
        let code = Rc::clone(arch.manage().get_default_code_space().expect("default code space"));
        is_stub_body(&arch, &Address::new(code, 0x1000))
    }

    #[test]
    fn a_stub_body_is_mapped_in_full() {
        assert!(stub_body(&[0xc2, 0, 0], 3), "a mapped ret imm16 is a stub");
        assert!(!stub_body(&[0xc2], 3), "a ret imm16 past the mapping is not");
    }

    #[test]
    fn rejects_ordinary_returning_names() {
        assert!(!is_known_noreturn_name("process_files"));
        assert!(!is_known_noreturn_name("main"));
        assert!(!is_known_noreturn_name("error"));
        assert!(!is_known_noreturn_name(""));
        assert!(!is_known_noreturn_name("_"));
    }
}
