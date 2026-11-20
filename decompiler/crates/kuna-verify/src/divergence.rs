//! Declared divergences: defaults that knowingly trade fidelity for
//! readability.
//!
//! An entry is a predicate over the option, the architecture, the observable it
//! changes and the pattern of the difference, never an option name alone, so a
//! different difference under the same option is still `DIFFERS`.

/// A return-value difference against a typed contract.
pub struct ReturnMismatch<'a> {
    pub arch: &'a str,
    pub contract_size: u32,
    pub kuna_size: u32,
    pub machine: u128,
    pub kuna: u128,
}

pub struct Divergence {
    pub id: &'static str,
    /// The option that makes the trade, or the always-on check that does.
    pub option: &'static str,
    /// The architecture id prefix it applies to; empty for every target.
    pub arch: &'static str,
    pub observable: &'static str,
    pub pattern: &'static str,
    matches: fn(&ReturnMismatch) -> bool,
}

/// `switchrunon` cuts the rows of a bounds-free jump table that run on into the
/// next functions and dispatches none of them. A selector those rows served is
/// one the source never passes (a Rust enum's byte, C's unreachable default),
/// and on it the machine runs a neighbouring function's code. Matched when a
/// switch meets a selector no row carries and the machine's indirect branch
/// lands outside every block of the decompiled function.
pub const SWITCH_RUNON: Divergence = Divergence {
    id: "switchrunon-cut-row",
    option: "switchrunon (always on)",
    arch: "",
    observable: "the path a switch takes",
    pattern: "the selector matches no row of a table whose run-on rows were cut",
    matches: |_| false,
};

/// Options that print inlined library code as a call to a function with no
/// address (`msvcstrappend` folds MSVC's inlined `std::string` appends into
/// one). The machine runs the inline code; kuna's call has no counterpart.
pub const SYNTHESIZED_CALL: Divergence = Divergence {
    id: "synthesized-call",
    option: "msvcstrappend",
    arch: "",
    observable: "the calls a function makes",
    pattern: "kuna calls a function that has no address",
    matches: |_| false,
};

impl Divergence {
    pub fn matches(&self, m: &ReturnMismatch) -> bool {
        m.arch.starts_with(self.arch) && (self.matches)(m)
    }
}

pub const DECLARED: &[Divergence] = &[Divergence {
    id: "returnpair-rdx-zero",
    option: "returnpair",
    arch: "x86:LE:64",
    observable: "the RDX half of a 16-byte return",
    pattern: "machine RDX = 0 and RAX agrees",
    matches: |m| {
        m.contract_size == 16 && m.kuna_size == 8 && (m.machine >> 64) == 0 && m.machine as u64 == m.kuna as u64
    },
}];

pub fn find(m: &ReturnMismatch) -> Option<&'static Divergence> {
    DECLARED.iter().find(|d| d.matches(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rdx_trim_matches_only_its_pattern() {
        let m = |arch, machine: u128, kuna: u128| ReturnMismatch { arch, contract_size: 16, kuna_size: 8, machine, kuna };
        assert!(find(&m("x86:LE:64:default", 0x8000_0000, 0xffff_ffff_ffff_ffff_0000_0000_8000_0000)).is_some());
        assert!(find(&m("x86:LE:64:default", 1u128 << 64, 0)).is_none());
        assert!(find(&m("x86:LE:64:default", 5, 6)).is_none());
        assert!(find(&m("ARM:LE:32:v8", 0x8000_0000, 0x8000_0000)).is_none());
    }
}
