//! Lazily bind primitive operand definitions in an immutable runtime specification.
//! Plans retain field definitions, while each evaluation reads its current context.

use std::cell::OnceCell;

use kuna_base::error::KunaResult;

use super::{ConstructorRef, OperandSymbol, SymbolKind, SymbolTable};
use crate::slghpatexpress::{
    PatternExpression, PatternExpressionContext, PatternValue, TokenField,
};

enum PrimitiveValue {
    Token(TokenField),
    Constant(i64),
}

impl PrimitiveValue {
    fn from_value(value: &PatternValue) -> Option<Self> {
        match value {
            PatternValue::TokenField(token) => Some(Self::Token(token.clone())),
            PatternValue::ConstantValue(value) => Some(Self::Constant(value.get_value())),
            _ => None,
        }
    }
}

pub(crate) struct OperandPlan {
    pub(crate) relative_offset: u32,
    value: PrimitiveValue,
}

impl OperandPlan {
    fn new(table: &SymbolTable, operand: &OperandSymbol) -> Option<Self> {
        if operand.get_offset_base() >= 0 {
            return None;
        }
        let value = if let Some(expression) = operand.get_defining_expression() {
            let PatternExpression::Value(value) = expression else {
                return None;
            };
            PrimitiveValue::from_value(value)?
        } else {
            let symbol = table.find_symbol_by_id(operand.get_defining_symbol()?)?;
            match symbol.kind() {
                SymbolKind::Epsilon(_) | SymbolKind::Varnode(_) => PrimitiveValue::Constant(0),
                _ => PrimitiveValue::from_value(symbol.get_pattern_value()?)?,
            }
        };
        Some(Self {
            relative_offset: operand.get_relative_offset(),
            value,
        })
    }

    #[inline]
    pub(crate) fn get_value(
        &self,
        walker: &(impl PatternExpressionContext + ?Sized),
    ) -> KunaResult<i64> {
        match &self.value {
            PrimitiveValue::Token(token) => token.get_value(walker),
            PrimitiveValue::Constant(value) => Ok(*value),
        }
    }
}

type ConstructorPlans = OnceCell<Box<[Option<OperandPlan>]>>;

pub(crate) struct OperandPlans {
    tables: Vec<Vec<ConstructorPlans>>,
}

impl OperandPlans {
    pub(crate) fn new(table: &SymbolTable) -> Self {
        let tables = table
            .symbollist
            .iter()
            .map(|symbol| {
                let Some(symbol) = symbol else {
                    return Vec::new();
                };
                let SymbolKind::Subtable(subtable) = symbol.kind() else {
                    return Vec::new();
                };
                (0..subtable.construct.len())
                    .map(|_| OnceCell::new())
                    .collect()
            })
            .collect();
        Self { tables }
    }

    #[inline]
    pub(crate) fn get(
        &self,
        table: &SymbolTable,
        ct: ConstructorRef,
        index: i32,
    ) -> Option<&OperandPlan> {
        let slot = self
            .tables
            .get(ct.table_id as usize)?
            .get(ct.ct_id as usize)?;
        let plans: &[Option<OperandPlan>] = match slot.get() {
            Some(plans) => plans,
            None => Self::prepare_constructor(slot, table, ct),
        };
        plans.get(index as usize)?.as_ref()
    }

    #[cold]
    fn prepare_constructor<'a>(
        slot: &'a ConstructorPlans,
        table: &SymbolTable,
        ct: ConstructorRef,
    ) -> &'a [Option<OperandPlan>] {
        slot.get_or_init(|| {
            let Ok(constructor) = table.get_constructor(ct) else {
                return Box::new([]);
            };
            constructor
                .operands
                .iter()
                .map(|&id| {
                    let symbol = table.find_symbol_by_id(id)?;
                    let SymbolKind::Operand(operand) = symbol.kind() else {
                        return None;
                    };
                    OperandPlan::new(table, operand)
                })
                .collect()
        })
    }
}
