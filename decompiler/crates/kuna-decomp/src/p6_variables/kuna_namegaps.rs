//! A caller's rename leaves its default number unused.
//!
//! The naming pass numbers unnamed locals in location order (`v1`, `v2`, ... or
//! `iVar1` under Ghidra names). A `name`/`type` directive is read against the
//! first decompile's output and applied by a second, where the renamed local
//! binds its new name and takes no number, so every later local used to move
//! down one: after `name v9 idx` the old `v10` printed as `v9`, and a following
//! directive naming `v9` reached a different variable than the caller saw.
//!
//! [`carry`] records what the first decompile's renames did, and the next
//! decompile's allocator (`varmap.rs (ScopeLocal::next_default_name)`) consults
//! it: the numbers renamed locals held are skipped, and a local whose number a
//! rename took (`name v3 v7` takes `v7`) gets the number the renamed local left
//! or, when it left none (a parameter or a named local renamed to `v7`), the
//! first number past the previous decompile's last. Without a rename the gaps
//! are empty and numbering is unchanged.

use std::collections::BTreeSet;

use kuna_base::types::int4;

use crate::funcdata::Funcdata;

/// What one decompile's renames did to the default numbering, handed to the next.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NameGaps {
    /// The default indices renamed locals held.
    pub vacated: BTreeSet<int4>,
    /// The default-shaped names a rename gave a local.
    pub given: BTreeSet<String>,
    /// One past the last default index the decompile handed out.
    pub end: int4,
}

impl NameGaps {
    pub fn is_empty(&self) -> bool {
        self.vacated.is_empty() && self.given.is_empty()
    }
}

/// The index of a default local name, `v9` -> 9 (or `iVar9` -> 9 under Ghidra
/// names); `None` for any other name.
pub fn default_name_index(name: &str, angr: bool) -> Option<int4> {
    let digits = if angr {
        name.strip_prefix('v')?
    } else {
        let at = name.find("Var")?;
        if name[..at].chars().count() > 1 {
            return None;
        }
        &name[at + 3..]
    };
    if digits.is_empty() || digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse().ok()
}

/// The allocator's view of [`NameGaps`] during one naming pass.
#[derive(Clone, Debug, Default)]
pub struct GapState {
    gaps: NameGaps,
    handed: BTreeSet<int4>,
    overflow: int4,
}

impl GapState {
    pub fn new(gaps: NameGaps) -> GapState {
        let overflow = gaps.end;
        GapState { gaps, handed: BTreeSet::new(), overflow }
    }

    pub fn is_empty(&self) -> bool {
        self.gaps.is_empty()
    }

    /// Must the allocator pass over index `i`?
    pub fn skips(&self, i: int4) -> bool {
        self.gaps.vacated.contains(&i) || self.handed.contains(&i)
    }

    /// Did a rename take `name` from the local about to receive it?
    pub fn displaces(&self, name: &str) -> bool {
        self.gaps.given.contains(name)
    }

    /// The index for a displaced local: the lowest vacated one no one holds,
    /// else the next past [`NameGaps::end`]. `free` says whether a rendered
    /// index is unclaimed by a locked Symbol.
    pub fn displaced_index(&mut self, render: &dyn Fn(int4) -> String, free: &dyn Fn(&str) -> bool) -> int4 {
        let open = |i: int4, gaps: &NameGaps| {
            let name = render(i);
            !gaps.given.contains(&name) && free(&name)
        };
        let vacant = self.gaps.vacated.iter().copied().find(|i| !self.handed.contains(i) && open(*i, &self.gaps));
        let index = match vacant {
            Some(i) => i,
            None => loop {
                let i = self.overflow;
                self.overflow += 1;
                if !self.skips(i) && open(i, &self.gaps) {
                    break i;
                }
            },
        };
        self.handed.insert(index);
        index
    }

    /// One past every index this pass handed a displaced local.
    pub fn overflow(&self) -> int4 {
        self.overflow
    }
}

/// The gaps the next decompile of `fd`'s function must leave: those `fd` was
/// decompiled with, plus what the directives applied to it since renamed.
pub fn carry(fd: &Funcdata) -> NameGaps {
    let prior = fd.kuna_name_gaps();
    let mut next = NameGaps {
        vacated: prior.vacated.clone(),
        given: prior.given.clone(),
        end: prior.end.max(fd.kuna_default_name_end()),
    };
    let Some(lm) = fd.get_scope_local() else { return next };
    let angr = fd.get_arch().name_style_angr;
    for directive in fd.kuna_directive_symbols() {
        let symbol = lm.database().symbol(directive.symbol);
        if symbol.is_name_undefined() || !symbol.is_name_locked() {
            continue;
        }
        if !next.given.remove(&directive.printed) {
            if let Some(i) = default_name_index(&directive.printed, angr) {
                next.vacated.insert(i);
            }
        }
        if default_name_index(&symbol.name, angr).is_some() {
            next.given.insert(symbol.name.clone());
        }
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_default_shapes_have_an_index() {
        assert_eq!(default_name_index("v9", true), Some(9));
        assert_eq!(default_name_index("v12", true), Some(12));
        for name in ["v", "v0", "v09", "vx", "idx", "a1", "v1_1", "iVar3"] {
            assert_eq!(default_name_index(name, true), None, "{name}");
        }
        assert_eq!(default_name_index("iVar3", false), Some(3));
        assert_eq!(default_name_index("Var3", false), Some(3));
        for name in ["myVar3", "v3", "iVar", "iVar03"] {
            assert_eq!(default_name_index(name, false), None, "{name}");
        }
    }

    #[test]
    fn a_displaced_local_takes_a_vacated_index_then_one_past_the_end() {
        let gaps = NameGaps {
            vacated: [3].into_iter().collect(),
            given: ["v7".to_string(), "v8".to_string()].into_iter().collect(),
            end: 12,
        };
        let mut state = GapState::new(gaps);
        let render = |i: int4| format!("v{i}");
        assert!(state.skips(3) && !state.skips(4));
        assert!(state.displaces("v7") && !state.displaces("v3"));
        assert_eq!(state.displaced_index(&render, &|_| true), 3);
        assert_eq!(state.displaced_index(&render, &|name| name != "v12"), 13);
        assert!(state.skips(13));
        assert_eq!(state.overflow(), 14);
    }
}
