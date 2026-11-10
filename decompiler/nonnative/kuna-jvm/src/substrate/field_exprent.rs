//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/FieldExprent.java`.
//!
//! kuna fix `kt_unescaped_name`: in Kotlin output, the field's name is
//! written as the Kotlin writer writes a name (`kuna_fix_kt_unescaped_name`).

use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::expr_util::ExprUtil;
use crate::exprent::{Exprent, ExprentKind};
use crate::field_descriptor::FieldDescriptor;
use crate::i_matchable::MatchProperties;
use crate::java_subclass;
use crate::kotlin::kuna_fix_kt_unescaped_name;
use crate::kuna_fixes::{self, KunaFix};
use crate::link_constant::LinkConstant;
use crate::match_engine::MatchEngine;
use crate::match_node::{MatchNode, MatchValue};
use crate::rt::{BitSet, JHashMap, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::text_buffer::TextBuffer;
use crate::text_util::TextUtil;
use crate::var_exprent::VarExprent;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;

pub struct FieldExprentData {
    pub name: String,
    pub classname: String,
    pub is_static: bool,
    /// `None` for a static field.
    pub instance: Option<Exprent>,
    pub descriptor: FieldDescriptor,
    pub force_qualified: bool,
    pub is_qualifier: bool,
    pub was_condy: bool,
}

java_subclass!(pub FieldExprent: Exprent, ExprentKind::Field(FieldExprentData),
               is_field, as_field, try_as_field);

impl JavaEq for FieldExprent {
    fn java_equals(&self, other: &Self) -> bool {
        self.0.equals(other.0)
    }
}

impl JavaHash for FieldExprent {
    fn java_hash(&self) -> i32 {
        self.0.hash_code()
    }
}

/// `DecompilerContext.getStructContext().getClass(name)`, which is null for a null name.
fn struct_class(name: Option<&str>) -> Option<StructClass> {
    name.and_then(|name| DecompilerContext::get_struct_context().get_class(name))
}

impl FieldExprent {
    /// Java: `FieldExprent(LinkConstant, Exprent, BitSet)`.
    pub fn new_link(
        cn: &LinkConstant,
        instance: Option<Exprent>,
        bytecode_offsets: Option<&BitSet>,
    ) -> FieldExprent {
        FieldExprent::new(
            cn.elementname.clone(),
            cn.classname
                .clone()
                .expect("NullPointerException: classname"),
            instance.is_none(),
            instance,
            FieldDescriptor::parse_descriptor(&cn.descriptor),
            bytecode_offsets,
        )
    }

    /// Java: `FieldExprent(String, String, boolean, Exprent, FieldDescriptor, BitSet)`.
    pub fn new(
        name: String,
        classname: String,
        is_static: bool,
        instance: Option<Exprent>,
        descriptor: FieldDescriptor,
        bytecode_offsets: Option<&BitSet>,
    ) -> FieldExprent {
        FieldExprent::new_full(
            name,
            classname,
            is_static,
            instance,
            descriptor,
            bytecode_offsets,
            false,
            false,
        )
    }

    /// Java: `FieldExprent(String, String, boolean, Exprent, FieldDescriptor, BitSet, boolean, boolean)`.
    #[allow(clippy::too_many_arguments)]
    pub fn new_full(
        name: String,
        classname: String,
        is_static: bool,
        instance: Option<Exprent>,
        descriptor: FieldDescriptor,
        bytecode_offsets: Option<&BitSet>,
        force_qualified: bool,
        was_condy: bool,
    ) -> FieldExprent {
        let id = Exprent::next_id();
        let e = Exprent::alloc_with_id(
            id,
            ExprentKind::Field(FieldExprentData {
                name,
                classname,
                is_static,
                instance,
                descriptor,
                force_qualified,
                is_qualifier: false,
                was_condy,
            }),
        );
        e.add_bytecode_offsets(bytecode_offsets);
        FieldExprent(e)
    }

    pub fn get_expr_type(self) -> VarType {
        self.d(|d| d.descriptor.type_.clone())
    }

    pub fn get_inferred_expr_type(self, _upper_bound: Option<&VarType>) -> VarType {
        let (name, classname, instance, descriptor) = self.d(|d| {
            (
                d.name.clone(),
                d.classname.clone(),
                d.instance,
                d.descriptor.clone(),
            )
        });
        let mut cl = struct_class(Some(&classname));
        let types = cl.map(|c| c.get_all_generics());

        let mut ft = None;
        while let Some(c) = cl {
            ft = c.get_field(&name, &descriptor.descriptor_string);
            if ft.is_some() {
                break;
            }
            cl = match c.super_class() {
                None => None,
                Some(super_class) => struct_class(super_class.get_string_opt().as_deref()),
            };
        }

        if let Some(ft) = ft {
            if let Some(signature) = ft.get_signature() {
                let c = cl.expect("NullPointerException: cl");
                signature.verify_type(c.get_signature().as_ref(), &descriptor.type_);

                let class_generics = types.and_then(|t| t.get_cloned(&c.qualified_name()));
                let mut ret = match &class_generics {
                    Some(map) => signature.type_().remap(&**map),
                    None => signature
                        .type_()
                        .remap(&JHashMap::<VarType, VarType>::new()),
                };

                if let Some(instance) = instance {
                    if c.get_signature().is_some() {
                        let inst_type = instance.get_inferred_expr_type(None);

                        if inst_type.is_generic() && inst_type.type_ != CodeType::Genvar {
                            let ginstance = inst_type.as_generic();

                            let cl2 = struct_class(inst_type.value.as_deref());
                            if let Some(sig) = cl2.and_then(|c2| c2.get_signature()) {
                                let mut temp_map: JHashMap<VarType, Option<VarType>> =
                                    JHashMap::new();
                                sig.generic_type()
                                    .map_gen_vars_to(&ginstance, &mut temp_map);
                                let new = ret
                                    .as_ref()
                                    .expect("NullPointerException: ret")
                                    .remap(&temp_map);

                                if new.is_some() {
                                    ret = new;
                                } else {
                                    ret = Some(descriptor.type_.clone());
                                }
                            }
                        }
                    }
                }

                return ret.expect("FieldExprent.getInferredExprType: null type");
            }
        }

        descriptor.type_.clone()
    }

    pub fn get_exprent_use(self) -> i32 {
        match self.get_instance() {
            None => Exprent::MULTIPLE_USES,
            Some(instance) => instance.get_exprent_use() & Exprent::MULTIPLE_USES,
        }
    }

    pub fn get_all_exprents_list(self, lst: &mut Vec<Exprent>) {
        self.d(|d| {
            if let Some(instance) = d.instance {
                lst.push(instance);
            }
        });
    }

    pub fn copy(self) -> Exprent {
        let (name, classname, is_static, instance, descriptor, force_qualified, was_condy) = self
            .d(|d| {
                (
                    d.name.clone(),
                    d.classname.clone(),
                    d.is_static,
                    d.instance,
                    d.descriptor.clone(),
                    d.force_qualified,
                    d.was_condy,
                )
            });
        let bytecode = self.get_bytecode();
        FieldExprent::new_full(
            name,
            classname,
            is_static,
            instance.map(|i| i.copy()),
            descriptor,
            bytecode.as_ref(),
            force_qualified,
            was_condy,
        )
        .0
    }

    fn is_ambiguous(self) -> bool {
        if let Some(method) = DecompilerContext::get_current_method_wrapper() {
            let method_struct = method.with(|d| d.method_struct);
            if let Some(attr) = method_struct.get_local_variable_attr() {
                let name = self.get_name();
                return attr.contains_name(&name);
            }
        }

        false
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        self.to_java_condy_comment(indent, true)
    }

    /// Java (`develop/1.13.0`): `toJava(int, boolean includeCondyComment)`.
    pub fn to_java_condy_comment(self, indent: i32, include_condy_comment: bool) -> TextBuffer {
        let mut buf = TextBuffer::new();
        let (name, classname, is_static, instance, descriptor, is_qualifier, was_condy) =
            self.d(|d| {
                (
                    d.name.clone(),
                    d.classname.clone(),
                    d.is_static,
                    d.instance,
                    d.descriptor.clone(),
                    d.is_qualifier,
                    d.was_condy,
                )
            });

        if was_condy && include_condy_comment {
            buf.append("/* $VF: constant dynamic */ ");
        }

        if is_static {
            let primitive_types = ExprUtil::PRIMITIVE_TYPES();
            if name == "TYPE" && primitive_types.contains_key(&classname) {
                buf.append(primitive_types.get(&classname).expect("PRIMITIVE_TYPES"));
                buf.append(".class");
                return buf;
            }
            if self.use_qualified_static() {
                let short_name = DecompilerContext::get_import_collector()
                    .get_short_name_in_class_context(&ExprProcessor::build_java_class_name(
                        &classname,
                    ));
                buf.append_all_classes(&short_name, Some(&classname));
                buf.append(".");
            }
        } else {
            let mut super_qualifier: Option<String> = None;

            if let Some(inst_var) = instance.and_then(|i| i.try_as_var()) {
                let pair = inst_var.get_var_version_pair();

                if let Some(current_method) = DecompilerContext::get_current_method_wrapper() {
                    let varproc = current_method.with(|d| d.varproc);
                    let this_classname = varproc.get_this_vars().borrow().get_cloned(&pair);

                    if let Some(this_classname) = this_classname {
                        if classname != this_classname {
                            super_qualifier = Some(this_classname);
                        }
                    }
                }
            }

            if let Some(super_qualifier) = &super_qualifier {
                TextUtil::write_qualified_super(&mut buf, super_qualifier);
            } else {
                if !is_qualifier {
                    buf.push_newline_group(indent, 1);
                }
                if let Some(instance) = instance {
                    instance.set_is_qualifier();
                }
                let mut buff = TextBuffer::new();
                let instance = instance.expect("NullPointerException: instance");
                let casted = ExprProcessor::get_casted_exprent(
                    instance,
                    &VarType::new_value(CodeType::Object, 0, classname.as_str()),
                    &mut buff,
                    indent,
                    true,
                );

                if casted || instance.get_precedence() > self.0.get_precedence() {
                    buff.enclose_with_parens();
                }

                buf.append_buffer(&buff);
                if instance.allow_newline_after_qualifier() {
                    buf.append_possible_newline();
                }
                if !is_qualifier {
                    buf.pop_newline_group();
                }
            }

            if buf.content_equals(VarExprent::VAR_NAMELESS_ENCLOSURE) {
                buf.set_length(0);
            } else {
                buf.append(".");
            }
        }

        let bytecode = self.get_bytecode();
        buf.add_bytecode_mapping(bytecode.as_ref());

        if kuna_fixes::enabled(KunaFix::KtUnescapedName) {
            let printed = kuna_fix_kt_unescaped_name::member(&name);
            buf.append_field_descriptor(&printed, false, &classname, &name, &descriptor);
        } else {
            buf.append_field_descriptor(&name, false, &classname, &name, &descriptor);
        }

        buf
    }

    fn use_qualified_static(self) -> bool {
        let node = DecompilerContext::get_current_class_node();
        let (classname, force_qualified) = self.d(|d| (d.classname.clone(), d.force_qualified));
        node.is_none_or(|node| classname != node.with(|d| d.class_struct).qualified_name())
            || self.is_ambiguous()
            || force_qualified
    }

    pub fn replace_exprent(self, old_expr: Exprent, new_expr: Exprent) {
        self.d_mut(|d| {
            if Some(old_expr) == d.instance {
                d.instance = Some(new_expr);
            }
        });
    }

    pub fn equals(self, o: Exprent) -> bool {
        if o == self.0 {
            return true;
        }
        let Some(ft) = o.try_as_field() else {
            return false;
        };
        let (name, classname, is_static, instance, descriptor) = self.d(|d| {
            (
                d.name.clone(),
                d.classname.clone(),
                d.is_static,
                d.instance,
                d.descriptor.clone(),
            )
        });
        name == ft.get_name()
            && classname == ft.get_classname()
            && is_static == ft.is_static()
            && instance.java_equals(&ft.get_instance())
            && descriptor.equals(&ft.get_descriptor())
    }

    pub fn get_classname(self) -> String {
        self.d(|d| d.classname.clone())
    }

    pub fn get_descriptor(self) -> FieldDescriptor {
        self.d(|d| d.descriptor.clone())
    }

    pub fn get_instance(self) -> Option<Exprent> {
        self.d(|d| d.instance)
    }

    pub fn is_was_condy(self) -> bool {
        self.d(|d| d.was_condy)
    }

    pub fn is_static(self) -> bool {
        self.d(|d| d.is_static)
    }

    pub fn get_name(self) -> String {
        self.d(|d| d.name.clone())
    }

    /// Java: `forceQualified(boolean)`.
    pub fn force_qualified(self, value: bool) {
        self.d_mut(|d| d.force_qualified = value)
    }

    pub fn get_bytecode_range(self, mut values: Option<&mut BitSet>) {
        let instance = self.get_instance();
        Exprent::measure_bytecode_exprent(values.as_deref_mut(), instance);
        self.measure_bytecode(values);
    }

    pub fn set_is_qualifier(self) {
        self.d_mut(|d| d.is_qualifier = true)
    }

    pub fn allow_newline_after_qualifier(self) -> bool {
        if self.is_static() && !self.use_qualified_static() {
            return false;
        }
        Exprent::allow_newline_after_qualifier_base(self.0)
    }

    pub fn process_sforms(
        self,
        s_forms_constructor: &mut SFormsConstructor,
        var_maps: &mut VarMapHolder,
        stat: Statement,
        calc_live_vars: bool,
    ) {
        if let Some(instance) = self.get_instance() {
            instance.process_sforms(s_forms_constructor, var_maps, stat, calc_live_vars);
        }
        s_forms_constructor.field_read(self, &var_maps.get_normal());
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, match_node: &MatchNode, engine: &MatchEngine) -> bool {
        if !Exprent::match_base(self.0, match_node, engine) {
            return false;
        }

        if let Some(rule) = match_node.get_raw_rule(MatchProperties::ExprentName) {
            let value = rule
                .value
                .as_ref()
                .expect("NullPointerException: rule value");
            if rule.is_variable() {
                return engine.check_and_set_variable_value(
                    &value.as_string(),
                    Some(MatchValue::String(self.get_name())),
                );
            } else {
                return value.equals(&MatchValue::String(self.get_name()));
            }
        }

        true
    }
}
