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
        walker.cur.to_mut().point = Some(1);
        walker.cur.to_mut().depth = 1;
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

#[test]
fn constructor_off_the_walk_resolves_no_operands() {
    let (table, ctx, engine) = setup(true);
    let mut walker = ParserWalker::new(&ctx, &table, &engine);
    walker.base_state();
    assert!(walker
        .operand_value(0, 0, 1)
        .unwrap_err()
        .to_string()
        .contains("outside current context"));
    let oob = OobWalker {
        ctx: &ctx,
        cross: None,
        engine: &engine,
        table: &table,
        state: OobState {
            ct: ConstructorRef {
                table_id: 0,
                ct_id: 0,
            },
            offset: 3,
            node: None,
        },
        depth: 1,
    };
    assert!(oob
        .operand_value(1, 0, 0)
        .unwrap_err()
        .to_string()
        .contains("not resolved"));
    assert_eq!(oob.operand_value(3, 0, 0).unwrap(), 0x14);
}

#[test]
fn borrowed_builder_cursor_uses_target_tree_and_cross_addresses() {
    let (table, mut target, engine) = setup(true);
    let space = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        1,
        0,
        1,
        1,
    ));
    target.addr = Address::new(Rc::clone(&space), 0x100);
    target.naddr = Address::new(Rc::clone(&space), 0x104);
    target.context[0] = 0x1234_5678;
    let mut caller = ParserContext::new(1);
    caller.addr = Address::new(Rc::clone(&space), 0x900);
    caller.naddr = Address::new(Rc::clone(&space), 0x908);
    caller.calladdr = Address::new(Rc::clone(&space), 0xabc);
    caller.context[0] = u32::MAX;
    let contexts: Vec<_> = [target, caller]
        .into_iter()
        .map(|ctx| ResolvedCtx {
            addr: ctx.addr.clone(),
            ctx: ParserContextGuard {
                context: Some(ctx),
                pool: Rc::new(RefCell::new(Vec::new())),
            },
        })
        .collect();
    let mut cache = PcodeCacher::new();
    let mut builder = SleighBuilder {
        labelbase: 0,
        labelcount: 0,
        cur: WalkCursor::new(),
        cur_ctx: 0,
        cross_ctx: Some(1),
        contexts: &contexts,
        table: &table,
        templates: &[],
        engine: &engine,
        const_space: Rc::clone(&space),
        uniq_space: space,
        uniquemask: 0,
        uniqueoffset: 0,
        cache: &mut cache,
    };
    builder.cur.point = Some(0);
    let mut walker = builder.walker();
    assert!(matches!(walker.cur, Cow::Borrowed(_)));
    assert_eq!(walker.get_addr().get_offset(), 0x900);
    assert_eq!(walker.get_naddr().get_offset(), 0x908);
    assert_eq!(walker.get_dest_addr().unwrap().get_offset(), 0xabc);
    assert_eq!(walker.get_instruction_bytes(0, 1).unwrap(), 0x13);
    assert_eq!(walker.get_context_bytes(0, 4).unwrap(), 0x1234_5678);
    assert_eq!(walker.get_constructor().unwrap().ct_id, 0);

    // Navigation on the temporary reader detaches, preserving the builder's
    // position and breadcrumb path.
    walker.push_operand(1).unwrap();
    assert!(matches!(walker.cur, Cow::Owned(_)));
    assert_eq!(walker.get_constructor().unwrap().ct_id, 1);
    assert_eq!(builder.cur.point, Some(0));
    assert_eq!(builder.cur.depth, 0);
    assert_eq!(builder.cur.breadcrumb[0], 0);
    walker.pop_operand().unwrap();
    assert_eq!(walker.get_constructor().unwrap().ct_id, 0);
    drop(walker);

    builder.cur.point = Some(1);
    builder.cur.depth = 1;
    builder.cur.breadcrumb[0] = 2;
    let mut walker = builder.walker();
    walker.base_state();
    assert_eq!(walker.get_constructor().unwrap().ct_id, 0);
    assert_eq!(walker.cur.depth, 0);
    assert_eq!(walker.cur.breadcrumb[0], 0);
    assert_eq!(builder.cur.point, Some(1));
    assert_eq!(builder.cur.depth, 1);
    assert_eq!(builder.cur.breadcrumb[0], 2);
}

#[test]
fn change_walker_reader_navigates_independently() {
    let (table, mut ctx, engine) = setup(true);
    let mut change = ParserWalkerChange::new(&mut ctx, &table, &engine);
    change.base_state();
    {
        let mut reader = change.as_reader();
        reader.push_operand(1).unwrap();
        assert_eq!(reader.get_constructor().unwrap().ct_id, 1);
        assert_eq!(change.cur.point, Some(0));
        assert_eq!(change.cur.depth, 0);
        assert_eq!(change.cur.breadcrumb[0], 0);
        reader.pop_operand().unwrap();
        assert_eq!(reader.get_constructor().unwrap().ct_id, 0);
    }
    change.push_operand(1).unwrap();
    assert_eq!(change.as_reader().get_constructor().unwrap().ct_id, 1);
}
