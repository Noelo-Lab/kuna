//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/VarExprent.java`.

use crate::check_types_result::CheckTypesResult;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentKind};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::instruction::Instruction;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::jchar::{is_java_identifier_part, is_java_identifier_start};
use crate::rt::{BitSet, JavaEq, JavaHash, JavaStr};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;
use crate::{java_subclass, unported};

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
        unported!("VarExprent.getInferredExprType")
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

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("VarExprent.toJava")
    }

    #[allow(dead_code)]
    fn write_name(self, _buffer: &mut TextBuffer, _name: &str) {
        unported!("VarExprent.writeName")
    }

    #[allow(dead_code)]
    fn is_valid_name(name: &str) -> bool {
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
        unported!("VarExprent.getDefinitionType")
    }

    pub fn get_definition_var_type(self) -> VarType {
        unported!("VarExprent.getDefinitionVarType")
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
                        vt = Some(signature.generic_type.0.clone());
                    } else if vt.is_none() {
                        vt = Some(VarType::new(CodeType::Object, 0, Some(qaul_name)));
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
        unported!("VarExprent.checkExprTypeBounds")
    }

    /// Java: `isVarReferenced(Statement, VarExprent...)`.
    pub fn is_var_referenced(self, _stat: Statement, _whitelist: &[VarExprent]) -> bool {
        unported!("VarExprent.isVarReferenced(Statement)")
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
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("VarExprent.processSforms")
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        unported!("VarExprent.toString")
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
    pub fn match_(self, _match_node: &MatchNode, _engine: &mut MatchEngine) -> bool {
        unported!("VarExprent.match")
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
