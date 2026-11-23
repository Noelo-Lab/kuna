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

/// Options that print code as a call that is not in the binary:
/// `msvcstrappend` folds MSVC's inlined `std::string` appends into a call with
/// no address, `outline` a region into a call at no instruction. The machine
/// runs the code; kuna's call has no counterpart.
pub const SYNTHESIZED_CALL: Divergence = Divergence {
    id: "synthesized-call",
    option: "msvcstrappend, outline",
    arch: "",
    observable: "the calls a function makes",
    pattern: "kuna calls a function that has no address, or from no instruction",
    matches: |_| false,
};

/// `gowritebarrier` folds Go's write-barrier block, its call to
/// `runtime.gcWriteBarrier*` included, into the plain store it guards.
pub const GO_WRITE_BARRIER: Divergence = Divergence {
    id: "gowritebarrier",
    option: "gowritebarrier",
    arch: "",
    observable: "the calls a function makes",
    pattern: "the machine calls runtime.gcWriteBarrier*, which kuna folded away",
    matches: |_| false,
};

/// `gobounds` (Go) and `securitycheck` (Rust) remove the branch to a
/// bounds, slice or division check's panic helper, keeping the access it
/// guards. On an input that fails the check the machine calls the helper.
pub const PANIC_CHECK: Divergence = Divergence {
    id: "panic-check",
    option: "gobounds, securitycheck",
    arch: "",
    observable: "the calls a function makes",
    pattern: "the machine calls a Go or Rust check's panic helper, whose branch kuna removed",
    matches: |_| false,
};

/// `ptrauthtrap` cuts an arm64e pointer-authentication check: the bit test of
/// an authenticated return address and its `brk #0xc470..0xc473`, which an
/// authentic pointer never reaches. The machine does not authenticate (SLEIGH
/// hides `autibsp`), so the address it tests is whatever was loaded.
pub const PTRAUTH_TRAP: Divergence = Divergence {
    id: "ptrauth-trap",
    option: "ptrauthtrap",
    arch: "AARCH64",
    observable: "whether the function traps",
    pattern: "the machine reaches a pointer-authentication brk whose check kuna cut",
    matches: |_| false,
};

/// `repstring` (the default) prints a `rep movs` as `memcpy`, which the
/// instruction is not when the destination starts inside the source, ahead of
/// it: the instruction then repeats the bytes it copied first, as an LZ77
/// decoder's match copy relies on. Matched on a run whose `memcpy` builtin
/// meets that overlap.
pub const COPY_OVERLAP: Divergence = Divergence {
    id: "copy-overlap",
    option: "repstring",
    arch: "x86",
    observable: "memory a `rep movs` writes",
    pattern: "a memcpy builtin whose destination lies inside its source, ahead of it",
    matches: |_| false,
};

/// `nanignore compare` (the default) treats a float conversion round trip as
/// the identity, as upstream does, so a signalling NaN comes back as it went in
/// where the machine's conversions return it quieted.
pub const NAN_QUIETING: Divergence = Divergence {
    id: "nan-quieting",
    option: "nanignore",
    arch: "",
    observable: "a float return",
    pattern: "both sides return a NaN that differs only in its quiet bit",
    matches: |_| false,
};

/// `syscallregs auto` (the default) gives a system call its result register
/// only on an image the loader identified as an operating system's user-space
/// program; on any other image the call writes nothing, as upstream models it.
/// The machine is then run the same way, and this is declared when a kernel
/// writing the result register, as Linux and the BSDs do, would change what
/// the function returns, writes or calls.
pub const SYSCALL_RESULT: Divergence = Divergence {
    id: "syscall-result",
    option: "syscallregs",
    arch: "",
    observable: "a system call's result",
    pattern: "on an image that names no operating system, the result the kernel would return changes an observable",
    matches: |_| false,
};

/// Whether two float values of `size` bytes are NaNs that differ only in the
/// quiet bit.
pub fn nan_quieting(size: u32, a: u128, b: u128) -> bool {
    let (quiet, exp, frac) = match size {
        4 => (1u128 << 22, 0xffu128 << 23, (1u128 << 23) - 1),
        8 => (1u128 << 51, 0x7ffu128 << 52, (1u128 << 52) - 1),
        _ => return false,
    };
    let nan = |v: u128| v & exp == exp && v & frac != 0;
    a != b && nan(a) && nan(b) && a ^ b == quiet
}

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

    #[test]
    fn nan_quieting_is_the_quiet_bit_alone() {
        assert!(nan_quieting(4, 0xffc5_7572, 0xff85_7572));
        assert!(!nan_quieting(4, 0x7fc0_0000, 0x7fc0_0001));
        assert!(!nan_quieting(4, 0x3f80_0000, 0x3fc0_0000));
    }
}
