//! (kuna `swiftdemangle`) The demangling tree: one node per grammar
//! production, built in postfix order by the parser and rendered by the
//! printers. Substitutions re-reference earlier nodes, so the tree is a DAG
//! of immutable, reference-counted nodes.

use std::rc::Rc;

pub type NodeRef = Rc<Node>;

/// The production a node stands for; the names follow `Mangling.rst`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Global,
    Suffix,
    Identifier,
    Module,
    PrivateDeclName,
    LocalDeclName,
    RelatedEntityDeclName,
    PrefixOperator,
    PostfixOperator,
    InfixOperator,
    Class,
    Structure,
    Enum,
    Protocol,
    TypeAlias,
    OtherNominalType,
    Extension,
    AnonymousContext,
    OpaqueType,
    OpaqueReturnType,
    OpaqueReturnTypeOf,
    OpaqueReturnTypeIndex,
    OpaqueReturnTypeParent,
    OpaqueTypeDescriptor,
    OpaqueTypeDescriptorAccessor,
    OpaqueTypeDescriptorAccessorImpl,
    OpaqueTypeDescriptorAccessorKey,
    OpaqueTypeDescriptorAccessorVar,
    OpaqueTypeDescriptorRecord,
    Function,
    Variable,
    Subscript,
    Getter,
    Setter,
    ModifyAccessor,
    Modify2Accessor,
    ReadAccessor,
    Read2Accessor,
    WillSet,
    DidSet,
    GlobalGetter,
    MaterializeForSet,
    InitAccessor,
    UnsafeMutableAddressor,
    UnsafeAddressor,
    OwningMutableAddressor,
    OwningAddressor,
    NativeOwningMutableAddressor,
    NativeOwningAddressor,
    NativePinningMutableAddressor,
    NativePinningAddressor,
    Allocator,
    Constructor,
    Destructor,
    Deallocator,
    IsolatedDeallocator,
    IVarInitializer,
    IVarDestroyer,
    Initializer,
    PropertyWrapperBackingInitializer,
    PropertyWrapperInitFromProjectedValue,
    DefaultArgumentInitializer,
    ExplicitClosure,
    ImplicitClosure,
    Static,
    EnumCase,
    GenericTypeParamDecl,
    AssociatedTypeDecl,
    RuntimeAttributeGenerator,
    AssocTypePath,
    Directness,
    GenericTypeMetadataPattern,
    ReflectionMetadataBuiltinDescriptor,
    ReflectionMetadataFieldDescriptor,
    ReflectionMetadataAssocTypeDescriptor,
    ReflectionMetadataSuperclassDescriptor,
    TypeMetadata,
    TypeMetadataAccessFunction,
    TypeMetadataCompletionFunction,
    TypeMetadataInstantiationFunction,
    TypeMetadataInstantiationCache,
    TypeMetadataSingletonInitializationCache,
    TypeMetadataLazyCache,
    TypeMetadataDemanglingCache,
    FullTypeMetadata,
    NominalTypeDescriptor,
    NominalTypeDescriptorRecord,
    Metaclass,
    ClassMetadataBaseOffset,
    MethodLookupFunction,
    ObjCMetadataUpdateFunction,
    ObjCResilientClassStub,
    FullObjCResilientClassStub,
    ModuleDescriptor,
    ExtensionDescriptor,
    AnonymousDescriptor,
    ProtocolDescriptor,
    ProtocolDescriptorRecord,
    ProtocolRequirementsBaseDescriptor,
    ProtocolSelfConformanceDescriptor,
    ProtocolSelfConformanceWitnessTable,
    ProtocolSelfConformanceWitness,
    ProtocolConformanceDescriptor,
    ProtocolConformanceDescriptorRecord,
    ProtocolWitnessTable,
    ProtocolWitnessTableAccessor,
    ProtocolWitnessTablePattern,
    GenericProtocolWitnessTable,
    GenericProtocolWitnessTableInstantiationFunction,
    ResilientProtocolWitnessTable,
    LazyProtocolWitnessTableAccessor,
    LazyProtocolWitnessTableCacheVariable,
    AssociatedTypeMetadataAccessor,
    AssociatedTypeWitnessTableAccessor,
    BaseWitnessTableAccessor,
    AssociatedTypeDescriptor,
    AssociatedConformanceDescriptor,
    DefaultAssociatedConformanceAccessor,
    DefaultAssociatedTypeMetadataAccessor,
    BaseConformanceDescriptor,
    ProtocolWitness,
    ValueWitness,
    ValueWitnessTable,
    FieldOffset,
    EnumCaseTag,
    PropertyDescriptor,
    MethodDescriptor,
    DispatchThunk,
    ObjCAttribute,
    NonObjCAttribute,
    DynamicAttribute,
    DirectMethodReferenceAttribute,
    VTableAttribute,
    VTableThunk,
    ReabstractionThunk,
    ReabstractionThunkHelper,
    ReabstractionThunkHelperWithSelf,
    ReabstractionThunkHelperWithGlobalActor,
    PartialApplyForwarder,
    PartialApplyObjCForwarder,
    MergedFunction,
    OutlinedCopy,
    OutlinedConsume,
    OutlinedRetain,
    OutlinedRelease,
    OutlinedInitializeWithTake,
    OutlinedInitializeWithCopy,
    OutlinedAssignWithTake,
    OutlinedAssignWithCopy,
    OutlinedDestroy,
    OutlinedInitializeWithTakeNoValueWitness,
    OutlinedInitializeWithCopyNoValueWitness,
    OutlinedAssignWithTakeNoValueWitness,
    OutlinedAssignWithCopyNoValueWitness,
    OutlinedDestroyNoValueWitness,
    OutlinedEnumTagStore,
    OutlinedEnumProjectDataForLoad,
    OutlinedEnumGetTag,
    OutlinedVariable,
    OutlinedReadOnlyObject,
    OutlinedBridgedMethod,
    AsyncFunctionPointer,
    AsyncAwaitResumePartialFunction,
    AsyncSuspendResumePartialFunction,
    CoroFunctionPointer,
    DefaultOverride,
    DynamicallyReplaceableFunctionImpl,
    DynamicallyReplaceableFunctionKey,
    DynamicallyReplaceableFunctionVar,
    BackDeploymentThunk,
    BackDeploymentFallback,
    HasSymbolQuery,
    DistributedThunk,
    DistributedAccessor,
    AccessibleFunctionRecord,
    KeyPathGetterThunkHelper,
    KeyPathSetterThunkHelper,
    KeyPathEqualsThunkHelper,
    KeyPathHashThunkHelper,
    KeyPathUnappliedMethodThunkHelper,
    KeyPathAppliedMethodThunkHelper,
    CurryThunk,
    GlobalVariableOnceDeclList,
    GlobalVariableOnceFunction,
    GlobalVariableOnceToken,
    CanonicalSpecializedGenericMetaclass,
    CanonicalSpecializedGenericTypeMetadataAccessFunction,
    MetadataInstantiationCache,
    NoncanonicalSpecializedGenericTypeMetadata,
    NoncanonicalSpecializedGenericTypeMetadataCache,
    CanonicalPrespecializedGenericTypeCachingOnceToken,
    UniqueExtendedExistentialTypeShapeSymbolicReference,
    ExtendedExistentialTypeShape,
    ObjCAsyncCompletionHandlerImpl,
    PredefinedObjCAsyncCompletionHandlerImpl,
    GenericSpecialization,
    GenericSpecializationNotReAbstracted,
    GenericSpecializationInResilienceDomain,
    GenericSpecializationPrespecialized,
    GenericSpecializationParam,
    GenericPartialSpecialization,
    GenericPartialSpecializationNotReAbstracted,
    InlinedGenericFunction,
    FunctionSignatureSpecialization,
    FunctionSignatureSpecializationParam,
    FunctionSignatureSpecializationReturn,
    FunctionSignatureSpecializationParamKind,
    FunctionSignatureSpecializationParamPayload,
    SpecializationPassID,
    IsSerialized,
    DroppedArgument,
    AsyncRemoved,
    MetatypeRepresentation,
    Type,
    TypeMangling,
    FunctionType,
    NoEscapeFunctionType,
    EscapingAutoClosureType,
    AutoClosureType,
    ThinFunctionType,
    CFunctionPointer,
    ObjCBlock,
    EscapingObjCBlock,
    UncurriedFunctionType,
    ArgumentTuple,
    ReturnType,
    Tuple,
    TupleElement,
    TupleElementName,
    VariadicMarker,
    LabelList,
    TypeList,
    EmptyList,
    FirstElementMarker,
    BoundGenericClass,
    BoundGenericStructure,
    BoundGenericEnum,
    BoundGenericProtocol,
    BoundGenericOtherNominalType,
    BoundGenericTypeAlias,
    BoundGenericFunction,
    Metatype,
    ExistentialMetatype,
    ProtocolList,
    ProtocolListWithClass,
    ProtocolListWithAnyObject,
    ConstrainedExistential,
    ConstrainedExistentialRequirementList,
    ConstrainedExistentialSelf,
    DynamicSelf,
    InOut,
    Shared,
    Owned,
    Isolated,
    Sending,
    NoDerivative,
    CompileTimeLiteral,
    Weak,
    Unowned,
    Unmanaged,
    ErrorType,
    BuiltinTypeName,
    BuiltinFixedArray,
    SugaredOptional,
    SugaredArray,
    SugaredDictionary,
    SugaredParen,
    ThrowsAnnotation,
    TypedThrowsAnnotation,
    AsyncAnnotation,
    ConcurrentFunctionType,
    DifferentiableFunctionType,
    GlobalActorFunctionType,
    IsolatedAnyFunctionType,
    NonIsolatedCallerFunctionType,
    SendingResultFunctionType,
    ImplFunctionType,
    ImplFunctionAttribute,
    ImplFunctionConvention,
    ImplFunctionConventionName,
    ImplParameter,
    ImplResult,
    ImplYield,
    ImplErrorResult,
    ImplConvention,
    ImplParameterResultDifferentiability,
    ImplParameterSending,
    ImplParameterIsolated,
    ImplParameterImplicitLeading,
    ImplEscaping,
    ImplErasedIsolation,
    ImplSendingResult,
    ImplPatternSubstitutions,
    ImplInvocationSubstitutions,
    ImplCoroutineKind,
    ImplDifferentiabilityKind,
    ClangType,
    SILBoxType,
    SILBoxTypeWithLayout,
    SILBoxLayout,
    SILBoxMutableField,
    SILBoxImmutableField,
    Pack,
    PackExpansion,
    PackElement,
    PackElementLevel,
    DependentGenericType,
    DependentGenericSignature,
    DependentGenericParamCount,
    DependentGenericParamType,
    DependentGenericParamPackMarker,
    DependentGenericParamValueMarker,
    DependentGenericConformanceRequirement,
    DependentGenericSameTypeRequirement,
    DependentGenericSameShapeRequirement,
    DependentGenericLayoutRequirement,
    DependentGenericInverseConformanceRequirement,
    DependentMemberType,
    DependentAssociatedTypeRef,
    DependentAssociatedConformance,
    DependentProtocolConformanceRoot,
    DependentProtocolConformanceInherited,
    DependentProtocolConformanceAssociated,
    DependentProtocolConformanceOpaque,
    AssociatedType,
    AssociatedTypeRef,
    ProtocolConformance,
    ProtocolConformanceRefInTypeModule,
    ProtocolConformanceRefInProtocolModule,
    ProtocolConformanceRefInOtherModule,
    ConcreteProtocolConformance,
    PackProtocolConformance,
    AnyProtocolConformanceList,
    RetroactiveConformance,
    Index,
    Number,
    NegativeInteger,
    Integer,
    UnknownIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Payload {
    None,
    Text(String),
    Index(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node {
    pub kind: Kind,
    pub payload: Payload,
    pub children: Vec<NodeRef>,
}

/// A crafted name can nest tens of thousands of levels; dropping that
/// recursively would exhaust the stack, so children are released from a
/// worklist.
impl Drop for Node {
    fn drop(&mut self) {
        let mut pending = std::mem::take(&mut self.children);
        while let Some(child) = pending.pop() {
            if let Ok(mut node) = Rc::try_unwrap(child) {
                pending.append(&mut node.children);
            }
        }
    }
}

impl Node {
    pub fn new(kind: Kind) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::None,
            children: Vec::new(),
        })
    }

    pub fn text(kind: Kind, text: impl Into<String>) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::Text(text.into()),
            children: Vec::new(),
        })
    }

    pub fn index(kind: Kind, index: u64) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::Index(index),
            children: Vec::new(),
        })
    }

    pub fn with(kind: Kind, children: Vec<NodeRef>) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::None,
            children,
        })
    }

    pub fn with_text(kind: Kind, text: impl Into<String>, children: Vec<NodeRef>) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::Text(text.into()),
            children,
        })
    }

    pub fn with_index(kind: Kind, index: u64, children: Vec<NodeRef>) -> NodeRef {
        Rc::new(Node {
            kind,
            payload: Payload::Index(index),
            children,
        })
    }

    pub fn child(&self, i: usize) -> Option<&NodeRef> {
        self.children.get(i)
    }

    pub fn first(&self) -> Option<&NodeRef> {
        self.children.first()
    }

    pub fn last(&self) -> Option<&NodeRef> {
        self.children.last()
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_text(&self) -> &str {
        match &self.payload {
            Payload::Text(t) => t,
            _ => "",
        }
    }

    pub fn get_index(&self) -> u64 {
        match &self.payload {
            Payload::Index(i) => *i,
            _ => 0,
        }
    }

    pub fn has_text(&self) -> bool {
        matches!(self.payload, Payload::Text(_))
    }

    pub fn has_index(&self) -> bool {
        matches!(self.payload, Payload::Index(_))
    }

    pub fn child_of_kind(&self, kind: Kind) -> Option<&NodeRef> {
        self.children.iter().find(|c| c.kind == kind)
    }

    /// A copy of this node with `child` appended.
    pub fn plus(self: &Rc<Self>, child: NodeRef) -> NodeRef {
        let mut n = (**self).clone();
        n.children.push(child);
        Rc::new(n)
    }

    /// A copy of this node under another kind.
    pub fn rekind(self: &Rc<Self>, kind: Kind) -> NodeRef {
        let mut n = (**self).clone();
        n.kind = kind;
        Rc::new(n)
    }
}

impl Kind {
    pub fn is_decl_name(self) -> bool {
        matches!(
            self,
            Kind::Identifier
                | Kind::LocalDeclName
                | Kind::PrivateDeclName
                | Kind::RelatedEntityDeclName
                | Kind::PrefixOperator
                | Kind::PostfixOperator
                | Kind::InfixOperator
        )
    }

    pub fn is_any_generic(self) -> bool {
        matches!(
            self,
            Kind::Structure
                | Kind::Class
                | Kind::Enum
                | Kind::Protocol
                | Kind::OtherNominalType
                | Kind::TypeAlias
        )
    }

    pub fn is_entity(self) -> bool {
        if self == Kind::Type {
            return true;
        }
        self.is_context()
    }

    pub fn is_context(self) -> bool {
        matches!(
            self,
            Kind::Allocator
                | Kind::AnonymousContext
                | Kind::Class
                | Kind::Constructor
                | Kind::Deallocator
                | Kind::DefaultArgumentInitializer
                | Kind::Destructor
                | Kind::DidSet
                | Kind::Enum
                | Kind::ExplicitClosure
                | Kind::Extension
                | Kind::Function
                | Kind::Getter
                | Kind::GlobalGetter
                | Kind::IVarInitializer
                | Kind::IVarDestroyer
                | Kind::ImplicitClosure
                | Kind::Initializer
                | Kind::InitAccessor
                | Kind::IsolatedDeallocator
                | Kind::MaterializeForSet
                | Kind::ModifyAccessor
                | Kind::Modify2Accessor
                | Kind::Module
                | Kind::NativeOwningAddressor
                | Kind::NativeOwningMutableAddressor
                | Kind::NativePinningAddressor
                | Kind::NativePinningMutableAddressor
                | Kind::OtherNominalType
                | Kind::OwningAddressor
                | Kind::OwningMutableAddressor
                | Kind::PropertyWrapperBackingInitializer
                | Kind::PropertyWrapperInitFromProjectedValue
                | Kind::Protocol
                | Kind::ReadAccessor
                | Kind::Read2Accessor
                | Kind::Setter
                | Kind::Static
                | Kind::Structure
                | Kind::Subscript
                | Kind::TypeAlias
                | Kind::UnsafeAddressor
                | Kind::UnsafeMutableAddressor
                | Kind::Variable
                | Kind::WillSet
                | Kind::OpaqueReturnTypeOf
        )
    }

    pub fn is_requirement(self) -> bool {
        matches!(
            self,
            Kind::DependentGenericParamPackMarker
                | Kind::DependentGenericParamValueMarker
                | Kind::DependentGenericSameTypeRequirement
                | Kind::DependentGenericSameShapeRequirement
                | Kind::DependentGenericLayoutRequirement
                | Kind::DependentGenericConformanceRequirement
                | Kind::DependentGenericInverseConformanceRequirement
        )
    }

    pub fn is_function_attr(self) -> bool {
        matches!(
            self,
            Kind::FunctionSignatureSpecialization
                | Kind::GenericSpecialization
                | Kind::GenericSpecializationPrespecialized
                | Kind::InlinedGenericFunction
                | Kind::GenericSpecializationNotReAbstracted
                | Kind::GenericSpecializationInResilienceDomain
                | Kind::GenericPartialSpecialization
                | Kind::GenericPartialSpecializationNotReAbstracted
                | Kind::ObjCAttribute
                | Kind::NonObjCAttribute
                | Kind::DynamicAttribute
                | Kind::DirectMethodReferenceAttribute
                | Kind::VTableAttribute
                | Kind::PartialApplyForwarder
                | Kind::PartialApplyObjCForwarder
                | Kind::OutlinedVariable
                | Kind::OutlinedReadOnlyObject
                | Kind::OutlinedBridgedMethod
                | Kind::MergedFunction
                | Kind::DistributedThunk
                | Kind::DistributedAccessor
                | Kind::DynamicallyReplaceableFunctionImpl
                | Kind::DynamicallyReplaceableFunctionKey
                | Kind::DynamicallyReplaceableFunctionVar
                | Kind::AsyncFunctionPointer
                | Kind::AsyncAwaitResumePartialFunction
                | Kind::AsyncSuspendResumePartialFunction
                | Kind::AccessibleFunctionRecord
                | Kind::BackDeploymentThunk
                | Kind::BackDeploymentFallback
                | Kind::HasSymbolQuery
                | Kind::CoroFunctionPointer
                | Kind::DefaultOverride
        )
    }
}
