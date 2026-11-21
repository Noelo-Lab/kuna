//! (kuna `fid`) The prototype a `.fid` record carries for the function it names.
//!
//! A `.fid` database built from an object that describes its functions keeps
//! each one's prototype as C text beside the name, with the type declarations
//! it needs. [`declaration`] writes that text from the prototype parked on a
//! function symbol, in the vocabulary the C declaration parser reads back:
//! sized core names (`uint4`), a forward `typedef struct N N;` for every
//! record so pointers among them parse in any order, then each definition
//! before its first by-value use.
//!
//! A matched function is decompiled against the parked prototype like any
//! other declared one. A fingerprint match is not proof of identity, so the
//! finished function is checked: [`reads_undeclared_parameter`] holds when the
//! body reads parameter storage the prototype does not declare, and the drive
//! then withdraws the prototype, decompiles the function as it does without
//! one and says so in the function header. A parameter the body never reads is
//! no disagreement.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::architecture::Architecture;
use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::fspec::PrototypePieces;
use crate::funcdata::Funcdata;
use crate::kuna_langtypes::SpellCtx;

/// A prototype and the type declarations it needs, as C text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub prototype: String,
    pub typedefs: Vec<String>,
}

/// The header warning of a function whose `.fid` prototype was withdrawn.
pub const WITHDRAWN: &str =
    "WARNING: the .fid prototype was dropped: the body reads parameter storage it does not declare";

/// `pieces` as C text, or `None` when its return type has no plain spelling
/// (a function returning a function pointer).
pub fn declaration(pieces: &PrototypePieces) -> Option<Declaration> {
    const MARK: &str = "@";
    let rt = SpellCtx::OFF;
    let ret = match &pieces.outtype {
        Some(ct) if ct.get_metatype() != type_metatype::TYPE_VOID => {
            let text = crate::printc::declaration_text(ct, MARK, rt.clone());
            text.strip_suffix(MARK)?.trim_end().to_string()
        }
        _ => "void".to_string(),
    };
    let mut params: Vec<String> = pieces
        .intypes
        .iter()
        .enumerate()
        .map(|(i, ct)| {
            let name = pieces.innames.get(i).map_or("", |n| identifier(n).unwrap_or(""));
            crate::printc::declaration_text(ct, name, rt.clone())
        })
        .collect();
    if pieces.first_var_arg_slot >= 0 {
        params.push("...".to_string());
    }
    let params = if params.is_empty() { "void".to_string() } else { params.join(",") };
    let name = identifier(&pieces.name).unwrap_or("f");
    let sep = if ret.ends_with('*') { "" } else { " " };

    let roots = pieces.outtype.iter().chain(&pieces.intypes);
    let types = crate::kuna_structdefs::dedup_by_name(&crate::kuna_structdefs::referenced_by(roots));
    let record = |ct: &Rc<Datatype>| {
        ct.get_typedef().is_none()
            && matches!(&ct.kind, DatatypeKind::Struct { .. } | DatatypeKind::Union { .. })
    };
    let mut typedefs: Vec<String> = Vec::new();
    for ct in types.iter().filter(|ct| record(ct)) {
        let kw = if matches!(&ct.kind, DatatypeKind::Union { .. }) { "union" } else { "struct" };
        let name = crate::printc::sanitize_type_name(ct.get_name());
        typedefs.push(format!("typedef {kw} {name} {name};"));
    }
    for ct in types.iter().filter(|ct| !(record(ct) && (ct.is_incomplete() || !laid_out_again(ct)))) {
        let text = one_line(&crate::kuna_structdefs::definition_text(ct, rt.clone()));
        if !text.is_empty() {
            typedefs.push(sized_enum(ct, text.replace("undefined1 _pad", "uint1 _pad")));
        }
    }
    Some(Declaration { prototype: format!("{ret}{sep}{name}({params})"), typedefs })
}

/// Does the C parser, laying out the definition text of `ct` afresh, put every
/// member where `ct` has it? Not when the text moves bitfields to the end, nor
/// for a member off its natural alignment or a size the alignment does not
/// divide; such a record is stored as its forward declaration alone.
fn laid_out_again(ct: &Datatype) -> bool {
    let align = |t: &Datatype| t.get_alignment().max(1);
    match &ct.kind {
        DatatypeKind::Struct { field, bitfield } => {
            let mut end = 0;
            let mut most = 1;
            for f in field {
                let t = &f.field_type;
                if f.offset < end || f.offset % align(t) != 0 || t.get_align_size() != t.get_size() {
                    return false;
                }
                end = f.offset + t.get_size();
                most = most.max(align(t));
            }
            bitfield.is_empty() && ct.get_size() >= end && ct.get_size() % most == 0
        }
        DatatypeKind::Union { field } => {
            ct.get_size() == field.iter().map(|f| f.field_type.get_size()).max().unwrap_or(0)
        }
        _ => true,
    }
}

/// An enum's definition with its width as a C23 underlying type: the parser
/// otherwise makes it `int`-wide, which moves every member after a packed or
/// `-fshort-enums` one.
fn sized_enum(ct: &Datatype, text: String) -> String {
    if !ct.is_enum_type() || ct.get_typedef().is_some() {
        return text;
    }
    let name = crate::printc::sanitize_type_name(ct.get_name());
    let sign = if ct.get_metatype() == type_metatype::TYPE_INT { "int" } else { "uint" };
    text.replacen(
        &format!("typedef enum {name} {{"),
        &format!("typedef enum {name} : {sign}{} {{", ct.get_size()),
        1,
    )
}

fn identifier(name: &str) -> Option<&str> {
    let mut chars = name.chars();
    let first = chars.next()?;
    ((first.is_ascii_alphabetic() || first == '_') && chars.all(|c| c.is_ascii_alphanumeric() || c == '_'))
        .then_some(name)
}

/// `text` without its comments, on one line.
fn one_line(text: &str) -> String {
    let mut code = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("/*") {
        code.push_str(&rest[..start]);
        rest = rest[start..].find("*/").map_or("", |end| &rest[start + end + 2..]);
    }
    code.push_str(rest);
    code.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Does the finished `fd` read an input in parameter storage that its
/// prototype does not declare? Never for a variadic prototype.
pub fn reads_undeclared_parameter(fd: &Funcdata) -> bool {
    let proto = fd.get_func_proto();
    if !proto.has_model() || proto.is_dotdotdot() {
        return false;
    }
    let overlaps = |a: &Address, asize: int4, b: &Address, bsize: int4| {
        a.overlap(0, b, bsize) >= 0 || b.overlap(0, a, asize) >= 0
    };
    let declared: Vec<(Address, int4)> = (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .map(|p| (p.get_address(), p.get_size()))
        .collect();
    let entries: Vec<(Address, int4)> = proto
        .model()
        .input()
        .get_entry()
        .iter()
        .map(|e| (Address::new(Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect();
    fd.vbank().iter_def_flag(crate::varnode::varnode_flags::input).any(|id| {
        fd.vbank().get(id).is_some_and(|v| {
            v.is_input()
                && !v.has_no_descend()
                && entries.iter().any(|(a, s)| overlaps(v.get_addr(), v.get_size(), a, *s))
                && !declared.iter().any(|(a, s)| overlaps(v.get_addr(), v.get_size(), a, *s))
        })
    })
}

/// Withdraw the `.fid` prototype of the function at `addr` and say so in its header.
pub fn withdraw(arch: &mut Architecture, addr: &Address) {
    arch.fid_prototypes.remove(&addr.get_offset());
    if let Some((sid, _)) = arch.symboltab.find_function_across_scopes(addr) {
        arch.symboltab.clear_function_proto_pieces(sid);
    }
    arch.commentdb.add_comment_no_duplicate(
        crate::comment::comment_type::WARNINGHEADER,
        addr,
        addr,
        WITHDRAWN,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_and_line_breaks_leave_one_declaration() {
        assert_eq!(
            one_line("struct hdr {\n    uint4 magic; /* le */\n    uint4 len;\n};\n"),
            "struct hdr { uint4 magic; uint4 len; };"
        );
        assert_eq!(one_line("typedef struct FILE FILE; /* opaque */\n"), "typedef struct FILE FILE;");
        assert_eq!(one_line("int a; /* unterminated"), "int a;");
    }

    #[test]
    fn only_a_c_identifier_is_spelled() {
        assert_eq!(identifier("parse_header"), Some("parse_header"));
        assert_eq!(identifier("Account::deposit"), None);
        assert_eq!(identifier("9lives"), None);
        assert_eq!(identifier(""), None);
    }

    fn scalar(size: int4, name: &str) -> Rc<Datatype> {
        let mut t = Datatype::new_with_align(size, size, type_metatype::TYPE_INT);
        t.name = name.into();
        t.display_name = name.into();
        t.flags |= crate::dtype::flags::coretype;
        Rc::new(t)
    }

    fn record(name: &str, size: int4, fields: &[(int4, &Rc<Datatype>)]) -> Datatype {
        let mut s = Datatype::new_with_align(size, 8, type_metatype::TYPE_STRUCT);
        s.name = name.into();
        s.display_name = name.into();
        s.id = Datatype::hash_name(name);
        let field = fields
            .iter()
            .enumerate()
            .map(|(i, (off, t))| crate::dtype::TypeField::new(i as int4, *off, format!("m{i}"), Rc::clone(t)))
            .collect();
        s.kind = DatatypeKind::Struct { field, bitfield: vec![] };
        s
    }

    #[test]
    fn a_record_is_stored_whole_only_when_the_parser_lays_it_out_the_same() {
        let (i4, i8) = (scalar(4, "int4"), scalar(8, "int8"));
        let padded = record("padded", 16, &[(0, &i4), (8, &i8)]);
        assert!(laid_out_again(&padded));
        assert!(!laid_out_again(&record("packed", 12, &[(0, &i4), (4, &i8)])));
        assert!(!laid_out_again(&record("overlap", 16, &[(0, &i8), (4, &i4)])));
        assert!(!laid_out_again(&record("short", 12, &[(0, &i8), (8, &i4)])));
        let mut bits = record("bits", 8, &[(0, &i4)]);
        if let DatatypeKind::Struct { bitfield, .. } = &mut bits.kind {
            bitfield.push(crate::dtype::TypeBitField::new(1, 3, false, "b", Rc::clone(&i4)));
        }
        assert!(!laid_out_again(&bits));

        let mut ptr = Datatype::new_with_align(8, 8, type_metatype::TYPE_PTR);
        ptr.kind = DatatypeKind::Pointer { ptrto: Rc::new(padded), spaceid: None, truncate: None, wordsize: 1 };
        let pieces = PrototypePieces {
            name: "f".into(),
            intypes: vec![Rc::new(ptr)],
            innames: vec!["p".into()],
            first_var_arg_slot: -1,
            ..Default::default()
        };
        let declared = declaration(&pieces).unwrap();
        assert_eq!(
            declared.typedefs,
            ["typedef struct padded padded;", "struct padded { int4 m0; uint1 _pad4[4]; int8 m1; };"]
        );
    }

    #[test]
    fn an_enum_is_stored_at_its_width() {
        use crate::dtype::TypeFactory;
        let types = crate::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        let _ = types.cache_core_types();
        let shell = types.get_type_enum_sized("kind", 1, type_metatype::TYPE_ENUM_UINT).unwrap();
        let names = std::collections::BTreeMap::from([(0, "K_A".to_string()), (1, "K_B".to_string())]);
        let kind = types.set_enum_values(&shell, names).unwrap();
        let pieces = PrototypePieces {
            name: "f".into(),
            intypes: vec![kind],
            innames: vec!["k".into()],
            first_var_arg_slot: -1,
            ..Default::default()
        };
        assert_eq!(
            declaration(&pieces).unwrap().typedefs,
            ["typedef enum kind : uint1 { K_A = 0x0, K_B = 0x1 } kind;"]
        );
    }
}
