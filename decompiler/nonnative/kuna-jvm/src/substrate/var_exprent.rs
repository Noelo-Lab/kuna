//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/VarExprent.java`.
//!
//! kuna fix `kt_outer_this`: in a class written as Kotlin, an enclosing
//! class's instance (`Outer.this`) is written `this@Outer`.
//!
//! kuna fix `kt_unescaped_name`: in Kotlin output, a variable's name is
//! written as the Kotlin writer writes it (`kuna_fix_kt_unescaped_name`).
//!
//! kuna fix `kt_raw_generic_type`: a Kotlin local the plugin gave type
//! arguments has that type (`kuna_fix_kt_raw_generic_type`).

use crate::check_types_result::CheckTypesResult;
use crate::class_writer::ClassWriter;
use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::{Exprent, ExprentKind};
use crate::generic_main::GenericMain;
use crate::generic_type::GenericType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::i_matchable::MatchProperties;
use crate::instruction::Instruction;
use crate::java_subclass;
use crate::k_var_exprent::KVarExprent;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::kotlin::kuna_fix_kt_unescaped_name;
use crate::kuna_fixes::{self, KunaFix};
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::method_descriptor::MethodDescriptor;
use crate::rt::jchar::{is_java_identifier_part, is_java_identifier_start};
use crate::rt::{jnum, rethrow, try_catch, BitSet, JavaEq, JavaHash, JavaStr};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_type_processor::FinalType;
use crate::var_version_pair::VarVersionPair;

pub struct VarExprentData {
    pub index: i32,
    /// The declared type; `get_var_type` prefers the processor's view unless this is known.
    pub var_type: Option<VarType>,
    pub definition: bool,
    pub processor: Option<VarProcessor>,
    pub version: i32,
    pub class_def: bool,
    pub stack: bool,
    pub lvt: Option<LocalVariable>,
    /// Only for real (non-stack) variables during the first stage, used by finally processing.
    pub backing: Option<Instruction>,
    pub is_effectively_final: bool,
    pub writing_pattern: bool,
    pub bound_type: Option<VarType>,
    pub is_intersection_type: bool,
    pub is_catch_temp_var: bool,
}

java_subclass!(pub VarExprent: Exprent, ExprentKind::Var(VarExprentData),
               is_var, as_var, try_as_var);

impl JavaEq for VarExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for VarExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl VarExprent {
    pub const STACK_BASE: i32 = 10000;
    pub const VAR_NAMELESS_ENCLOSURE: &'static str = "<VAR_NAMELESS_ENCLOSURE>";
    const FORCE_VARVER_NAME: bool = false;

    /// Java: `VarExprent(int, VarType, VarProcessor)`.
    pub fn new(
        index: i32,
        var_type: Option<VarType>,
        processor: Option<VarProcessor>,
    ) -> VarExprent {
        VarExprent::new_bytecode(index, var_type, processor, None)
    }

    /// Java: `VarExprent(int, VarType, VarProcessor, BitSet)`.
    pub fn new_bytecode(
        index: i32,
        var_type: Option<VarType>,
        processor: Option<VarProcessor>,
        bytecode: Option<&BitSet>,
    ) -> VarExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Var(VarExprentData {
                index,
                var_type,
                definition: false,
                processor,
                version: 0,
                class_def: false,
                stack: false,
                lvt: None,
                backing: None,
                is_effectively_final: false,
                writing_pattern: false,
                bound_type: None,
                is_intersection_type: false,
                is_catch_temp_var: false,
            }),
        );
        e.add_bytecode_offsets(bytecode);
        VarExprent(e)
    }

    pub fn get_expr_type(self) -> VarType {
        self.get_var_type()
    }

    pub fn get_inferred_expr_type(self, _upper_bound: Option<&VarType>) -> VarType {
        if let Some(typed) = self.kuna_typed() {
            return typed;
        }
        let lvt = self.get_lvt();
        if let Some(signature) = lvt.as_ref().and_then(|l| l.get_signature()) {
            match try_catch(|| GenericType::parse(&signature)) {
                Ok(t) => return t,
                Err(caught) => {
                    if !caught.is("StringIndexOutOfBoundsException") {
                        rethrow(caught);
                    }
                    eprintln!("{}", caught.message);
                }
            }
        } else if let Some(lvt) = &lvt {
            return lvt.get_var_type();
        }
        self.get_var_type()
    }

    pub fn get_exprent_use(self) -> i32 {
        Exprent::MULTIPLE_USES | Exprent::SIDE_EFFECTS_FREE
    }

    pub fn get_all_exprents_list(self, _lst: &mut Vec<Exprent>) {}

    pub fn copy(self) -> Exprent {
        let (
            index,
            processor,
            definition,
            version,
            class_def,
            stack,
            lvt,
            is_effectively_final,
            is_catch_temp_var,
        ) = self.d(|d| {
            (
                d.index,
                d.processor,
                d.definition,
                d.version,
                d.class_def,
                d.stack,
                d.lvt.clone(),
                d.is_effectively_final,
                d.is_catch_temp_var,
            )
        });
        let bytecode = self.get_bytecode();
        let var = VarExprent::new_bytecode(
            index,
            Some(self.get_var_type()),
            processor,
            bytecode.as_ref(),
        );
        var.set_definition(definition);
        var.set_version(version);
        var.set_class_def(class_def);
        var.set_stack(stack);
        var.set_lvt(lvt);
        var.set_effectively_final(is_effectively_final);
        var.set_catch_temp_var(is_catch_temp_var);
        var.0
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buffer = TextBuffer::new();

        let bytecode = self.get_bytecode();
        buffer.add_bytecode_mapping(bytecode.as_ref());

        let (class_def, var_type, definition, writing_pattern, processor, index, is_intersection) =
            self.d(|d| {
                (
                    d.class_def,
                    d.var_type.clone(),
                    d.definition,
                    d.writing_pattern,
                    d.processor,
                    d.index,
                    d.is_intersection_type,
                )
            });

        if class_def {
            let var_type = var_type.expect("NullPointerException: varType");
            let child = var_type.value.as_deref().and_then(|name| {
                DecompilerContext::get_class_processor()
                    .get_map_root_classes()
                    .borrow()
                    .get(&name.to_string())
                    .copied()
            });
            ClassWriter::new().write_class(
                child.expect("NullPointerException: class node"),
                &mut buffer,
                indent,
            );
        } else {
            let var_version = self.get_var_version_pair();

            if definition {
                if !writing_pattern
                    && processor
                        .is_some_and(|p| p.get_var_final(var_version) == FinalType::ExplicitFinal)
                {
                    buffer.append("final ");
                }
                let definition_type = self.get_definition_var_type();
                let name = ExprProcessor::get_cast_type_name(&definition_type);
                if name == ExprProcessor::UNREPRESENTABLE_TYPE_STRING || is_intersection {
                    buffer.append("var");
                } else {
                    buffer.append_cast_type_name(&definition_type);
                }

                buffer.append(" ");
            }

            let name = self.get_name();
            let method = DecompilerContext::get_current_method_wrapper()
                .filter(|_| "this" != name || index != 0);
            if let Some(mut method) = method {
                let mut var_index = index;
                let mut this_var: Option<String> = None;
                if let Some(processor) = processor {
                    let mut source = processor.get_var_source(self.get_var_version_pair());
                    let node = DecompilerContext::get_current_class_node();

                    while let (Some(src), Some(node)) = (source.clone(), node) {
                        method = node
                            .get_wrapper()
                            .expect("NullPointerException: wrapper")
                            .get_methods()
                            .borrow()
                            .get_with_key(&src.a)
                            .expect("NullPointerException: method wrapper");
                        var_index = src.b.var;
                        source = method.with(|d| d.varproc).get_var_source(src.b);
                    }

                    this_var = processor
                        .get_this_vars()
                        .borrow()
                        .get_cloned(&self.get_var_version_pair());
                }

                let (method_struct, class_struct, varproc) =
                    method.with(|d| (d.method_struct, d.class_struct, d.varproc));
                let descriptor =
                    MethodDescriptor::parse_descriptor(&method_struct.get_descriptor());
                let original_index = varproc.get_var_original_index(var_index);
                let i = original_index.unwrap_or(var_index);
                let params_size = descriptor
                    .params
                    .iter()
                    .fold(0i32, |acc, v| acc.wrapping_add(v.stack_size));
                let param = i
                    <= params_size
                        - if method_struct.has_modifier(ACC_STATIC) {
                            1
                        } else {
                            0
                        };

                match this_var {
                    Some(_)
                        if name.ends_with(".this")
                            && kuna_fixes::enabled(KunaFix::KtOuterThis)
                            && kuna_fix_kt_java_cast_syntax::writing_kotlin() =>
                    {
                        buffer.append(&KVarExprent::outer_this_label(&name[..name.len() - 5]));
                    }
                    Some(this_var) if name.contains(".this") => {
                        let i = name.find(".this").expect("contains .this");
                        buffer.append_class(&name[..i], false, &this_var);
                        buffer.append(".");
                        self.write_name(&mut buffer, &name[i + 1..]);
                    }
                    _ => {
                        let mut write_name = name.clone();
                        if !VarExprent::is_valid_name(&name) && !name.contains(".this") {
                            write_name = self.get_var_ver_name(self.get_var_version_pair());
                        }
                        let printed = if kuna_fixes::enabled(KunaFix::KtUnescapedName) {
                            kuna_fix_kt_unescaped_name::variable(&write_name)
                        } else {
                            write_name.clone()
                        };

                        buffer.append_variable_descriptor(
                            &printed,
                            definition,
                            param,
                            &class_struct.qualified_name(),
                            &method_struct.get_name(),
                            &descriptor,
                            var_index,
                            &name,
                        );
                        if write_name != name {
                            buffer.append("/* $VF was: ").append(&name).append(" */");
                        }
                    }
                }
            } else {
                let mut this_var: Option<String> = None;
                if let Some(processor) = processor {
                    this_var = processor
                        .get_this_vars()
                        .borrow()
                        .get_cloned(&self.get_var_version_pair());
                }

                match this_var {
                    Some(_)
                        if name.ends_with(".this")
                            && kuna_fixes::enabled(KunaFix::KtOuterThis)
                            && kuna_fix_kt_java_cast_syntax::writing_kotlin() =>
                    {
                        buffer.append(&KVarExprent::outer_this_label(&name[..name.len() - 5]));
                    }
                    Some(this_var) if name.contains(".this") => {
                        let i = name.find(".this").expect("contains .this");
                        buffer.append_class(&name[..i], false, &this_var);
                        buffer.append(".");
                        self.write_name(&mut buffer, &name[i + 1..]);
                    }
                    _ => self.write_name(&mut buffer, &name),
                }
            }
        }

        buffer
    }

    fn write_name(self, buffer: &mut TextBuffer, name: &str) {
        let mut write_name = name.to_string();
        if !VarExprent::is_valid_name(name) {
            write_name = self.get_var_ver_name(self.get_var_version_pair());
        }

        if kuna_fixes::enabled(KunaFix::KtUnescapedName) {
            buffer.append(&kuna_fix_kt_unescaped_name::variable(&write_name));
        } else {
            buffer.append(&write_name);
        }
        if write_name != name {
            buffer.append("/* $VF was: ").append(name).append(" */");
        }
    }

    pub(crate) fn is_valid_name(name: &str) -> bool {
        if name == VarExprent::VAR_NAMELESS_ENCLOSURE {
            return true;
        }

        for (i, c) in name.utf16().into_iter().enumerate() {
            if (i == 0 && !is_java_identifier_start(c)) || (i > 0 && !is_java_identifier_part(c)) {
                return false;
            }
        }

        true
    }

    pub fn get_var_version_pair(self) -> VarVersionPair {
        self.d(|d| VarVersionPair {
            var: d.index,
            version: d.version,
        })
    }

    pub fn get_definition_type(self) -> String {
        let name = ExprProcessor::get_cast_type_name(&self.get_definition_var_type());
        if name == ExprProcessor::UNREPRESENTABLE_TYPE_STRING {
            "var".to_string()
        } else {
            name
        }
    }

    pub fn get_definition_var_type(self) -> VarType {
        if let Some(typed) = self.kuna_typed() {
            return typed;
        }
        if DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
            let (lvt, processor, index) = self.d(|d| (d.lvt.clone(), d.processor, d.index));
            if let Some(lvt) = lvt {
                if DecompilerContext::get_option(
                    IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES,
                ) {
                    if let Some(signature) = lvt.get_signature() {
                        if let Some(descriptor) = GenericMain::parse_field_signature(&signature) {
                            descriptor.verify_loosely(&lvt.get_var_type());
                            return descriptor.type_();
                        }
                    }
                }
                return self.get_var_type();
            }

            if let Some(method) = DecompilerContext::get_current_method_wrapper() {
                let mut original_index: Option<i32> = None;
                if let Some(processor) = processor {
                    original_index = processor.get_var_original_index(index);
                }
                let visible_offset = match self.get_bytecode() {
                    None => -1,
                    Some(bytecode) => bytecode.length() as i32,
                };
                if let Some(original_index) = original_index {
                    let method_struct = method.with(|d| d.method_struct);
                    let mut type_: Option<VarType> = None;
                    if let Some(lvt_attr) = method_struct.get_local_variable_attr() {
                        if let Some(descriptor) =
                            lvt_attr.get_descriptor(original_index, visible_offset)
                        {
                            type_ = Some(VarType::new_signature(&descriptor));
                        }
                    }

                    if DecompilerContext::get_option(
                        IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES,
                    ) {
                        let lvt_signatures = method_struct.get_attribute(
                            StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE,
                        );
                        if let Some(lvt_signatures) = lvt_signatures {
                            let signature =
                                lvt_signatures.get_signature(original_index, visible_offset);
                            if let Some(signature) = signature {
                                if let Some(descriptor) =
                                    GenericMain::parse_field_signature(&signature)
                                {
                                    if let Some(type_) = &type_ {
                                        descriptor.verify_loosely(type_);
                                    }
                                    return descriptor.type_();
                                }
                            }
                        }
                    }

                    if let Some(type_) = type_ {
                        return type_;
                    }
                }
            }
        }

        self.get_var_type()
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ve) = o.try_as_var() else {
            return false;
        };
        self.get_index() == ve.get_index()
            && self.get_version() == ve.get_version()
            && self.get_var_type().equals(&ve.get_var_type())
    }

    pub fn equals_versions(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ve) = o.try_as_var() else {
            return false;
        };
        self.get_index() == ve.get_index() && self.get_version() == ve.get_version()
    }

    pub fn get_bytecode_range(self, values: Option<&mut BitSet>) {
        self.measure_bytecode(values);
    }

    pub fn get_index(self) -> i32 {
        self.d(|d| d.index)
    }

    pub fn set_index(self, index: i32) {
        self.d_mut(|d| d.index = index)
    }

    pub fn get_var_type(self) -> VarType {
        let (lvt, processor, var_type) =
            self.d(|d| (d.lvt.clone(), d.processor, d.var_type.clone()));
        if let Some(lvt) = &lvt {
            if DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
                return VarType::new_signature(&lvt.get_descriptor());
            }
        }

        let mut vt: Option<VarType> = None;
        if let Some(processor) = processor {
            let pair = self.get_var_version_pair();
            let name = processor.get_var_name(pair);
            vt = name.and_then(|name| Exprent::with_inferred_lambda_types(|m| m.get_cloned(&name)));
            if vt.is_none() {
                vt = processor.get_var_type(pair);
                let this_vars = processor.get_this_vars();
                let qaul_name = this_vars.borrow().get_cloned(&pair);
                if this_vars.borrow().contains_key(&pair) {
                    let qaul_name = qaul_name.expect("NullPointerException: this var name");
                    let signature = DecompilerContext::get_struct_context()
                        .get_class(&qaul_name)
                        .and_then(|cls| cls.get_signature());
                    if let Some(signature) = signature {
                        vt = Some(signature.generic_type().0.clone());
                    } else if vt.is_none() {
                        vt = Some(VarType::new_value(CodeType::Object, 0, qaul_name));
                    }
                }
            }
        }

        if vt.is_none()
            || var_type
                .as_ref()
                .is_some_and(|t| t.type_ != CodeType::Unknown)
        {
            vt = var_type;
        }

        vt.unwrap_or_else(VarType::VARTYPE_UNKNOWN)
    }

    pub fn set_var_type(self, var_type: VarType) {
        self.d_mut(|d| d.var_type = Some(var_type))
    }

    pub fn is_definition(self) -> bool {
        self.d(|d| d.definition)
    }

    pub fn set_definition(self, definition: bool) {
        self.d_mut(|d| d.definition = definition)
    }

    pub fn get_processor(self) -> Option<VarProcessor> {
        self.d(|d| d.processor)
    }

    pub fn get_version(self) -> i32 {
        self.d(|d| d.version)
    }

    pub fn set_version(self, version: i32) {
        self.d_mut(|d| d.version = version)
    }

    pub fn is_class_def(self) -> bool {
        self.d(|d| d.class_def)
    }

    pub fn set_class_def(self, class_def: bool) {
        self.d_mut(|d| d.class_def = class_def)
    }

    pub fn is_stack(self) -> bool {
        self.d(|d| d.stack)
    }

    pub fn set_stack(self, stack: bool) {
        self.d_mut(|d| d.stack = stack)
    }

    pub fn get_backing_instr(self) -> Option<Instruction> {
        self.d(|d| d.backing)
    }

    pub fn set_backing_instr(self, backing: Option<Instruction>) {
        self.d_mut(|d| d.backing = backing)
    }

    /// Java: `setLVT(LocalVariable)`; also records the LVT type with the processor.
    /// Fix `kt_raw_generic_type`: the type with arguments the Kotlin plugin
    /// gave this local (a local without a LocalVariableTypeTable signature),
    /// where Vineflower's inference sees it.
    pub fn kuna_typed(self) -> Option<VarType> {
        self.kuna_declared()
            .map(|t| t.type_)
            .filter(crate::kotlin::kuna_fix_kt_raw_generic_type::visible)
    }

    /// Fix `kt_raw_generic_type`: the type the Kotlin plugin gave this local.
    pub fn kuna_declared(self) -> Option<crate::var_processor::KunaLocalType> {
        if !kuna_fixes::enabled(KunaFix::KtRawGenericType) {
            return None;
        }
        let (processor, index, signed) = self.d(|d| {
            (
                d.processor,
                d.index,
                d.lvt.as_ref().is_some_and(|l| l.get_signature().is_some()),
            )
        });
        if signed {
            return None;
        }
        processor?.kuna_typed(index)
    }

    pub fn set_lvt(self, var: Option<LocalVariable>) {
        let processor = self.d_mut(|d| {
            d.lvt = var.clone();
            d.processor
        });
        if let (Some(processor), Some(lvt)) = (processor, &var) {
            processor.set_var_type(self.get_var_version_pair(), &lvt.get_var_type());
        }
    }

    /// Java: `getLVT()`.
    pub fn get_lvt(self) -> Option<LocalVariable> {
        self.d(|d| d.lvt.clone())
    }

    pub fn set_effectively_final(self, is_effectively_final: bool) {
        self.d_mut(|d| d.is_effectively_final = is_effectively_final)
    }

    pub fn is_effectively_final(self) -> bool {
        self.d(|d| d.is_effectively_final)
    }

    pub fn set_writing_pattern(self) {
        self.d_mut(|d| d.writing_pattern = true)
    }

    pub fn get_name(self) -> String {
        if self.0.is_kotlin() {
            return crate::k_dispatch::var_get_name(self);
        }
        VarExprent::get_name_base(self)
    }

    pub fn get_name_base(self) -> String {
        let pair = self.get_var_version_pair();
        if !VarExprent::FORCE_VARVER_NAME {
            let (processor, lvt) = self.d(|d| (d.processor, d.lvt.clone()));
            if let Some(processor) = processor {
                if let Some(clashing_name) = processor.get_clashing_name(pair) {
                    return clashing_name;
                }
            }

            if let Some(lvt) = &lvt {
                if DecompilerContext::get_option(IFernflowerPreferences::USE_DEBUG_VAR_NAMES) {
                    return lvt.get_name();
                }
            }

            if let Some(processor) = processor {
                if let Some(ret) = processor.get_var_name(pair) {
                    return ret;
                }
            }
        }

        self.get_var_ver_name(pair)
    }

    fn get_var_ver_name(self, pair: VarVersionPair) -> String {
        if pair.version == 0 {
            format!("var{}", pair.var)
        } else {
            format!("var{}_{}", pair.var, self.get_version())
        }
    }

    pub fn set_bound_type(self, bound_type: Option<VarType>) {
        self.d_mut(|d| d.bound_type = bound_type)
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let (lvt, bound_type) = self.d(|d| (d.lvt.clone(), d.bound_type.clone()));
        if let Some(lvt) = lvt {
            let mut ret = CheckTypesResult::new();
            ret.add_expr_lower_bound(self.0, &lvt.get_var_type());
            return Some(ret);
        }

        if let Some(bound_type) = bound_type {
            let mut ret = CheckTypesResult::new();
            ret.add_expr_lower_bound(self.0, &bound_type);
            return Some(ret);
        }

        None
    }

    /// Java: `isVarReferenced(Statement, VarExprent...)`.
    pub fn is_var_referenced(self, stat: Statement, whitelist: &[VarExprent]) -> bool {
        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    if self.is_var_referenced(st, whitelist) {
                        return true;
                    }
                }
                for exp in stat.get_stat_exprents() {
                    if self.is_var_referenced_exp(exp, whitelist) {
                        return true;
                    }
                }
            }
            Some(exprents) => {
                for exp in exprents.to_vec() {
                    if self.is_var_referenced_exp(exp, whitelist) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Java: `isVarReferenced(Exprent, VarExprent...)`.
    pub fn is_var_referenced_exp(self, exp: Exprent, whitelist: &[VarExprent]) -> bool {
        let mut lst = exp.get_all_exprents_rec(true);
        lst.push(exp);
        let pair = self.get_var_version_pair();
        let lst: Vec<Exprent> = lst
            .into_iter()
            .filter(|&e| {
                e != self.0
                    && e.try_as_var()
                        .is_some_and(|v| pair == v.get_var_version_pair())
            })
            .collect();

        for var in lst {
            let mut allowed = false;
            for white in whitelist {
                if var == white.0 {
                    allowed = true;
                    break;
                }
            }
            if !allowed {
                return true;
            }
        }
        false
    }

    pub fn allow_newline_after_qualifier(self) -> bool {
        false
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        s_forms_constructor.var_read(var_maps, stat, calc_live_vars, self);
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        let (index, version, definition) = self.d(|d| (d.index, d.version, d.definition));
        format!(
            "VarExprent[{},{}{}]: {{{}}}",
            index,
            version,
            if definition { " Def" } else { "" },
            Exprent::to_string_base(self.0)
        )
    }

    pub fn set_intersection_type(self, intersection: bool) {
        self.d_mut(|d| d.is_intersection_type = intersection)
    }

    pub fn is_intersection_type(self) -> bool {
        self.d(|d| d.is_intersection_type)
    }

    pub fn set_catch_temp_var(self, catch_var: bool) {
        self.d_mut(|d| d.is_catch_temp_var = catch_var)
    }

    pub fn is_catch_temp_var(self) -> bool {
        self.d(|d| d.is_catch_temp_var)
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        if let Some(rule) = match_node.get_raw_rule(MatchProperties::ExprentVarIndex) {
            let value = rule
                .value
                .as_ref()
                .expect("NullPointerException: rule value");
            if rule.is_variable() {
                return engine.check_and_set_variable_value(
                    &value.as_string(),
                    Some(MatchValue::Integer(self.get_index())),
                );
            } else {
                return self.get_index() == jnum::parse_int(&value.as_string());
            }
        }

        true
    }

    pub fn get_pattern_vars(self) -> Vec<VarExprent> {
        vec![self]
    }
}

/// Java `VarExprent.DefinitionLocker`: clears `definition` for its lifetime and
/// restores it on drop (`try (var lock = var.new DefinitionLocker())`).
pub struct DefinitionLocker {
    var: VarExprent,
    is_def: bool,
}

impl DefinitionLocker {
    pub fn new(var: VarExprent) -> DefinitionLocker {
        let is_def = var.is_definition();
        var.set_definition(false);
        DefinitionLocker { var, is_def }
    }

    pub fn close(self) {}
}

impl Drop for DefinitionLocker {
    fn drop(&mut self) {
        if self.var.is_live() {
            self.var.set_definition(self.is_def);
        }
    }
}
