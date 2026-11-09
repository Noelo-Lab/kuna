//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ExprProcessor.java`.
#![allow(unused_variables, dead_code)]

use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::{OPC_ARRAYLENGTH, OPC_CHECKCAST, OPC_INSTANCEOF};
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::FunctionType;
use crate::if_exprent::Type as IfType;
use crate::list_stack::ListStack;
use crate::method_descriptor::MethodDescriptor;
use crate::monitor_exprent::Type as MonitorType;
use crate::new_exprent::NewExprent;
use crate::primitive_exprs_list::PrimitiveExprsList;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JList, JavaEq, JavaHash};
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::unported;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;

thread_local! {
    static MAP_CONSTS: JHashMap<i32, FunctionType> = {
        let mut map = JHashMap::new();
        map.put(OPC_ARRAYLENGTH, FunctionType::ArrayLength);
        map.put(OPC_CHECKCAST, FunctionType::Cast);
        map.put(OPC_INSTANCEOF, FunctionType::Instanceof);
        map
    };

    static CONSTS: [VarType; 14] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_CLASS(),
        VarType::VARTYPE_STRING(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_VOID(),
        VarType::VARTYPE_METHODTYPE(),
    ];

    static VAR_TYPES: [VarType; 5] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_OBJECT(),
    ];

    static ARR_TYPES: [VarType; 8] = [
        VarType::VARTYPE_INT(),
        VarType::VARTYPE_LONG(),
        VarType::VARTYPE_FLOAT(),
        VarType::VARTYPE_DOUBLE(),
        VarType::VARTYPE_OBJECT(),
        VarType::VARTYPE_BOOLEAN(),
        VarType::VARTYPE_CHAR(),
        VarType::VARTYPE_SHORT(),
    ];
}

const FUNC1: [FunctionType; 5] = [
    FunctionType::Add,
    FunctionType::Sub,
    FunctionType::Mul,
    FunctionType::Div,
    FunctionType::Rem,
];

const FUNC2: [FunctionType; 6] = [
    FunctionType::Shl,
    FunctionType::Shr,
    FunctionType::Ushr,
    FunctionType::And,
    FunctionType::Or,
    FunctionType::Xor,
];

const FUNC3: [FunctionType; 15] = [
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
];

const FUNC4: [FunctionType; 5] = [
    FunctionType::Lcmp,
    FunctionType::Fcmpl,
    FunctionType::Fcmpg,
    FunctionType::Dcmpl,
    FunctionType::Dcmpg,
];

const FUNC5: [IfType; 6] = [
    IfType::Eq,
    IfType::Ne,
    IfType::Lt,
    IfType::Ge,
    IfType::Gt,
    IfType::Le,
];

const FUNC6: [IfType; 8] = [
    IfType::Icmpeq,
    IfType::Icmpne,
    IfType::Icmplt,
    IfType::Icmpge,
    IfType::Icmpgt,
    IfType::Icmple,
    IfType::Acmpeq,
    IfType::Acmpne,
];

const FUNC7: [IfType; 2] = [IfType::Null, IfType::Nonnull];

const FUNC8: [MonitorType; 2] = [MonitorType::Enter, MonitorType::Exit];

const ARR_TYPE_IDS: [CodeType; 8] = [
    CodeType::Boolean,
    CodeType::Char,
    CodeType::Float,
    CodeType::Double,
    CodeType::Byte,
    CodeType::Short,
    CodeType::Int,
    CodeType::Long,
];

const TYPE_NAMES: [&str; 8] = [
    "byte", "char", "double", "float", "int", "long", "short", "boolean",
];

/// The stack-to-expression lift of one method body.
///
/// A plain owned value: built per method by `MethodProcessor` and never stored.
pub struct ExprProcessor {
    method_descriptor: MethodDescriptor,
    var_processor: VarProcessor,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum NullCastType {
    Cast,
    DontCast,
    DontCastAtAll,
}

impl NullCastType {
    pub fn cast(self) -> bool {
        match self {
            NullCastType::Cast => true,
            NullCastType::DontCast => false,
            NullCastType::DontCastAtAll => false,
        }
    }
}

impl JavaHash for NullCastType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for NullCastType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl ExprProcessor {
    pub const UNDEFINED_TYPE_STRING: &'static str = "<undefinedtype>";
    pub const UNREPRESENTABLE_TYPE_STRING: &'static str = "<unrepresentable>";
    pub const UNKNOWN_TYPE_STRING: &'static str = "<unknown>";
    pub const NULL_TYPE_STRING: &'static str = "<null>";

    pub fn new(md: &MethodDescriptor, var_proc: VarProcessor) -> ExprProcessor {
        ExprProcessor {
            method_descriptor: md.clone(),
            var_processor: var_proc,
        }
    }

    pub fn process_statement(&self, root: RootStatement, cl: StructClass) {
        unported!("ExprProcessor.processStatement")
    }

    fn copy_var_exprents(mut data: PrimitiveExprsList) -> PrimitiveExprsList {
        let stack = data.get_stack();
        Self::copy_entries(stack);
        data
    }

    pub fn copy_entries(stack: &mut [Exprent]) {
        for i in 0..stack.len() {
            stack[i] = stack[i].copy();
        }
    }

    fn collect_catch_vars(
        stat: Statement,
        flatthelper: &FlattenStatementsHelper,
        map: &mut JHashMap<String, VarExprent>,
    ) {
        unported!("ExprProcessor.collectCatchVars")
    }

    fn init_statement_exprents(stat: Statement) {
        stat.init_exprents();

        let sub_stats = stat.get_stats().borrow().to_vec();

        for st in sub_stats {
            Self::init_statement_exprents(st);
        }
    }

    pub fn process_block(
        &self,
        stat: BasicBlockStatement,
        data: &mut PrimitiveExprsList,
        cl: StructClass,
    ) {
        unported!("ExprProcessor.processBlock")
    }

    fn push_ex(&self, stack: &mut ListStack<Exprent>, exprlist: &JList<Exprent>, exprent: Exprent) {
        self.push_ex_type(stack, exprlist, exprent, None);
    }

    /// Java: pushEx(ListStack, List, Exprent, VarType)
    fn push_ex_type(
        &self,
        stack: &mut ListStack<Exprent>,
        exprlist: &JList<Exprent>,
        exprent: Exprent,
        vartype: Option<&VarType>,
    ) {
        unported!("ExprProcessor.pushEx")
    }

    fn insert_by_offset_ex(
        &self,
        offset: i32,
        stack: &mut ListStack<Exprent>,
        exprlist: &JList<Exprent>,
        copyoffset: i32,
    ) {
        unported!("ExprProcessor.insertByOffsetEx")
    }

    pub fn canonicalize_casts(stat: RootStatement) -> bool {
        let mut res = false;
        while Self::canonicalize_casts_stat(stat.0) {
            res = true;
        }

        res
    }

    /// Java: canonicalizeCasts(Statement)
    fn canonicalize_casts_stat(stat: Statement) -> bool {
        unported!("ExprProcessor.canonicalizeCasts")
    }

    pub fn mark_expr_oddities(root: RootStatement) {
        unported!("ExprProcessor.markExprOddities")
    }

    /// Java: markExprOddities(RootStatement, Statement)
    fn mark_expr_oddities_stat(root: RootStatement, stat: Statement) {
        unported!("ExprProcessor.markExprOddities")
    }

    fn mark_expr_oddity(root: RootStatement, ex: Exprent, block: bool) {
        unported!("ExprProcessor.markExprOddity")
    }

    pub fn get_type_name(type_: &VarType) -> String {
        Self::get_type_name_short(type_, true)
    }

    /// Java: getTypeName(VarType, boolean)
    pub fn get_type_name_short(type_: &VarType, get_short: bool) -> String {
        unported!("ExprProcessor.getTypeName")
    }

    pub fn is_invalid_type_name(name: &str) -> bool {
        Self::UNDEFINED_TYPE_STRING == name
            || Self::NULL_TYPE_STRING == name
            || Self::UNKNOWN_TYPE_STRING == name
    }

    pub fn get_cast_type_name(type_: &VarType) -> String {
        Self::get_cast_type_name_short(type_, true)
    }

    /// Java: getCastTypeName(VarType, boolean)
    pub fn get_cast_type_name_short(type_: &VarType, get_short: bool) -> String {
        let mut s = Self::get_type_name_short(type_, get_short);
        TextUtil::append(&mut s, "[]", type_.array_dim);
        s
    }

    pub fn get_expression_data(var: VarExprent) -> PrimitiveExprsList {
        unported!("ExprProcessor.getExpressionData")
    }

    pub fn ends_with_semicolon(expr: Exprent) -> bool {
        !(expr.is_switch_head()
            || expr.is_monitor()
            || expr.is_if()
            || expr.try_as_var().is_some_and(|v| v.is_class_def()))
    }

    pub fn add_deleted_goto_instruction_mapping(stat: Statement, buffer: &mut TextBuffer) {
        unported!("ExprProcessor.addDeletedGotoInstructionMapping")
    }

    pub fn jmp_wrapper(stat: Statement, indent: i32, semicolon: bool) -> TextBuffer {
        unported!("ExprProcessor.jmpWrapper")
    }

    pub fn build_java_class_name(name: &str) -> String {
        unported!("ExprProcessor.buildJavaClassName")
    }

    pub fn list_to_java(lst: Option<&JList<Exprent>>, indent: i32) -> TextBuffer {
        unported!("ExprProcessor.listToJava")
    }

    pub fn get_default_array_value(arr_type: &VarType) -> ConstExprent {
        unported!("ExprProcessor.getDefaultArrayValue")
    }

    pub fn get_casted_exprent(
        exprent: Exprent,
        left_type: &VarType,
        buffer: &mut TextBuffer,
        indent: i32,
        cast_null: bool,
    ) -> bool {
        Self::get_casted_exprent_full(
            exprent,
            left_type,
            buffer,
            indent,
            if cast_null {
                NullCastType::Cast
            } else {
                NullCastType::DontCast
            },
            false,
            false,
            false,
        )
    }

    /// Java: getCastedExprent(Exprent, VarType, TextBuffer, int, NullCastType, boolean, boolean, boolean)
    #[allow(clippy::too_many_arguments)]
    pub fn get_casted_exprent_full(
        exprent: Exprent,
        left_type: &VarType,
        buffer: &mut TextBuffer,
        indent: i32,
        cast_null: NullCastType,
        cast_always: bool,
        cast_narrowing: bool,
        unbox: bool,
    ) -> bool {
        unported!("ExprProcessor.getCastedExprent")
    }

    pub fn narrow_generic_cast_type(expr: Exprent, type_: &VarType) -> Exprent {
        unported!("ExprProcessor.narrowGenericCastType")
    }

    fn do_generic_types_cast(ex: Exprent, left: Option<&VarType>, right: Option<&VarType>) -> bool {
        unported!("ExprProcessor.doGenericTypesCast")
    }

    fn should_generic_types_cast(
        named: &JHashMap<VarType, Vec<VarType>>,
        left_type: &VarType,
        right_type: &VarType,
    ) -> bool {
        unported!("ExprProcessor.shouldGenericTypesCast")
    }

    fn is_int_constant(exprent: Exprent) -> bool {
        if let Some(c) = exprent.try_as_const() {
            return matches!(
                c.get_const_type().type_,
                CodeType::Byte
                    | CodeType::Bytechar
                    | CodeType::Short
                    | CodeType::Shortchar
                    | CodeType::Int
            );
        }

        false
    }

    fn is_narrowed_int_type(type_: &VarType) -> bool {
        VarType::VARTYPE_INT().higher_in_lattice_than(type_)
            || type_.equals(&VarType::VARTYPE_BYTE_OBJ())
            || type_.equals(&VarType::VARTYPE_SHORT_OBJ())
    }

    fn lambda_needs_cast(left: &VarType, exprent: NewExprent) -> bool {
        unported!("ExprProcessor.lambdaNeedsCast")
    }
}
