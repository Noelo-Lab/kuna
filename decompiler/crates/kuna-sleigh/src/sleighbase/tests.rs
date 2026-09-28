use super::*;

use kuna_base::marshal::XmlDecode;
use kuna_base::xml::xml_tree;
use kuna_num::opcodes::OpCode;

#[test]
fn xml_symbol_table_decodes_named_opcodes() {
    let document = xml_tree(include_bytes!(
        "../../../kuna-slacomp/tests/golden/xml_ops.xml"
    ))
    .unwrap();
    let children = document.get_root().get_children();
    let spaces = children.iter().find(|e| e.get_name() == "spaces").unwrap();
    let symbols = children
        .iter()
        .find(|e| e.get_name() == "symbol_table")
        .unwrap();

    let mut base = SleighBase::new();
    let registry = base.registry().clone();
    let empty_manager = AddrSpaceManager::new();
    let mut space_decoder = XmlDecode::new_with_root(&empty_manager, &registry, spaces, 0);
    base.decode_sla_spaces(&mut space_decoder, false).unwrap();

    let mut decoder = XmlDecode::new_with_root(base.manager(), &registry, symbols, 0);
    let mut templates = Vec::new();
    let mut trans = SlaTrans {
        const_space: base.manager().get_constant_space().unwrap().clone(),
        templates: &mut templates,
    };
    let mut table = SymbolTable::new();
    table.decode(&mut decoder, &mut trans).unwrap();

    let opcodes: Vec<_> = templates
        .iter()
        .flat_map(|tpl| tpl.get_opvec().iter().map(|op| op.get_opcode()))
        .collect();
    assert_eq!(opcodes, [crate::semantics::BUILD, OpCode::CPUI_INT_ADD]);
}
