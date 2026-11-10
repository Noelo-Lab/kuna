//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/AssignmentExprent.java`.
//!
//! kuna fix `compound_shift_count_type` (`kuna_fixes`): `checkExprTypeBounds`
//! does not give the shift count of `<<=`, `>>=` or `>>>=` the shifted
//! variable's type.
//!
//! kuna fix `kt_compound_bitwise_assignment`: in Kotlin output, `x |= y`
//! (and `&=`, `^=`, `<<=`, `>>=`, `>>>=`) is written `x = x or y`
//! (`kuna_fix_kt_compound_bitwise_assignment`).
//!
//! kuna fix `definition_const_type` (`kuna_fixes`): a declaration whose type
//! comes from the local variable table writes a boolean-typed constant
//! initializer in that type when the type is integral.
//!
//! kuna fix `disjoint_generic_cast`: the value written here is marked as
//! one with a declared target type (`kuna_fix_disjoint_generic_cast`).

use crate::check_types_result::CheckTypesResult;
use crate::p2_lift::kuna_fix_disjoint_generic_cast;
use crate::code_constants::ACC_FINAL;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::{Exprent, ExprentKind, ExprentType};
use crate::function_exprent::FunctionType;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::interpreter_util::InterpreterUtil;
use crate::kotlin::kuna_fix_kt_compound_bitwise_assignment;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::kuna_fixes::{self, KunaFix};
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::{instance_of, str_equals, VarType};
use crate::var_version_pair::VarVersionPair;
use crate::{java_subclass, jthrow};

pub struct AssignmentExprentData {
    pub left: Exprent,
    pub right: Exprent,
    /// The compound operator of `x op= y`; `None` for a plain `=`.
    pub cond_type: Option<FunctionType>,
}

java_subclass!(pub AssignmentExprent: Exprent, ExprentKind::Assignment(AssignmentExprentData),
               is_assignment, as_assignment, try_as_assignment);

impl JavaEq for AssignmentExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for AssignmentExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

impl AssignmentExprent {
    /// Java: `AssignmentExprent(Exprent, Exprent, BitSet)`.
    pub fn new(
        left: Exprent,
        right: Exprent,
        bytecode_offsets: Option<&BitSet>,
    ) -> AssignmentExprent {
        AssignmentExprent::new_cond(left, right, None, bytecode_offsets)
    }

    /// Java: `AssignmentExprent(Exprent, Exprent, FunctionExprent.FunctionType, BitSet)`.
    pub fn new_cond(
        left: Exprent,
        right: Exprent,
        cond_type: Option<FunctionType>,
        bytecode_offsets: Option<&BitSet>,
    ) -> AssignmentExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Assignment(AssignmentExprentData {
                left,
                right,
                cond_type,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        AssignmentExprent(e)
    }

    pub fn get_expr_type(self) -> VarType {
        let (left, right) = self.d(|d| (d.left, d.right));
        let (Some(left_type), Some(right_type)) =
            (left.get_expr_type_opt(), right.get_expr_type_opt())
        else {
            jthrow!(NullPointerException; "VarType.join on null");
        };
        let r_type = VarType::join(&left_type, &right_type);
        match r_type {
            Some(t) => t,
            None => left.get_expr_type(),
        }
    }

    pub fn get_inferred_expr_type(self, upper_bounds: Option<&VarType>) -> VarType {
        self.get_left().get_inferred_expr_type(upper_bounds)
    }

    pub fn check_expr_type_bounds(self) -> Option<CheckTypesResult> {
        let mut result = CheckTypesResult::new();
        let (left, right) = self.d(|d| (d.left, d.right));

        let type_left = left.get_expr_type_opt();
        let type_right = right.get_expr_type_opt();
        let (Some(type_left), Some(mut type_right)) = (type_left, type_right) else {
            jthrow!(NullPointerException; "typeFamily of null");
        };

        let shift = matches!(
            self.d(|d| d.cond_type),
            Some(FunctionType::Shl | FunctionType::Shr | FunctionType::Ushr)
        );
        if type_left.type_family.is_greater(type_right.type_family) {
            if shift && kuna_fixes::enabled(KunaFix::CompoundShiftCountType) {
                result.add_expr_lower_bound(right, &VarType::VARTYPE_BYTECHAR());
            } else {
                result.add_expr_lower_bound(
                    right,
                    &VarType::find_family_bottom(type_left.type_family),
                );
            }
        } else if type_left.type_family.is_lesser(type_right.type_family) {
            result.add_expr_lower_bound(left, &type_right);
        } else {
            if let Some(func) = right.try_as_function() {
                if func.get_func_type() == FunctionType::Ternary
                    && VarType::VARTYPE_INT().equals(&type_right)
                {
                    type_right = VarType::VARTYPE_BYTECHAR();
                }
            }

            match VarType::join(&type_left, &type_right) {
                Some(joined) => result.add_expr_lower_bound(left, &joined),
                None => jthrow!(NullPointerException; "Null not expected here"),
            }
        }

        Some(result)
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            lst.push(d.left);
            lst.push(d.right);
        });
    }

    pub fn copy(self) -> Exprent {
        let (left, right, cond_type) = self.d(|d| (d.left, d.right, d.cond_type));
        let bytecode = self.get_bytecode();
        AssignmentExprent::new_cond(left.copy(), right.copy(), cond_type, bytecode.as_ref()).0
    }

    pub fn get_precedence(self) -> i32 {
        13
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let left = self.get_left();
        let mut left_type = left.get_inferred_expr_type(None);

        let mut field_in_class_init = false;
        let mut hidden_field = false;
        if let Some(field) = left.try_as_field() {
            if let Some(node) = DecompilerContext::get_current_class_node() {
                let class_struct = node.with(|d| d.class_struct);
                let fd = class_struct
                    .get_field(&field.get_name(), &field.get_descriptor().descriptor_string);
                if let Some(fd) = fd {
                    if field.is_static() && fd.has_modifier(ACC_FINAL) {
                        field_in_class_init = true;
                    }
                    if let Some(wrapper) = node.get_wrapper() {
                        let key =
                            InterpreterUtil::make_unique_key(&fd.get_name(), &fd.get_descriptor());
                        if wrapper.get_hidden_members().borrow().contains(&key) {
                            hidden_field = true;
                        }
                    }
                }
            }
        }

        if hidden_field {
            return TextBuffer::new();
        }

        let mut buffer = TextBuffer::new();

        if field_in_class_init {
            let field = left.as_field();
            let name = field.get_name();
            buffer.append_field_descriptor(
                &name,
                false,
                &field.get_classname(),
                &name,
                &field.get_descriptor(),
            );
        } else {
            buffer.append_buffer(&left.to_java(indent));
        }

        if let Some(right_const) = self.get_right().try_as_const() {
            if kuna_fixes::enabled(KunaFix::DefinitionConstType) {
                if let Some(var) = left.try_as_var().filter(|v| v.is_definition()) {
                    let declared = var.get_definition_var_type();
                    if left_type.type_family == TypeFamily::Boolean
                        && right_const.get_const_type().type_family == TypeFamily::Boolean
                        && declared.array_dim == 0
                        && declared.type_family == TypeFamily::Integer
                    {
                        right_const.set_const_type(declared.clone());
                        left_type = declared;
                    }
                }
            }
            right_const.adjust_const_type(&left_type);
        }

        self.optimize_cast_for_assign();

        let right = self.get_right();
        match self.get_cond_type() {
            None => {
                buffer.append(" = ");

                let cast_name = {
                    let _lock = DecompilerContext::get_import_collector().lock();
                    ExprProcessor::get_cast_type_name(&left_type)
                };

                if cast_name == ExprProcessor::UNREPRESENTABLE_TYPE_STRING {
                    buffer.append_buffer(&right.to_java(indent));
                } else {
                    kuna_fix_disjoint_generic_cast::mark_declared_target();
                    ExprProcessor::get_casted_exprent_full(
                        right,
                        &left_type,
                        &mut buffer,
                        indent,
                        NullCastType::DontCastAtAll,
                        false,
                        false,
                        false,
                    );
                }
            }
            Some(cond_type) => {
                let kotlin = (kuna_fixes::enabled(KunaFix::KtCompoundBitwiseAssignment)
                    && kuna_fix_kt_java_cast_syntax::writing_kotlin())
                .then(|| {
                    kuna_fix_kt_compound_bitwise_assignment::write(
                        left, &left_type, cond_type, right, indent,
                    )
                })
                .flatten();
                if let Some(kotlin) = kotlin {
                    buffer.append_buffer(&kotlin);
                } else {
                    buffer
                        .append(" ")
                        .append(cond_type.operator().unwrap_or("null"))
                        .append("= ");
                    buffer.append_buffer(&right.to_java(indent));
                }
            }
        }

        buffer.add_start_bytecode_mapping(self.get_bytecode().as_ref());

        if let Some(var_left) = self.get_left().try_as_var() {
            if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILER_COMMENTS) {
                if var_left.is_definition() {
                    if let Some(processor) = var_left.get_processor() {
                        if processor
                            .get_synthetic_semaphores()
                            .borrow()
                            .contains(&var_left.get_index())
                        {
                            buffer.append(" /* VF: Semaphore variable */");
                        }
                    }
                }
            }
        }

        buffer
    }

    fn optimize_cast_for_assign(self) {
        let (left, right) = self.d(|d| (d.left, d.right));
        let Some(func) = right.try_as_function() else {
            return;
        };

        if func.get_func_type() != FunctionType::Cast {
            return;
        }

        let cast = func.get_lst_operands().get(1);

        if !func.does_cast() && left.is_var() {
            if instance_of(&right.get_expr_type().value, &cast.get_expr_type().value) {
                let cast_val = func.get_lst_operands().get(0);

                if left.get_expr_type().array_dim > cast_val.get_expr_type().array_dim {
                    func.set_needs_cast(true);
                    return;
                }
            }
        }

        let left_type = left.get_inferred_expr_type(None);

        if !left_type.is_generic_type() {
            return;
        }

        let Some(method) = DecompilerContext::get_current_method_wrapper() else {
            return;
        };

        let (mt, class_struct) = method.with(|d| (d.method_struct, d.class_struct));
        let Some(descriptor) = mt.get_signature() else {
            return;
        };

        if descriptor.type_parameters().is_empty() {
            return;
        }

        let index = descriptor
            .type_parameters()
            .iter()
            .position(|p| left_type.value.as_deref() == Some(p.as_str()));
        let Some(index) = index else {
            return;
        };

        if let Some(class_descriptor) = class_struct.get_signature() {
            let snapshot = descriptor.type_parameter_bounds()[index].clone();
            for type_ in snapshot {
                let idex = class_descriptor
                    .fparameters()
                    .iter()
                    .position(|p| type_.value.as_deref() == Some(p.as_str()));

                if let Some(idex) = idex {
                    let extra = class_descriptor.fbounds()[idex].clone();
                    descriptor.borrow_mut().type_parameter_bounds[index].extend(extra);
                }
            }
        }

        let right_type = cast.get_inferred_expr_type(Some(&left_type));

        let mut did_reset = false;
        let types = descriptor.type_parameter_bounds()[index].clone();
        for type_ in &types {
            if str_equals(&right_type.value, &type_.value) {
                cast.as_const().set_const_type(left_type.clone());
                did_reset = true;
            }
        }

        if did_reset {
            func.0.get_inferred_expr_type_opt(None);
        }
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if old_expr == d.left {
                d.left = new_expr;
            }
            if old_expr == d.right {
                d.right = new_expr;
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(as_) = o.try_as_assignment() else {
            return false;
        };
        let (left, right, cond_type) = self.d(|d| (d.left, d.right, d.cond_type));
        left.equals(as_.get_left())
            && right.equals(as_.get_right())
            && cond_type == as_.get_cond_type()
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let (left, right) = self.d(|d| (d.left, d.right));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(left));
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), Some(right));
        self.measure_bytecode(values);
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        let dest = self.get_left();
        match dest.get_type() {
            ExprentType::Var => {
                let dest_var = dest.as_var();

                if self.get_cond_type().is_some() {
                    dest_var
                        .0
                        .process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
                    self.get_right().process_sforms(
                        s_forms_constructor,
                        var_maps,
                        stat,
                        calc_live_vars,
                    );

                    let var_map = var_maps.to_normal();

                    let phantom =
                        s_forms_constructor.get_or_create_phantom(dest_var.get_var_version_pair());
                    var_map.borrow_mut().set_current_var_pair(phantom);
                } else {
                    self.get_right().process_sforms(
                        s_forms_constructor,
                        var_maps,
                        stat,
                        calc_live_vars,
                    );
                    let normal = var_maps.to_normal();
                    s_forms_constructor.update_var_exprent(dest_var, stat, &normal, calc_live_vars);

                    if s_forms_constructor.track_direct_assignments {
                        let right = self.get_right();
                        match right.get_type() {
                            ExprentType::Var => {
                                let rightpaar = right.as_var().get_var_version_pair();
                                s_forms_constructor.mark_direct_assignment(
                                    dest_var.get_var_version_pair(),
                                    rightpaar,
                                );
                            }
                            ExprentType::Field => {
                                let index = s_forms_constructor.get_field_index(right.as_field());
                                let rightpaar = VarVersionPair::new(index, 0);
                                s_forms_constructor.mark_direct_assignment(
                                    dest_var.get_var_version_pair(),
                                    rightpaar,
                                );
                            }
                            _ => {}
                        }
                    }
                }
            }
            ExprentType::Field => {
                self.get_left()
                    .process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
                var_maps.assert_is_normal();
                self.get_right().process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.to_normal();
                var_maps.get_normal().borrow_mut().remove_all_fields();
            }
            _ => {
                self.get_left()
                    .process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
                var_maps.assert_is_normal();
                self.get_right().process_sforms(
                    s_forms_constructor,
                    var_maps,
                    stat,
                    calc_live_vars,
                );
                var_maps.to_normal();
            }
        }
    }

    pub fn get_left(self) -> Exprent {
        self.d(|d| d.left)
    }

    pub fn get_right(self) -> Exprent {
        self.d(|d| d.right)
    }

    pub fn set_right(self, right: Exprent) {
        self.d_mut(|d| d.right = right)
    }

    pub fn get_cond_type(self) -> Option<FunctionType> {
        self.d(|d| d.cond_type)
    }

    pub fn set_cond_type(self, cond_type: Option<FunctionType>) {
        self.d_mut(|d| d.cond_type = cond_type)
    }
}
