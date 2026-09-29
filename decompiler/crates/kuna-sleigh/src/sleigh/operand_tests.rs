use super::*;
use crate::globalcontext::ContextInternal;
use crate::slghpatexpress::{OperandValue, PatternValue, TokenField};
use crate::slghsymbol::{Constructor, SleighSymbol};

struct EmptyImage;
impl LoadImage for EmptyImage {
    fn get_file_name(&self) -> &str {
        "synthetic-operands"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _: i64) {}
    fn load_fill(&mut self, out: &mut [u8], _: &Address) -> KunaResult<()> {
        out.fill(0);
        Ok(())
    }
}

fn operand(index: i32) -> PatternExpression {
    PatternExpression::Value(PatternValue::OperandValue(OperandValue::new(index, 0, 0)))
}

fn setup(resolved: bool) -> (SymbolTable, ParserContext, Sleigh) {
    let ct = ConstructorRef {
        table_id: 0,
        ct_id: 0,
    };
    let mut table = SymbolTable::new();
    table.add_scope();
    let mut top = SleighSymbol::new_subtable(b"instruction");
    if let SymbolKind::Subtable(sub) = top.kind_mut() {
        for _ in 0..2 {
            let mut constructor = Constructor::new();
            constructor.set_parent(0);
            for id in 1..=5 {
                constructor.add_operand(id);
            }
            sub.add_constructor(constructor);
        }
    }
    table.add_global_symbol(top).unwrap();
    let token = PatternValue::TokenField(TokenField::new(1, false, false, 0, 7));
    for index in 0..5 {
        let mut symbol = SleighSymbol::new_operand(format!("op{index}").as_bytes(), index, ct);
        if let SymbolKind::Operand(op) = symbol.kind_mut() {
            op.set_offset(
                if resolved && index == 1 { 0 } else { -1 },
                if index == 0 { 2 } else { 1 },
            );
            match index {
                0 => op.define_operand_expression(operand(1)).unwrap(),
                1 => op
                    .define_operand_expression(PatternExpression::Value(token.clone()))
                    .unwrap(),
                2 => op.define_operand_expression(operand(2)).unwrap(),
                3 => op.define_operand_symbol(6).unwrap(),
                _ => {}
            }
        }
        assert_eq!(table.add_global_symbol(symbol).unwrap(), index as u32 + 1);
    }
    assert_eq!(
        table
            .add_global_symbol(SleighSymbol::new_value(b"field", token))
            .unwrap(),
        6
    );
    let mut ctx = ParserContext::new(1);
    let mut parent = ConstructState::with_operands(5);
    parent.ct = Some(ct);
    parent.offset = 3;
    parent.resolve[1] = Some(1);
    let mut child = ConstructState::with_operands(0);
    child.ct = Some(ConstructorRef {
        table_id: 0,
        ct_id: 1,
    });
    child.parent = Some(0);
    child.offset = 10;
    ctx.state = vec![parent, child];
    for (i, b) in ctx.buf.iter_mut().enumerate() {
        *b = 0x10 + i as u8;
    }
    (
        table,
        ctx,
        Sleigh::new(Box::new(EmptyImage), Box::new(ContextInternal::new())),
    )
}

#[test]
fn nested_operands_use_simulated_offsets_and_resolved_children() {
    for resolved in [false, true] {
        let (table, ctx, engine) = setup(resolved);
        let mut walker = ParserWalker::new(&ctx, &table, &engine);
        walker.cur.point = Some(1);
        walker.cur.depth = 1;
        assert_eq!(
            walker.operand_value(0, 0, 0).unwrap(),
            if resolved { 0x1a } else { 0x16 }
        );
        assert_eq!(walker.operand_value(3, 0, 0).unwrap(), 0x14);
        assert_eq!(walker.operand_value(4, 0, 0).unwrap(), 0);
    }
}

#[test]
fn invalid_and_recursive_operand_references_return_errors() {
    let (table, mut ctx, engine) = setup(true);
    ctx.state[0].resolve[1] = Some(usize::MAX);
    let mut walker = ParserWalker::new(&ctx, &table, &engine);
    walker.base_state();
    assert!(walker.operand_value(-1, 0, 0).is_err());
    assert!(walker.operand_value(5, 0, 0).is_err());
    assert!(walker.operand_value(0, u32::MAX, 0).is_err());
    assert!(walker.operand_value(0, 0, 99).is_err());
    assert!(walker.operand_value(0, 0, 0).is_err());
    assert!(walker
        .operand_value(2, 0, 0)
        .unwrap_err()
        .to_string()
        .contains("maximum operand expression depth"));
}
