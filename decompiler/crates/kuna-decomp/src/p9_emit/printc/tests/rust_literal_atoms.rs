use super::*;
use crate::context::{ArchContext, TypeOp};
use crate::printlanguage::{Atom, SyntaxHighlight as Highlight, TagType};
use kuna_base::address::Address;
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use std::rc::Rc;

fn render(tag: TagType, highlight: Highlight, value: &str) -> String {
    let mut print = PrintC::new();
    print.set_name("rust-language");
    print.set_output_stream();
    print.push_atom(&Atom::syntax(value, tag, highlight));
    print.emit_mut().output_str().to_string()
}

#[test]
fn constructed_null_calls_keep_their_parentheses_for_both_highlights() {
    for highlight in [Highlight::var_color, Highlight::const_color] {
        assert_eq!(
            render(TagType::LiteralToken, highlight, "core::ptr::null_mut()"),
            "core::ptr::null_mut()"
        );
    }
}

#[test]
fn recovered_identifiers_and_enum_names_still_require_spelling() {
    for highlight in [Highlight::var_color, Highlight::const_color] {
        assert_eq!(
            render(TagType::VarToken, highlight, "core::ptr::null_mut()"),
            "core::ptr::null_mut__"
        );
        assert_eq!(
            render(TagType::VarToken, highlight, "kind::Some(value)"),
            "kind::Some_value_"
        );
        assert_eq!(
            render(TagType::VarToken, highlight, "9variant"),
            "_9variant"
        );
    }
}

#[test]
fn constructed_expression_tokens_keep_the_original_operation_and_varnode() {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
        .unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manager.insert_space(Rc::clone(&ram)).unwrap();
    manager.set_default_code_space(2).unwrap();
    let global = Rc::new(ArchContext::new(manager));
    let mut fd = Funcdata::new(
        "f",
        "f",
        global,
        Address::new(Rc::clone(&ram), 0x1000),
        0x10000000,
        0x40,
    )
    .unwrap();
    let op = fd.new_op(1, Address::new(ram, 0x1000));
    fd.obank_mut()
        .change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
    let constant = fd.new_constant(8, 0);
    fd.op_set_input(op, constant, 0).unwrap();
    let expected_op = fd.obank().get(op).unwrap().get_time() as u64;
    let expected_var = fd.vbank().get(constant).unwrap().get_create_index() as u64;
    let mut records = Vec::new();
    for tag in [TagType::VarToken, TagType::LiteralToken] {
        let mut print = PrintC::new();
        print.set_name("rust-language");
        print.set_markup(true);
        print.emit_fd = Some(&fd as *const Funcdata);
        print.push_atom(&Atom::with_op_vn(
            "core::ptr::null_mut()",
            tag,
            Highlight::var_color,
            op_key(op),
            vn_key(constant),
        ));
        print.emit_mut().take_markup_bytes();
        let provenance = print.emit_mut().take_markup_provenance();
        print.emit_fd = None;
        assert_eq!(provenance.associations.len(), 1);
        assert_eq!(provenance.associations[0].opref, Some(expected_op));
        assert_eq!(provenance.associations[0].varref, Some(expected_var));
        records.push(provenance);
    }
    assert_eq!(records[0], records[1]);
}
