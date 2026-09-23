//! (kuna `structheadless`) A record whose first bytes a function never reads.
//!
//! # The gap
//!
//! `structsynth` requires an access at offset 0 before it believes a pointer
//! points at a record. A function that reads a record only past its first
//! member therefore keeps the pointee it had -- `void *` for a pointer known
//! only by its dereferences -- and every access costs two casts:
//!
//! ```text
//! unsigned int sub_53e7(void *a0)
//!   if ((dat_2b358) && (*(char *)((long)a0 + 0xb9)))
//!     v1 = *(unsigned int *)((long)a0 + 0xac);
//! ```
//!
//! `ls`'s `statx_to_stat` reads eighteen members of a `struct statx`, none of
//! them `stx_mask` at 0; `sort`'s key comparators read a `struct keyfield` from
//! `ignore` at 0x20 on. Over the 45 castbench binaries 1,238 parameters are
//! refused for this reason alone, against 1,716 accepted.
//!
//! # The rule
//!
//! Under `closed`, a parameter read at two or more constant offsets none of which
//! is zero is a record when every caller of its function is a known direct call
//! (`Funcdata::kuna_calleevote_closed`) and nothing outside the function gave
//! the parameter its pointee (`kuna_structsynth::pointee_is_given`). Every other
//! condition a parameter is held to still applies. The bytes before the first
//! access become `undefined1` filler, exactly as a hole does.
//!
//! # Why a closed function
//!
//! A function reached through a pointer takes the generic `void *` its contract
//! declares: a `qsort` comparator or a hash-table callback reads fields of what
//! it was handed exactly the way a record reader does. Against DWARF, the
//! headless parameters of functions whose address is stored are 202 struct
//! pointers and 53 `void *`; in closed functions they are 883 struct pointers,
//! no `void *`, and five others.
//!
//! # Why one more propagation pass
//!
//! `structsynth` reads the type lattice only once a propagation pass changed
//! nothing, and a function whose main loop ends on the pass that first typed it
//! is never offered. A record read at offset 0 gives the pointer-arithmetic
//! rules something to rewrite, so its function always iterates again; a headless
//! record gives them nothing, and 287 of the refused parameters sit in functions
//! that never iterate. Such a function asks for the one pass
//! `kuna_structsynth::wants_settle_pass` already asks for on behalf of a lone
//! field.

use kuna_base::error::{KunaError, KunaResult};

use crate::context::VarnodeId;
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;

/// (kuna) Whether a record needs an access at offset 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StructHeadlessMode {
    /// Upstream `structsynth`: a record is read at offset 0.
    #[default]
    Off,
    /// A closed function's parameter may be a record read only past its start.
    Closed,
}

impl StructHeadlessMode {
    /// Does the rule act at all?
    pub fn fires(self) -> bool {
        self != StructHeadlessMode::Off
    }
}

/// (kuna) Parse `option structheadless off|closed`; the caller writes the live field.
pub struct OptionStructHeadless;

impl OptionStructHeadless {
    /// The option name.
    pub const NAME: &'static str = "structheadless";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> KunaResult<(StructHeadlessMode, String)> {
        let mode = match p1 {
            "off" => StructHeadlessMode::Off,
            "closed" => StructHeadlessMode::Closed,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown structheadless value: {other} (expected off|closed)"
                )))
            }
        };
        Ok((mode, format!("Headless record synthesis set to {p1}")))
    }
}

/// May `base` be a record with no access at offset 0: a parameter of a function
/// whose callers are all known direct calls, under `closed`?
pub(crate) fn admits(data: &Funcdata, base: VarnodeId) -> bool {
    data.get_arch().struct_headless.fires()
        && data.kuna_calleevote_closed()
        && data.vbank().get(base).is_some_and(|v| v.is_input())
}

/// Does a declared call in `family` -- one that returns the value or takes it --
/// name the record the value points at, while `vote` is a callee's synthesized
/// record?  The declaration then outranks the recovery at the call site: `newgrp`
/// holds `getgrnam`'s `struct group *` and hands it to a function that reads the
/// group past its start, whose own record would otherwise retype the variable.
pub(crate) fn yields_to_a_declared_record(data: &Funcdata, family: &[VarnodeId], vote: &Datatype) -> bool {
    if !data.get_arch().struct_headless.fires() || !crate::kuna_structsynth::points_at_synthesized_record(vote) {
        return false;
    }
    let names_a_record = |t: &Datatype| {
        crate::kuna_structsynth::points_at_named_composite(t) && !crate::kuna_structsynth::points_at_synthesized_record(t)
    };
    family.iter().filter_map(|&v| data.vbank().get(v).map(|n| (v, n))).any(|(v, node)| {
        let written = node.get_def().and_then(|d| {
            let fc = data.get_call_specs(data.get_call_specs_index(d)?);
            let proto = fc.proto();
            (proto.is_output_locked() && data.obank().get(d)?.get_out() == Some(v))
                .then(|| proto.get_output_type().cloned())
                .flatten()
        });
        written.is_some_and(|t| names_a_record(&t))
            || node.descend_iter().any(|r| {
                let Some(o) = data.obank().get(r) else { return false };
                let Some(i) = data.get_call_specs_index(r) else { return false };
                let fc = data.get_call_specs(i);
                (1..o.num_input()).filter(|&s| o.get_in(s) == Some(v)).any(|s| {
                    fc.proto().get_param(s - 1).is_some_and(|p| {
                        p.is_type_locked() && p.get_type().is_some_and(|t| names_a_record(t))
                    })
                })
            })
    })
}

/// Do these offsets read as a record past its start: two or more, none of them
/// zero?
pub(crate) fn is_headless<'a>(mut offsets: impl Iterator<Item = &'a kuna_base::types::intb>) -> bool {
    let mut n = 0;
    offsets.all(|o| {
        n += 1;
        *o != 0
    }) && n >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_parses_both_modes_and_rejects_anything_else() {
        assert_eq!(OptionStructHeadless.apply("off").unwrap().0, StructHeadlessMode::Off);
        assert_eq!(OptionStructHeadless.apply("closed").unwrap().0, StructHeadlessMode::Closed);
        assert!(OptionStructHeadless.apply("on").is_err());
        assert!(!StructHeadlessMode::Off.fires());
        assert!(StructHeadlessMode::Closed.fires());
        assert_eq!(StructHeadlessMode::default(), StructHeadlessMode::Off);
    }

    #[test]
    fn two_offsets_past_the_start_are_headless_and_one_or_a_zero_is_not() {
        assert!(is_headless([0x30, 0xac, 0xb9].iter()));
        assert!(is_headless([8, 0x10].iter()));
        assert!(!is_headless([0x18].iter()));
        assert!(!is_headless([0, 8].iter()));
        assert!(!is_headless([].iter()));
    }
}
