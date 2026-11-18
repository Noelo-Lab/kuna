//! (kuna `fid`) Apply the prototypes of matched `.fid` records.
//!
//! Runs last in the analysis commit, so every other prototype source has
//! parked what it knows: a record's prototype goes only to a function that
//! carries the record's name and that nothing else declares, and an
//! `--assert` given later still replaces it. Each record's type declarations
//! are parsed first; one that names a type the program already has is left
//! out, the program's own definition standing. A prototype that does not
//! parse is dropped with a header warning on its function. The decompile
//! drive checks a parked one against the body
//! ([`kuna_decomp::kuna_fidproto`]).

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_analysis::pass::FidProtoFact;
use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::comment::comment_type;

use crate::engine::ConsoleProgram;

pub(crate) fn apply(prog: &mut ConsoleProgram, code_space: &Rc<AddrSpace>, facts: Vec<FidProtoFact>) {
    let mut parsed: BTreeSet<String> = BTreeSet::new();
    for fact in facts {
        let addr = Address::new(Rc::clone(code_space), fact.addr);
        let symbols = &prog.arch().symboltab;
        let Some((sid, _)) = symbols.find_function_across_scopes(&addr) else { continue };
        if symbols.symbol(sid).get_name() != fact.name || symbols.function_proto_pieces_of(sid).is_some() {
            continue;
        }
        for decl in &fact.typedefs {
            if parsed.insert(decl.clone()) {
                let _ = crate::assertions::parse_declaration(prog, decl);
            }
        }
        match crate::assertions::parse_prototype(prog, &fact.prototype) {
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
