//! Port of `org/jetbrains/java/decompiler/modules/decompiler/exps/FieldExprent.java`.

use crate::exprent::{Exprent, ExprentKind};
use crate::field_descriptor::FieldDescriptor;
use crate::link_constant::LinkConstant;
use crate::match_engine::MatchEngine;
use crate::match_node::MatchNode;
use crate::rt::{BitSet, JavaEq, JavaHash};
use crate::s_forms_constructor::SFormsConstructor;
use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::var_map_holder::VarMapHolder;
use crate::var_type::VarType;
use crate::{java_subclass, unported};

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
        unported!("FieldExprent.getInferredExprType")
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

    #[allow(dead_code)]
    fn is_ambiguous(self) -> bool {
        unported!("FieldExprent.isAmbiguous")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("FieldExprent.toJava")
    }

    fn use_qualified_static(self) -> bool {
        unported!("FieldExprent.useQualifiedStatic")
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
        _s_forms_constructor: &mut SFormsConstructor,
        _var_maps: &mut VarMapHolder,
        _stat: Statement,
        _calc_live_vars: bool,
    ) {
        unported!("FieldExprent.processSforms")
    }

    /// Java: `match(MatchNode, MatchEngine)`.
    pub fn match_(self, _match_node: &MatchNode, _engine: &MatchEngine) -> bool {
        unported!("FieldExprent.match")
    }
}
