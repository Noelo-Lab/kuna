//! Prepared runtime expression evaluation, retaining the original tree for
//! encoding, inspection, compiler rewrites and replacement-value evaluation.

use kuna_base::error::KunaResult;
use kuna_base::types::Wrap;

use crate::slghpatexpress::{
    OperandValue, PatternExpression, PatternExpressionContext, PatternValue, TokenField,
};

const STACK_CAPACITY: usize = 8;

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Shl,
    Shr,
    And,
    Or,
    Xor,
    Div,
}

impl BinaryOp {
    #[inline]
    fn apply(self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left.wadd(right),
            Self::Sub => left.wsub(right),
            Self::Mul => left.wmul(right),
            Self::Shl => left.wshl(right as u32),
            Self::Shr => left.wshr(right as u32),
            Self::And => left & right,
            Self::Or => left | right,
            Self::Xor => left ^ right,
            Self::Div => left.wdiv(right),
        }
    }
}

#[derive(Debug, Clone)]
enum Step {
    Constant(i64),
    Leaf(PatternValue),
    Operand(OperandValue),
    Token(TokenField),
    Immediate(BinaryOp, i64),
    LeftImmediate(BinaryOp, i64),
    BinaryLeaf(BinaryOp, PatternValue),
    BinaryOperand(BinaryOp, OperandValue),
    BinaryToken(BinaryOp, TokenField),
    DifferenceBitLeaf(PatternValue, u32),
    DifferenceBitOperand(OperandValue, u32),
    DifferenceBitToken(TokenField, u32),
    DifferenceBitImmediate(i64, u32),
    DifferenceBitLeftImmediate(i64, u32),
    DifferenceBitStack(u32),
    Save,
    BinaryStack(BinaryOp),
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub(crate) struct ExpressionPlan {
    steps: Box<[Step]>,
    stack_depth: usize,
    root: std::mem::Discriminant<PatternExpression>,
}

impl ExpressionPlan {
    pub(crate) fn new(expression: &PatternExpression) -> Option<Self> {
        if matches!(expression, PatternExpression::Value(_)) {
            return None;
        }
        let mut steps = Vec::new();
        let mut stack_depth = 0;
        compile(expression, &mut steps, 0, &mut stack_depth);
        (stack_depth <= STACK_CAPACITY).then(|| Self {
            steps: fuse_bit_tests(steps).into_boxed_slice(),
            stack_depth,
            root: std::mem::discriminant(expression),
        })
    }

    pub(crate) fn applies_to(&self, expression: &PatternExpression) -> bool {
        self.root == std::mem::discriminant(expression)
    }

    pub(crate) fn get_value(
        &self,
        walker: &(impl PatternExpressionContext + ?Sized),
    ) -> KunaResult<i64> {
        if self.stack_depth == 0 {
            return self.execute(walker, &mut []);
        }
        let mut stack = [0; STACK_CAPACITY];
        self.execute(walker, &mut stack)
    }

    fn execute(
        &self,
        walker: &(impl PatternExpressionContext + ?Sized),
        stack: &mut [i64],
    ) -> KunaResult<i64> {
        let mut value = 0;
        let mut depth = 0;
        for step in &self.steps {
            match step {
                Step::Constant(constant) => value = *constant,
                Step::Leaf(leaf) => value = leaf.get_value(walker)?,
                Step::Operand(operand) => value = operand.get_value(walker)?,
                Step::Token(token) => value = token.get_value(walker)?,
                Step::Immediate(op, right) => value = op.apply(value, *right),
                Step::LeftImmediate(op, left) => value = op.apply(*left, value),
                Step::BinaryLeaf(op, right) => value = op.apply(value, right.get_value(walker)?),
                Step::BinaryOperand(op, right) => value = op.apply(value, right.get_value(walker)?),
                Step::BinaryToken(op, right) => value = op.apply(value, right.get_value(walker)?),
                Step::DifferenceBitLeaf(right, shift) => {
                    value = value.wsub(right.get_value(walker)?).wshr(*shift) & 1;
                }
                Step::DifferenceBitOperand(right, shift) => {
                    value = value.wsub(right.get_value(walker)?).wshr(*shift) & 1;
                }
                Step::DifferenceBitToken(right, shift) => {
                    value = value.wsub(right.get_value(walker)?).wshr(*shift) & 1;
                }
                Step::DifferenceBitImmediate(right, shift) => {
                    value = value.wsub(*right).wshr(*shift) & 1;
                }
                Step::DifferenceBitLeftImmediate(left, shift) => {
                    value = left.wsub(value).wshr(*shift) & 1;
                }
                Step::DifferenceBitStack(shift) => {
                    depth -= 1;
                    value = stack[depth].wsub(value).wshr(*shift) & 1;
                }
                Step::Save => {
                    stack[depth] = value;
                    depth += 1;
                }
                Step::BinaryStack(op) => {
                    depth -= 1;
                    value = op.apply(stack[depth], value);
                }
                Step::Negate => value = value.wneg(),
                Step::Not => value = !value,
            }
        }
        Ok(value)
    }
}

fn fuse_bit_tests(steps: Vec<Step>) -> Vec<Step> {
    let mut result = Vec::with_capacity(steps.len());
    for step in steps {
        if matches!(&step, Step::Immediate(BinaryOp::And, 1)) && result.len() >= 2 {
            if let Some(Step::Immediate(BinaryOp::Shr, shift)) = result.last() {
                let shift = *shift as u32;
                if matches!(
                    &result[result.len() - 2],
                    Step::BinaryLeaf(BinaryOp::Sub, _)
                        | Step::BinaryOperand(BinaryOp::Sub, _)
                        | Step::BinaryToken(BinaryOp::Sub, _)
                        | Step::Immediate(BinaryOp::Sub, _)
                        | Step::LeftImmediate(BinaryOp::Sub, _)
                        | Step::BinaryStack(BinaryOp::Sub)
                ) {
                    result.pop();
                    let difference = result.pop().unwrap();
                    result.push(match difference {
                        Step::BinaryLeaf(_, right) => Step::DifferenceBitLeaf(right, shift),
                        Step::BinaryOperand(_, right) => Step::DifferenceBitOperand(right, shift),
                        Step::BinaryToken(_, right) => Step::DifferenceBitToken(right, shift),
                        Step::Immediate(_, right) => Step::DifferenceBitImmediate(right, shift),
                        Step::LeftImmediate(_, left) => {
                            Step::DifferenceBitLeftImmediate(left, shift)
                        }
                        Step::BinaryStack(_) => Step::DifferenceBitStack(shift),
                        _ => unreachable!(),
                    });
                    continue;
                }
            }
        }
        result.push(step);
    }
    result
}

fn binary(
    expression: &PatternExpression,
) -> Option<(BinaryOp, &PatternExpression, &PatternExpression)> {
    let (op, children) = match expression {
        PatternExpression::Plus(b) => (BinaryOp::Add, b),
        PatternExpression::Sub(b) => (BinaryOp::Sub, b),
        PatternExpression::Mult(b) => (BinaryOp::Mul, b),
        PatternExpression::LeftShift(b) => (BinaryOp::Shl, b),
        PatternExpression::RightShift(b) => (BinaryOp::Shr, b),
        PatternExpression::And(b) => (BinaryOp::And, b),
        PatternExpression::Or(b) => (BinaryOp::Or, b),
        PatternExpression::Xor(b) => (BinaryOp::Xor, b),
        PatternExpression::Div(b) => (BinaryOp::Div, b),
        _ => return None,
    };
    Some((op, children.get_left(), children.get_right()))
}

fn constant(expression: &PatternExpression) -> Option<i64> {
    match expression {
        PatternExpression::Value(PatternValue::ConstantValue(value)) => Some(value.get_value()),
        PatternExpression::Minus(child) => Some(constant(child.get_unary())?.wneg()),
        PatternExpression::Not(child) => Some(!constant(child.get_unary())?),
        _ => {
            let (op, left, right) = binary(expression)?;
            let left = constant(left)?;
            let right = constant(right)?;
            if matches!(op, BinaryOp::Div) && right == 0 {
                return None;
            }
            Some(op.apply(left, right))
        }
    }
}

fn compile(
    expression: &PatternExpression,
    steps: &mut Vec<Step>,
    depth: usize,
    maximum: &mut usize,
) {
    if let Some(value) = constant(expression) {
        steps.push(Step::Constant(value));
        return;
    }
    match expression {
        PatternExpression::Value(value) => steps.push(match value {
            PatternValue::OperandValue(value) => Step::Operand(value.clone()),
            PatternValue::TokenField(value) => Step::Token(value.clone()),
            _ => Step::Leaf(value.clone()),
        }),
        PatternExpression::Minus(child) => {
            compile(child.get_unary(), steps, depth, maximum);
            steps.push(Step::Negate);
        }
        PatternExpression::Not(child) => {
            compile(child.get_unary(), steps, depth, maximum);
            steps.push(Step::Not);
        }
        _ => {
            let (op, left, right) = binary(expression).expect("binary expression");
            if !matches!(right, PatternExpression::Value(_)) {
                if let Some(value) = constant(left) {
                    compile(right, steps, depth, maximum);
                    steps.push(Step::LeftImmediate(op, value));
                    return;
                }
            }
            compile(left, steps, depth, maximum);
            if let Some(value) = constant(right) {
                steps.push(Step::Immediate(op, value));
            } else if let PatternExpression::Value(value) = right {
                steps.push(match value {
                    PatternValue::OperandValue(value) => Step::BinaryOperand(op, value.clone()),
                    PatternValue::TokenField(value) => Step::BinaryToken(op, value.clone()),
                    _ => Step::BinaryLeaf(op, value.clone()),
                });
            } else {
                steps.push(Step::Save);
                *maximum = (*maximum).max(depth + 1);
                compile(right, steps, depth + 1, maximum);
                steps.push(Step::BinaryStack(op));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slghpatexpress::{BinaryExpression, ConstantValue, OperandValue, UnaryExpression};
    use kuna_base::address::Address;
    use kuna_base::error::{KunaError, KunaResult};
    use std::cell::RefCell;

    struct Walker {
        values: [i64; 4],
        fail: Option<i32>,
        reads: RefCell<Vec<(i32, u32, u32)>>,
    }

    impl PatternExpressionContext for Walker {
        fn get_instruction_bytes(&self, _: i32, _: i32) -> KunaResult<u32> {
            Err(KunaError::bad_data("instruction read"))
        }
        fn get_context_bytes(&self, _: i32, _: i32) -> KunaResult<u32> {
            unreachable!()
        }
        fn get_addr(&self) -> Address {
            Address::default()
        }
        fn get_naddr(&self) -> Address {
            Address::default()
        }
        fn get_n2addr(&self) -> KunaResult<Address> {
            Err(KunaError::sleigh("next2"))
        }
        fn operand_value(&self, index: i32, table: u32, ct: u32) -> KunaResult<i64> {
            self.reads.borrow_mut().push((index, table, ct));
            if self.fail == Some(index) {
                return Err(KunaError::sleigh("operand failure"));
            }
            Ok(self.values[index as usize])
        }
    }

    fn literal(value: i64) -> PatternExpression {
        PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(value)))
    }

    fn operand(index: i32) -> PatternExpression {
        PatternExpression::Value(PatternValue::OperandValue(OperandValue::new(index, 7, 3)))
    }

    fn compare(expression: &PatternExpression) {
        let Some(plan) = ExpressionPlan::new(expression) else {
            return;
        };
        for values in [
            [i64::MIN, -1, 64, i64::MAX],
            [0, 1, -65, 7],
            [19, -13, 129, -1024],
        ] {
            for fail in [None, Some(0), Some(1), Some(2), Some(3)] {
                let walker = Walker {
                    values,
                    fail,
                    reads: RefCell::new(Vec::new()),
                };
                let expected = expression.get_value(&walker).map_err(|e| e.to_string());
                let reads = walker.reads.take();
                let actual = plan.get_value(&walker).map_err(|e| e.to_string());
                assert_eq!(actual, expected, "{expression:?}");
                assert_eq!(
                    *walker.reads.borrow(),
                    reads,
                    "evaluation order: {expression:?}"
                );
            }
        }
    }

    fn generated(seed: &mut u64, depth: usize) -> PatternExpression {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let value = (*seed >> 32) as usize;
        if depth == 0 || value % 5 == 0 {
            return if value & 1 == 0 {
                literal(*seed as i64)
            } else {
                operand((value % 4) as i32)
            };
        }
        if value % 11 == 0 {
            return PatternExpression::Not(UnaryExpression::new(generated(seed, depth - 1)));
        }
        let children =
            BinaryExpression::new(generated(seed, depth - 1), generated(seed, depth - 1));
        match value % 8 {
            0 => PatternExpression::Plus(children),
            1 => PatternExpression::Sub(children),
            2 => PatternExpression::Mult(children),
            3 => PatternExpression::LeftShift(children),
            4 => PatternExpression::RightShift(children),
            5 => PatternExpression::And(children),
            6 => PatternExpression::Or(children),
            _ => PatternExpression::Xor(children),
        }
    }

    #[test]
    fn prepared_expressions_preserve_values_failures_and_read_order() {
        let mut seed = 0x5e16_2026;
        for _ in 0..400 {
            compare(&generated(&mut seed, 5));
        }
        compare(&PatternExpression::Div(BinaryExpression::new(
            operand(0),
            literal(-1),
        )));
        compare(&PatternExpression::Minus(UnaryExpression::new(operand(0))));
        compare(&PatternExpression::Plus(BinaryExpression::new(
            operand(0),
            PatternExpression::Value(PatternValue::Next2InstructionValue(
                crate::slghpatexpress::Next2InstructionValue,
            )),
        )));
    }

    #[test]
    fn subtraction_bit_tests_preserve_wrapping_shifts_and_failures() {
        let operands = [
            operand(1),
            literal(31),
            PatternExpression::Plus(BinaryExpression::new(operand(1), literal(5))),
            PatternExpression::Value(PatternValue::TokenField(TokenField::new(
                1, false, false, 0, 7,
            ))),
            PatternExpression::Value(PatternValue::Next2InstructionValue(
                crate::slghpatexpress::Next2InstructionValue,
            )),
        ];
        for left in [operand(0), literal(63)] {
            for right in &operands {
                for shift in [0, 6, 7, 63, 64, -1, 129] {
                    let difference =
                        PatternExpression::Sub(BinaryExpression::new(left.clone(), right.clone()));
                    let shifted = PatternExpression::RightShift(BinaryExpression::new(
                        difference,
                        literal(shift),
                    ));
                    compare(&PatternExpression::And(BinaryExpression::new(
                        shifted,
                        literal(1),
                    )));
                }
            }
        }
    }

    #[test]
    fn preparation_preserves_constant_division_failures_until_evaluation() {
        let expression = PatternExpression::Div(BinaryExpression::new(literal(1), literal(0)));
        let plan = ExpressionPlan::new(&expression).unwrap();
        let walker = Walker {
            values: [0; 4],
            fail: None,
            reads: RefCell::new(Vec::new()),
        };
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| plan.get_value(&walker)))
                .is_err()
        );
    }

    #[test]
    fn expression_mutation_invalidates_prepared_operand_references() {
        let walker = Walker {
            values: [3, 17, 23, 42],
            fail: None,
            reads: RefCell::new(Vec::new()),
        };
        let mut expression = PatternExpression::Plus(BinaryExpression::new(operand(0), literal(2)));
        expression.prepare_runtime();
        assert_eq!(expression.get_value(&walker).unwrap(), 5);
        expression.remap_operand_index(&[1]);
        assert_eq!(expression.get_value(&walker).unwrap(), 19);
        expression.prepare_runtime();
        assert_eq!(expression.get_value(&walker).unwrap(), 19);
        expression.remap_table_id(&|_| 11);
        assert_eq!(expression.get_value(&walker).unwrap(), 19);
        assert_eq!(walker.reads.borrow().last(), Some(&(1, 11, 3)));
        expression.prepare_runtime();
        let PatternExpression::Plus(children) = &mut expression else {
            unreachable!()
        };
        *children.get_right_mut() = literal(5);
        assert_eq!(expression.get_value(&walker).unwrap(), 22);
    }

    #[test]
    fn changing_the_outer_operator_declines_a_plan_for_the_old_operator() {
        let walker = Walker {
            values: [3, 17, 23, 42],
            fail: None,
            reads: RefCell::new(Vec::new()),
        };
        let mut expression = PatternExpression::Plus(BinaryExpression::new(operand(0), literal(2)));
        expression.prepare_runtime();
        assert_eq!(expression.get_value(&walker).unwrap(), 5);
        let PatternExpression::Plus(children) = expression else {
            unreachable!()
        };
        let subtract = PatternExpression::Sub(children);
        assert_eq!(subtract.get_value(&walker).unwrap(), 1);
        let mut expression = PatternExpression::Minus(UnaryExpression::new(operand(0)));
        expression.prepare_runtime();
        assert_eq!(expression.get_value(&walker).unwrap(), -3);
        let PatternExpression::Minus(child) = expression else {
            unreachable!()
        };
        assert_eq!(
            PatternExpression::Not(child).get_value(&walker).unwrap(),
            !3
        );
    }

    #[test]
    fn large_right_subtrees_retain_tree_evaluation() {
        let walker = Walker {
            values: [3, 17, 23, 42],
            fail: None,
            reads: RefCell::new(Vec::new()),
        };
        let mut expression = operand(3);
        for _ in 0..16 {
            expression = PatternExpression::Plus(BinaryExpression::new(operand(0), expression));
        }
        assert!(ExpressionPlan::new(&expression).is_none());
        expression.prepare_runtime();
        assert_eq!(expression.get_value(&walker).unwrap(), 90);
        assert_eq!(walker.reads.borrow().len(), 17);
    }

    #[test]
    fn a_shared_lazy_plan_reads_each_callers_operand_values() {
        use std::sync::{Arc, Barrier};
        let mut expression = PatternExpression::Sub(BinaryExpression::new(
            operand(0),
            PatternExpression::Plus(BinaryExpression::new(operand(1), literal(2))),
        ));
        expression.prepare_runtime();
        let expression = Arc::new(expression);
        let barrier = Arc::new(Barrier::new(8));
        let threads: Vec<_> = (0..8i64)
            .map(|value| {
                let expression = Arc::clone(&expression);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    let walker = Walker {
                        values: [7 * value, value, 0, 0],
                        fail: None,
                        reads: RefCell::new(Vec::new()),
                    };
                    barrier.wait();
                    assert_eq!(expression.get_value(&walker).unwrap(), 6 * value - 2);
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
