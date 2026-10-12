//! Address selections that land inside another function's extent.
//!
//! `--addr` declares that a function starts where it points, which is how an
//! entry discovery missed gets decompiled at all, so such a selection is never
//! refused. But the same command at an address inside a function discovery did
//! find prints a plausible `sub_<addr>` body that is a fragment of its
//! neighbour, and nothing said so. Both surfaces now do: `kuna decompile` and
//! `decompile-all --addr` print a `note:` on stderr naming the containing
//! inventory entry and the offset, and the `--json` record of each addressed
//! target carries the same fact as `interior_to`. A `--define-function` at the
//! address makes it an entry, which is the way to say the fragment is meant.

use std::collections::BTreeMap;

use kuna_console::engine::ConsoleProgram;

use crate::decompile_all::{resolve_addr_selectors, Args};
use crate::jsonfmt::Json;

/// The containing entry of one addressed target, in the target's display units.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Interior {
    pub(crate) addr: u64,
    pub(crate) owner: String,
    pub(crate) entry: u64,
}

/// Each `--addr` target's containing entry, keyed by the target's byte
/// address; `None` for a target that is itself an entry.
pub(crate) type Interiors = BTreeMap<u64, Option<Interior>>;

/// Resolve every `--addr` selection's containing entry and note each one that
/// lands inside another function.
pub(crate) fn selections(prog: &ConsoleProgram, args: &Args) -> Interiors {
    let mut found = Interiors::new();
    for target in resolve_addr_selectors(prog, args, false).unwrap_or_default() {
        let byte = target.addr.get_offset();
        let interior = prog.entry_containing(byte).map(|owner| Interior {
            addr: prog.output_code_offset(byte),
            owner: owner.name,
            entry: prog.output_code_offset(owner.addr.get_offset()),
        });
        if let Some(interior) = &interior {
            eprintln!("{}", note(interior));
        }
        found.insert(byte, interior);
    }
    found
}

/// The stderr line, one wording for every surface.
pub(crate) fn note(i: &Interior) -> String {
    format!(
        "note: {:#x} is interior to {} ({:#x}+{:#x}), not an entry kuna discovered; \
         decompiled from {:#x} as asked (--define-function {:#x} declares a function there)",
        i.addr,
        i.owner,
        i.entry,
        i.addr - i.entry,
        i.addr,
        i.addr
    )
}

/// The record field: `null` for an entry, else the containing function.
pub(crate) fn json(interior: Option<&Interior>) -> Json {
    match interior {
        None => Json::Null,
        Some(i) => Json::Object(vec![
            ("name".into(), Json::Str(i.owner.clone())),
            ("address".into(), Json::Number(i.entry.to_string())),
            ("address_hex".into(), Json::Str(format!("{:#x}", i.entry))),
            ("offset".into(), Json::Number((i.addr - i.entry).to_string())),
        ]),
    }
}

/// The `function containing` answer in a `decomp_dbg` transcript:
/// `<addr> is interior to <name> at <entry> (+<offset>)`.
pub(crate) fn from_transcript(transcript: &str) -> Option<Interior> {
    transcript.lines().find_map(|line| {
        let line = line.trim();
        let line = line.strip_prefix("[decomp]>").unwrap_or(line).trim();
        let (addr, rest) = line.split_once(" is interior to ")?;
        let (rest, _) = rest.strip_suffix(')')?.rsplit_once(" (+")?;
        let (owner, entry) = rest.rsplit_once(" at ")?;
        Some(Interior { addr: hex(addr)?, owner: owner.to_string(), entry: hex(entry)? })
    })
}

fn hex(token: &str) -> Option<u64> {
    u64::from_str_radix(token.strip_prefix("0x")?, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsonfmt::dumps_compact;

    #[test]
    fn transcript_answer_round_trips_through_the_note_and_the_record() {
        let transcript = "[decomp]> function containing 0x140001040\n\
                          0x140001040 is interior to sub_140001030 at 0x140001030 (+0x10)\n\
                          Registered sub_140001040 at 0x140001040 without a body assertion\n";
        let interior = from_transcript(transcript).expect("parsed");
        assert_eq!(
            interior,
            Interior { addr: 0x140001040, owner: "sub_140001030".into(), entry: 0x140001030 }
        );
        assert_eq!(
            note(&interior),
            "note: 0x140001040 is interior to sub_140001030 (0x140001030+0x10), not an entry \
             kuna discovered; decompiled from 0x140001040 as asked (--define-function \
             0x140001040 declares a function there)"
        );
        assert_eq!(
            dumps_compact(&json(Some(&interior))),
            r#"{"name":"sub_140001030","address":5368713264,"address_hex":"0x140001030","offset":16}"#
        );
        assert_eq!(dumps_compact(&json(None)), "null");
    }

    #[test]
    fn an_owner_name_with_spaces_and_parentheses_survives() {
        let line = "0x401010 is interior to Foo::bar(int, char) at 0x401000 (+0x10)";
        let interior = from_transcript(line).expect("parsed");
        assert_eq!(interior.owner, "Foo::bar(int, char)");
        assert_eq!(interior.entry, 0x401000);
        assert!(from_transcript("0x401010 is interior to no function").is_none());
    }
}
