//! Port of `org/vineflower/kotlin/expr/KFunctionExprent.java`.
//!
//! kuna fix `kt_java_cast_syntax`: a numeric conversion is written as a call
//! of Kotlin's conversion function (`kuna_fix_kt_java_cast_syntax`), where
//! Vineflower writes Java's cast, `(long)x`.
//!
//! kuna fix `kt_narrow_bitwise_operand`: a `Byte`, `Short` or `Char` operand
//! of a bitwise or shift operator is widened to `Int`
//! (`kuna_fix_kt_narrow_bitwise_operand`).
//!
//! kuna fix `kt_reference_inequality`: Java's `!=` of two references is
//! written `!==`, and an identity `==` that a negation made of it `===`
//! (`kuna_fix_kt_reference_inequality`).
//!
//! kuna fix `kt_string_literal_escape`: a template's string pieces write a
//! form feed `\u000c` and its `Char` pieces are escaped
//! (`kuna_fix_kt_string_literal_escape`).
//!
//! kuna fix `kt_copied_equality`: a copy of an `==` that tests equality is
//! not made `===` (`kuna_fix_kt_copied_equality`).
//!
//! kuna fix `kt_negated_equality`: `(a == b) xor true` is written `a != b`
//! (`kuna_fix_kt_negated_equality`).

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_logger::Severity;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::jthrow;
use crate::k_dispatch::KExprentExt;
use crate::k_types::KTypes;
use crate::k_utils::KUtils;
use crate::kotlin::kuna_fix_kt_copied_equality;
use crate::kotlin::kuna_fix_kt_narrow_bitwise_operand;
use crate::kotlin::kuna_fix_kt_negated_equality;
use crate::kotlin::kuna_fix_kt_reference_inequality;
use crate::kotlin::kuna_fix_kt_string_literal_escape;
use crate::kotlin_exprent;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::jstring::from_utf16;
use crate::rt::BitSet;
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;
use crate::typed::Typed;
use crate::var_type::VarType;

/// `KFunctionExprent.KFunctionType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum KFunctionType {
    None,
    Equals3,
    IfNull,
    GetKclass,
    StrTemplate,
}

impl KFunctionType {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            KFunctionType::None => "NONE",
            KFunctionType::Equals3 => "EQUALS3",
            KFunctionType::IfNull => "IF_NULL",
            KFunctionType::GetKclass => "GET_KCLASS",
            KFunctionType::StrTemplate => "STR_TEMPLATE",
        }
    }
}

impl Typed for KFunctionType {}

/// A `Typed` function type: `getAnyFunctionType()` and `setFuncType(Typed)`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AnyFunctionType {
    Java(FunctionType),
    Kotlin(KFunctionType),
}

pub struct KFunctionExprentData {
    pub k_type: KFunctionType,
    /// kuna: the comparison is Java's `==` or `!=` of two references
    /// (`if_acmp`), an identity test (fix `kt_reference_inequality`).
    pub identity: bool,
}

kotlin_exprent!(pub KFunctionExprent: FunctionExprent, Function(KFunctionExprentData),
                is_k_function, as_k_function, try_as_k_function);

fn npe(what: &str) -> ! {
    jthrow!(NullPointerException; "{}", what)
}

impl KFunctionExprent {
    /// Java: KFunctionExprent(FunctionExprent)
    pub fn new(func: FunctionExprent) -> KFunctionExprent {
        let expr_type = func.0.get_expr_type_opt();
        KFunctionExprent::new_typed(func, KFunctionType::None, expr_type)
    }

    /// Java: KFunctionExprent(KFunctionType, List<Exprent>, BitSet)
    pub fn new_k_type(
        func_type: KFunctionType,
        operands: Vec<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> KFunctionExprent {
        let kex = KFunctionExprent::new_func_type(FunctionType::Other, operands, bytecode_offsets);
        kex.kd_mut(|d| d.k_type = func_type);
        kex
    }

    /// Java: KFunctionExprent(FunctionType, List<Exprent>, BitSet)
    pub fn new_func_type(
        func_type: FunctionType,
        operands: Vec<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> KFunctionExprent {
        let operands = KUtils::replace_exprents(&operands);
        KFunctionExprent::extend(FunctionExprent::new(func_type, operands, bytecode_offsets))
    }

    /// Java: KFunctionExprent(FunctionExprent, KFunctionType, VarType) (private)
    fn new_typed(
        func: FunctionExprent,
        k_type: KFunctionType,
        expr_type: Option<VarType>,
    ) -> KFunctionExprent {
        let func_type = func.get_func_type();
        let operands = KUtils::replace_exprents(&func.get_lst_operands().to_vec());
        let bytecode = func.get_bytecode();
        let kex =
            KFunctionExprent::extend(FunctionExprent::new(func_type, operands, bytecode.as_ref()));

        kex.kd_mut(|d| d.k_type = k_type);
        kex.set_implicit_type(expr_type);
        kex.set_needs_cast(func.does_cast());

        if kex.get_func_type() == FunctionType::Eq {
            let left = kex.0 .0.get_all_exprents()[0];
            let right = kex.0 .0.get_all_exprents()[1];

            let is_null =
                |t: Option<VarType>| t.is_some_and(|t| t.ptr_eq(&VarType::VARTYPE_NULL()));
            if !is_null(left.get_expr_type_opt()) && !is_null(right.get_expr_type_opt()) {
                kex.set_func_type_typed(AnyFunctionType::Kotlin(KFunctionType::Equals3));
            }
        }
        if kuna_fixes::enabled(KunaFix::KtReferenceInequality) {
            kuna_fix_kt_reference_inequality::mark(kex);
        }

        kex
    }

    /// The subclass step of the constructors: the new object becomes a `KFunctionExprent`.
    fn extend(func: FunctionExprent) -> KFunctionExprent {
        func.0
            .set_kt(Some(KExprentExt::Function(KFunctionExprentData {
                k_type: KFunctionType::None,
                identity: false,
            })));
        KFunctionExprent(func)
    }

    fn wrap(self, expr: Exprent, indent: i32) -> TextBuffer {
        self.0.wrap_operand_string(expr, true, indent)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();
        buf.add_bytecode_mapping(self.get_bytecode().as_ref());
        let lst_operands = self.get_lst_operands();

        self.optimize_type();

        match self.get_func_type() {
            FunctionType::Other => {
                let k_type = self.kd(|d| d.k_type);
                match k_type {
                    KFunctionType::Equals3 => {
                        let left = self.wrap(lst_operands.get(0), indent);
                        buf.append_buffer(&left).append(" === ");
                        let right = self.wrap(lst_operands.get(1), indent);
                        buf.append_buffer(&right);
                        return buf;
                    }
                    KFunctionType::IfNull => {
                        let left = self.wrap(lst_operands.get(0), indent);
                        buf.append_buffer(&left).append(" ?: ");
                        let right = self.wrap(lst_operands.get(1), indent);
                        buf.append_buffer(&right);
                        return buf;
                    }
                    KFunctionType::GetKclass => {
                        let operand = lst_operands.get(0);
                        if let Some(var_exprent) = operand.try_as_var() {
                            if !var_exprent.get_var_type().equals(&VarType::VARTYPE_CLASS()) {
                                jthrow!(IllegalArgumentException; "Variable accessing KClass is not a Class");
                            }
                            buf.append_buffer(&var_exprent.0.to_java(0))
                                .append(".kotlin");
                            return buf;
                        } else if let Some(const_exprent) = operand.try_as_const() {
                            let value = const_exprent
                                .get_value()
                                .unwrap_or_else(|| npe("ConstExprent.getValue()"))
                                .java_to_string();
                            let type_ = VarType::new_signature_cl(&value, !value.starts_with('['));
                            buf.append(&KTypes::get_kotlin_type(Some(&type_)));
                        } else if let Some(field_exprent) = operand.try_as_field() {
                            let primitive_type = field_exprent.get_classname();
                            let type_ = VarType::new_signature_cl(&primitive_type, true);
                            buf.append(&KTypes::get_kotlin_type(Some(&type_)));
                        } else {
                            buf.append_buffer(&operand.to_java(0));
                        }
                        buf.append("::class");
                        return buf;
                    }
                    KFunctionType::StrTemplate => {
                        buf.append_char('"');
                        let operands: Vec<Exprent> = lst_operands.iter().collect();
                        for (i, expr) in operands.iter().copied().enumerate() {
                            if let Some(const_expr) = expr.try_as_const() {
                                let const_type = const_expr.get_const_type();
                                if VarType::VARTYPE_STRING().equals(&const_type) {
                                    let ascii = DecompilerContext::get_option(
                                        IFernflowerPreferences::ASCII_STRING_CHARACTERS,
                                    );
                                    let string = const_expr
                                        .get_value()
                                        .unwrap_or_else(|| npe("ConstExprent.getValue()"));
                                    let value = ConstExprent::convert_string_to_java(
                                        string.as_string(),
                                        ascii,
                                    );
                                    if kuna_fixes::enabled(KunaFix::KtStringLiteralEscape) {
                                        buf.append(&kuna_fix_kt_string_literal_escape::kotlin_text(
                                            &value, true,
                                        ));
                                    } else {
                                        buf.append(&value.replace('$', "\\$"));
                                    }
                                } else if VarType::VARTYPE_CHAR().equals(&const_type) {
                                    let unit = const_expr
                                        .get_value()
                                        .unwrap_or_else(|| npe("ConstExprent.getValue()"))
                                        .as_int()
                                        as u16;
                                    if kuna_fixes::enabled(KunaFix::KtStringLiteralEscape) {
                                        let ascii = DecompilerContext::get_option(
                                            IFernflowerPreferences::ASCII_STRING_CHARACTERS,
                                        );
                                        buf.append(
                                            &kuna_fix_kt_string_literal_escape::template_char(
                                                unit, ascii,
                                            ),
                                        );
                                    } else {
                                        buf.append(&from_utf16(&[unit]));
                                    }
                                } else {
                                    if VarType::is_primitive(&const_type) {
                                        DecompilerContext::get_logger().write_message(
                                            &format!(
                                                "Primitive constant type in string concatenation: {}",
                                                const_type
                                            ),
                                            Severity::Warn,
                                        );
                                    }
                                    buf.append("${")
                                        .append_buffer(&expr.to_java(indent))
                                        .append("}");
                                }
                            } else if expr.is_var() {
                                let name = expr.to_java(indent);
                                if kuna_fixes::enabled(KunaFix::KtStringLiteralEscape)
                                    && !kuna_fix_kt_string_literal_escape::simple_reference(
                                        &name.convert_to_string_and_allow_data_discard(),
                                        operands.get(i + 1).copied(),
                                    )
                                {
                                    buf.append("${").append_buffer(&name).append("}");
                                } else {
                                    buf.append("$").append_buffer(&name);
                                }
                            } else {
                                buf.append("${")
                                    .append_buffer(&expr.to_java(indent))
                                    .append("}");
                            }
                        }
                        buf.append_char('"');
                        return buf;
                    }
                    KFunctionType::None => {}
                }

                jthrow!(IllegalStateException; "Unknown function type: {}", k_type.name());
            }
            FunctionType::Ternary => {
                let condition = lst_operands.get(0);
                let if_true = lst_operands.get(1);
                let if_false = lst_operands.get(2);

                if condition
                    .try_as_k_function()
                    .is_some_and(|c| c.get_func_type() == FunctionType::Instanceof)
                {
                    if let Some(cast) = if_true.try_as_k_function() {
                        if cast.get_func_type() == FunctionType::Cast
                            && if_false
                                .get_expr_type_opt()
                                .is_some_and(|t| t.ptr_eq(&VarType::VARTYPE_NULL()))
                        {
                            buf.append_buffer(&cast.get_lst_operands().get(0).to_java(indent));
                            buf.append(" as? ");
                            buf.append_buffer(&cast.get_lst_operands().get(1).to_java(indent));
                            return buf;
                        }
                    }
                }

                buf.push_newline_group(indent, 1);
                buf.append("if (");
                let condition = self.wrap(condition, indent);
                buf.append_buffer(&condition)
                    .append(")")
                    .append_possible_newline_alt(" ");
                let if_true = self.wrap(if_true, indent);
                buf.append_buffer(&if_true)
                    .append_possible_newline_alt(" ")
                    .append("else")
                    .append_possible_newline_alt(" ");
                let if_false = self.wrap(if_false, indent);
                buf.append_buffer(&if_false);
                buf.pop_newline_group();

                return buf;
            }
            FunctionType::Instanceof => {
                let left = self.wrap(lst_operands.get(0), indent);
                buf.append_buffer(&left).append(" is ");
                let right = self.wrap(lst_operands.get(1), indent);
                buf.append_buffer(&right);

                return buf;
            }
            FunctionType::BoolNot => {
                if let Some(func) = lst_operands.get(0).try_as_k_function() {
                    if func.get_func_type() == FunctionType::Instanceof {
                        let left = self.wrap(func.get_lst_operands().get(0), indent);
                        buf.append_buffer(&left).append(" !is ");
                        let right = self.wrap(func.get_lst_operands().get(1), indent);
                        buf.append_buffer(&right);
                        return buf;
                    }
                }
            }
            FunctionType::Cast => {
                if !self.does_cast() {
                    buf.append_buffer(&lst_operands.get(0).to_java(indent));
                    return buf;
                }
                let left = self.wrap(lst_operands.get(0), indent);
                buf.append_buffer(&left).append(" as ");
                buf.append_buffer(&lst_operands.get(1).to_java(indent));
                return buf;
            }
            func_type
                if func_type.cast_type().is_some()
                    && self.does_cast()
                    && kuna_fixes::enabled(KunaFix::KtJavaCastSyntax) =>
            {
                let operand = lst_operands.get(0);
                if let Some(inv) = operand.try_as_invocation() {
                    if inv.is_unboxing_call() {
                        inv.force_unboxing(true);
                    }
                }
                let to = func_type.cast_type().expect("a conversion");
                let from =
                    kuna_fix_kt_java_cast_syntax::written_type(operand, operand.get_expr_type());
                if let Some(cast) = kuna_fix_kt_java_cast_syntax::form(&to, &from) {
                    buf.append_buffer(&kuna_fix_kt_java_cast_syntax::write(
                        cast, operand, &to, indent,
                    ));
                    return buf;
                }
            }
            FunctionType::BitNot => {
                let operand = self.bitwise_operand(lst_operands.get(0), indent);
                buf.append_buffer(&operand);
                buf.append(".inv()");
                return buf;
            }
            FunctionType::Eq | FunctionType::Ne
                if kuna_fixes::enabled(KunaFix::KtReferenceInequality)
                    && kuna_fix_kt_reference_inequality::identity(self) =>
            {
                let operator = if self.get_func_type() == FunctionType::Eq {
                    " === "
                } else {
                    " !== "
                };
                let left = self.wrap(lst_operands.get(0), indent);
                buf.append_buffer(&left).append(operator);
                let right = self.wrap(lst_operands.get(1), indent);
                buf.append_buffer(&right);
                return buf;
            }
            FunctionType::And => return self.infix(buf, &lst_operands.to_vec(), " and ", indent),
            FunctionType::Or => return self.infix(buf, &lst_operands.to_vec(), " or ", indent),
            FunctionType::Xor => {
                if let Some(comparison) = kuna_fix_kt_negated_equality::negated(self.0 .0) {
                    return kuna_fix_kt_negated_equality::write(buf, comparison, indent);
                }
                return self.infix(buf, &lst_operands.to_vec(), " xor ", indent);
            }
            FunctionType::Shl => return self.infix(buf, &lst_operands.to_vec(), " shl ", indent),
            FunctionType::Shr => return self.infix(buf, &lst_operands.to_vec(), " shr ", indent),
            FunctionType::Ushr => return self.infix(buf, &lst_operands.to_vec(), " ushr ", indent),
            _ => {}
        }

        buf.append_buffer(&FunctionExprent::to_java(self.0, indent));
        buf
    }

    fn infix(
        self,
        mut buf: TextBuffer,
        lst_operands: &[Exprent],
        operator: &str,
        indent: i32,
    ) -> TextBuffer {
        let left = self.bitwise_operand(lst_operands[0], indent);
        buf.append_buffer(&left).append(operator);
        let right = self.bitwise_operand(lst_operands[1], indent);
        buf.append_buffer(&right);
        buf
    }

    /// An operand of a bitwise or shift operator; fix `kt_narrow_bitwise_operand`
    /// widens a `Byte`, `Short` or `Char` one to `Int`.
    fn bitwise_operand(self, expr: Exprent, indent: i32) -> TextBuffer {
        if kuna_fixes::enabled(KunaFix::KtNarrowBitwiseOperand) {
            if let Some(widened) = kuna_fix_kt_narrow_bitwise_operand::write(expr, indent) {
                return widened;
            }
        }
        self.wrap(expr, indent)
    }

    /// Java `getExprType()`; throws `NullPointerException` where it is `null`.
    pub fn get_expr_type(self) -> VarType {
        match self.get_expr_type_opt() {
            Some(t) => t,
            None => npe("KFunctionExprent.getExprType() is null"),
        }
    }

    /// Java `getExprType()` exactly, `null` included.
    pub fn get_expr_type_opt(self) -> Option<VarType> {
        match self.kd(|d| d.k_type) {
            KFunctionType::Equals3 => Some(VarType::VARTYPE_BOOLEAN()),
            KFunctionType::IfNull => {
                let param1 = self.get_lst_operands().get(0);
                let param2 = self.get_lst_operands().get(1);
                let type1 = param1.get_expr_type_opt();
                let type2 = param2.get_expr_type_opt();
                let (Some(type1), Some(type2)) = (type1, type2) else {
                    npe("VarType.join on null");
                };
                let supertype = VarType::join(&type1, &type2);

                Some(supertype.unwrap_or_else(VarType::VARTYPE_OBJECT))
            }
            KFunctionType::GetKclass => Some(VarType::VARTYPE_CLASS()),
            KFunctionType::StrTemplate => Some(VarType::VARTYPE_STRING()),
            KFunctionType::None => FunctionExprent::get_expr_type_opt(self.0),
        }
    }

    /// Computes bounds of its own and drops them: the result is always the
    /// `FunctionExprent` one, which throws for a Kotlin function type.
    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();

        let param1 = self.get_lst_operands().get(0);
        let type1 = param1.get_expr_type_opt();
        let mut param2: Option<Exprent> = None;
        let mut type2: Option<VarType> = None;

        if self.get_lst_operands().len() > 1 {
            let p2 = self.get_lst_operands().get(1);
            param2 = Some(p2);
            type2 = p2.get_expr_type_opt();
        }

        let type1_ref = || type1.as_ref().unwrap_or_else(|| npe("type1"));
        let type2_ref = || type2.as_ref().unwrap_or_else(|| npe("type2"));

        match self.kd(|d| d.k_type) {
            KFunctionType::IfNull => {
                let supertype = self.0 .0.get_expr_type();
                result.add_expr_lower_bound(
                    param1,
                    &VarType::find_family_bottom(supertype.type_family),
                );
                if let Some(param2) = param2 {
                    result.add_expr_lower_bound(
                        param2,
                        &VarType::find_family_bottom(supertype.type_family),
                    );
                }
            }
            KFunctionType::Equals3 => {
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
                            if let Some(param2) = param2 {
                                result.add_expr_lower_bound(param2, &VarType::VARTYPE_BYTECHAR());
                            }
                        }
                    }
                } else if type2_ref().type_ == CodeType::Boolean
                    && type1_ref().higher_in_lattice_than(type2_ref())
                {
                    if let Some(param2) = param2 {
                        result.add_expr_lower_bound(param2, &VarType::VARTYPE_BYTECHAR());
                    }
                }
            }
            _ => {}
        }

        drop(result);
        FunctionExprent::check_expr_type_bounds(self.0)
    }

    fn optimize_type(self) {
        if self.get_any_function_type() == AnyFunctionType::Kotlin(KFunctionType::Equals3) {
            let l = self.get_lst_operands().get(0);
            let r = self.get_lst_operands().get(1);

            let family = |e: Exprent| {
                e.get_expr_type_opt()
                    .unwrap_or_else(|| npe("getExprType() is null"))
                    .type_family
            };
            if family(l) != TypeFamily::Object || family(r) != TypeFamily::Object {
                self.0.set_func_type(FunctionType::Eq);
            }
        }
    }

    pub fn get_any_function_type(self) -> AnyFunctionType {
        let func_type = self.get_func_type();

        if func_type == FunctionType::Other {
            let k_type = self.kd(|d| d.k_type);
            if k_type == KFunctionType::None {
                jthrow!(IllegalStateException; "No function type at all set!");
            }

            return AnyFunctionType::Kotlin(k_type);
        }

        AnyFunctionType::Java(func_type)
    }

    /// Java: setFuncType(FunctionType)
    pub fn set_func_type(self, func_type: FunctionType) {
        self.set_func_type_typed(AnyFunctionType::Java(func_type));
    }

    /// Java: setFuncType(Typed); `super.setFuncType` is `FunctionExprent::set_func_type_base`.
    pub fn set_func_type_typed(self, typed: AnyFunctionType) {
        match typed {
            AnyFunctionType::Java(func_type) => {
                FunctionExprent::set_func_type_base(self.0, func_type);
                self.kd_mut(|d| d.k_type = KFunctionType::None);
            }
            AnyFunctionType::Kotlin(k_type) => {
                FunctionExprent::set_func_type_base(self.0, FunctionType::Other);
                self.kd_mut(|d| d.k_type = k_type);
            }
        }
    }

    pub fn get_precedence(self) -> i32 {
        match self.kd(|d| d.k_type) {
            KFunctionType::Equals3 => 6,
            KFunctionType::IfNull => 11,
            KFunctionType::GetKclass => 1,
            KFunctionType::StrTemplate => FunctionType::StrConcat.precedence(),
            KFunctionType::None => FunctionExprent::get_precedence(self.0),
        }
    }

    pub fn copy(self) -> Exprent {
        let func = FunctionExprent::copy(self.0).as_function();
        let k_type = self.kd(|d| d.k_type);
        let expr_type = self.0 .0.get_expr_type_opt();
        let copy = KFunctionExprent::new_typed(func, k_type, expr_type);
        if kuna_fixes::enabled(KunaFix::KtCopiedEquality) {
            kuna_fix_kt_copied_equality::keep_function_type(self, copy);
        }
        if kuna_fixes::enabled(KunaFix::KtReferenceInequality) {
            let identity = self.kd(|d| d.identity);
            copy.kd_mut(|d| d.identity = identity);
        }
        copy.0 .0
    }
}
