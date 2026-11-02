//! (kuna) `switchrunon` — a jump table with no range guard stops where its
//! rows leave the function for good.
//!
//! A switch whose selector the compiler knows to be in range has no bounds
//! check: Rust matching on a `u8` enum passed by value, or C with an
//! unreachable default. The basic model then has no guard to bound the table
//! and reads one row per value the selector's bits allow, 256 for a byte. The
//! function's own rows are the first few; the rest are the next functions'
//! tables, which the linker placed right after it, read as offsets from the
//! wrong base. Flow follows every one of them into the neighbouring
//! functions, whose own tables run on the same way, so a 1.3 KB serde-hkx
//! serializer decodes 4,791 blocks of 64 other functions.
//!
//! With the option on, the sanity check of a table its model found no guard
//! for keeps only its leading rows inside the function (from its entry to the
//! next known function entry) when every later row lands outside it, in at
//! least two other functions. Those functions' tables lie after this one, so
//! the first row out runs forward, past the entry, and stays in the
//! function's section. A table whose first row out lands below the entry
//! (where gcc puts a function's split-off `.cold` block) or in another
//! section (BOLT's `.text.cold` and `.text.warm` fragments), whose outside
//! rows all land in one function, or that comes back into its own function
//! after leaving it, is kept whole, so a split-off case or a misplaced
//! function start cannot be cut. The rows cut are not dispatched.
//!
//! The model records the conditional branches just before the dispatch as
//! guards whether or not they test the selector. With `switchrunonguard`
//! on, a table also counts as unguarded when the variable it was sized
//! from can still take the largest value its width allows: the `jnz` on a
//! call's Result tag in front of a Rust deserializer's byte key dispatch
//! limits nothing, and a `cmp eax,1; je` that sends one key elsewhere only
//! takes out a value. A range that then wraps past its top (2..0xff, then
//! 0) is read in value order, so its last row, case 0, is not a row that
//! comes back into the function.

/// The number of leading rows of `rows` (table destinations) that belong to
/// the function at `entry`, when the rest run on forward into other
/// functions: `owner` names the known function entry at or before an
/// address. `None` keeps the table whole.
pub fn own_rows(entry: u64, rows: &[u64], owner: impl Fn(u64) -> Option<u64>) -> Option<usize> {
    let own = |row: u64| owner(row) == Some(entry);
    let cut = rows.iter().position(|&row| !own(row)).filter(|&cut| cut > 0 && rows[cut] > entry)?;
    let mut others = rows[cut..].iter().map(|&row| owner(row));
    let first = others.next()?;
    let mut spread = false;
    for other in std::iter::once(first).chain(others) {
        if other == Some(entry) {
            return None;
        }
        spread |= other != first;
    }
    spread.then_some(cut)
}

#[cfg(test)]
mod tests;
