//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/IfExprent.java`.

use crate::const_exprent::ConstExprent;
use crate::exprent::{Exprent, ExprentKind};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::list_stack::ListStack;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, jthrow, unported};

/// Java `IfExprent.Type`. Every variant but `Value` is paired with its negation
/// at `ordinal ^ 1`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Eq,
    Ne,
    Lt,
    Ge,
    Gt,
    Le,
    Null,
    Nonnull,
    Icmpeq,
    Icmpne,
    Icmplt,
    Icmpge,
    Icmpgt,
    Icmple,
    Acmpeq,
    Acmpne,
    Value,
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl Type {
    pub const VALUES: [Type; 17] = [
        Type::Eq,
        Type::Ne,
        Type::Lt,
        Type::Ge,
        Type::Gt,
        Type::Le,
        Type::Null,
        Type::Nonnull,
        Type::Icmpeq,
        Type::Icmpne,
        Type::Icmplt,
        Type::Icmpge,
        Type::Icmpgt,
        Type::Icmple,
        Type::Acmpeq,
        Type::Acmpne,
        Type::Value,
    ];

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            Type::Eq => "EQ",
            Type::Ne => "NE",
            Type::Lt => "LT",
            Type::Ge => "GE",
            Type::Gt => "GT",
            Type::Le => "LE",
            Type::Null => "NULL",
            Type::Nonnull => "NONNULL",
            Type::Icmpeq => "ICMPEQ",
            Type::Icmpne => "ICMPNE",
            Type::Icmplt => "ICMPLT",
            Type::Icmpge => "ICMPGE",
            Type::Icmpgt => "ICMPGT",
            Type::Icmple => "ICMPLE",
            Type::Acmpeq => "ACMPEQ",
            Type::Acmpne => "ACMPNE",
            Type::Value => "VALUE",
        }
    }

    /// Java field `functionType`.
    pub fn function_type(self) -> Option<FunctionType> {
        match self {
            Type::Eq | Type::Null | Type::Icmpeq | Type::Acmpeq => Some(FunctionType::Eq),
            Type::Ne | Type::Nonnull | Type::Icmpne | Type::Acmpne => Some(FunctionType::Ne),
            Type::Lt | Type::Icmplt => Some(FunctionType::Lt),
            Type::Ge | Type::Icmpge => Some(FunctionType::Ge),
            Type::Gt | Type::Icmpgt => Some(FunctionType::Gt),
            Type::Le | Type::Icmple => Some(FunctionType::Le),
            Type::Value => None,
        }
    }

    pub fn get_negative(self) -> Type {
        if self == Type::Value {
            jthrow!(IllegalArgumentException; "");
        }
        Type::VALUES[(self.ordinal() ^ 1) as usize]
    }
}

pub struct IfExprentData {
    pub condition: Exprent,
}

java_subclass!(pub IfExprent: Exprent, ExprentKind::If(IfExprentData),
               is_if, as_if, try_as_if);

impl JavaEq for IfExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for IfExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl IfExprent {
    /// Java: `IfExprent(Type, ListStack<Exprent>, BitSet)`. The `IfExprent` id is
    /// taken before the comparison operands it builds.
    pub fn new(
        if_type: Type,
        stack: &mut ListStack<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> IfExprent {
        let id = Exprent::next_id();

        if if_type.ordinal() <= Type::Le.ordinal() {
            stack.push(ConstExprent::new_int(0, true, None).0);
        } else if if_type.ordinal() <= Type::Nonnull.ordinal() {
            stack.push(ConstExprent::new(VarType::VARTYPE_NULL(), None, None).0);
        }

        let condition = match if_type.function_type() {
            None => stack.pop(),
            Some(function_type) => {
                FunctionExprent::new_stack(function_type, stack, bytecode_offsets).0
            }
        };
        IfExprent::alloc_if(id, condition, bytecode_offsets)
    }

    /// Java: `IfExprent(Exprent, BitSet)` (private).
    fn new_cond(condition: Exprent, bytecode_offsets: Option<&BitSet>) -> IfExprent {
        let id = Exprent::next_id();
        IfExprent::alloc_if(id, condition, bytecode_offsets)
    }

    fn alloc_if(id: i32, condition: Exprent, bytecode_offsets: Option<&BitSet>) -> IfExprent {
        let e = Exprent::alloc_with_id(id, ExprentKind::If(IfExprentData { condition }));
        e.add_bytecode_offsets(bytecode_offsets);
        IfExprent(e)
    }

    pub fn copy(self) -> Exprent {
        let condition = self.get_condition();
        let bytecode = self.get_bytecode();
        IfExprent::new_cond(condition.copy(), bytecode.as_ref()).0
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| lst.push(d.condition));
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("IfExprent.toJava")
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.condition {
                d.condition = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ie) = o.try_as_if() else {
            return false;
        };
        self.get_condition().equals(ie.get_condition())
    }

    pub fn negate_if(self) -> IfExprent {
        let condition = self.get_condition();
        let bytecode = condition.get_bytecode();
        let negated =
            FunctionExprent::new_single(FunctionType::BoolNot, condition, bytecode.as_ref());
        self.d_mut(|d| d.condition = negated.0);
        self
    }

    pub fn get_condition(self) -> Exprent {
        self.d(|d| d.condition)
    }

    pub fn set_condition(self, condition: Exprent) {
        self.d_mut(|d| d.condition = condition)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let condition = self.get_condition();
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(condition));
        self.measure_bytecode(values);
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        self.get_condition()
            .process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
    }
}
