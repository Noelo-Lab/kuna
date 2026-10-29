//! (kuna `swiftdemangle`) The name-only printer: the dotted Swift
//! reference path kuna installs as a symbol name (`Swift.String.count.getter`,
//! `Toyota.Foo.init(x:y:)`, `<Toyota.Foo:Swift.Hashable>.hash(into:)`,
//! `Toyota.Foo.type_metadata`). It never contains a space; types appear only
//! where the name is meaningless without them (conformances, specializations,
//! thunks), in a compact spelling.

use super::node::{Kind, Node, NodeRef};

/// Render `node` as a name-only path, or `None` when a node kind has no
/// name-only rendering.
pub fn print_name(node: &NodeRef) -> Option<String> {
    let mut p = NamePrinter {
        out: String::new(),
        depth: 0,
    };
    p.global(node)?;
    if p.out.is_empty() || p.out.contains(' ') {
        return None;
    }
    Some(p.out)
}

struct NamePrinter {
    out: String,
    depth: usize,
}

fn tag(kind: Kind) -> Option<&'static str> {
    use Kind::*;
    Some(match kind {
        TypeMetadata => "type_metadata",
        TypeMetadataAccessFunction => "type_metadata_accessor",
        TypeMetadataCompletionFunction => "type_metadata_completion_function",
        TypeMetadataInstantiationFunction => "type_metadata_instantiation_function",
        TypeMetadataInstantiationCache => "type_metadata_instantiation_cache",
        TypeMetadataSingletonInitializationCache => "type_metadata_singleton_initialization_cache",
        TypeMetadataLazyCache => "type_metadata_lazy_cache",
        TypeMetadataDemanglingCache => "type_metadata_demangling_cache",
        FullTypeMetadata => "full_type_metadata",
        GenericTypeMetadataPattern => "generic_type_metadata_pattern",
        NominalTypeDescriptor => "nominal_type_descriptor",
        NominalTypeDescriptorRecord => "nominal_type_descriptor_record",
        Metaclass => "metaclass",
        ClassMetadataBaseOffset => "class_metadata_base_offset",
        MethodLookupFunction => "method_lookup_function",
        ObjCMetadataUpdateFunction => "objc_metadata_update_function",
        ObjCResilientClassStub => "objc_resilient_class_stub",
        FullObjCResilientClassStub => "full_objc_resilient_class_stub",
        ModuleDescriptor => "module_descriptor",
        ExtensionDescriptor => "extension_descriptor",
        AnonymousDescriptor => "anonymous_descriptor",
        ProtocolDescriptor => "protocol_descriptor",
        ProtocolDescriptorRecord => "protocol_descriptor_record",
        ProtocolRequirementsBaseDescriptor => "protocol_requirements_base_descriptor",
        ProtocolSelfConformanceDescriptor => "protocol_self_conformance_descriptor",
        ProtocolSelfConformanceWitnessTable => "protocol_self_conformance_witness_table",
        ProtocolSelfConformanceWitness => "protocol_self_conformance_witness",
        ProtocolConformanceDescriptor => "protocol_conformance_descriptor",
        ProtocolConformanceDescriptorRecord => "protocol_conformance_descriptor_record",
        ProtocolWitnessTable => "protocol_witness_table",
        ProtocolWitnessTableAccessor => "protocol_witness_table_accessor",
        ProtocolWitnessTablePattern => "protocol_witness_table_pattern",
        GenericProtocolWitnessTable => "generic_protocol_witness_table",
        GenericProtocolWitnessTableInstantiationFunction => {
            "generic_protocol_witness_table_instantiation_function"
        }
        ResilientProtocolWitnessTable => "resilient_protocol_witness_table",
        ValueWitnessTable => "value_witness_table",
        PropertyDescriptor => "property_descriptor",
        MethodDescriptor => "method_descriptor",
        DispatchThunk => "dispatch_thunk",
        CurryThunk => "curry_thunk",
        EnumCase => "enum_case",
        CanonicalSpecializedGenericMetaclass => "specialized_generic_metaclass",
        CanonicalSpecializedGenericTypeMetadataAccessFunction => {
            "specialized_generic_type_metadata_accessor"
        }
        MetadataInstantiationCache => "metadata_instantiation_cache",
        NoncanonicalSpecializedGenericTypeMetadata => {
            "noncanonical_specialized_generic_type_metadata"
        }
        NoncanonicalSpecializedGenericTypeMetadataCache => {
            "noncanonical_specialized_generic_type_metadata_cache"
        }
        CanonicalPrespecializedGenericTypeCachingOnceToken => {
            "specialized_generic_type_metadata_once_token"
        }
        OpaqueTypeDescriptor => "opaque_type_descriptor",
        OpaqueTypeDescriptorAccessor => "opaque_type_descriptor_accessor",
        OpaqueTypeDescriptorAccessorImpl => "opaque_type_descriptor_accessor_impl",
        OpaqueTypeDescriptorAccessorKey => "opaque_type_descriptor_accessor_key",
        OpaqueTypeDescriptorAccessorVar => "opaque_type_descriptor_accessor_var",
        OpaqueTypeDescriptorRecord => "opaque_type_descriptor_record",
        ReflectionMetadataBuiltinDescriptor => "reflection_builtin_descriptor",
        ReflectionMetadataFieldDescriptor => "reflection_field_descriptor",
        ReflectionMetadataAssocTypeDescriptor => "reflection_associated_type_descriptor",
        ReflectionMetadataSuperclassDescriptor => "reflection_superclass_descriptor",
        OutlinedCopy => "outlined_copy",
        OutlinedConsume => "outlined_consume",
        OutlinedRetain => "outlined_retain",
        OutlinedRelease => "outlined_release",
        OutlinedInitializeWithTake | OutlinedInitializeWithTakeNoValueWitness => {
            "outlined_init_with_take"
        }
        OutlinedInitializeWithCopy | OutlinedInitializeWithCopyNoValueWitness => {
            "outlined_init_with_copy"
        }
        OutlinedAssignWithTake | OutlinedAssignWithTakeNoValueWitness => {
            "outlined_assign_with_take"
        }
        OutlinedAssignWithCopy | OutlinedAssignWithCopyNoValueWitness => {
            "outlined_assign_with_copy"
        }
        OutlinedDestroy | OutlinedDestroyNoValueWitness => "outlined_destroy",
        OutlinedEnumTagStore => "outlined_enum_tag_store",
        OutlinedEnumProjectDataForLoad => "outlined_enum_project_data_for_load",
        OutlinedEnumGetTag => "outlined_enum_get_tag",
        GlobalVariableOnceFunction => "once_function",
        GlobalVariableOnceToken => "once_token",
        KeyPathGetterThunkHelper => "keypath_getter",
        KeyPathSetterThunkHelper => "keypath_setter",
        KeyPathEqualsThunkHelper => "keypath_equals",
        KeyPathHashThunkHelper => "keypath_hash",
        KeyPathUnappliedMethodThunkHelper => "keypath_unapplied_method",
        KeyPathAppliedMethodThunkHelper => "keypath_applied_method",
        AssociatedTypeDescriptor => "associated_type_descriptor",
        AssociatedConformanceDescriptor => "associated_conformance_descriptor",
        DefaultAssociatedConformanceAccessor => "default_associated_conformance_accessor",
        DefaultAssociatedTypeMetadataAccessor => "default_associated_type_metadata_accessor",
        BaseConformanceDescriptor => "base_conformance_descriptor",
        AssociatedTypeMetadataAccessor => "associated_type_metadata_accessor",
        AssociatedTypeWitnessTableAccessor => "associated_type_witness_table_accessor",
        BaseWitnessTableAccessor => "base_witness_table_accessor",
        LazyProtocolWitnessTableAccessor => "lazy_protocol_witness_table_accessor",
        LazyProtocolWitnessTableCacheVariable => "lazy_protocol_witness_table_cache",
        FieldOffset => "field_offset",
        VTableThunk => "vtable_thunk",
        _ => return None,
    })
}

/// The suffix tag of a function attribute that wraps the whole name.
fn attr_tag(kind: Kind) -> Option<&'static str> {
    use Kind::*;
    Some(match kind {
        ObjCAttribute => "objc",
        NonObjCAttribute => "nonobjc",
        DynamicAttribute => "dynamic",
        DirectMethodReferenceAttribute => "super",
        VTableAttribute => "override",
        MergedFunction => "merged",
        PartialApplyForwarder => "partial_apply",
        PartialApplyObjCForwarder => "partial_apply_objc",
        AsyncFunctionPointer => "async_function_pointer",
        CoroFunctionPointer => "coro_function_pointer",
        DefaultOverride => "default_override",
        DynamicallyReplaceableFunctionImpl => "dynamically_replaceable_impl",
        DynamicallyReplaceableFunctionKey => "dynamically_replaceable_key",
        DynamicallyReplaceableFunctionVar => "dynamically_replaceable_var",
        BackDeploymentThunk => "back_deployment_thunk",
        BackDeploymentFallback => "back_deployment_fallback",
        HasSymbolQuery => "has_symbol_query",
        DistributedThunk => "distributed_thunk",
        DistributedAccessor => "distributed_accessor",
        AccessibleFunctionRecord => "accessible_function_record",
        FunctionSignatureSpecialization => "function_signature_specialization",
        GenericSpecialization | GenericSpecializationInResilienceDomain => "generic_specialization",
        GenericSpecializationNotReAbstracted => "generic_specialization",
        GenericSpecializationPrespecialized => "generic_prespecialization",
        InlinedGenericFunction => "inlined_generic_function",
        GenericPartialSpecialization | GenericPartialSpecializationNotReAbstracted => {
            "generic_partial_specialization"
        }
        _ => return None,
    })
}

impl NamePrinter {
    fn s(&mut self, t: &str) {
        self.out.push_str(t);
    }

    fn global(&mut self, n: &NodeRef) -> Option<()> {
        if n.kind != Kind::Global {
            return None;
        }
        // Attributes come first in the tree and wrap what follows; they
        // render as trailing tags so the entity path stays in front.
        let mut tags: Vec<String> = Vec::new();
        let mut body: Vec<NodeRef> = Vec::new();
        let mut suffix: Option<String> = None;
        for c in &n.children {
            match c.kind {
                Kind::Suffix => suffix = Some(c.get_text().to_string()),
                Kind::PartialApplyForwarder | Kind::PartialApplyObjCForwarder => {
                    let inner = Node::with(Kind::Global, c.children.clone());
                    let mut p = NamePrinter {
                        out: String::new(),
                        depth: self.depth + 1,
                    };
                    p.global(&inner)?;
                    self.s(&p.out);
                    self.s(".");
                    self.s(attr_tag(c.kind)?);
                    for t in tags.iter().rev() {
                        self.s(".");
                        self.s(t);
                    }
                    if let Some(sfx) = &suffix {
                        self.s(sfx);
                    }
                    return Some(());
                }
                Kind::AsyncAwaitResumePartialFunction => {
                    tags.push(format!("await_resume_{}", c.first()?.get_index()));
                }
                Kind::AsyncSuspendResumePartialFunction => {
                    tags.push(format!("suspend_resume_{}", c.first()?.get_index()));
                }
                Kind::OutlinedVariable => tags.push(format!("outlined_variable_{}", c.get_index())),
                Kind::OutlinedReadOnlyObject => {
                    tags.push(format!("outlined_read_only_object_{}", c.get_index()))
                }
                Kind::GenericSpecialization
                | Kind::GenericSpecializationInResilienceDomain
                | Kind::GenericSpecializationNotReAbstracted
                | Kind::GenericSpecializationPrespecialized
                | Kind::InlinedGenericFunction => {
                    let mut t = attr_tag(c.kind)?.to_string();
                    let params: Vec<&NodeRef> = c
                        .children
                        .iter()
                        .filter(|p| p.kind == Kind::GenericSpecializationParam)
                        .collect();
                    if !params.is_empty() {
                        t.push('<');
                        for (i, p) in params.iter().enumerate() {
                            if i > 0 {
                                t.push(',');
                            }
                            t.push_str(&self.type_name(p.first()?)?);
                        }
                        t.push('>');
                    }
                    tags.push(t);
                }
                k if k.is_function_attr() => tags.push(attr_tag(k)?.to_string()),
                _ => body.push(c.clone()),
            }
        }
        if body.len() != 1 {
            return None;
        }
        self.node(&body[0])?;
        // Tags were collected outermost-first; the innermost applies first.
        for t in tags.iter().rev() {
            self.s(".");
            self.s(t);
        }
        if let Some(sfx) = suffix {
            self.s(&sfx);
        }
        Some(())
    }

    /// The context path of an entity: a module, a nominal type, an
    /// extension (rendered as the extended type), or an enclosing entity.
    fn context(&mut self, n: &NodeRef) -> Option<()> {
        match n.kind {
            Kind::Module => {
                self.s(n.get_text());
                Some(())
            }
            Kind::Extension => self.context(n.child(1)?),
            Kind::Type => self.context(n.first()?),
            _ => self.node(n),
        }
    }

    fn decl_name(&mut self, n: &NodeRef) -> Option<()> {
        match n.kind {
            Kind::Identifier => {
                // A punycoded C++ template name can carry spaces.
                let t = n.get_text().replace(' ', "");
                self.s(&t);
                Some(())
            }
            Kind::PrivateDeclName => {
                if n.len() > 1 {
                    self.decl_name(n.child(1)?)
                } else {
                    None
                }
            }
            Kind::LocalDeclName => {
                self.decl_name(n.child(1)?)?;
                self.s(&format!("#{}", n.first()?.get_index().saturating_add(1)));
                Some(())
            }
            Kind::PrefixOperator | Kind::PostfixOperator | Kind::InfixOperator => {
                self.s(n.get_text());
                Some(())
            }
            _ => None,
        }
    }

    /// `(a:b:)` for an entity whose type is a function type; `()` for no
    /// parameters.
    fn labels(&mut self, entity: &Node) -> Option<()> {
        let labels = entity.child_of_kind(Kind::LabelList);
        let ty = entity.child_of_kind(Kind::Type)?;
        let mut func = ty.first()?.clone();
        while func.kind == Kind::DependentGenericType {
            func = func.child(1)?.first()?.clone();
        }
        let params = match func.kind {
            Kind::FunctionType | Kind::NoEscapeFunctionType => func
                .child_of_kind(Kind::ArgumentTuple)?
                .first()?
                .first()?
                .clone(),
            _ => return None,
        };
        let count = if params.kind == Kind::Tuple {
            params.len()
        } else {
            1
        };
        self.s("(");
        for i in 0..count {
            match labels.and_then(|l| l.child(i)) {
                Some(l) if l.kind == Kind::Identifier => self.s(l.get_text()),
                _ => self.s("_"),
            }
            self.s(":");
        }
        self.s(")");
        Some(())
    }

    fn entity(&mut self, n: &NodeRef, name: Option<&str>, with_labels: bool) -> Option<()> {
        let mut n = n.clone();
        if n.kind == Kind::BoundGenericFunction {
            n = n.first()?.clone();
        }
        let ctx = n.first()?.clone();
        let before = self.out.len();
        self.context(&ctx)?;
        if self.out.len() != before {
            self.s(".");
        }
        match name {
            Some(name) => self.s(name),
            None => self.decl_name(n.child(1)?)?,
        }
        if with_labels {
            self.labels(&n)?;
        }
        Some(())
    }

    fn storage(&mut self, n: &NodeRef, accessor: &str) -> Option<()> {
        let storage = n.first()?;
        match storage.kind {
            Kind::Variable => self.entity(storage, None, false)?,
            Kind::Subscript => self.entity(storage, Some("subscript"), true)?,
            _ => return None,
        }
        self.s(".");
        self.s(accessor);
        Some(())
    }

    fn conformance(&mut self, n: &NodeRef) -> Option<()> {
        if n.kind != Kind::ProtocolConformance || n.len() == 4 {
            return None;
        }
        self.s("<");
        let t = self.type_name(n.first()?)?;
        self.s(&t);
        self.s(":");
        let p = self.type_name(n.child(1)?)?;
        self.s(&p);
        self.s(">");
        Some(())
    }

    fn node(&mut self, n: &NodeRef) -> Option<()> {
        if self.depth > 256 {
            return None;
        }
        self.depth += 1;
        let r = self.node_inner(n);
        self.depth -= 1;
        r
    }

    fn node_inner(&mut self, n: &NodeRef) -> Option<()> {
        use Kind::*;
        match n.kind {
            Class | Structure | Enum | Protocol | TypeAlias | OtherNominalType
            | GenericTypeParamDecl => self.entity(n, None, false),
            BoundGenericClass
            | BoundGenericStructure
            | BoundGenericEnum
            | BoundGenericProtocol
            | BoundGenericOtherNominalType
            | BoundGenericTypeAlias => {
                let t = self.type_name(n)?;
                self.s(&t);
                Some(())
            }
            Module => {
                self.s(n.get_text());
                Some(())
            }
            Extension => self.context(n),
            Function | BoundGenericFunction => self.entity(n, None, true),
            Variable => self.entity(n, None, false),
            Subscript => self.entity(n, Some("subscript"), true),
            Getter | GlobalGetter => self.storage(n, "getter"),
            Setter => self.storage(n, "setter"),
            MaterializeForSet => self.storage(n, "materializeForSet"),
            WillSet => self.storage(n, "willset"),
            DidSet => self.storage(n, "didset"),
            ReadAccessor => self.storage(n, "read"),
            Read2Accessor => self.storage(n, "read2"),
            ModifyAccessor => self.storage(n, "modify"),
            Modify2Accessor => self.storage(n, "modify2"),
            InitAccessor => self.storage(n, "init"),
            UnsafeMutableAddressor => self.storage(n, "unsafeMutableAddressor"),
            UnsafeAddressor => self.storage(n, "unsafeAddressor"),
            OwningMutableAddressor => self.storage(n, "owningMutableAddressor"),
            OwningAddressor => self.storage(n, "owningAddressor"),
            NativeOwningMutableAddressor => self.storage(n, "nativeOwningMutableAddressor"),
            NativeOwningAddressor => self.storage(n, "nativeOwningAddressor"),
            NativePinningMutableAddressor => self.storage(n, "nativePinningMutableAddressor"),
            NativePinningAddressor => self.storage(n, "nativePinningAddressor"),
            Allocator => {
                let name = if n.first()?.kind == Class {
                    "__allocating_init"
                } else {
                    "init"
                };
                self.entity(n, Some(name), true)
            }
            Constructor => self.entity(n, Some("init"), true),
            Destructor => self.entity(n, Some("deinit"), false),
            Deallocator => {
                let name = if n.first()?.kind == Class {
                    "__deallocating_deinit"
                } else {
                    "deinit"
                };
                self.entity(n, Some(name), false)
            }
            IsolatedDeallocator => {
                let name = if n.first()?.kind == Class {
                    "__isolated_deallocating_deinit"
                } else {
                    "deinit"
                };
                self.entity(n, Some(name), false)
            }
            IVarInitializer => self.entity(n, Some("__ivar_initializer"), false),
            IVarDestroyer => self.entity(n, Some("__ivar_destroyer"), false),
            Initializer => self.entity(n, Some("initializer"), false),
            PropertyWrapperBackingInitializer => {
                self.entity(n, Some("property_wrapper_backing_initializer"), false)
            }
            PropertyWrapperInitFromProjectedValue => {
                self.entity(n, Some("property_wrapper_init_from_projected_value"), false)
            }
            DefaultArgumentInitializer => {
                let idx = n.child(1)?.get_index();
                self.entity(n, Some(&format!("default_argument#{idx}")), false)
            }
            ExplicitClosure | ImplicitClosure => {
                let idx = n.child(1)?.get_index().saturating_add(1);
                let word = if n.kind == ExplicitClosure {
                    "closure"
                } else {
                    "implicit_closure"
                };
                self.entity(n, Some(&format!("{word}#{idx}")), false)
            }
            Static => self.node(n.first()?),
            ProtocolWitness => {
                self.conformance(n.first()?)?;
                let req = n.child(1)?.clone();
                // The requirement entity, rendered without its own context.
                let mut p = NamePrinter {
                    out: String::new(),
                    depth: self.depth,
                };
                p.node(&req)?;
                let ctx_len = {
                    let mut q = NamePrinter {
                        out: String::new(),
                        depth: self.depth,
                    };
                    q.requirement_context(&req)?;
                    q.out.len()
                };
                let member = p.out.get(ctx_len..)?.to_string();
                self.s(&member);
                Some(())
            }
            ProtocolWitnessTable
            | ProtocolWitnessTableAccessor
            | ProtocolWitnessTablePattern
            | GenericProtocolWitnessTable
            | GenericProtocolWitnessTableInstantiationFunction
            | ResilientProtocolWitnessTable
            | ProtocolConformanceDescriptor
            | ProtocolConformanceDescriptorRecord
            | ReflectionMetadataAssocTypeDescriptor => {
                self.conformance(n.first()?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            LazyProtocolWitnessTableAccessor | LazyProtocolWitnessTableCacheVariable => {
                self.conformance(n.child(1)?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            AssociatedTypeMetadataAccessor => {
                self.conformance(n.first()?)?;
                self.s(".");
                self.decl_name(n.child(1)?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            AssociatedTypeWitnessTableAccessor | BaseWitnessTableAccessor => {
                self.conformance(n.first()?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            AssociatedTypeDescriptor | DefaultAssociatedTypeMetadataAccessor => {
                let name = n.first()?;
                let proto = self.type_name(name.child(1)?)?;
                self.s(&proto);
                self.s(".");
                self.s(name.first()?.get_text());
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            AssociatedConformanceDescriptor
            | DefaultAssociatedConformanceAccessor
            | BaseConformanceDescriptor => {
                let t = self.type_name(n.first()?)?;
                self.s(&t);
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            ValueWitness => {
                let t = self.type_name(n.first()?)?;
                self.s(&t);
                self.s(".");
                self.s(n.get_text());
                self.s("_value_witness");
                Some(())
            }
            FieldOffset => {
                self.node(n.child(1)?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            VTableThunk => {
                self.node(n.child(1)?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            ReabstractionThunk | ReabstractionThunkHelper | ReabstractionThunkHelperWithSelf => {
                let n_types = n.len() - usize::from(n.first()?.kind == DependentGenericSignature);
                let from = self.type_name(n.child(n.len() - 2)?)?;
                let to = self.type_name(n.child(n.len() - 1)?)?;
                let _ = n_types;
                self.s(if n.kind == ReabstractionThunk {
                    "reabstraction_thunk"
                } else {
                    "reabstraction_thunk_helper"
                });
                self.s("<");
                self.s(&from);
                self.s(",");
                self.s(&to);
                self.s(">");
                Some(())
            }
            KeyPathGetterThunkHelper
            | KeyPathSetterThunkHelper
            | KeyPathUnappliedMethodThunkHelper
            | KeyPathAppliedMethodThunkHelper => {
                self.node(n.first()?)?;
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            KeyPathEqualsThunkHelper | KeyPathHashThunkHelper => {
                let t = self.type_name(n.first()?)?;
                self.s(&t);
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            GlobalVariableOnceFunction | GlobalVariableOnceToken => {
                let ctx = n.first()?.clone();
                let before = self.out.len();
                self.context(&ctx)?;
                if self.out.len() != before {
                    self.s(".");
                }
                let list = n.child(1)?;
                for (i, v) in list.children.iter().enumerate() {
                    if i > 0 {
                        self.s("_");
                    }
                    self.decl_name(v)?;
                }
                self.s(".");
                self.s(tag(n.kind)?);
                Some(())
            }
            OpaqueReturnTypeOf => self.node(n.first()?),
            TypeMangling => {
                let t = self.type_name(n.first()?)?;
                self.s(&t);
                Some(())
            }
            Type => self.node(n.first()?),
            AnonymousContext => {
                self.node(n.child(1)?)?;
                self.s(".anonymous_context");
                Some(())
            }
            Identifier | PrivateDeclName | LocalDeclName => self.decl_name(n),
            _ => {
                // The `<marker> for <subject>` family.
                let t = tag(n.kind)?;
                let subject = n.first()?.clone();
                match subject.kind {
                    Type => {
                        let s = self.type_name(&subject)?;
                        self.s(&s);
                    }
                    _ => self.node(&subject)?,
                }
                self.s(".");
                self.s(t);
                Some(())
            }
        }
    }

    /// The context prefix of a requirement entity (the protocol path), so
    /// a witness can print the member relative to the conformance.
    fn requirement_context(&mut self, req: &NodeRef) -> Option<()> {
        let mut e = req.clone();
        loop {
            match e.kind {
                Kind::Static => e = e.first()?.clone(),
                Kind::Getter
                | Kind::Setter
                | Kind::ModifyAccessor
                | Kind::Modify2Accessor
                | Kind::ReadAccessor
                | Kind::Read2Accessor
                | Kind::WillSet
                | Kind::DidSet
                | Kind::MaterializeForSet
                | Kind::GlobalGetter
                | Kind::InitAccessor
                | Kind::UnsafeAddressor
                | Kind::UnsafeMutableAddressor => e = e.first()?.clone(),
                Kind::BoundGenericFunction => e = e.first()?.clone(),
                _ => break,
            }
        }
        let ctx = e.first()?.clone();
        self.context(&ctx)
    }

    /// The compact, space-free spelling of a type.
    fn type_name(&mut self, n: &NodeRef) -> Option<String> {
        if self.depth > 256 {
            return None;
        }
        self.depth += 1;
        let r = self.type_name_inner(n);
        self.depth -= 1;
        r
    }

    fn type_name_inner(&mut self, n: &NodeRef) -> Option<String> {
        use Kind::*;
        Some(match n.kind {
            Type => self.type_name(n.first()?)?,
            Class | Structure | Enum | Protocol | TypeAlias | OtherNominalType => {
                let mut p = NamePrinter {
                    out: String::new(),
                    depth: self.depth,
                };
                p.entity(n, None, false)?;
                p.out
            }
            BoundGenericClass
            | BoundGenericStructure
            | BoundGenericEnum
            | BoundGenericProtocol
            | BoundGenericOtherNominalType
            | BoundGenericTypeAlias => {
                let base = n.first()?;
                let args = n.child(1)?;
                let unbound = base.first()?;
                let module = unbound.first();
                let name = unbound.child(1);
                let swift = module.is_some_and(|m| m.kind == Module && m.get_text() == "Swift");
                let ident =
                    |t: &str| name.is_some_and(|i| i.kind == Identifier && i.get_text() == t);
                if n.len() == 2 && swift {
                    if n.kind == BoundGenericEnum && ident("Optional") && args.len() == 1 {
                        let inner = self.type_name(args.first()?)?;
                        return Some(if is_simple(args.first()?) {
                            format!("{inner}?")
                        } else {
                            format!("({inner})?")
                        });
                    }
                    if n.kind == BoundGenericStructure && ident("Array") && args.len() == 1 {
                        return Some(format!("[{}]", self.type_name(args.first()?)?));
                    }
                    if n.kind == BoundGenericStructure && ident("Dictionary") && args.len() == 2 {
                        return Some(format!(
                            "[{}:{}]",
                            self.type_name(args.first()?)?,
                            self.type_name(args.child(1)?)?
                        ));
                    }
                }
                let mut s = self.type_name(base)?;
                s.push('<');
                for (i, a) in args.children.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    s.push_str(&self.type_name(a)?);
                }
                s.push('>');
                s
            }
            Tuple => {
                let mut s = String::from("(");
                for (i, e) in n.children.iter().enumerate() {
                    if i > 0 {
                        s.push(',');
                    }
                    if let Some(l) = e.child_of_kind(TupleElementName) {
                        s.push_str(l.get_text());
                        s.push(':');
                    }
                    s.push_str(&self.type_name(e.child_of_kind(Type)?)?);
                    if e.child_of_kind(VariadicMarker).is_some() {
                        s.push_str("...");
                    }
                }
                s.push(')');
                s
            }
            FunctionType
            | NoEscapeFunctionType
            | EscapingAutoClosureType
            | AutoClosureType
            | ThinFunctionType
            | CFunctionPointer
            | ObjCBlock
            | EscapingObjCBlock
            | UncurriedFunctionType => {
                let args = n.child_of_kind(ArgumentTuple)?.first()?;
                let ret = n.child_of_kind(ReturnType)?.first()?;
                let params = args.first()?;
                let mut s = String::new();
                if params.kind == Tuple {
                    s.push_str(&self.type_name(args)?);
                } else {
                    s.push('(');
                    s.push_str(&self.type_name(args)?);
                    s.push(')');
                }
                if n.child_of_kind(AsyncAnnotation).is_some() {
                    s.push_str("async");
                }
                if n.child_of_kind(ThrowsAnnotation).is_some()
                    || n.child_of_kind(TypedThrowsAnnotation).is_some()
                {
                    s.push_str("throws");
                }
                s.push_str("->");
                s.push_str(&self.type_name(ret)?);
                s
            }
            DependentGenericType => self.type_name(n.child(1)?)?,
            DependentGenericParamType => n.get_text().to_string(),
            DependentMemberType => {
                let base = self.type_name(n.first()?)?;
                let member = n.child(1)?;
                format!("{base}.{}", member.first()?.get_text())
            }
            AssociatedTypeRef => {
                format!("{}.{}", self.type_name(n.first()?)?, n.child(1)?.get_text())
            }
            Metatype | ExistentialMetatype => {
                let t = self.type_name(n.last()?)?;
                if is_simple(n.last()?) {
                    format!("{t}.Type")
                } else {
                    format!("({t}).Type")
                }
            }
            ProtocolList => {
                let list = n.first()?;
                if list.is_empty() {
                    "Any".to_string()
                } else {
                    let parts: Option<Vec<String>> =
                        list.children.iter().map(|p| self.type_name(p)).collect();
                    parts?.join("&")
                }
            }
            ProtocolListWithAnyObject => {
                let list = n.first()?.first()?;
                let mut parts: Vec<String> = Vec::new();
                for p in &list.children {
                    parts.push(self.type_name(p)?);
                }
                parts.push("Swift.AnyObject".to_string());
                parts.join("&")
            }
            ProtocolListWithClass => {
                let list = n.first()?.first()?;
                let mut parts: Vec<String> = vec![self.type_name(n.child(1)?)?];
                for p in &list.children {
                    parts.push(self.type_name(p)?);
                }
                parts.join("&")
            }
            ConstrainedExistential => format!("any_{}", self.type_name(n.first()?)?),
            DynamicSelf | ConstrainedExistentialSelf => "Self".to_string(),
            InOut => format!("inout({})", self.type_name(n.first()?)?),
            Shared | Owned | Isolated | Sending | NoDerivative | CompileTimeLiteral => {
                self.type_name(n.first()?)?
            }
            Weak => format!("weak({})", self.type_name(n.first()?)?),
            Unowned => format!("unowned({})", self.type_name(n.first()?)?),
            Unmanaged => format!("unmanaged({})", self.type_name(n.first()?)?),
            ErrorType => "<ERROR>".to_string(),
            BuiltinTypeName => n.get_text().to_string(),
            SugaredOptional => format!("{}?", self.type_name(n.first()?)?),
            SugaredArray => format!("[{}]", self.type_name(n.first()?)?),
            SugaredDictionary => format!(
                "[{}:{}]",
                self.type_name(n.first()?)?,
                self.type_name(n.child(1)?)?
            ),
            SugaredParen => format!("({})", self.type_name(n.first()?)?),
            OpaqueReturnType => "some".to_string(),
            OpaqueType => {
                let mut p = NamePrinter {
                    out: String::new(),
                    depth: self.depth,
                };
                p.node(n.first()?)?;
                format!("{}.opaque#{}", p.out, n.child(1)?.get_index())
            }
            Pack => {
                let parts: Option<Vec<String>> =
                    n.children.iter().map(|p| self.type_name(p)).collect();
                format!("Pack{{{}}}", parts?.join(","))
            }
            PackExpansion => format!("repeat({})", self.type_name(n.first()?)?),
            PackElement => format!("each({})", self.type_name(n.first()?)?),
            Integer => n.get_index().to_string(),
            NegativeInteger => format!("-{}", n.get_index()),
            BuiltinFixedArray => {
                format!(
                    "Builtin.FixedArray<{},{}>",
                    self.type_name(n.first()?)?,
                    self.type_name(n.child(1)?)?
                )
            }
            SILBoxType => format!("box({})", self.type_name(n.first()?)?),
            ImplFunctionType => {
                let mut params = Vec::new();
                let mut results = Vec::new();
                for c in &n.children {
                    match c.kind {
                        ImplParameter | ImplYield => params.push(self.type_name(c.last()?)?),
                        ImplResult | ImplErrorResult => results.push(self.type_name(c.last()?)?),
                        _ => {}
                    }
                }
                format!("({})->({})", params.join(","), results.join(","))
            }
            _ => return None,
        })
    }
}

fn is_simple(n: &Node) -> bool {
    match n.kind {
        Kind::Type => n.first().is_some_and(|c| is_simple(c)),
        Kind::ProtocolList => n.first().is_some_and(|l| l.len() <= 1),
        Kind::ProtocolListWithAnyObject | Kind::ProtocolListWithClass => n
            .first()
            .and_then(|p| p.first())
            .is_some_and(|l| l.is_empty()),
        Kind::FunctionType | Kind::NoEscapeFunctionType | Kind::InOut => false,
        _ => true,
    }
}
