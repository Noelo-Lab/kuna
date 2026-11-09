//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/FunctionExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::exprent::{Exprent, ExprentKind};
use crate::list_stack::ListStack;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::{BitSet, JList, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::typed::Typed;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, jthrow, unported};

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

    pub fn get_expr_type(self) -> VarType {
        let (func_type, implicit_type, lst_operands) =
            self.d(|d| (d.func_type, d.implicit_type.clone(), d.lst_operands.clone()));
        if let Some(static_type) = func_type.type_() {
            return static_type;
        }

        use FunctionType::*;
        match func_type {
            Imm | Mmi | Ipp | Ppi => implicit_type.expect("NullPointerException: implicitType"),
            Shl | Shr | Ushr | BitNot | Neg => {
                FunctionExprent::get_max_var_type(&[lst_operands.get(0).get_expr_type()])
            }
            Add | Sub | Mul | Div | Rem => FunctionExprent::get_max_var_type(&[
                lst_operands.get(0).get_expr_type(),
                lst_operands.get(1).get_expr_type(),
            ]),
            And | Or | Xor => {
                let type1 = lst_operands.get(0).get_expr_type();
                let type2 = lst_operands.get(1).get_expr_type();
                if type1.type_ == CodeType::Boolean && type2.type_ == CodeType::Boolean {
                    VarType::VARTYPE_BOOLEAN()
                } else {
                    FunctionExprent::get_max_var_type(&[type1, type2])
                }
            }
            Cast => lst_operands.get(1).get_expr_type(),
            Ternary => {
                let param1 = lst_operands.get(1);
                let param2 = lst_operands.get(2);
                let Some(supertype) =
                    VarType::join(&param1.get_expr_type(), &param2.get_expr_type())
                else {
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
            Other => jthrow!(PluginImplementationException; ""),
            _ => jthrow!(IllegalStateException; "No type for funcType={}", func_type.name()),
        }
    }

    pub fn get_inferred_expr_type(self, _upper_bound: Option<&VarType>) -> VarType {
        unported!("FunctionExprent.getInferredExprType")
    }

    #[allow(dead_code)]
    fn are_generic_types_same(_right: &VarType, _upper_bound: &VarType) -> bool {
        unported!("FunctionExprent.areGenericTypesSame")
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
        unported!("FunctionExprent.checkExprTypeBounds")
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

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("FunctionExprent.toJava")
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
            jthrow!(PluginImplementationException; "");
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
        _expr: Exprent,
        _eq: bool,
        _indent: i32,
        _newline_group: bool,
    ) -> TextBuffer {
        unported!("FunctionExprent.wrapOperandString")
    }

    #[allow(dead_code)]
    fn is_associative(func_type: FunctionType) -> bool {
        ASSOCIATIVITY.contains(&func_type)
    }

    fn get_max_var_type(arr: &[VarType]) -> VarType {
        let type_primitives = [CodeType::Double, CodeType::Float, CodeType::Long];
        let types = [
            VarType::VARTYPE_DOUBLE(),
            VarType::VARTYPE_FLOAT(),
            VarType::VARTYPE_LONG(),
        ];
        for i in 0..type_primitives.len() {
            for an_arr in arr {
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
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("FunctionExprent.processSforms")
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, _match_node: &MatchNode, _engine: &MatchEngine) -> bool {
        unported!("FunctionExprent.match")
    }
}
