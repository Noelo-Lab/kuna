//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/Exprent.java`.
//!
//! `Exprent` is the arena handle; each Java subclass is a `java_subclass!` view
//! over one `ExprentKind` variant. Every virtual Java method is a dispatcher
//! here that calls the overriding view in path form and falls back to the
//! base body (`*_base`).

use std::cell::RefCell;

use crate::annotation_exprent::{AnnotationExprent, AnnotationExprentData};
use crate::arena_object;
use crate::array_exprent::{ArrayExprent, ArrayExprentData};
use crate::assert_exprent::{AssertExprent, AssertExprentData};
use crate::assignment_exprent::{AssignmentExprent, AssignmentExprentData};
use crate::check_types_result::CheckTypesResult;
use crate::classes_processor::ClassNode;
use crate::const_exprent::{ConstExprent, ConstExprentData};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::{ExitExprent, ExitExprentData};
use crate::field_exprent::{FieldExprent, FieldExprentData};
use crate::function_exprent::{FunctionExprent, FunctionExprentData};
use crate::generic_type::GenericType;
use crate::i_matchable::{IMatchable, MatchProperties};
use crate::if_exprent::{IfExprent, IfExprentData};
use crate::invocation_exprent::{InvocationExprent, InvocationExprentData};
use crate::k_dispatch::{self, KExprentExt};
use crate::kuna_fix_static_type_variable_scope;
use crate::kuna_fixes::{self, KunaFix};
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::method_wrapper::MethodWrapper;
use crate::monitor_exprent::{MonitorExprent, MonitorExprentData};
use crate::new_exprent::{NewExprent, NewExprentData};
use crate::pattern_exprent::{PatternExprent, PatternExprentData};
use crate::rt::{jnum, BitSet, JHashMap, JHashSet, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::switch_exprent::{SwitchExprent, SwitchExprentData};
use crate::switch_head_exprent::{SwitchHeadExprent, SwitchHeadExprentData};
use crate::text_buffer::TextBuffer;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::{VarExprent, VarExprentData};
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::yield_exprent::{YieldExprent, YieldExprentData};

/// `s.matches("-?\\d+")` (`\d` is ASCII-only in Java regex).
fn is_signed_decimal(s: &str) -> bool {
    let digits = s.strip_prefix('-').unwrap_or(s);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

pub struct ExprentData {
    pub id: i32,
    pub bytecode: Option<BitSet>,
    pub kind: ExprentKind,
    /// Present exactly when the object is an instance of a Kotlin class (see `k_dispatch`).
    pub kt: Option<Box<KExprentExt>>,
}

pub enum ExprentKind {
    Annotation(AnnotationExprentData),
    Array(ArrayExprentData),
    Assert(AssertExprentData),
    Assignment(AssignmentExprentData),
    Const(ConstExprentData),
    Exit(ExitExprentData),
    Field(FieldExprentData),
    Function(FunctionExprentData),
    If(IfExprentData),
    Invocation(Box<InvocationExprentData>),
    Monitor(MonitorExprentData),
    New(NewExprentData),
    Pattern(PatternExprentData),
    Switch(SwitchExprentData),
    SwitchHead(SwitchHeadExprentData),
    Var(VarExprentData),
    Yield(YieldExprentData),
}

impl ExprentKind {
    pub fn type_(&self) -> ExprentType {
        match self {
            ExprentKind::Annotation(_) => ExprentType::Annotation,
            ExprentKind::Array(_) => ExprentType::Array,
            ExprentKind::Assert(_) => ExprentType::Assert,
            ExprentKind::Assignment(_) => ExprentType::Assignment,
            ExprentKind::Const(_) => ExprentType::Const,
            ExprentKind::Exit(_) => ExprentType::Exit,
            ExprentKind::Field(_) => ExprentType::Field,
            ExprentKind::Function(_) => ExprentType::Function,
            ExprentKind::If(_) => ExprentType::If,
            ExprentKind::Invocation(_) => ExprentType::Invocation,
            ExprentKind::Monitor(_) => ExprentType::Monitor,
            ExprentKind::New(_) => ExprentType::New,
            ExprentKind::Pattern(_) => ExprentType::Pattern,
            ExprentKind::Switch(_) => ExprentType::Switch,
            ExprentKind::SwitchHead(_) => ExprentType::SwitchHead,
            ExprentKind::Var(_) => ExprentType::Var,
            ExprentKind::Yield(_) => ExprentType::Yield,
        }
    }
}

arena_object!(pub Exprent => ExprentData, Class);

/// Java `Exprent.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum ExprentType {
    Annotation,
    Array,
    Assert,
    Assignment,
    Const,
    Exit,
    Field,
    Function,
    If,
    Invocation,
    Monitor,
    New,
    Pattern,
    Switch,
    SwitchHead,
    Var,
    Yield,
    Other,
}

impl ExprentType {
    pub const VALUES: [ExprentType; 18] = [
        ExprentType::Annotation,
        ExprentType::Array,
        ExprentType::Assert,
        ExprentType::Assignment,
        ExprentType::Const,
        ExprentType::Exit,
        ExprentType::Field,
        ExprentType::Function,
        ExprentType::If,
        ExprentType::Invocation,
        ExprentType::Monitor,
        ExprentType::New,
        ExprentType::Pattern,
        ExprentType::Switch,
        ExprentType::SwitchHead,
        ExprentType::Var,
        ExprentType::Yield,
        ExprentType::Other,
    ];

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            ExprentType::Annotation => "ANNOTATION",
            ExprentType::Array => "ARRAY",
            ExprentType::Assert => "ASSERT",
            ExprentType::Assignment => "ASSIGNMENT",
            ExprentType::Const => "CONST",
            ExprentType::Exit => "EXIT",
            ExprentType::Field => "FIELD",
            ExprentType::Function => "FUNCTION",
            ExprentType::If => "IF",
            ExprentType::Invocation => "INVOCATION",
            ExprentType::Monitor => "MONITOR",
            ExprentType::New => "NEW",
            ExprentType::Pattern => "PATTERN",
            ExprentType::Switch => "SWITCH",
            ExprentType::SwitchHead => "SWITCH_HEAD",
            ExprentType::Var => "VAR",
            ExprentType::Yield => "YIELD",
            ExprentType::Other => "OTHER",
        }
    }
}

impl JavaHash for ExprentType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for ExprentType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

thread_local! {
    static INFERRED_LAMBDA_TYPES: RefCell<JHashMap<String, VarType>> = const { RefCell::new(JHashMap::new()) };
}

impl JavaEq for Exprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(*other)
    }
}

impl JavaHash for Exprent {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl IMatchable for Exprent {
    fn find_object(&self, match_node: &MatchNode, index: i32) -> Option<Box<dyn IMatchable>> {
        Exprent::find_object(*self, match_node, index).map(|e| Box::new(e) as Box<dyn IMatchable>)
    }

    fn match_(&self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        Exprent::match_(*self, match_node, engine)
    }
}

impl Exprent {
    pub const MULTIPLE_USES: i32 = 1;
    pub const SIDE_EFFECTS_FREE: i32 = 2;
    pub const BOTH_FLAGS: i32 = 3;

    /// The `super(type)` step of every subclass constructor: takes the next
    /// `EXPRESSION_COUNTER` id. Call it exactly where the Java constructor
    /// chain reaches `Exprent(Type)`, then allocate with [`Exprent::alloc_with_id`].
    pub fn next_id() -> i32 {
        DecompilerContext::get_counter_container()
            .get_counter_and_increment(CounterContainer::EXPRESSION_COUNTER)
    }

    pub fn alloc_with_id(id: i32, kind: ExprentKind) -> Exprent {
        Exprent::alloc(ExprentData {
            id,
            bytecode: None,
            kind,
            kt: None,
        })
    }

    /// Java `Exprent.type`.
    pub fn get_type(self) -> ExprentType {
        self.with(|d| d.kind.type_())
    }

    /// Java `Exprent.id`.
    pub fn get_id(self) -> i32 {
        self.with(|d| d.id)
    }

    /// Java `Exprent.bytecode` (a copy of the set).
    pub fn get_bytecode(self) -> Option<BitSet> {
        self.with(|d| d.bytecode.clone())
    }

    pub fn set_bytecode(self, bytecode: Option<BitSet>) {
        self.with_mut(|d| d.bytecode = bytecode)
    }

    /// Java `Exprent.inferredLambdaTypes.get()` (a thread-local map).
    pub fn with_inferred_lambda_types<R>(f: impl FnOnce(&mut JHashMap<String, VarType>) -> R) -> R {
        INFERRED_LAMBDA_TYPES.with(|m| f(&mut m.borrow_mut()))
    }

    pub fn get_precedence(self) -> i32 {
        let (type_, kt) = self.get_type_kt();
        if kt {
            if let Some(precedence) = k_dispatch::exprent_get_precedence(self) {
                return precedence;
            }
        }
        match type_ {
            ExprentType::Assignment => AssignmentExprent::get_precedence(self.as_assignment()),
            ExprentType::Const => ConstExprent::get_precedence(self.as_const()),
            ExprentType::Function => FunctionExprent::get_precedence(self.as_function()),
            ExprentType::New => NewExprent::get_precedence(self.as_new()),
            ExprentType::Switch => SwitchExprent::get_precedence(self.as_switch()),
            _ => Exprent::get_precedence_base(self),
        }
    }

    pub fn get_precedence_base(self) -> i32 {
        0
    }

    pub fn get_expr_type(self) -> VarType {
        let (type_, kt) = self.get_type_kt();
        if kt {
            if let Some(expr_type) = k_dispatch::exprent_get_expr_type(self) {
                return expr_type;
            }
        }
        match type_ {
            ExprentType::Array => ArrayExprent::get_expr_type(self.as_array()),
            ExprentType::Assignment => AssignmentExprent::get_expr_type(self.as_assignment()),
            ExprentType::Const => ConstExprent::get_expr_type(self.as_const()),
            ExprentType::Field => FieldExprent::get_expr_type(self.as_field()),
            ExprentType::Function => FunctionExprent::get_expr_type(self.as_function()),
            ExprentType::Invocation => InvocationExprent::get_expr_type(self.as_invocation()),
            ExprentType::New => NewExprent::get_expr_type(self.as_new()),
            ExprentType::Pattern => PatternExprent::get_expr_type(self.as_pattern()),
            ExprentType::Switch => SwitchExprent::get_expr_type(self.as_switch()),
            ExprentType::SwitchHead => SwitchHeadExprent::get_expr_type(self.as_switch_head()),
            ExprentType::Var => VarExprent::get_expr_type(self.as_var()),
            ExprentType::Yield => YieldExprent::get_expr_type(self.as_yield()),
            _ => Exprent::get_expr_type_base(self),
        }
    }

    pub fn get_expr_type_base(self) -> VarType {
        VarType::VARTYPE_VOID()
    }

    /// Java `getExprType()` including its `null`: a `++`/`--` without an
    /// implicit type (directly or as a switch selector). [`Exprent::get_expr_type`]
    /// throws `NullPointerException` there instead.
    pub fn get_expr_type_opt(self) -> Option<VarType> {
        let (type_, kt) = self.get_type_kt();
        if kt {
            if let Some(expr_type) = k_dispatch::exprent_get_expr_type_opt(self) {
                return expr_type;
            }
        }
        match type_ {
            ExprentType::Function => FunctionExprent::get_expr_type_opt(self.as_function()),
            ExprentType::SwitchHead => self.as_switch_head().get_value().get_expr_type_opt(),
            _ => Some(self.get_expr_type()),
        }
    }

    pub fn get_inferred_expr_type(self, upper_bound: Option<&VarType>) -> VarType {
        match self.get_type() {
            ExprentType::Array => {
                ArrayExprent::get_inferred_expr_type(self.as_array(), upper_bound)
            }
            ExprentType::Assignment => {
                AssignmentExprent::get_inferred_expr_type(self.as_assignment(), upper_bound)
            }
            ExprentType::Field => {
                FieldExprent::get_inferred_expr_type(self.as_field(), upper_bound)
            }
            ExprentType::Function => {
                FunctionExprent::get_inferred_expr_type(self.as_function(), upper_bound)
            }
            ExprentType::Invocation => {
                InvocationExprent::get_inferred_expr_type(self.as_invocation(), upper_bound)
            }
            ExprentType::New => NewExprent::get_inferred_expr_type(self.as_new(), upper_bound),
            ExprentType::Var => VarExprent::get_inferred_expr_type(self.as_var(), upper_bound),
            _ => Exprent::get_inferred_expr_type_base(self, upper_bound),
        }
    }

    pub fn get_inferred_expr_type_base(self, _upper_bound: Option<&VarType>) -> VarType {
        self.get_expr_type()
    }

    /// Java `getInferredExprType(VarType)` including its `null` (a `++`/`--`
    /// without an implicit type, directly or as an assignment's left side), for
    /// callers that tolerate or discard it.
    pub fn get_inferred_expr_type_opt(self, upper_bound: Option<&VarType>) -> Option<VarType> {
        match self.get_type() {
            ExprentType::Function => {
                FunctionExprent::get_inferred_expr_type_opt(self.as_function(), upper_bound)
            }
            ExprentType::Assignment => self
                .as_assignment()
                .get_left()
                .get_inferred_expr_type_opt(upper_bound),
            _ => Some(self.get_inferred_expr_type(upper_bound)),
        }
    }

    pub fn get_exprent_use(self) -> i32 {
        match self.get_type() {
            ExprentType::Array => ArrayExprent::get_exprent_use(self.as_array()),
            ExprentType::Const => ConstExprent::get_exprent_use(self.as_const()),
            ExprentType::Field => FieldExprent::get_exprent_use(self.as_field()),
            ExprentType::Function => FunctionExprent::get_exprent_use(self.as_function()),
            ExprentType::Var => VarExprent::get_exprent_use(self.as_var()),
            _ => Exprent::get_exprent_use_base(self),
        }
    }

    pub fn get_exprent_use_base(self) -> i32 {
        0
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let (type_, kt) = self.get_type_kt();
        if kt {
            if let Some(result) = k_dispatch::exprent_check_expr_type_bounds(self) {
                return result;
            }
        }
        match type_ {
            ExprentType::Array => ArrayExprent::check_expr_type_bounds(self.as_array()),
            ExprentType::Assert => AssertExprent::check_expr_type_bounds(self.as_assert()),
            ExprentType::Assignment => {
                AssignmentExprent::check_expr_type_bounds(self.as_assignment())
            }
            ExprentType::Exit => ExitExprent::check_expr_type_bounds(self.as_exit()),
            ExprentType::Function => FunctionExprent::check_expr_type_bounds(self.as_function()),
            ExprentType::Invocation => {
                InvocationExprent::check_expr_type_bounds(self.as_invocation())
            }
            ExprentType::New => NewExprent::check_expr_type_bounds(self.as_new()),
            ExprentType::Pattern => PatternExprent::check_expr_type_bounds(self.as_pattern()),
            ExprentType::SwitchHead => {
                SwitchHeadExprent::check_expr_type_bounds(self.as_switch_head())
            }
            ExprentType::Var => VarExprent::check_expr_type_bounds(self.as_var()),
            ExprentType::Yield => YieldExprent::check_expr_type_bounds(self.as_yield()),
            _ => Exprent::check_expr_type_bounds_base(self),
        }
    }

    pub fn check_expr_type_bounds_base(self) -> Option<CheckTypesResult> {
        None
    }

    pub fn contains_exprent(self, exprent: Exprent) -> bool {
        for ex in self.get_all_exprents_rec_self(true, true) {
            if ex.equals(exprent) {
                return true;
            }
        }
        false
    }

    pub fn contains_var(self, var: VarVersionPair) -> bool {
        if let Some(varex) = self.try_as_var() {
            return varex.get_var_version_pair() == var;
        }

        let lst = self.get_all_exprents();
        for e in lst.iter().rev() {
            if e.contains_var(var) {
                return true;
            }
        }
        false
    }

    /// Java: `getAllExprents(boolean)`.
    pub fn get_all_exprents_rec(self, recursive: bool) -> Vec<Exprent> {
        self.get_all_exprents_rec_self(recursive, false)
    }

    /// Java: `getAllExprents(boolean, boolean)`.
    pub fn get_all_exprents_rec_self(self, recursive: bool, self_: bool) -> Vec<Exprent> {
        let mut lst = Vec::new();
        self.get_all_exprents_rec_list(recursive, &mut lst);
        if self_ {
            lst.push(self);
        }
        lst
    }

    /// Java: `getAllExprents(boolean, List<Exprent>)`.
    fn get_all_exprents_rec_list(self, recursive: bool, list: &mut Vec<Exprent>) {
        let start = list.len();
        self.get_all_exprents_list(list);
        let end = list.len();
        if start > end {
            ValidationHelper::assert_true(
                false,
                &format!("inconsistent list size! {start} <= {end}"),
            );
        }

        if recursive {
            for i in (start..end).rev() {
                let e = list[i];
                e.get_all_exprents_rec_list(true, list);
            }
        }
    }

    pub fn get_all_variables(self) -> JHashSet<VarVersionPair> {
        let mut lst_all_exprents = self.get_all_exprents_rec(true);
        lst_all_exprents.push(self);

        let mut set = JHashSet::new();
        for expr in lst_all_exprents {
            if let Some(var) = expr.try_as_var() {
                set.add(var.get_var_version_pair());
            }
        }
        set
    }

    /// Java: `getAllExprents()`.
    pub fn get_all_exprents(self) -> Vec<Exprent> {
        let mut list = Vec::new();
        self.get_all_exprents_list(&mut list);
        list
    }

    /// Java: `getAllExprents(List<Exprent>)`; only ever appends to `list`.
    pub fn get_all_exprents_list(self, list: &mut Vec<Exprent>) {
        crate::rt::stack::guard();
        match self.get_type() {
            ExprentType::Annotation => {
                AnnotationExprent::get_all_exprents_list(self.as_annotation(), list)
            }
            ExprentType::Array => ArrayExprent::get_all_exprents_list(self.as_array(), list),
            ExprentType::Assert => AssertExprent::get_all_exprents_list(self.as_assert(), list),
            ExprentType::Assignment => {
                AssignmentExprent::get_all_exprents_list(self.as_assignment(), list)
            }
            ExprentType::Const => ConstExprent::get_all_exprents_list(self.as_const(), list),
            ExprentType::Exit => ExitExprent::get_all_exprents_list(self.as_exit(), list),
            ExprentType::Field => FieldExprent::get_all_exprents_list(self.as_field(), list),
            ExprentType::Function => {
                FunctionExprent::get_all_exprents_list(self.as_function(), list)
            }
            ExprentType::If => IfExprent::get_all_exprents_list(self.as_if(), list),
            ExprentType::Invocation => {
                InvocationExprent::get_all_exprents_list(self.as_invocation(), list)
            }
            ExprentType::Monitor => MonitorExprent::get_all_exprents_list(self.as_monitor(), list),
            ExprentType::New => NewExprent::get_all_exprents_list(self.as_new(), list),
            ExprentType::Pattern => PatternExprent::get_all_exprents_list(self.as_pattern(), list),
            ExprentType::Switch => SwitchExprent::get_all_exprents_list(self.as_switch(), list),
            ExprentType::SwitchHead => {
                SwitchHeadExprent::get_all_exprents_list(self.as_switch_head(), list)
            }
            ExprentType::Var => VarExprent::get_all_exprents_list(self.as_var(), list),
            ExprentType::Yield => YieldExprent::get_all_exprents_list(self.as_yield(), list),
            ExprentType::Other => unreachable!("no Exprent of type OTHER"),
        }
    }

    pub fn copy(self) -> Exprent {
        let (type_, kt) = self.get_type_kt();
        if kt {
            if let Some(copy) = k_dispatch::exprent_copy(self) {
                return copy;
            }
        }
        match type_ {
            ExprentType::Annotation => AnnotationExprent::copy(self.as_annotation()),
            ExprentType::Array => ArrayExprent::copy(self.as_array()),
            ExprentType::Assert => AssertExprent::copy(self.as_assert()),
            ExprentType::Assignment => AssignmentExprent::copy(self.as_assignment()),
            ExprentType::Const => ConstExprent::copy(self.as_const()),
            ExprentType::Exit => ExitExprent::copy(self.as_exit()),
            ExprentType::Field => FieldExprent::copy(self.as_field()),
            ExprentType::Function => FunctionExprent::copy(self.as_function()),
            ExprentType::If => IfExprent::copy(self.as_if()),
            ExprentType::Invocation => InvocationExprent::copy(self.as_invocation()),
            ExprentType::Monitor => MonitorExprent::copy(self.as_monitor()),
            ExprentType::New => NewExprent::copy(self.as_new()),
            ExprentType::Pattern => PatternExprent::copy(self.as_pattern()),
            ExprentType::Switch => SwitchExprent::copy(self.as_switch()),
            ExprentType::SwitchHead => SwitchHeadExprent::copy(self.as_switch_head()),
            ExprentType::Var => VarExprent::copy(self.as_var()),
            ExprentType::Yield => YieldExprent::copy(self.as_yield()),
            ExprentType::Other => unreachable!("no Exprent of type OTHER"),
        }
    }

    /// Java: `toJava()`.
    pub fn to_java_default(self) -> TextBuffer {
        self.to_java(0)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        crate::rt::stack::guard();
        let (type_, kt) = self.get_type_kt();
        if kt {
            return k_dispatch::exprent_to_java(self, indent);
        }
        match type_ {
            ExprentType::Annotation => AnnotationExprent::to_java(self.as_annotation(), indent),
            ExprentType::Array => ArrayExprent::to_java(self.as_array(), indent),
            ExprentType::Assert => AssertExprent::to_java(self.as_assert(), indent),
            ExprentType::Assignment => AssignmentExprent::to_java(self.as_assignment(), indent),
            ExprentType::Const => ConstExprent::to_java(self.as_const(), indent),
            ExprentType::Exit => ExitExprent::to_java(self.as_exit(), indent),
            ExprentType::Field => FieldExprent::to_java(self.as_field(), indent),
            ExprentType::Function => FunctionExprent::to_java(self.as_function(), indent),
            ExprentType::If => IfExprent::to_java(self.as_if(), indent),
            ExprentType::Invocation => InvocationExprent::to_java(self.as_invocation(), indent),
            ExprentType::Monitor => MonitorExprent::to_java(self.as_monitor(), indent),
            ExprentType::New => NewExprent::to_java(self.as_new(), indent),
            ExprentType::Pattern => PatternExprent::to_java(self.as_pattern(), indent),
            ExprentType::Switch => SwitchExprent::to_java(self.as_switch(), indent),
            ExprentType::SwitchHead => SwitchHeadExprent::to_java(self.as_switch_head(), indent),
            ExprentType::Var => VarExprent::to_java(self.as_var(), indent),
            ExprentType::Yield => YieldExprent::to_java(self.as_yield(), indent),
            ExprentType::Other => unreachable!("no Exprent of type OTHER"),
        }
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        match self.get_type() {
            ExprentType::Array => {
                ArrayExprent::replace_exprent(self.as_array(), old_expr, new_expr)
            }
            ExprentType::Assert => {
                AssertExprent::replace_exprent(self.as_assert(), old_expr, new_expr)
            }
            ExprentType::Assignment => {
                AssignmentExprent::replace_exprent(self.as_assignment(), old_expr, new_expr)
            }
            ExprentType::Exit => ExitExprent::replace_exprent(self.as_exit(), old_expr, new_expr),
            ExprentType::Field => {
                FieldExprent::replace_exprent(self.as_field(), old_expr, new_expr)
            }
            ExprentType::Function => {
                FunctionExprent::replace_exprent(self.as_function(), old_expr, new_expr)
            }
            ExprentType::If => IfExprent::replace_exprent(self.as_if(), old_expr, new_expr),
            ExprentType::Invocation => {
                InvocationExprent::replace_exprent(self.as_invocation(), old_expr, new_expr)
            }
            ExprentType::Monitor => {
                MonitorExprent::replace_exprent(self.as_monitor(), old_expr, new_expr)
            }
            ExprentType::New => NewExprent::replace_exprent(self.as_new(), old_expr, new_expr),
            ExprentType::SwitchHead => {
                SwitchHeadExprent::replace_exprent(self.as_switch_head(), old_expr, new_expr)
            }
            ExprentType::Yield => {
                YieldExprent::replace_exprent(self.as_yield(), old_expr, new_expr)
            }
            _ => Exprent::replace_exprent_base(self, old_expr, new_expr),
        }
    }

    pub fn replace_exprent_base(self, _old_expr: Exprent, _new_expr: Exprent) {}

    pub fn add_bytecode_offsets(self, bytecode_offsets: Option<&BitSet>) {
        if let Some(offsets) = bytecode_offsets {
            self.with_mut(|d| d.bytecode.get_or_insert_with(BitSet::new).or(offsets));
        }
    }

    pub fn get_bytecode_range(self, values: Option<&mut BitSet>) {
        match self.get_type() {
            ExprentType::Annotation => {
                AnnotationExprent::get_bytecode_range(self.as_annotation(), values)
            }
            ExprentType::Array => ArrayExprent::get_bytecode_range(self.as_array(), values),
            ExprentType::Assert => AssertExprent::get_bytecode_range(self.as_assert(), values),
            ExprentType::Assignment => {
                AssignmentExprent::get_bytecode_range(self.as_assignment(), values)
            }
            ExprentType::Const => ConstExprent::get_bytecode_range(self.as_const(), values),
            ExprentType::Exit => ExitExprent::get_bytecode_range(self.as_exit(), values),
            ExprentType::Field => FieldExprent::get_bytecode_range(self.as_field(), values),
            ExprentType::Function => {
                FunctionExprent::get_bytecode_range(self.as_function(), values)
            }
            ExprentType::If => IfExprent::get_bytecode_range(self.as_if(), values),
            ExprentType::Invocation => {
                InvocationExprent::get_bytecode_range(self.as_invocation(), values)
            }
            ExprentType::Monitor => MonitorExprent::get_bytecode_range(self.as_monitor(), values),
            ExprentType::New => NewExprent::get_bytecode_range(self.as_new(), values),
            ExprentType::Pattern => PatternExprent::get_bytecode_range(self.as_pattern(), values),
            ExprentType::Switch => SwitchExprent::get_bytecode_range(self.as_switch(), values),
            ExprentType::SwitchHead => {
                SwitchHeadExprent::get_bytecode_range(self.as_switch_head(), values)
            }
            ExprentType::Var => VarExprent::get_bytecode_range(self.as_var(), values),
            ExprentType::Yield => YieldExprent::get_bytecode_range(self.as_yield(), values),
            ExprentType::Other => unreachable!("no Exprent of type OTHER"),
        }
    }

    /// Java: `measureBytecode(BitSet)`.
    pub fn measure_bytecode(self, values: Option<&mut BitSet>) {
        if let Some(values) = values {
            self.with(|d| {
                if let Some(bytecode) = &d.bytecode {
                    values.or(bytecode);
                }
            });
        }
    }

    /// Java: `measureBytecode(BitSet, Exprent)`.
    pub fn measure_bytecode_exprent(values: Option<&mut BitSet>, exprent: Option<Exprent>) {
        if let Some(exprent) = exprent {
            exprent.get_bytecode_range(values);
        }
    }

    /// Java: `measureBytecode(BitSet, List<? extends Exprent>)`.
    pub fn measure_bytecode_list(mut values: Option<&mut BitSet>, list: &[Exprent]) {
        if !list.is_empty() {
            for e in list {
                e.get_bytecode_range(values.as_deref_mut());
            }
        }
    }

    pub fn sort_indexed(lst: &[Exprent]) -> Vec<Exprent> {
        let mut ret: Vec<Exprent> = Vec::new();
        let mut defs: Vec<VarExprent> = Vec::new();

        let sort_defs = |defs: &mut Vec<VarExprent>| {
            defs.sort_by(|o1, o2| o1.get_index().wrapping_sub(o2.get_index()).cmp(&0));
        };

        for &exp in lst {
            let is_def = exp.try_as_var().is_some_and(|v| v.is_definition());
            if !is_def {
                if !defs.is_empty() {
                    sort_defs(&mut defs);
                    ret.extend(defs.iter().map(|v| v.0));
                    defs.clear();
                }
                ret.push(exp);
            } else {
                defs.push(exp.as_var());
            }
        }

        if !defs.is_empty() {
            sort_defs(&mut defs);
            ret.extend(defs.iter().map(|v| v.0));
        }
        ret
    }

    pub fn gather_generics(
        self,
        upper_bound: Option<&VarType>,
        ret: &VarType,
        generics_map: &mut JHashMap<VarType, Option<VarType>>,
    ) {
        if let Some(upper_bound) = upper_bound {
            if upper_bound.is_generic()
                && ret.is_generic()
                && upper_bound.array_dim == ret.array_dim
            {
                let upper_generic = upper_bound.as_generic();
                let ret_generic = ret.as_generic();
                let left = upper_generic.get_arguments().len();
                let right = ret_generic.get_arguments().len();
                if left == right {
                    ret_generic.map_gen_vars_to(&upper_generic, generics_map);
                }
            }
        }
    }

    /// Java: `getGenericArgs(List<String>, Map<VarType, VarType>, List<VarType>)`.
    pub fn get_generic_args_into(
        self,
        fparams: &[String],
        generics_map: &JHashMap<VarType, Option<VarType>>,
        generic_args: &mut Vec<VarType>,
    ) {
        for type_ in fparams {
            let arg = generics_map
                .get(&GenericType::parse(&format!("T{type_};")))
                .cloned()
                .flatten();
            match arg {
                Some(arg)
                    if !(arg.is_generic()
                        && arg.as_generic().get_wildcard() != GenericType::WILDCARD_NO) =>
                {
                    generic_args.push(arg);
                }
                _ => {
                    generic_args.clear();
                    break;
                }
            }
        }
    }

    pub fn append_parameters(self, buf: &mut TextBuffer, generic_args: &[VarType]) {
        if generic_args.is_empty() {
            return;
        }
        buf.append("<");
        for i in 0..generic_args.len() {
            buf.append_cast_type_name(&generic_args[i]);
            if i + 1 < generic_args.len() {
                buf.append(", ");
            }
        }
        buf.append(">");
    }

    /// The type variables in scope (class, method and enclosing ones) mapped
    /// to their bounds; the bound lists are copies of the signatures' lists.
    /// With the fix `static_type_variable_scope`, a class's type variables
    /// are left out where the code is in a static method of that class or of
    /// a class it encloses.
    pub fn get_named_generics(self) -> JHashMap<VarType, Vec<VarType>> {
        let mut ret: JHashMap<VarType, Vec<VarType>> = JHashMap::new();
        let mut class_: Option<ClassNode> = DecompilerContext::get_current_class_node();
        let mut method: Option<MethodWrapper> = DecompilerContext::get_current_method_wrapper();
        let scoped = kuna_fixes::enabled(KunaFix::StaticTypeVariableScope);
        let mut static_context = false;

        loop {
            if scoped {
                static_context |= method.is_some_and(|m| {
                    kuna_fix_static_type_variable_scope::is_static_method(class_, m)
                });
            }
            let cls = class_
                .filter(|_| !static_context)
                .and_then(|c| c.with(|d| d.class_struct).get_signature());
            if let Some(cls) = cls {
                let fparameters = cls.fparameters().clone();
                for x in 0..fparameters.len() {
                    let bounds = cls.fbounds()[x].clone();
                    ret.put(GenericType::parse(&format!("T{};", fparameters[x])), bounds);
                }
            }

            let mtd = method.and_then(|m| m.with(|d| d.method_struct).get_signature());
            if let Some(mtd) = mtd {
                let type_parameters = mtd.type_parameters().clone();
                for x in 0..type_parameters.len() {
                    let bounds = mtd.type_parameter_bounds()[x].clone();
                    ret.put(
                        GenericType::parse(&format!("T{};", type_parameters[x])),
                        bounds,
                    );
                }
            }

            let Some(node) = class_ else {
                break;
            };
            let (parent, enclosing_method) = node.with(|d| (d.parent, d.enclosing_method.clone()));
            let Some(parent) = parent else {
                break;
            };
            let Some(parent_wrapper) = parent.get_wrapper() else {
                break;
            };
            method = match enclosing_method {
                None => None,
                Some(key) => parent_wrapper.get_methods().borrow().get_with_key(&key),
            };
            class_ = Some(parent);
        }
        ret
    }

    pub fn set_invocation_instance(self) {
        match self.get_type() {
            ExprentType::Function => FunctionExprent::set_invocation_instance(self.as_function()),
            ExprentType::Invocation => {
                InvocationExprent::set_invocation_instance(self.as_invocation())
            }
            ExprentType::New => NewExprent::set_invocation_instance(self.as_new()),
            _ => Exprent::set_invocation_instance_base(self),
        }
    }

    pub fn set_invocation_instance_base(self) {}

    pub fn set_is_qualifier(self) {
        match self.get_type() {
            ExprentType::Field => FieldExprent::set_is_qualifier(self.as_field()),
            ExprentType::Function => FunctionExprent::set_is_qualifier(self.as_function()),
            ExprentType::Invocation => InvocationExprent::set_is_qualifier(self.as_invocation()),
            _ => Exprent::set_is_qualifier_base(self),
        }
    }

    pub fn set_is_qualifier_base(self) {}

    pub fn allow_newline_after_qualifier(self) -> bool {
        match self.get_type() {
            ExprentType::Field => FieldExprent::allow_newline_after_qualifier(self.as_field()),
            ExprentType::Function => {
                FunctionExprent::allow_newline_after_qualifier(self.as_function())
            }
            ExprentType::Var => VarExprent::allow_newline_after_qualifier(self.as_var()),
            _ => Exprent::allow_newline_after_qualifier_base(self),
        }
    }

    pub fn allow_newline_after_qualifier_base(self) -> bool {
        true
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        match self.get_type() {
            ExprentType::Assignment => AssignmentExprent::process_sforms(
                self.as_assignment(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::Field => FieldExprent::process_sforms(
                self.as_field(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::Function => FunctionExprent::process_sforms(
                self.as_function(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::If => IfExprent::process_sforms(
                self.as_if(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::Invocation => InvocationExprent::process_sforms(
                self.as_invocation(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::New => NewExprent::process_sforms(
                self.as_new(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            ExprentType::Var => VarExprent::process_sforms(
                self.as_var(),
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
            _ => Exprent::process_sforms_base(
                self,
                s_forms_constructor,
                var_maps,
                stat,
                calc_live_vars,
            ),
        }
    }

    pub fn process_sforms_base(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        for ex in self.get_all_exprents() {
            ex.process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
            var_maps.to_normal();
        }
    }

    pub fn find_object(self, match_node: &MatchNode, index: i32) -> Option<Exprent> {
        if match_node.get_type() != MatchNode::MATCHNODE_EXPRENT {
            return None;
        }

        let lst_all_exprents = self.get_all_exprents();
        if lst_all_exprents.is_empty() {
            return None;
        }

        let position = match_node
            .get_rule_value(MatchProperties::ExprentPosition)
            .map(|v| v.as_string());
        if let Some(position) = position {
            if is_signed_decimal(&position) {
                let size = lst_all_exprents.len() as i32;
                let at = size.wrapping_add(jnum::parse_int(&position)) % size;
                return Some(lst_all_exprents[at as usize]);
            }
        } else if index < lst_all_exprents.len() as i32 {
            return Some(lst_all_exprents[index as usize]);
        }

        None
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        match self.get_type() {
            ExprentType::Const => ConstExprent::match_(self.as_const(), match_node, engine),
            ExprentType::Exit => ExitExprent::match_(self.as_exit(), match_node, engine),
            ExprentType::Field => FieldExprent::match_(self.as_field(), match_node, engine),
            ExprentType::Function => {
                FunctionExprent::match_(self.as_function(), match_node, engine)
            }
            ExprentType::Invocation => {
                InvocationExprent::match_(self.as_invocation(), match_node, engine)
            }
            ExprentType::Var => VarExprent::match_(self.as_var(), match_node, engine),
            _ => Exprent::match_base(self, match_node, engine),
        }
    }

    pub fn match_base(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if match_node.get_type() != MatchNode::MATCHNODE_EXPRENT {
            return false;
        }

        let type_ = self.get_type();
        match_node.iterate_rules(|key, value| {
            if key == MatchProperties::ExprentType
                && !matches!(&value.value, Some(MatchValue::ExprentType(t)) if *t == type_)
            {
                return false;
            }

            key != MatchProperties::ExprentRet
                || engine.check_and_set_variable_value(
                    &value
                        .value
                        .as_ref()
                        .expect("NullPointerException: EXPRENT_RET value")
                        .as_string(),
                    Some(MatchValue::Exprent(self)),
                )
        })
    }

    /// Java: `toString()`.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        match self.get_type() {
            ExprentType::Const => ConstExprent::to_string(self.as_const()),
            ExprentType::Var => VarExprent::to_string(self.as_var()),
            _ => Exprent::to_string_base(self),
        }
    }

    pub fn to_string_base(self) -> String {
        let _lock = DecompilerContext::get_import_collector().lock();
        self.to_java(0).convert_to_string_and_allow_data_discard()
    }

    /// Java `equals(Object)`.
    pub fn equals(self, o: Exprent) -> bool {
        match self.get_type() {
            ExprentType::Annotation => AnnotationExprent::equals(self.as_annotation(), o),
            ExprentType::Array => ArrayExprent::equals(self.as_array(), o),
            ExprentType::Assignment => AssignmentExprent::equals(self.as_assignment(), o),
            ExprentType::Const => ConstExprent::equals(self.as_const(), o),
            ExprentType::Exit => ExitExprent::equals(self.as_exit(), o),
            ExprentType::Field => FieldExprent::equals(self.as_field(), o),
            ExprentType::Function => FunctionExprent::equals(self.as_function(), o),
            ExprentType::If => IfExprent::equals(self.as_if(), o),
            ExprentType::Invocation => InvocationExprent::equals(self.as_invocation(), o),
            ExprentType::Monitor => MonitorExprent::equals(self.as_monitor(), o),
            ExprentType::New => NewExprent::equals(self.as_new(), o),
            ExprentType::SwitchHead => SwitchHeadExprent::equals(self.as_switch_head(), o),
            ExprentType::Var => VarExprent::equals(self.as_var(), o),
            _ => self == o,
        }
    }

    /// Java `hashCode()`: identity except for `ConstExprent`.
    pub fn hash_code(self) -> i32 {
        match self.try_as_const() {
            Some(c) => ConstExprent::hash_code(c),
            None => self.index() as i32,
        }
    }
}
