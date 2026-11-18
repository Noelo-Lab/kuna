//! kuna fix `kt_inline_evaluation_order` (spec 10.16): where the Kotlin
//! inliner may move a local's value that calls or reads an array.
//!
//! `StackVarsProcessor` inlines a stack variable into the next statement at
//! the position the variable has there, and for a stack variable that position
//! is where the value was on the operand stack: nothing the next statement
//! evaluates before it was computed after it. The Kotlin plugin runs the same
//! inliner over every local (`INLINE_ALL_VARS`), and a local carries no such
//! guarantee: `val x = g(); use(h(), x)` became `use(h(), g())`, calling `h`
//! first, `val x = g(); use(if (c) x else 0)` became `use(if (c) g() else 0)`,
//! calling `g` only when `c` holds, and `val x = a[0]; use(g(), x)` read the
//! element after `g()`, which may change it.
//!
//! With the fix, a local that is not a stack variable is inlined, by the
//! classes of [`Movability`], only where its value means the same: a value
//! that reads only locals and fields moves as before (the inliner checks the
//! locals' versions and the fields' liveness); one that also reads an array
//! element moves only into the next statement (the next exprent of its list,
//! or the first of the single node that follows, such as a loop condition),
//! to a use evaluated before anything there that calls or stores to the heap;
//! one that calls, allocates or stores moves only into the next statement, to
//! a use evaluated on every evaluation of it and before anything that touches
//! the heap. As for field
//! reads in Vineflower, the order of exceptions a moved value or its
//! neighbours may throw (a null receiver, a failed cast, an index out of
//! bounds) is not kept.

use crate::code_type::CodeType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::var_version_pair::VarVersionPair;

pub struct KtInlineEvaluationOrder;

/// What moving a value to a later point can change.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Movability {
    /// Reads locals and fields only: the inliner's version and liveness checks suffice.
    Anywhere,
    /// Also reads array elements, which a call or a store may change.
    HeapRead,
    /// Calls, allocates or stores.
    Effects,
}

/// What an evaluated exprent did.
const WRITES_HEAP: u8 = 1;
const READS_HEAP: u8 = 2;
const READS_WRITTEN_LOCAL: u8 = 4;

enum Walk {
    /// The use: whether everything before it allows the move.
    Found(bool),
    /// Not found: what the exprent's evaluation does.
    Missing(u8),
}

impl KtInlineEvaluationOrder {
    /// Whether the value `value` of a local that is not a stack variable may
    /// replace its use `used`; `next` is the exprent evaluated right after the
    /// definition: the next one of its list, or the first one of the single
    /// node that follows.
    pub fn may_move(value: Exprent, next: Option<Exprent>, used: VarVersionPair) -> bool {
        let movability = Self::movability(value);
        if movability == Movability::Anywhere {
            return true;
        }
        let Some(next) = next else {
            return false;
        };
        let written = Self::assigned_locals(value);
        matches!(
            Self::walk(next, used, &written, movability),
            Walk::Found(true)
        )
    }

    /// Whether `used` is in `next`, the exprent evaluated right after the
    /// definition, where nothing that may change the heap comes before it:
    /// where a value whose field reads the SSAU form does not track may go
    /// (fix `field_read_all_paths`).
    pub fn heap_unchanged_before(
        value: Exprent,
        next: Option<Exprent>,
        used: VarVersionPair,
    ) -> bool {
        let Some(next) = next else {
            return false;
        };
        let written = Self::assigned_locals(value);
        matches!(
            Self::walk(next, used, &written, Movability::HeapRead),
            Walk::Found(true)
        )
    }

    pub fn movability(value: Exprent) -> Movability {
        match value.get_type() {
            ExprentType::Const | ExprentType::Var => Movability::Anywhere,
            ExprentType::Field => value
                .as_field()
                .get_instance()
                .map_or(Movability::Anywhere, Self::movability),
            ExprentType::Array => {
                let array = value.as_array();
                Movability::HeapRead
                    .max(Self::movability(array.get_array()))
                    .max(Self::movability(array.get_index()))
            }
            ExprentType::Function => {
                let function = value.as_function();
                if Self::own_flags(function) & WRITES_HEAP != 0 {
                    return Movability::Effects;
                }
                Self::operands(function)
                    .into_iter()
                    .map(Self::movability)
                    .max()
                    .unwrap_or(Movability::Anywhere)
            }
            _ => Movability::Effects,
        }
    }

    /// What `function`'s own operation does: `++`/`--` on a field or an array
    /// element store to the heap, string concatenation of an object calls its
    /// `toString`.
    fn own_flags(function: FunctionExprent) -> u8 {
        match function.get_func_type() {
            FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi => {
                if function.get_lst_operands().get(0).is_var() {
                    0
                } else {
                    WRITES_HEAP | READS_HEAP
                }
            }
            FunctionType::StrConcat => {
                let calls = function.get_lst_operands().iter().any(|operand| {
                    let t = operand.get_expr_type();
                    let string = t.type_ == CodeType::Object
                        && t.array_dim == 0
                        && t.value.as_deref() == Some("java/lang/String");
                    let primitive =
                        t.array_dim == 0 && !matches!(t.type_, CodeType::Object | CodeType::Genvar);
                    !string && !primitive
                });
                if calls {
                    WRITES_HEAP | READS_HEAP
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// The operands `function` evaluates (a cast's type operand is not one).
    fn operands(function: FunctionExprent) -> Vec<Exprent> {
        let operands = function.get_lst_operands().to_vec();
        if function.get_func_type() == FunctionType::Cast {
            operands.into_iter().take(1).collect()
        } else {
            operands
        }
    }

    fn assigned_locals(value: Exprent) -> Vec<i32> {
        let mut res = Vec::new();
        for expr in value.get_all_exprents_rec_self(true, true) {
            let target = if let Some(assignment) = expr.try_as_assignment() {
                Some(assignment.get_left())
            } else if let Some(function) = expr.try_as_function() {
                matches!(
                    function.get_func_type(),
                    FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi
                )
                .then(|| function.get_lst_operands().get(0))
            } else {
                None
            };
            if let Some(var) = target.and_then(|t| t.try_as_var()) {
                res.push(var.get_index());
            }
        }
        res
    }

    /// Whether what was evaluated before the use (`done`) lets a value of
    /// `movability` move there.
    fn allows(done: u8, movability: Movability) -> bool {
        match movability {
            Movability::Anywhere => true,
            Movability::HeapRead => done & WRITES_HEAP == 0,
            Movability::Effects => done == 0,
        }
    }

    /// Looks for `used` in `expr`, its operands in evaluation order.
    fn walk(expr: Exprent, used: VarVersionPair, written: &[i32], movability: Movability) -> Walk {
        let mut children: Vec<(Exprent, bool)> = Vec::new();
        let own = match expr.get_type() {
            ExprentType::Var => {
                let var = expr.as_var();
                if var.get_var_version_pair() == used {
                    return Walk::Found(true);
                }
                let flags = if written.contains(&var.get_index()) {
                    READS_WRITTEN_LOCAL
                } else {
                    0
                };
                return Walk::Missing(flags);
            }
            ExprentType::Const => return Walk::Missing(0),
            ExprentType::Assignment => {
                let assignment = expr.as_assignment();
                let left = assignment.get_left();
                if assignment.get_cond_type().is_some() {
                    children.push((left, false));
                } else if let Some(field) = left.try_as_field() {
                    children.extend(field.get_instance().map(|i| (i, false)));
                } else if let Some(array) = left.try_as_array() {
                    children.push((array.get_array(), false));
                    children.push((array.get_index(), false));
                }
                children.push((assignment.get_right(), false));
                if left.is_var() {
                    0
                } else {
                    WRITES_HEAP
                }
            }
            ExprentType::Invocation => {
                let invocation = expr.as_invocation();
                children.extend(invocation.get_instance().map(|i| (i, false)));
                children.extend(invocation.get_lst_parameters().iter().map(|p| (p, false)));
                WRITES_HEAP | READS_HEAP
            }
            ExprentType::New => {
                let new = expr.as_new();
                if let Some(constructor) = new.get_constructor() {
                    children.extend(constructor.get_lst_parameters().iter().map(|p| (p, false)));
                }
                children.extend(new.get_lst_dims().iter().map(|d| (d, false)));
                children.extend(new.get_lst_array_elements().iter().map(|e| (e, false)));
                WRITES_HEAP | READS_HEAP
            }
            ExprentType::Function => {
                let function = expr.as_function();
                let conditional_from = match function.get_func_type() {
                    FunctionType::Ternary | FunctionType::BooleanAnd | FunctionType::BooleanOr => 1,
                    _ => usize::MAX,
                };
                for (i, operand) in Self::operands(function).into_iter().enumerate() {
                    children.push((operand, i >= conditional_from));
                }
                Self::own_flags(function)
            }
            ExprentType::Field => {
                children.extend(expr.as_field().get_instance().map(|i| (i, false)));
                READS_HEAP
            }
            ExprentType::Array => {
                let array = expr.as_array();
                children.push((array.get_array(), false));
                children.push((array.get_index(), false));
                READS_HEAP
            }
            ExprentType::Exit => {
                children.extend(expr.as_exit().get_value().map(|v| (v, false)));
                0
            }
            ExprentType::If => {
                children.push((expr.as_if().get_condition(), false));
                0
            }
            ExprentType::SwitchHead => {
                children.push((expr.as_switch_head().get_value(), false));
                0
            }
            ExprentType::Monitor => {
                children.push((expr.as_monitor().get_value(), false));
                WRITES_HEAP | READS_HEAP
            }
            _ => {
                let found = expr
                    .get_all_exprents_rec_self(true, true)
                    .into_iter()
                    .any(|e| {
                        e.try_as_var()
                            .is_some_and(|v| v.get_var_version_pair() == used)
                    });
                return if found {
                    Walk::Found(false)
                } else {
                    Walk::Missing(WRITES_HEAP | READS_HEAP)
                };
            }
        };

        let mut done = 0;
        for (child, conditional) in children {
            match Self::walk(child, used, written, movability) {
                Walk::Found(ok) => {
                    let unconditional = !conditional || movability != Movability::Effects;
                    return Walk::Found(ok && unconditional && Self::allows(done, movability));
                }
                Walk::Missing(flags) => done |= flags,
            }
        }
        Walk::Missing(done | own)
    }
}
