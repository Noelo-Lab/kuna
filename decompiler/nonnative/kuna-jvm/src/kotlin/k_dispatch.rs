//! The Kotlin half of the exprent and statement dispatchers.
//!
//! A Kotlin expression or statement (`KFunctionExprent extends FunctionExprent`,
//! `KIfStatement extends IfStatement`, ...) is its Java base object with
//! `ExprentData::kt` / `StatementData::kt` set, so it stays an instance of the
//! Java class for every core helper. Each core dispatcher of a method a Kotlin
//! class overrides checks the flag first and comes here, and this module calls
//! the Kotlin view in path form. An `Option` result of `None` means the Kotlin
//! class inherits that method from its Java class: the dispatcher goes on with
//! the Java dispatch.
//!
//! The views derive from the Java views, so a method call on one may resolve to
//! the Java body: call overridden methods through the `Exprent`/`Statement`
//! handle (`view.0.0.to_java(i)`), as the Java's virtual call does.

use crate::check_types_result::CheckTypesResult;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::jthrow;
use crate::k_annotation_exprent::{KAnnotationExprent, KAnnotationExprentData};
use crate::k_catch_statement::KCatchStatement;
use crate::k_const_exprent::KConstExprent;
use crate::k_do_statement::KDoStatement;
use crate::k_exit_exprent::{KExitExprent, KExitExprentData};
use crate::k_field_exprent::KFieldExprent;
use crate::k_function_exprent::{KFunctionExprent, KFunctionExprentData};
use crate::k_if_statement::KIfStatement;
use crate::k_invocation_exprent::{KInvocationExprent, KInvocationExprentData};
use crate::k_new_exprent::{KNewExprent, KNewExprentData};
use crate::k_sequence_statement::KSequenceStatement;
use crate::k_switch_head_exprent::KSwitchHeadExprent;
use crate::k_switch_statement::KSwitchStatement;
use crate::k_var_exprent::{KVarExprent, KVarExprentData};
use crate::statement::{Statement, StatementType};
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

/// The fields a Kotlin expression class adds to its Java class, one variant
/// per class (`ExprentData::kt`).
pub enum KExprentExt {
    Annotation(KAnnotationExprentData),
    Const,
    Exit(KExitExprentData),
    Field,
    Function(KFunctionExprentData),
    Invocation(KInvocationExprentData),
    New(KNewExprentData),
    SwitchHead,
    Var(KVarExprentData),
}

/// Which Kotlin expression class an exprent is.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum KExprentType {
    Annotation,
    Const,
    Exit,
    Field,
    Function,
    Invocation,
    New,
    SwitchHead,
    Var,
}

impl KExprentExt {
    pub fn type_(&self) -> KExprentType {
        match self {
            KExprentExt::Annotation(_) => KExprentType::Annotation,
            KExprentExt::Const => KExprentType::Const,
            KExprentExt::Exit(_) => KExprentType::Exit,
            KExprentExt::Field => KExprentType::Field,
            KExprentExt::Function(_) => KExprentType::Function,
            KExprentExt::Invocation(_) => KExprentType::Invocation,
            KExprentExt::New(_) => KExprentType::New,
            KExprentExt::SwitchHead => KExprentType::SwitchHead,
            KExprentExt::Var(_) => KExprentType::Var,
        }
    }

    /// The Java class a Kotlin expression class extends.
    pub fn base_type(&self) -> ExprentType {
        match self {
            KExprentExt::Annotation(_) => ExprentType::Annotation,
            KExprentExt::Const => ExprentType::Const,
            KExprentExt::Exit(_) => ExprentType::Exit,
            KExprentExt::Field => ExprentType::Field,
            KExprentExt::Function(_) => ExprentType::Function,
            KExprentExt::Invocation(_) => ExprentType::Invocation,
            KExprentExt::New(_) => ExprentType::New,
            KExprentExt::SwitchHead => ExprentType::SwitchHead,
            KExprentExt::Var(_) => ExprentType::Var,
        }
    }
}

impl Exprent {
    /// `instanceof KExprent`.
    pub fn is_kotlin(self) -> bool {
        self.with(|d| d.kt.is_some())
    }

    /// `get_type()` and [`Exprent::is_kotlin`] in one read: what every dispatcher needs.
    #[inline]
    pub fn get_type_kt(self) -> (ExprentType, bool) {
        self.with(|d| (d.kind.type_(), d.kt.is_some()))
    }

    /// The Kotlin class of a Kotlin object.
    pub fn get_kotlin_type(self) -> Option<KExprentType> {
        self.with(|d| d.kt.as_deref().map(KExprentExt::type_))
    }

    /// Makes this object an instance of a Kotlin class (the constructor's
    /// subclass step), or of its Java class again with `None`.
    pub fn set_kt(self, kt: Option<KExprentExt>) {
        if let Some(ext) = &kt {
            if ext.base_type() != self.get_type() {
                jthrow!(IllegalStateException; "{:?} cannot extend {}", ext.type_(), self.get_type().name());
            }
        }
        self.with_mut(|d| d.kt = kt.map(Box::new));
    }
}

impl Statement {
    /// Whether this is an instance of a Kotlin statement class.
    pub fn is_kotlin(self) -> bool {
        self.with(|d| d.kt)
    }

    /// Makes this statement an instance of the Kotlin class of its type.
    pub fn set_kt(self, kt: bool) {
        self.with_mut(|d| d.kt = kt);
    }
}

fn kotlin_type(e: Exprent) -> KExprentType {
    match e.get_kotlin_type() {
        Some(t) => t,
        None => jthrow!(IllegalStateException; "not a Kotlin exprent"),
    }
}

/// `toJava(int)`: every Kotlin expression class overrides it.
pub fn exprent_to_java(e: Exprent, indent: i32) -> TextBuffer {
    match kotlin_type(e) {
        KExprentType::Annotation => KAnnotationExprent::to_java(e.as_k_annotation(), indent),
        KExprentType::Const => KConstExprent::to_java(e.as_k_const(), indent),
        KExprentType::Exit => KExitExprent::to_java(e.as_k_exit(), indent),
        KExprentType::Field => KFieldExprent::to_java(e.as_k_field(), indent),
        KExprentType::Function => KFunctionExprent::to_java(e.as_k_function(), indent),
        KExprentType::Invocation => KInvocationExprent::to_java(e.as_k_invocation(), indent),
        KExprentType::New => KNewExprent::to_java(e.as_k_new(), indent),
        KExprentType::SwitchHead => KSwitchHeadExprent::to_java(e.as_k_switch_head(), indent),
        KExprentType::Var => KVarExprent::to_java(e.as_k_var(), indent),
    }
}

/// `getPrecedence()`.
pub fn exprent_get_precedence(e: Exprent) -> Option<i32> {
    match kotlin_type(e) {
        KExprentType::Function => Some(KFunctionExprent::get_precedence(e.as_k_function())),
        KExprentType::Invocation => Some(KInvocationExprent::get_precedence(e.as_k_invocation())),
        KExprentType::New => Some(KNewExprent::get_precedence(e.as_k_new())),
        _ => None,
    }
}

/// `copy()`.
pub fn exprent_copy(e: Exprent) -> Option<Exprent> {
    match kotlin_type(e) {
        KExprentType::Exit => Some(KExitExprent::copy(e.as_k_exit())),
        KExprentType::Function => Some(KFunctionExprent::copy(e.as_k_function())),
        KExprentType::Invocation => Some(KInvocationExprent::copy(e.as_k_invocation())),
        KExprentType::New => Some(KNewExprent::copy(e.as_k_new())),
        KExprentType::Var => Some(KVarExprent::copy(e.as_k_var())),
        _ => None,
    }
}

/// `getExprType()`.
pub fn exprent_get_expr_type(e: Exprent) -> Option<VarType> {
    match kotlin_type(e) {
        KExprentType::Function => Some(KFunctionExprent::get_expr_type(e.as_k_function())),
        _ => None,
    }
}

/// `getExprType()` with its `null` (see `Exprent::get_expr_type_opt`).
pub fn exprent_get_expr_type_opt(e: Exprent) -> Option<Option<VarType>> {
    match kotlin_type(e) {
        KExprentType::Function => Some(KFunctionExprent::get_expr_type_opt(e.as_k_function())),
        _ => None,
    }
}

/// `checkExprTypeBounds()`.
pub fn exprent_check_expr_type_bounds(e: Exprent) -> Option<Option<CheckTypesResult>> {
    match kotlin_type(e) {
        KExprentType::Function => Some(KFunctionExprent::check_expr_type_bounds(e.as_k_function())),
        _ => None,
    }
}

/// `FunctionExprent.setFuncType(FunctionType)` of a `KFunctionExprent`.
pub fn function_set_func_type(f: FunctionExprent, func_type: FunctionType) {
    KFunctionExprent::set_func_type(f.0.as_k_function(), func_type)
}

/// `VarExprent.getName()` of a `KVarExprent`.
pub fn var_get_name(v: VarExprent) -> String {
    KVarExprent::get_name(v.0.as_k_var())
}

/// `Statement.toJava(int)` of a Kotlin statement.
pub fn statement_to_java(s: Statement, indent: i32) -> TextBuffer {
    match s.type_() {
        StatementType::If => KIfStatement::to_java(s.as_k_if(), indent),
        StatementType::Do => KDoStatement::to_java(s.as_k_do(), indent),
        StatementType::Switch => KSwitchStatement::to_java(s.as_k_switch(), indent),
        StatementType::Sequence => KSequenceStatement::to_java(s.as_k_sequence(), indent),
        StatementType::TryCatch => KCatchStatement::to_java(s.as_k_catch(), indent),
        other => jthrow!(IllegalStateException; "no Kotlin statement of type {:?}", other),
    }
}

/// Declares a Kotlin expression class as a view over its Java view:
///
/// `kotlin_exprent!(pub KFunctionExprent: FunctionExprent, Function(KFunctionExprentData), is_k_function, as_k_function, try_as_k_function);`
///
/// (a class that adds no field leaves out the data type). `Exprent` gains the
/// `instanceof`, cast and checked-cast methods; the view gets `kd`/`kd_mut` to
/// reach the added fields.
#[macro_export]
macro_rules! kotlin_exprent {
    ($(#[$meta:meta])* $vis:vis $view:ident : $base:ident, $variant:ident ( $data:ty ),
     $is:ident, $as_:ident, $try_as:ident) => {
        $crate::kotlin_exprent!(@view $(#[$meta])* $vis $view : $base, $variant, $is, $as_, $try_as);

        impl $view {
            /// Read the fields the Kotlin class adds.
            pub fn kd<R>(self, f: impl FnOnce(&$data) -> R) -> R {
                (self.0).0.with(|base| match base.kt.as_deref() {
                    Some($crate::k_dispatch::KExprentExt::$variant(data)) => f(data),
                    _ => unreachable!(concat!("not a ", stringify!($view))),
                })
            }

            /// Mutate the fields the Kotlin class adds.
            pub fn kd_mut<R>(self, f: impl FnOnce(&mut $data) -> R) -> R {
                (self.0).0.with_mut(|base| match base.kt.as_deref_mut() {
                    Some($crate::k_dispatch::KExprentExt::$variant(data)) => f(data),
                    _ => unreachable!(concat!("not a ", stringify!($view))),
                })
            }
        }
    };
    ($(#[$meta:meta])* $vis:vis $view:ident : $base:ident, $variant:ident,
     $is:ident, $as_:ident, $try_as:ident) => {
        $crate::kotlin_exprent!(@view $(#[$meta])* $vis $view : $base, $variant, $is, $as_, $try_as);
    };
    (@view $(#[$meta:meta])* $vis:vis $view:ident : $base:ident, $variant:ident,
     $is:ident, $as_:ident, $try_as:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $view(pub $base);

        impl ::std::ops::Deref for $view {
            type Target = $base;
            fn deref(&self) -> &$base {
                &self.0
            }
        }

        impl From<$view> for $base {
            fn from(view: $view) -> $base {
                view.0
            }
        }

        impl From<$view> for $crate::exprent::Exprent {
            fn from(view: $view) -> $crate::exprent::Exprent {
                (view.0).0
            }
        }

        impl ::std::fmt::Debug for $view {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}#{}", stringify!($view), (self.0).0.index())
            }
        }

        impl $crate::exprent::Exprent {
            pub fn $is(self) -> bool {
                self.with(|base| {
                    base.kt
                        .as_deref()
                        .is_some_and(|k| k.type_() == $crate::k_dispatch::KExprentType::$variant)
                })
            }

            pub fn $try_as(self) -> Option<$view> {
                self.$is().then_some($view($base(self)))
            }

            #[track_caller]
            pub fn $as_(self) -> $view {
                if !self.$is() {
                    $crate::rt::exception::throw(
                        "ClassCastException",
                        concat!("Exprent is not a ", stringify!($view)),
                    );
                }
                $view($base(self))
            }
        }
    };
}

/// Declares a Kotlin statement class as a view over its Java view:
///
/// `kotlin_statement!(pub KIfStatement: IfStatement, If, is_k_if, as_k_if, try_as_k_if);`
#[macro_export]
macro_rules! kotlin_statement {
    ($(#[$meta:meta])* $vis:vis $view:ident : $base:ident, $stat_type:ident,
     $is:ident, $as_:ident, $try_as:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $view(pub $base);

        impl ::std::ops::Deref for $view {
            type Target = $base;
            fn deref(&self) -> &$base {
                &self.0
            }
        }

        impl From<$view> for $base {
            fn from(view: $view) -> $base {
                view.0
            }
        }

        impl From<$view> for $crate::statement::Statement {
            fn from(view: $view) -> $crate::statement::Statement {
                (view.0).0
            }
        }

        impl ::std::fmt::Debug for $view {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}#{}", stringify!($view), (self.0).0.index())
            }
        }

        impl $crate::statement::Statement {
            pub fn $is(self) -> bool {
                self.with(|base| {
                    base.kt && base.type_ == $crate::statement::StatementType::$stat_type
                })
            }

            pub fn $try_as(self) -> Option<$view> {
                self.$is().then_some($view($base(self)))
            }

            #[track_caller]
            pub fn $as_(self) -> $view {
                if !self.$is() {
                    $crate::rt::exception::throw(
                        "ClassCastException",
                        concat!("Statement is not a ", stringify!($view)),
                    );
                }
                $view($base(self))
            }
        }
    };
}
