//! (kuna `declorder`) The order a function's local declarations print in.
//!
//! The printer declares one local per line, sorted by name.  A plain string
//! sort puts `v10`..`v19` between `v1` and `v2`, so a function with ten or more
//! locals declares `v1, v10, v11, v12, v2, v3, ...` and a reader looking for
//! `v2` has to scan past every two-digit name first.  `natural` compares the
//! digit runs inside two names by their numeric value and everything else
//! character by character, so the same locals print `v1, v2, v3, ..., v10, v11`.
//!
//! Only the printed order changes: the collapses and the repeated-name suffixes
//! the declaration pass applies run over the same name-sorted list as before,
//! and the reorder happens once the final spellings are known.
//!
//! It applies to angr-style names (`namestyle angr`, the default).  Ghidra-style
//! names carry hexadecimal offsets (`local_1c`, `uStack_a8`) that a decimal
//! digit-run comparison would scramble, so under `namestyle ghidra` the plain
//! string order stays.

use std::cmp::Ordering;

use kuna_base::error::{KunaError, KunaResult};

/// The value of `declorder`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeclOrder {
    /// Character order, `v1, v10, v2`.
    #[default]
    Name,
    /// Digit runs compare as numbers, `v1, v2, v10`.
    Natural,
}

impl DeclOrder {
    pub fn as_str(self) -> &'static str {
        match self {
            DeclOrder::Name => "name",
            DeclOrder::Natural => "natural",
        }
    }
}

/// The `declorder` option parser.
pub struct OptionDeclOrder;

impl OptionDeclOrder {
    pub const NAME: &'static str = "declorder";

    pub fn apply(&self, p1: &str) -> KunaResult<(DeclOrder, String)> {
        let mode = match p1 {
            "name" => DeclOrder::Name,
            "natural" => DeclOrder::Natural,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown declorder value: {other} (expected natural|name)"
                )))
            }
        };
        Ok((mode, format!("Local declaration order set to {}", mode.as_str())))
    }
}

/// Compare two names with each maximal ASCII digit run read as a number.  Equal
/// numbers spelled with different leading zeros, and names that are otherwise
/// equal, fall back to plain string order, so this is a total order that agrees
/// with `==`.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (x, y) = (a.as_bytes(), b.as_bytes());
    let (mut i, mut j) = (0, 0);
    while i < x.len() && j < y.len() {
        if x[i].is_ascii_digit() && y[j].is_ascii_digit() {
            let (si, sj) = (i, j);
            while i < x.len() && x[i].is_ascii_digit() {
                i += 1;
            }
            while j < y.len() && y[j].is_ascii_digit() {
                j += 1;
            }
            let (dx, dy) = (strip_zeros(&x[si..i]), strip_zeros(&y[sj..j]));
            let ord = dx.len().cmp(&dy.len()).then_with(|| dx.cmp(dy));
            if ord != Ordering::Equal {
                return ord;
            }
        } else {
            let ord = x[i].cmp(&y[j]);
            if ord != Ordering::Equal {
                return ord;
            }
            i += 1;
            j += 1;
        }
    }
    (x.len() - i).cmp(&(y.len() - j)).then_with(|| a.cmp(b))
}

fn strip_zeros(d: &[u8]) -> &[u8] {
    let k = d.iter().position(|&c| c != b'0').unwrap_or(d.len());
    &d[k..]
}

/// Put already-named declarations in the printed order `mode` asks for;
/// `angr_names` is whether the function is named in the angr style.
pub fn order_decls<T>(decls: &mut [(T, String)], mode: DeclOrder, angr_names: bool) {
    if mode == DeclOrder::Natural && angr_names {
        decls.sort_by(|a, b| natural_cmp(&a.1, &b.1));
    }
}

#[cfg(test)]
#[path = "kuna_declorder/tests.rs"]
mod tests;
