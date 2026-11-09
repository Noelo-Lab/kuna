//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/FunctionExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::expr_processor::ExprProcessor;
use crate::exprent::{Exprent, ExprentKind, ExprentType};
use crate::i_matchable::MatchProperties;
use crate::int_helper::IntHelper;
use crate::list_stack::ListStack;
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::pattern::Pattern;
use crate::rt::{BitSet, JList, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::typed::Typed;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::{instance_of, VarType};
use crate::{java_subclass, jthrow};

/// Java `FunctionExprent.FunctionType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum FunctionType {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Rem,
    Shl,
    Shr,
    Ushr,

    BitNot,
    BoolNot,
    Neg,

    I2l,
    I2f,
    I2d,
    L2i,
    L2f,
    L2d,
    F2i,
    F2l,
    F2d,
    D2i,
    D2l,
    D2f,
    I2b,
    I2c,
    I2s,

    Cast,
    Instanceof,

    ArrayLength,
    Imm,
    Mmi,
    Ipp,
    Ppi,

    Ternary,

    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,

    Eq,
    Ne,
    Lt,
    Ge,
    Gt,
    Le,

    BooleanAnd,
    BooleanOr,
    StrConcat,

    Other,
}

impl Typed for FunctionType {}

impl JavaHash for FunctionType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for FunctionType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl FunctionType {
    pub const VALUES: [FunctionType; 52] = [
        FunctionType::Add,
        FunctionType::Sub,
        FunctionType::Mul,
        FunctionType::Div,
        FunctionType::And,
        FunctionType::Or,
        FunctionType::Xor,
        FunctionType::Rem,
        FunctionType::Shl,
        FunctionType::Shr,
        FunctionType::Ushr,
        FunctionType::BitNot,
        FunctionType::BoolNot,
        FunctionType::Neg,
        FunctionType::I2l,
        FunctionType::I2f,
        FunctionType::I2d,
        FunctionType::L2i,
        FunctionType::L2f,
        FunctionType::L2d,
        FunctionType::F2i,
        FunctionType::F2l,
        FunctionType::F2d,
        FunctionType::D2i,
        FunctionType::D2l,
        FunctionType::D2f,
        FunctionType::I2b,
        FunctionType::I2c,
        FunctionType::I2s,
        FunctionType::Cast,
        FunctionType::Instanceof,
        FunctionType::ArrayLength,
        FunctionType::Imm,
        FunctionType::Mmi,
        FunctionType::Ipp,
        FunctionType::Ppi,
        FunctionType::Ternary,
        FunctionType::Lcmp,
        FunctionType::Fcmpl,
        FunctionType::Fcmpg,
        FunctionType::Dcmpl,
        FunctionType::Dcmpg,
        FunctionType::Eq,
        FunctionType::Ne,
        FunctionType::Lt,
        FunctionType::Ge,
        FunctionType::Gt,
        FunctionType::Le,
        FunctionType::BooleanAnd,
        FunctionType::BooleanOr,
        FunctionType::StrConcat,
        FunctionType::Other,
    ];

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            FunctionType::Add => "ADD",
            FunctionType::Sub => "SUB",
            FunctionType::Mul => "MUL",
            FunctionType::Div => "DIV",
            FunctionType::And => "AND",
            FunctionType::Or => "OR",
            FunctionType::Xor => "XOR",
            FunctionType::Rem => "REM",
            FunctionType::Shl => "SHL",
            FunctionType::Shr => "SHR",
            FunctionType::Ushr => "USHR",
            FunctionType::BitNot => "BIT_NOT",
            FunctionType::BoolNot => "BOOL_NOT",
            FunctionType::Neg => "NEG",
            FunctionType::I2l => "I2L",
            FunctionType::I2f => "I2F",
            FunctionType::I2d => "I2D",
            FunctionType::L2i => "L2I",
            FunctionType::L2f => "L2F",
            FunctionType::L2d => "L2D",
            FunctionType::F2i => "F2I",
            FunctionType::F2l => "F2L",
            FunctionType::F2d => "F2D",
            FunctionType::D2i => "D2I",
            FunctionType::D2l => "D2L",
            FunctionType::D2f => "D2F",
            FunctionType::I2b => "I2B",
            FunctionType::I2c => "I2C",
            FunctionType::I2s => "I2S",
            FunctionType::Cast => "CAST",
            FunctionType::Instanceof => "INSTANCEOF",
            FunctionType::ArrayLength => "ARRAY_LENGTH",
            FunctionType::Imm => "IMM",
            FunctionType::Mmi => "MMI",
            FunctionType::Ipp => "IPP",
            FunctionType::Ppi => "PPI",
            FunctionType::Ternary => "TERNARY",
            FunctionType::Lcmp => "LCMP",
            FunctionType::Fcmpl => "FCMPL",
            FunctionType::Fcmpg => "FCMPG",
            FunctionType::Dcmpl => "DCMPL",
            FunctionType::Dcmpg => "DCMPG",
            FunctionType::Eq => "EQ",
            FunctionType::Ne => "NE",
            FunctionType::Lt => "LT",
            FunctionType::Ge => "GE",
            FunctionType::Gt => "GT",
            FunctionType::Le => "LE",
            FunctionType::BooleanAnd => "BOOLEAN_AND",
            FunctionType::BooleanOr => "BOOLEAN_OR",
            FunctionType::StrConcat => "STR_CONCAT",
            FunctionType::Other => "OTHER",
        }
    }

    /// `(arity, operator, precedence)` of the Java constructor call.
    fn props(self) -> (i32, Option<&'static str>, i32) {
        use FunctionType::*;
        match self {
            Add => (2, Some("+"), 3),
            Sub => (2, Some("-"), 3),
            Mul => (2, Some("*"), 2),
            Div => (2, Some("/"), 2),
            And => (2, Some("&"), 7),
            Or => (2, Some("|"), 9),
            Xor => (2, Some("^"), 8),
            Rem => (2, Some("%"), 2),
            Shl => (2, Some("<<"), 4),
            Shr => (2, Some(">>"), 4),
            Ushr => (2, Some(">>>"), 4),
            BitNot => (1, Some("~"), 1),
            BoolNot => (1, Some("!"), 1),
            Neg => (1, Some("-"), 1),
            I2l | I2f | I2d | L2i | L2f | L2d | F2i | F2l | F2d | D2i | D2l | D2f | I2b | I2c
            | I2s => (1, None, 1),
            Cast => (2, None, 1),
            Instanceof => (2, Some("instanceof"), 6),
            ArrayLength => (1, None, 0),
            Imm => (1, Some("--"), 1),
            Mmi => (1, Some("--"), 1),
            Ipp => (1, Some("++"), 1),
            Ppi => (1, Some("++"), 1),
            Ternary => (3, None, 12),
            Lcmp => (2, Some("__lcmp__"), -1),
            Fcmpl => (2, Some("__fcmpl__"), -1),
            Fcmpg => (2, Some("__fcmpg__"), -1),
            Dcmpl => (2, Some("__dcmpl__"), -1),
            Dcmpg => (2, Some("__dcmpg__"), -1),
            Eq => (2, Some("=="), 6),
            Ne => (2, Some("!="), 6),
            Lt => (2, Some("<"), 5),
            Ge => (2, Some(">="), 5),
            Gt => (2, Some(">"), 5),
            Le => (2, Some("<="), 5),
            BooleanAnd => (2, Some("&&"), 10),
            BooleanOr => (2, Some("||"), 11),
            StrConcat => (2, Some("+"), 3),
            Other => (0, Some("??????"), 999),
        }
    }

    /// Java field `arity`.
    pub fn arity(self) -> i32 {
        self.props().0
    }

    /// Java field `operator`.
    pub fn operator(self) -> Option<&'static str> {
        self.props().1
    }

    /// Java field `precedence`.
    pub fn precedence(self) -> i32 {
        self.props().2
    }

    /// Java field `castType`: the target of the primitive conversions `I2L` .. `I2S`.
    pub fn cast_type(self) -> Option<VarType> {
        use FunctionType::*;
        match self {
            I2l => Some(VarType::VARTYPE_LONG()),
            I2f => Some(VarType::VARTYPE_FLOAT()),
            I2d => Some(VarType::VARTYPE_DOUBLE()),
            L2i => Some(VarType::VARTYPE_INT()),
            L2f => Some(VarType::VARTYPE_FLOAT()),
            L2d => Some(VarType::VARTYPE_DOUBLE()),
            F2i => Some(VarType::VARTYPE_INT()),
            F2l => Some(VarType::VARTYPE_LONG()),
            F2d => Some(VarType::VARTYPE_DOUBLE()),
            D2i => Some(VarType::VARTYPE_INT()),
            D2l => Some(VarType::VARTYPE_LONG()),
            D2f => Some(VarType::VARTYPE_FLOAT()),
            I2b => Some(VarType::VARTYPE_BYTE()),
            I2c => Some(VarType::VARTYPE_CHAR()),
            I2s => Some(VarType::VARTYPE_SHORT()),
            _ => None,
        }
    }

    /// Java field `type`: the fixed result type, when the function has one.
    pub fn type_(self) -> Option<VarType> {
        use FunctionType::*;
        match self {
            BoolNot | Instanceof | Lcmp | Fcmpl | Fcmpg | Dcmpl | Dcmpg | Eq | Ne | Lt | Ge
            | Gt | Le | BooleanAnd | BooleanOr => Some(VarType::VARTYPE_BOOLEAN()),
            ArrayLength => Some(VarType::VARTYPE_INT()),
            StrConcat => Some(VarType::VARTYPE_STRING()),
            _ => self.cast_type(),
        }
    }

    pub fn is_arithmetic_binary_operation(self) -> bool {
        self.ordinal() <= FunctionType::Ushr.ordinal()
    }

    /// Java: `isMM()`.
    pub fn is_mm(self) -> bool {
        self == FunctionType::Mmi || self == FunctionType::Imm
    }

    /// Java: `isPP()`.
    pub fn is_pp(self) -> bool {
        self == FunctionType::Ppi || self == FunctionType::Ipp
    }

    /// Java: `isPPMM()`.
    pub fn is_ppmm(self) -> bool {
        self.is_pp() || self.is_mm()
    }

    /// Java: `isPostfixPPMM()`.
    pub fn is_postfix_ppmm(self) -> bool {
        self == FunctionType::Imm || self == FunctionType::Ipp
    }
}

#[track_caller]
fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

/// `(Integer) constExprent.getValue()`, unboxed.
fn const_int_value(c: ConstExprent) -> i32 {
    match c.get_value() {
        Some(v) => v.as_int(),
        None => npe("constant value"),
    }
}

/// Java `a == b` where `a` may be `null`.
fn same_type(a: &Option<VarType>, b: &VarType) -> bool {
    a.as_ref().is_some_and(|a| a.ptr_eq(b))
}

const ASSOCIATIVITY: [FunctionType; 8] = [
    FunctionType::Add,
    FunctionType::Mul,
    FunctionType::And,
    FunctionType::Or,
    FunctionType::Xor,
    FunctionType::BooleanAnd,
    FunctionType::BooleanOr,
    FunctionType::StrConcat,
];

pub struct FunctionExprentData {
    pub func_type: FunctionType,
    /// Set for `++`/`--` and string concatenation; `None` otherwise.
    pub implicit_type: Option<VarType>,
    pub lst_operands: JList<Exprent>,
    pub needs_cast: bool,
    pub disable_newline_group_creation: bool,
}

java_subclass!(pub FunctionExprent: Exprent, ExprentKind::Function(FunctionExprentData),
               is_function, as_function, try_as_function);

impl JavaEq for FunctionExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for FunctionExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl FunctionExprent {
    /// Java: `FunctionExprent(FunctionType, ListStack<Exprent>, BitSet)`.
    pub fn new_stack(
        func_type: FunctionType,
        stack: &mut ListStack<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> FunctionExprent {
        let id = Exprent::next_id();
        let lst_operands: JList<Exprent> = JList::new();
        if func_type.arity() == 1 {
            lst_operands.add(stack.pop());
        } else if func_type.arity() == 2 {
            let expr = stack.pop();
            lst_operands.add(stack.pop());
            lst_operands.add(expr);
        } else {
            jthrow!("no direct instantiation possible: {}", func_type.name());
        }
        FunctionExprent::alloc_function(id, func_type, lst_operands, bytecode_offsets)
    }

    /// Java: `FunctionExprent(FunctionType, List<Exprent>, BitSet)`. The list is
    /// kept, not copied: pass a `JList` to share it with the caller.
    pub fn new(
        func_type: FunctionType,
        operands: impl Into<JList<Exprent>>,
        bytecode_offsets: Option<&BitSet>,
    ) -> FunctionExprent {
        let id = Exprent::next_id();
        FunctionExprent::alloc_function(id, func_type, operands.into(), bytecode_offsets)
    }

    /// Java: `FunctionExprent(FunctionType, Exprent, BitSet)`.
    pub fn new_single(
        func_type: FunctionType,
        operand: Exprent,
        bytecode_offsets: Option<&BitSet>,
    ) -> FunctionExprent {
        let id = Exprent::next_id();
        let lst_operands: JList<Exprent> = JList::new();
        lst_operands.add(operand);
        FunctionExprent::alloc_function(id, func_type, lst_operands, bytecode_offsets)
    }

    fn alloc_function(
        id: i32,
        func_type: FunctionType,
        lst_operands: JList<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> FunctionExprent {
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Function(FunctionExprentData {
                func_type,
                implicit_type: None,
                lst_operands,
                needs_cast: true,
                disable_newline_group_creation: false,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        FunctionExprent(e)
    }

    /// Java `getExprType()`; where Java returns `null` (a `++`/`--` without an
    /// implicit type) this throws `NullPointerException`, see [`Self::get_expr_type_opt`].
    pub fn get_expr_type(self) -> VarType {
        match self.get_expr_type_opt() {
            Some(t) => t,
            None => jthrow!(NullPointerException; "FunctionExprent.getExprType() is null"),
        }
    }

    /// Java `getExprType()` exactly, `null` included.
    pub fn get_expr_type_opt(self) -> Option<VarType> {
        let (func_type, implicit_type, lst_operands) =
            self.d(|d| (d.func_type, d.implicit_type.clone(), d.lst_operands.clone()));
        if let Some(static_type) = func_type.type_() {
            return Some(static_type);
        }

        use FunctionType::*;
        Some(match func_type {
            Imm | Mmi | Ipp | Ppi => return implicit_type,
            Shl | Shr | Ushr | BitNot | Neg => {
                FunctionExprent::get_max_var_type(&[lst_operands.get(0).get_expr_type_opt()])
            }
            Add | Sub | Mul | Div | Rem => {
                let type1 = lst_operands.get(0).get_expr_type_opt();
                let type2 = lst_operands.get(1).get_expr_type_opt();
                FunctionExprent::get_max_var_type(&[type1, type2])
            }
            And | Or | Xor => {
                let type1 = lst_operands.get(0).get_expr_type_opt();
                let type2 = lst_operands.get(1).get_expr_type_opt();
                let deref =
                    |t: &Option<VarType>| t.as_ref().map_or_else(|| npe("type"), |t| t.type_);
                if deref(&type1) == CodeType::Boolean && deref(&type2) == CodeType::Boolean {
                    VarType::VARTYPE_BOOLEAN()
                } else {
                    FunctionExprent::get_max_var_type(&[type1, type2])
                }
            }
            Cast => return lst_operands.get(1).get_expr_type_opt(),
            Ternary => {
                let param1 = lst_operands.get(1);
                let param2 = lst_operands.get(2);
                let type1 = param1.get_expr_type_opt();
                let type2 = param2.get_expr_type_opt();
                let (Some(type1), Some(type2)) = (type1, type2) else {
                    npe("VarType.join on null");
                };
                let Some(supertype) = VarType::join(&type1, &type2) else {
                    jthrow!(IllegalStateException; "No common supertype for ternary expression");
                };

                if param1.is_const()
                    && param2.is_const()
                    && supertype.type_ != CodeType::Boolean
                    && VarType::VARTYPE_INT().higher_equal_in_lattice_than(&supertype)
                {
                    VarType::VARTYPE_INT()
                } else {
                    supertype
                }
            }
            Other => {
                jthrow!(PluginImplementationException; "Plugin should implement custom handling")
            }
            _ => jthrow!(IllegalStateException; "No type for funcType={}", func_type.name()),
        })
    }

    /// Java `getInferredExprType(VarType)`; throws `NullPointerException` where
    /// Java returns `null`, see [`Self::get_inferred_expr_type_opt`].
    pub fn get_inferred_expr_type(self, upper_bound: Option<&VarType>) -> VarType {
        match self.get_inferred_expr_type_opt(upper_bound) {
            Some(t) => t,
            None => jthrow!(NullPointerException; "FunctionExprent.getInferredExprType() is null"),
        }
    }

    /// Java `getInferredExprType(VarType)` exactly, `null` included.
    pub fn get_inferred_expr_type_opt(self, upper_bound: Option<&VarType>) -> Option<VarType> {
        let (func_type, lst_operands) = self.d(|d| (d.func_type, d.lst_operands.clone()));
        if func_type == FunctionType::Cast {
            self.set_needs_cast(true);
            let mut right = lst_operands.get(0).get_inferred_expr_type(upper_bound);
            let cast: Vec<VarType> = lst_operands
                .sub_vec(1, lst_operands.len())
                .iter()
                .map(|e| e.get_expr_type())
                .collect();

            let generic_bound = upper_bound.filter(|ub| ub.is_generic() || right.is_generic());
            if let Some(upper_bound) = generic_bound {
                let mut upper_bound = upper_bound.clone();
                let names = self.get_named_generics();
                let mut array_dim = 0;

                if upper_bound.array_dim == right.array_dim && upper_bound.array_dim > 0 {
                    array_dim = upper_bound.array_dim;
                    upper_bound = upper_bound.resize_array_dim(0);
                    right = right.resize_array_dim(0);
                }

                let mut types = names.get(&right);
                if types.is_none() {
                    types = names.get(&upper_bound);
                }

                if let Some(final_types) = types {
                    if cast.iter().all(|cast_type| {
                        final_types
                            .iter()
                            .any(|type_| instance_of(&type_.value, &cast_type.value))
                    }) {
                        self.set_needs_cast(false);
                    }
                } else {
                    self.set_needs_cast(
                        right.type_ == CodeType::Null
                            || !instance_of(&right.value, &upper_bound.value)
                            || !FunctionExprent::are_generic_types_same(&right, &upper_bound),
                    );
                }

                if !self.does_cast() {
                    if array_dim > 0 {
                        right = right.resize_array_dim(array_dim);
                    }

                    return Some(right);
                }
            } else {
                let final_right = right.clone();
                self.set_needs_cast(
                    right.type_ == CodeType::Null
                        || cast
                            .iter()
                            .any(|cast_type| !instance_of(&final_right.value, &cast_type.value))
                        || cast
                            .iter()
                            .any(|cast_type| final_right.array_dim != cast_type.array_dim),
                );
            }

            return self.0.get_expr_type_opt();
        } else if func_type == FunctionType::Ternary {
            let type1 = lst_operands.get(1).get_inferred_expr_type(upper_bound);
            let type2 = lst_operands.get(2).get_inferred_expr_type(upper_bound);

            if type1.type_ == CodeType::Null {
                return Some(type2);
            } else if type2.type_ == CodeType::Null {
                return Some(type1);
            }

            let mut union = VarType::join(&type1, &type2);

            if let Some(u) = &union {
                if lst_operands.get(1).is_const()
                    && lst_operands.get(2).is_const()
                    && u.type_ != CodeType::Boolean
                    && VarType::VARTYPE_INT().higher_equal_in_lattice_than(u)
                {
                    union = Some(VarType::VARTYPE_INT());
                }
            }

            return match union {
                Some(u) => Some(u),
                None => self.0.get_expr_type_opt(),
            };
        } else if func_type == FunctionType::Instanceof {
            for oper in lst_operands.iter() {
                oper.get_inferred_expr_type_opt(None);
            }
            return self.0.get_expr_type_opt();
        }

        for oper in lst_operands.iter() {
            oper.get_inferred_expr_type_opt(upper_bound);
        }

        self.0.get_expr_type_opt()
    }

    fn are_generic_types_same(right: &VarType, upper_bound: &VarType) -> bool {
        if !(right.is_generic_type() && upper_bound.is_generic_type()) {
            return true;
        }

        let right_generic = right.as_generic();
        let upper_bound_generic = upper_bound.as_generic();
        let right_args = right_generic.get_arguments();
        let upper_args = upper_bound_generic.get_arguments();

        if right_args.len() != upper_args.len() {
            return false;
        }

        for i in 0..upper_args.len() {
            let upper_type = &upper_args[i];
            let right_type = &right_args[i];

            if upper_type.is_some() && right_type.is_none() {
                return false;
            }
        }

        true
    }

    pub fn get_exprent_use(self) -> i32 {
        let (func_type, lst_operands) = self.d(|d| (d.func_type, d.lst_operands.clone()));
        if func_type.ordinal() >= FunctionType::Imm.ordinal()
            && func_type.ordinal() <= FunctionType::Ppi.ordinal()
        {
            0
        } else {
            let mut ret = Exprent::MULTIPLE_USES | Exprent::SIDE_EFFECTS_FREE;
            for expr in lst_operands.iter() {
                ret &= expr.get_exprent_use();
            }
            ret
        }
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();
        let (func_type, implicit_type, lst_operands) =
            self.d(|d| (d.func_type, d.implicit_type.clone(), d.lst_operands.clone()));

        let param1 = lst_operands.get(0);
        let type1 = param1.get_expr_type_opt();
        let mut param2: Option<Exprent> = None;
        let mut type2: Option<VarType> = None;

        if lst_operands.len() > 1 {
            let p2 = lst_operands.get(1);
            param2 = Some(p2);
            type2 = p2.get_expr_type_opt();
        }

        let type1_ref = || type1.as_ref().unwrap_or_else(|| npe("type1"));
        let type2_ref = || type2.as_ref().unwrap_or_else(|| npe("type2"));
        let param2_ref = || param2.unwrap_or_else(|| npe("param2"));

        use FunctionType::*;
        match func_type {
            Ternary => {
                let supertype = self.0.get_expr_type();
                result.add_expr_lower_bound(param1, &VarType::VARTYPE_BOOLEAN());
                result.add_expr_upper_bound(param1, &VarType::VARTYPE_BOOLEAN());

                result.add_expr_lower_bound(
                    param2_ref(),
                    &VarType::find_family_bottom(supertype.type_family),
                );
                result.add_expr_lower_bound(
                    lst_operands.get(2),
                    &VarType::find_family_bottom(supertype.type_family),
                );
                result.add_expr_upper_bound(param2_ref(), &supertype);
                result.add_expr_upper_bound(lst_operands.get(2), &supertype);
            }
            I2l | I2f | I2d | I2b | I2c | I2s => {
                result.add_expr_lower_bound(param1, &VarType::VARTYPE_BYTECHAR());
                result.add_expr_upper_bound(param1, &VarType::VARTYPE_INT());
            }
            Imm | Ipp | Mmi | Ppi => {
                let implicit = implicit_type
                    .as_ref()
                    .unwrap_or_else(|| npe("Null not expected here"));
                result.add_expr_lower_bound(param1, implicit);
                result.add_expr_upper_bound(param1, implicit);
            }
            Add | Sub | Mul | Div | Rem | Shl | Shr | Ushr | Lt | Ge | Gt | Le => {
                result.add_expr_lower_bound(param2_ref(), &VarType::VARTYPE_BYTECHAR());
                result.add_expr_lower_bound(param1, &VarType::VARTYPE_BYTECHAR());
            }
            BitNot | Neg => {
                result.add_expr_lower_bound(param1, &VarType::VARTYPE_BYTECHAR());
            }
            And | Or | Xor | Eq | Ne => {
                if type1_ref().type_ == CodeType::Boolean {
                    if type2_ref().higher_in_lattice_than(type1_ref()) {
                        result.add_expr_lower_bound(param1, &VarType::VARTYPE_BYTECHAR());
                    } else {
                        let param1_false_boolean = param1
                            .try_as_const()
                            .is_some_and(|c| !c.has_boolean_value());
                        let param2_false_boolean = param2
                            .and_then(|p| p.try_as_const())
                            .is_some_and(|c| !c.has_boolean_value());

                        if param1_false_boolean || param2_false_boolean {
                            result.add_expr_lower_bound(param1, &VarType::VARTYPE_BYTECHAR());
                            result.add_expr_lower_bound(param2_ref(), &VarType::VARTYPE_BYTECHAR());
                        }
                    }
                } else if type2_ref().type_ == CodeType::Boolean {
                    if type1_ref().higher_in_lattice_than(type2_ref()) {
                        result.add_expr_lower_bound(param2_ref(), &VarType::VARTYPE_BYTECHAR());
                    }
                }
            }
            Instanceof => {
                if lst_operands.len() > 2 && lst_operands.get(2).is_var() {
                    let bound = lst_operands.get(1).get_expr_type();
                    result.add_expr_lower_bound(lst_operands.get(2), &bound);
                }
            }
            StrConcat => {
                let type_ = implicit_type
                    .clone()
                    .unwrap_or_else(VarType::VARTYPE_STRING);
                if type1_ref().type_family == type_.type_family {
                    result.add_expr_lower_bound(param1, &type_);
                }

                if type2_ref().type_family == type_.type_family {
                    result.add_expr_lower_bound(param2_ref(), &type_);
                }
            }
            Cast => {
                result.add_expr_lower_bound(
                    param1,
                    &VarType::find_family_bottom(type2_ref().type_family),
                );
                result.add_expr_upper_bound(param1, type2_ref());
            }
            Other => {
                jthrow!(PluginImplementationException; "Plugin should implement custom handling")
            }
            _ => {}
        }

        Some(result)
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| lst.extend(d.lst_operands.iter()));
    }

    pub fn copy(self) -> Exprent {
        let (func_type, implicit_type, lst_operands) =
            self.d(|d| (d.func_type, d.implicit_type.clone(), d.lst_operands.clone()));
        let mut lst = Vec::new();
        for expr in lst_operands.iter() {
            lst.push(expr.copy());
        }
        let bytecode = self.get_bytecode();
        let func = FunctionExprent::new(func_type, lst, bytecode.as_ref());
        func.set_implicit_type(implicit_type);
        func.0
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(fe) = o.try_as_function() else {
            return false;
        };
        let (func_type, lst_operands) = self.d(|d| (d.func_type, d.lst_operands.to_vec()));
        func_type == fe.get_func_type() && lst_operands.java_equals(&fe.get_lst_operands().to_vec())
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        let lst_operands = self.get_lst_operands();
        for i in 0..lst_operands.len() {
            if old_expr == lst_operands.get(i) {
                lst_operands.set(i, new_expr);
            }
        }
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let func_type = self.get_func_type();
        if func_type == FunctionType::Other {
            jthrow!(PluginImplementationException; "Plugin should implement custom handling");
        }

        let lst_operands = self.get_lst_operands();
        let operator = func_type.operator().unwrap_or("null");
        let mut buf = TextBuffer::new();
        buf.add_bytecode_mapping(self.get_bytecode().as_ref());

        if func_type.is_arithmetic_binary_operation() {
            let left = lst_operands.get(0);
            let right = lst_operands.get(1);

            if func_type.is_arithmetic_binary_operation() {
                if let Some(right_const) = right.try_as_const() {
                    right_const.adjust_const_type(&left.get_expr_type());
                } else if let Some(left_const) = left.try_as_const() {
                    left_const.adjust_const_type(&right.get_expr_type());
                }
            }

            let mut left_operand = self.wrap_operand_string_group(left, false, indent, true);
            let mut right_operand = self.wrap_operand_string_group(right, true, indent, true);

            if func_type == FunctionType::And || func_type == FunctionType::Or {
                if let Some(right_const) = right.try_as_const() {
                    if right.get_expr_type().ptr_eq(&VarType::VARTYPE_INT()) {
                        let value = const_int_value(right_const);
                        right_operand.set_length(0);
                        right_operand.append(&IntHelper::adjusted_int_representation(value));
                    }
                }

                if let Some(left_const) = left.try_as_const() {
                    if left.get_expr_type().ptr_eq(&VarType::VARTYPE_INT()) {
                        let value = const_int_value(left_const);
                        left_operand.set_length(0);
                        left_operand.append(&IntHelper::adjusted_int_representation(value));
                    }
                }
            }

            let disable = self.d(|d| d.disable_newline_group_creation);
            if !disable {
                buf.push_newline_group(indent, 1);
            }
            buf.append_buffer(&left_operand)
                .append_possible_newline_alt(" ")
                .append(operator)
                .append(" ")
                .append_buffer(&right_operand);
            if !disable {
                buf.pop_newline_group();
            }

            return buf;
        }

        if func_type.ordinal() >= FunctionType::Eq.ordinal() {
            let left = lst_operands.get(0);
            let right = lst_operands.get(1);

            if func_type.ordinal() <= FunctionType::Le.ordinal() {
                if let Some(right_const) = right.try_as_const() {
                    if let Some(other) = left.get_expr_type_opt() {
                        right_const.adjust_const_type(&other);
                    }
                } else if let Some(left_const) = left.try_as_const() {
                    if let Some(other) = right.get_expr_type_opt() {
                        left_const.adjust_const_type(&other);
                    }
                }
            }

            let disable = self.d(|d| d.disable_newline_group_creation);
            if !disable {
                buf.push_newline_group(indent, 1);
            }
            let left_operand = self.wrap_operand_string_group(left, false, indent, true);
            buf.append_buffer(&left_operand)
                .append_possible_newline_alt(" ")
                .append(operator)
                .append(" ");
            let right_operand = self.wrap_operand_string_group(right, true, indent, true);
            buf.append_buffer(&right_operand);
            if !disable {
                buf.pop_newline_group();
            }
            return buf;
        }

        use FunctionType::*;
        match func_type {
            BitNot | BoolNot | Neg | Mmi | Ppi => {
                let mut operand = self.wrap_operand_string(lst_operands.get(0), true, indent);
                operand.prepend(operator);
                buf.append_buffer(&operand);
                return buf;
            }
            Ipp | Imm => {
                let mut operand = self.wrap_operand_string(lst_operands.get(0), true, indent);
                operand.append(operator);
                buf.append_buffer(&operand);
                return buf;
            }
            Cast => {
                if !self.does_cast() {
                    buf.append_buffer(&lst_operands.get(0).to_java(indent));
                    return buf;
                }
                for i in 1..lst_operands.len() {
                    if i > 1 {
                        buf.append(" & ");
                    }
                    buf.append_buffer(&lst_operands.get(i).to_java(indent));
                }
                buf.enclose_with_parens();
                let operand = self.wrap_operand_string(lst_operands.get(0), true, indent);
                buf.append_buffer(&operand);
                return buf;
            }
            ArrayLength => {
                let arr = lst_operands.get(0);

                buf.append_buffer(&self.wrap_operand_string(arr, false, indent));
                if arr.get_expr_type().array_dim == 0 {
                    let obj_arr = VarType::VARTYPE_OBJECT().resize_array_dim(1);
                    buf.enclose(
                        &format!("(({})", ExprProcessor::get_cast_type_name(&obj_arr)),
                        ")",
                    );
                    buf.add_type_name_token(&obj_arr, 2);
                }
                buf.append(".length");
                return buf;
            }
            Ternary => {
                buf.push_newline_group(indent, 1);
                let first = self.wrap_operand_string(lst_operands.get(0), true, indent);
                buf.append_buffer(&first)
                    .append_possible_newline_alt(" ")
                    .append("? ");
                let second = self.wrap_operand_string(lst_operands.get(1), true, indent);
                buf.append_buffer(&second)
                    .append_possible_newline_alt(" ")
                    .append(": ");
                let third = self.wrap_operand_string(lst_operands.get(2), true, indent);
                buf.append_buffer(&third);
                buf.pop_newline_group();
                return buf;
            }
            Instanceof => {
                let value = self.wrap_operand_string(lst_operands.get(0), true, indent);
                buf.append_buffer(&value).append(" instanceof ");

                if lst_operands.len() > 2 {
                    let pattern = Pattern::cast(lst_operands.get(2));
                    for var in pattern.get_pattern_vars() {
                        var.set_writing_pattern();
                    }

                    let tested = self.wrap_operand_string(lst_operands.get(2), true, indent);
                    buf.append_buffer(&tested);
                } else {
                    let tested = self.wrap_operand_string(lst_operands.get(1), true, indent);
                    buf.append_buffer(&tested);
                }

                return buf;
            }
            Lcmp | Fcmpl | Fcmpg | Dcmpl | Dcmpg => {
                let mut first = self.wrap_operand_string(lst_operands.get(0), true, indent);
                first.prepend(&format!("{operator}("));
                buf.append_buffer(&first).append(", ");
                let second = self.wrap_operand_string(lst_operands.get(1), true, indent);
                buf.append_buffer(&second).append(")");
                return buf;
            }
            _ => {}
        }

        if let Some(cast_type) = func_type.cast_type() {
            if let Some(inv) = lst_operands.get(0).try_as_invocation() {
                if inv.is_unboxing_call() {
                    inv.force_unboxing(true);
                }
            }

            if !self.does_cast() {
                buf.append_buffer(&lst_operands.get(0).to_java(indent));
                return buf;
            }

            buf.append(&ExprProcessor::get_type_name(&cast_type))
                .enclose_with_parens();
            let operand = self.wrap_operand_string(lst_operands.get(0), true, indent);
            buf.append_buffer(&operand);
            return buf;
        }

        jthrow!("invalid function")
    }

    fn unwrap_boxing(self, expr: Exprent) -> Exprent {
        let mut expr = expr;
        if let Some(inv) = expr.try_as_invocation() {
            if inv.is_unboxing_call() {
                let inner = inv.get_instance();
                if let Some(inner) = inner {
                    if inner
                        .try_as_function()
                        .is_some_and(|f| f.get_func_type() == FunctionType::Cast)
                    {
                        let bytecode = expr.get_bytecode();
                        inner.add_bytecode_offsets(bytecode.as_ref());
                        expr = inner;
                    }
                }
            }
        }
        expr
    }

    pub fn unwrap_box(self) {
        let lst_operands = self.get_lst_operands();
        for i in 0..lst_operands.len() {
            let unwrapped = self.unwrap_boxing(lst_operands.get(i));
            lst_operands.set(i, unwrapped);
        }
    }

    pub fn get_precedence(self) -> i32 {
        let func_type = self.get_func_type();
        if (func_type == FunctionType::Cast || func_type.cast_type().is_some()) && !self.does_cast()
        {
            return self.get_lst_operands().get(0).get_precedence();
        }

        if func_type == FunctionType::Other {
            jthrow!(PluginImplementationException; "Plugin should implement custom handling");
        }

        func_type.precedence()
    }

    pub fn get_simple_cast_type(self) -> Option<VarType> {
        self.get_func_type().cast_type()
    }

    /// Java: `wrapOperandString(Exprent, boolean, int)`.
    pub fn wrap_operand_string(self, expr: Exprent, eq: bool, indent: i32) -> TextBuffer {
        self.wrap_operand_string_group(expr, eq, indent, false)
    }

    /// Java: `wrapOperandString(Exprent, boolean, int, boolean)`.
    fn wrap_operand_string_group(
        self,
        expr: Exprent,
        eq: bool,
        indent: i32,
        newline_group: bool,
    ) -> TextBuffer {
        let func_type = self.get_func_type();
        let myprec = self.0.get_precedence();
        let exprprec = expr.get_precedence();

        let mut parentheses = exprprec > myprec;
        if !parentheses && eq {
            parentheses = exprprec == myprec;
            if parentheses {
                if expr
                    .try_as_function()
                    .is_some_and(|f| f.get_func_type() == func_type)
                {
                    if !same_type(&expr.get_expr_type_opt(), &VarType::VARTYPE_FLOAT())
                        && !same_type(&expr.get_expr_type_opt(), &VarType::VARTYPE_DOUBLE())
                    {
                        parentheses = !FunctionExprent::is_associative(func_type);
                    }
                }
            }
        }

        if newline_group && !parentheses && myprec == exprprec {
            if let Some(mut func_expr) = expr.try_as_function() {
                if func_expr.get_func_type() == FunctionType::Cast && !func_expr.does_cast() {
                    let sub_expr = func_expr.get_lst_operands().get(0);
                    if let Some(sub_func) = sub_expr.try_as_function() {
                        func_expr = sub_func;
                    }
                }
                func_expr.d_mut(|d| d.disable_newline_group_creation = true);
            }
        }

        let mut res = expr.to_java(indent);

        if parentheses {
            let old_res = res;
            res = TextBuffer::new();
            res.append("(");
            res.push_newline_group(indent, 1);
            res.append_possible_newline();
            res.append_buffer(&old_res);
            res.append_possible_newline_dedent("", true);
            res.pop_newline_group();
            res.append(")");
        }

        res
    }

    fn is_associative(func_type: FunctionType) -> bool {
        ASSOCIATIVITY.contains(&func_type)
    }

    /// A `null` element throws only when the scan reaches it, as in Java.
    fn get_max_var_type(arr: &[Option<VarType>]) -> VarType {
        let type_primitives = [CodeType::Double, CodeType::Float, CodeType::Long];
        let types = [
            VarType::VARTYPE_DOUBLE(),
            VarType::VARTYPE_FLOAT(),
            VarType::VARTYPE_LONG(),
        ];
        for i in 0..type_primitives.len() {
            for an_arr in arr {
                let an_arr = an_arr
                    .as_ref()
                    .unwrap_or_else(|| npe("getMaxVarType on null"));
                if an_arr.type_ == type_primitives[i] {
                    return types[i].clone();
                }
            }
        }
        VarType::VARTYPE_INT()
    }

    pub fn get_func_type(self) -> FunctionType {
        self.d(|d| d.func_type)
    }

    pub fn set_func_type(self, func_type: FunctionType) {
        if self.0.is_kotlin() {
            return crate::k_dispatch::function_set_func_type(self, func_type);
        }
        FunctionExprent::set_func_type_base(self, func_type)
    }

    pub fn set_func_type_base(self, func_type: FunctionType) {
        self.d_mut(|d| d.func_type = func_type)
    }

    /// The operand list itself (aliased, like the Java reference).
    pub fn get_lst_operands(self) -> JList<Exprent> {
        self.d(|d| d.lst_operands.clone())
    }

    pub fn set_implicit_type(self, implicit_type: Option<VarType>) {
        self.d_mut(|d| d.implicit_type = implicit_type)
    }

    pub fn does_cast(self) -> bool {
        self.d(|d| d.needs_cast)
    }

    pub fn set_needs_cast(self, needs_cast: bool) {
        self.d_mut(|d| d.needs_cast = needs_cast)
    }

    pub fn set_invocation_instance(self) {
        if self.get_func_type() == FunctionType::Cast {
            self.get_lst_operands().get(0).set_invocation_instance();
        }
    }

    pub fn set_is_qualifier(self) {
        if self.get_func_type() == FunctionType::Cast && !self.does_cast() {
            self.get_lst_operands().get(0).set_is_qualifier();
        }
    }

    pub fn allow_newline_after_qualifier(self) -> bool {
        if self.get_func_type() == FunctionType::Cast && !self.does_cast() {
            return self
                .get_lst_operands()
                .get(0)
                .allow_newline_after_qualifier();
        }
        Exprent::allow_newline_after_qualifier_base(self.0)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let lst_operands = self.get_lst_operands().to_vec();
        Exprent::measure_bytecode_list(values.as_deref_mut(), &lst_operands);
        self.measure_bytecode(values);
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        let lst_operands = self.get_lst_operands();
        match self.get_func_type() {
            FunctionType::Ternary => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );

                let mut b_var_maps = VarMapHolder::of_normal(&var_maps.get_if_true());
                lst_operands.get(1).process_sforms(
                    s_forms_constructor,
                    &mut b_var_maps,
                    stat,
                    calc_live_vars,
                );

                let if_false = var_maps.get_if_false();
                var_maps.set_normal(if_false);
                lst_operands.get(2).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );

                if b_var_maps.is_normal() && var_maps.is_normal() {
                    let b_normal = b_var_maps.get_normal();
                    var_maps.merge_normal(Some(&b_normal));
                } else if !var_maps.is_normal() {
                    let b_if_true = b_var_maps.get_if_true();
                    var_maps.merge_if_true(Some(&b_if_true));
                    let b_if_false = b_var_maps.get_if_false();
                    var_maps.merge_if_false(Some(&b_if_false));
                } else {
                    let normal = var_maps.get_normal();
                    b_var_maps.merge_if_true(Some(&normal));
                    let normal = var_maps.get_normal();
                    b_var_maps.merge_if_false(Some(&normal));

                    var_maps.set(&b_var_maps);
                }
            }
            FunctionType::BooleanAnd => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );

                var_maps.make_fully_mutable();
                let if_false = var_maps.get_if_false();
                let if_true = var_maps.get_if_true();
                var_maps.set_normal(if_true);

                lst_operands.get(1).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.merge_if_false(Some(&if_false));
            }
            FunctionType::BooleanOr => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );

                var_maps.make_fully_mutable();
                let if_true = var_maps.get_if_true();
                let if_false = var_maps.get_if_false();
                var_maps.set_normal(if_false);

                lst_operands.get(1).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.merge_if_true(Some(&if_true));
            }
            FunctionType::BoolNot => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.swap();
            }
            FunctionType::Instanceof => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.to_normal();

                if lst_operands.len() == 3 {
                    var_maps.make_fully_mutable();

                    let pattern = Pattern::cast(lst_operands.get(2));

                    for var in pattern.get_pattern_vars() {
                        let if_true = var_maps.get_if_true();
                        s_forms_constructor.update_var_exprent(var, stat, &if_true, calc_live_vars);
                    }
                }
            }
            FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi => {
                lst_operands.get(0).process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );

                let operand = lst_operands.get(0);
                match operand.get_type() {
                    ExprentType::Var => {
                        let var_exprent = operand.as_var();

                        let phantom_pair = s_forms_constructor
                            .get_or_create_phantom(var_exprent.get_var_version_pair());

                        var_maps
                            .get_normal()
                            .borrow_mut()
                            .set_current_var_pair(phantom_pair);
                    }
                    ExprentType::Field => {
                        var_maps.get_normal().borrow_mut().remove_all_fields();
                    }
                    _ => {}
                }
            }
            _ => {
                Exprent::process_sforms_base(
                    self.0,
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
            }
        }
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        let type_ = match match_node.get_rule_value(MatchProperties::ExprentFunctype) {
            None => None,
            Some(MatchValue::FunctionType(t)) => Some(t),
            Some(_) => jthrow!(ClassCastException; "match value is not a FunctionType"),
        };
        type_.is_none() || Some(self.get_func_type()) == type_
    }
}
