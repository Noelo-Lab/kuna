//! (kuna `fid`) Apply the prototypes of matched `.fid` records.
//!
//! Runs last in the analysis commit, so every other prototype source has
//! parked what it knows: a record's prototype goes only to a function that
//! carries the record's name and that nothing else declares, and an
//! `--assert` given later still replaces it. Each record's type declarations
//! are parsed first; one that names a type the program already has is left
//! out, the program's own definition standing. A prototype that does not
//! parse is dropped with a header warning on its function. The text comes
//! from a file, and the parser recurses once per bracket level, so a
//! declaration nested deeper than [`MAX_NESTING`] is not parsed at all. The
//! decompile drive checks a parked one against the body
//! ([`kuna_decomp::kuna_fidproto`]).

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_analysis::pass::FidProtoFact;
use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::comment::comment_type;
use kuna_decomp::dtype::DatatypeKind;
use kuna_decomp::fspec::PrototypePieces;

use crate::engine::ConsoleProgram;

const MAX_NESTING: usize = 64;

pub(crate) fn apply(prog: &mut ConsoleProgram, code_space: &Rc<AddrSpace>, facts: Vec<FidProtoFact>) {
    let mut parsed: BTreeSet<String> = BTreeSet::new();
    for fact in facts {
        let addr = Address::new(Rc::clone(code_space), fact.addr);
        let symbols = &prog.arch().symboltab;
        let Some((sid, _)) = symbols.find_function_across_scopes(&addr) else { continue };
        if symbols.symbol(sid).get_name() != fact.name || symbols.function_proto_pieces_of(sid).is_some() {
            continue;
        }
        for decl in fact.typedefs.iter() {
            if parsed.insert(decl.clone()) && nesting(decl) <= MAX_NESTING {
                let _ = crate::assertions::parse_declaration(prog, decl);
            }
        }
        let prototype = if nesting(&fact.prototype) > MAX_NESTING {
            Err(format!("brackets nested deeper than {MAX_NESTING}"))
        } else {
            crate::assertions::parse_prototype(prog, &fact.prototype).and_then(|parsed| {
                match opaque_by_value(&parsed.0) {
                    Some(name) => Err(format!("{name} is passed by value but has no definition")),
                    None => Ok(parsed),
                }
            })
        };
        match prototype {
            Ok((mut pieces, _)) => {
                pieces.name = fact.name;
                prog.arch_mut().set_function_prototype_pieces_at(&addr, pieces);
                prog.arch_mut().fid_prototypes.insert(fact.addr);
            }
            Err(why) => {
                let why = why.lines().next().unwrap_or_default();
                let text = format!("WARNING: the .fid prototype `{}` was dropped: {why}", fact.prototype);
                prog.arch_mut().commentdb.add_comment_no_duplicate(
                    comment_type::WARNINGHEADER,
                    &addr,
                    &addr,
                    &text,
                );
            }
        }
    }
}

/// A record type the prototype passes or returns by value without a
/// definition: an opaque size cannot be assigned storage.
fn opaque_by_value(pieces: &PrototypePieces) -> Option<String> {
    pieces.outtype.iter().chain(&pieces.intypes).find_map(|ct| {
        let mut t = Rc::clone(ct);
        while let Some(base) = t.get_typedef().cloned() {
            t = base;
        }
        let record = matches!(t.kind, DatatypeKind::Struct { .. } | DatatypeKind::Union { .. });
        (record && t.is_incomplete()).then(|| ct.get_name().to_string())
    })
}

fn nesting(text: &str) -> usize {
    let mut depth = 0usize;
    let mut most = 0;
    for byte in text.bytes() {
        match byte {
            b'(' | b'[' | b'{' => {
                depth += 1;
                most = most.max(depth);
            }
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    most
}

#[cfg(test)]
mod tests {
    #[test]
    fn nesting_counts_the_deepest_bracket() {
        assert_eq!(super::nesting("int f(void (*cb)(int a[4]))"), 3);
        assert_eq!(super::nesting("struct s { int a; };"), 1);
        assert_eq!(super::nesting(")))"), 0);
    }

    #[test]
    fn a_record_without_its_definition_is_not_passed_by_value() {
        use std::rc::Rc;

        use kuna_decomp::dtype::{flags, type_metatype, Datatype, DatatypeKind};
        use kuna_decomp::fspec::PrototypePieces;

        let mut shell = Datatype::new_with_align(0, 1, type_metatype::TYPE_STRUCT);
        shell.name = "bits".into();
        shell.display_name = "bits".into();
        shell.flags |= flags::type_incomplete;
        shell.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
        let shell = Rc::new(shell);
        let mut ptr = Datatype::new_with_align(8, 8, type_metatype::TYPE_PTR);
        ptr.kind = DatatypeKind::Pointer { ptrto: Rc::clone(&shell), spaceid: None, truncate: None, wordsize: 1 };
        let by_pointer = PrototypePieces { intypes: vec![Rc::new(ptr)], ..Default::default() };
        assert_eq!(super::opaque_by_value(&by_pointer), None);
        let returned = PrototypePieces { outtype: Some(Rc::clone(&shell)), ..Default::default() };
        assert_eq!(super::opaque_by_value(&returned).as_deref(), Some("bits"));
    }
}
