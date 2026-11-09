//! Port of `org/vineflower/kotlin/struct/DefaultArgsMap.java`.

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::ACC_STATIC;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::jthrow;
use crate::k_constructor::KConstructor;
use crate::k_element::KElement;
use crate::k_parameter::KParameter;
use crate::k_utils::KUtils;
use crate::kotlin_options::KotlinOptions;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::method_wrapper::MethodWrapper;
use crate::rt::JHashMap;
use crate::struct_ann_default_attribute::StructAnnDefaultAttribute;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

/// `Function<ClassWrapper, DefaultArgsMap>`: built at write time, in the class scope.
pub type DefaultArgsSupplier = Rc<dyn Fn(ClassWrapper) -> DefaultArgsMap>;

pub struct DefaultArgsMap {
    map: JHashMap<KParameter, Exprent>,
    default_method: Option<StructMethod>,
}

impl DefaultArgsMap {
    fn new(
        map: JHashMap<KParameter, Exprent>,
        default_method: Option<StructMethod>,
    ) -> DefaultArgsMap {
        DefaultArgsMap {
            map,
            default_method,
        }
    }

    /// `calling` is only read when `defaults` is present.
    pub fn from(
        defaults: Option<MethodWrapper>,
        calling: Option<MethodWrapper>,
        params: &[KParameter],
    ) -> DefaultArgsMap {
        let Some(defaults) = defaults else {
            return DefaultArgsMap::new(JHashMap::new(), None);
        };
        let calling = calling.expect("NullPointerException: calling");

        let mut map: JHashMap<KParameter, Exprent> = JHashMap::new();

        let mut start_of_bitmasks: i32 = 0;

        if !calling.with(|d| d.method_struct).has_modifier(ACC_STATIC) {
            start_of_bitmasks += 1;
        }

        for var in calling.desc().params.iter() {
            start_of_bitmasks = start_of_bitmasks.wrapping_add(var.stack_size);
        }

        let graph = defaults
            .get_or_build_graph()
            .expect("NullPointerException: graph");
        for node in graph.nodes() {
            let statement = node.statement();
            let Some(if_statement) = statement.try_as_if() else {
                continue;
            };
            let condition = if_statement.get_headexprent().get_condition();
            let Some(function) = condition.try_as_function() else {
                continue;
            };
            let Some(bitmask) = function.get_lst_operands().get(0).try_as_function() else {
                continue;
            };
            if bitmask.get_lst_operands().len() != 2 {
                continue;
            }

            let check = bitmask.get_lst_operands().get(0);
            let mask = bitmask.get_lst_operands().get(1);

            let Some(var) = check.try_as_var() else {
                continue;
            };
            if !mask.is_const() {
                continue;
            }

            let mask_value = mask.as_const().get_int_value();
            let mut mask_index = var
                .get_index()
                .wrapping_sub(start_of_bitmasks)
                .wrapping_mul(32);
            for i in 0..32 {
                if (mask_value & 1i32.wrapping_shl(i)) != 0 {
                    mask_index = mask_index.wrapping_add(i as i32);
                    break;
                }
            }

            let expr = if_statement
                .get_ifstat()
                .expect("NullPointerException: ifstat")
                .get_exprents()
                .expect("NullPointerException: exprents")
                .get(0);
            if let Some(assignment) = expr.try_as_assignment() {
                let right = assignment.get_right().copy();
                DefaultArgsMap::update_exprent(right, calling);
                if mask_index < 0 || mask_index as usize >= params.len() {
                    jthrow!(ArrayIndexOutOfBoundsException; "Index {} out of bounds for length {}", mask_index, params.len());
                }
                let param = params[mask_index as usize].clone();
                map.put(param, right);
            }
        }

        if KUtils::assertions_enabled() {
            for param in params {
                if map.contains_key(param) != Flags::DECLARES_DEFAULT_VALUE.get(param.flags) {
                    jthrow!(AssertionError; "Parameter {} has default value but no default value was found", param.name);
                }
            }
        }

        DefaultArgsMap::new(map, Some(defaults.with(|d| d.method_struct)))
    }

    pub fn from_annotation(cl: StructClass) -> DefaultArgsMap {
        let Some(primary_constructor) = KtAttrs::get(cl, &KConstructor::PRIMARY_CONSTRUCTOR())
        else {
            return DefaultArgsMap::new(JHashMap::new(), None);
        };

        let mut defaults_by_name: JHashMap<String, StructAnnDefaultAttribute> = JHashMap::new();
        for mt in cl.get_methods().to_vec() {
            let Some(KElement::Property(property)) = KtAttrs::get(mt, &KElement::KEY()) else {
                continue;
            };
            if !mt.has_attribute(StructGeneralAttribute::ATTRIBUTE_ANNOTATION_DEFAULT) {
                continue;
            }
            let attribute = mt
                .get_attribute(StructGeneralAttribute::ATTRIBUTE_ANNOTATION_DEFAULT)
                .expect("AnnotationDefault");
            if defaults_by_name.contains_key(&property.name) {
                jthrow!(IllegalStateException; "Duplicate key {}", property.name);
            }
            defaults_by_name.put(property.name.clone(), attribute);
        }

        let mut defaults: JHashMap<KParameter, Exprent> = JHashMap::new();
        for param in primary_constructor.parameters.iter() {
            let Some(attr) = defaults_by_name.get_cloned(&param.name) else {
                continue;
            };

            let kexpr = KUtils::replace_exprent(
                attr.get_default_value()
                    .expect("NullPointerException: defaultValue"),
            );
            let expr = match kexpr {
                Some(kexpr) => kexpr,
                None => attr
                    .get_default_value()
                    .expect("NullPointerException: defaultValue"),
            };
            if let Some(new_expr) = expr.try_as_k_new() {
                new_expr.set_in_annotation(true);
            }

            defaults.put(param.clone(), expr);
        }

        DefaultArgsMap::new(defaults, None)
    }

    pub fn to_java(&self, parameter: &KParameter, indent: i32) -> TextBuffer {
        let mut buffer = TextBuffer::new();
        let arg_string = DecompilerContext::get_property(KotlinOptions::UNKNOWN_DEFAULT_ARG_STRING)
            .expect("NullPointerException: kt-unknown-defaults");

        let Some(expr) = self.map.get_cloned(parameter) else {
            if !arg_string.is_empty() {
                buffer.append(" = ").append(&arg_string);
            }

            return buffer;
        };

        buffer.append(" = ").append_buffer(&expr.to_java(indent));

        buffer
    }

    fn update_exprent(expr: Exprent, calling: MethodWrapper) {
        if let Some(var_expr) = expr.try_as_var() {
            let (method_struct, varproc) = calling.with(|d| (d.method_struct, d.varproc));
            let attr = method_struct.get_local_variable_attr();
            if attr.is_some() {
                varproc.find_lvt(var_expr, 0);
            }
        }

        for child in expr.get_all_exprents() {
            DefaultArgsMap::update_exprent(child, calling);
        }
    }

    pub fn get_default_method(&self) -> Option<StructMethod> {
        self.default_method
    }
}
