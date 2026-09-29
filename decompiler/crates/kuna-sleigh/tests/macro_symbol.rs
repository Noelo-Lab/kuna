use std::rc::Rc;

use kuna_sleigh::semantics::ConstructTpl;
use kuna_sleigh::slghsymbol::MacroSymbol;

fn template(labels: u32) -> ConstructTpl {
    let mut body = ConstructTpl::new();
    body.set_num_labels(labels);
    body
}

#[test]
fn replacing_an_owned_definition_does_not_change_a_cloned_symbol() {
    let mut symbol = MacroSymbol::new(7);
    assert!(symbol.get_construct().is_none());
    symbol.set_construct(template(2));
    let cloned = symbol.clone();
    symbol.set_construct(template(3));
    assert_eq!(symbol.get_construct().unwrap().num_labels(), 3);
    assert_eq!(cloned.get_construct().unwrap().num_labels(), 2);
    assert_eq!(cloned.get_index(), 7);
}

#[test]
fn shared_definitions_are_borrowed_and_output_copies_remain_independent() {
    let body = Rc::new(template(2));
    let mut symbol = MacroSymbol::new(0);
    symbol.set_shared_construct(Rc::clone(&body));
    assert!(std::ptr::eq(symbol.get_construct().unwrap(), body.as_ref()));
    let mut output = symbol.get_construct().unwrap().clone();
    output.set_num_labels(5);
    assert_eq!(output.num_labels(), 5);
    assert_eq!(symbol.get_construct().unwrap().num_labels(), 2);
    assert_eq!(body.num_labels(), 2);
}
