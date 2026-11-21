//! (kuna) The Swift runtime's entry points, with the
//! signatures the runtime's public headers give them
//! (`swift/Runtime/HeapObject.h`, `Metadata.h`, `Casting.h`,
//! `Exclusivity.h`, `Concurrency.h`), so a call into the runtime reads as
//! what it is: `swift_allocObject(metadata, 0x20, 7)` takes a
//! `swift.type_metadata *` and two sizes, `swift_getGenericMetadata(…)`
//! returns a `swift.MetadataResponse` whose first word is the metadata,
//! `swift_dynamicCast` a `bool`. Bound by name, so an import stub, a GOT slot or an unstripped
//! copy of the runtime all take them.
//!
//! The standard library's own C entry points come along (`_swift_stdlib_*`,
//! from `stdlib/public/SwiftShims/swift/shims/AssertionReporting.h`,
//! `RuntimeShims.h`, `LibcShims.h`, `FoundationShims.h`,
//! `ThreadLocalStorage.h`): `_swift_stdlib_reportUnimplementedInitializer`
//! takes two strings with their lengths and the flags, so the live floating
//! registers of its caller stop reading as its first arguments.
//!
//! The runtime is C (every entry point here is `extern "C"` under the
//! platform convention), so these are plain prototypes; the `swiftcc`
//! entry points whose arguments ride in the Swift registers
//! (`swift_task_switch`, the key-path accessors) are left to recovery, and so
//! are the three whose last argument the compiler leaves unset when the callee
//! ignores it (`swift_once`'s context, `swift_getKeyPath`'s arguments,
//! `swift_getWitnessTable`'s instantiation arguments): a declared slot there
//! would make a stale register an argument, and a parameter of the caller.
//!
//! On arm64 the runtime also exports a retain and a release per register
//! (`swift/Runtime/CustomRRABI.h`): `swift_retain_x<N>` takes the object in
//! `x<N>` and returns it in `x0`, `swift_release_x<N>` reads `x<N>` and returns
//! nothing, so an optimized caller releases a value without moving it into
//! `x0`. Outside every calling convention, those take their one parameter as
//! explicit storage.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::{Datatype, TypeFactory, TypeField, type_metatype};
use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces, PrototypePieces};
use object::read::Object;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// The types the table spells.
#[derive(Clone, Copy)]
enum Ty {
    Void,
    /// `void *`.
    Ptr,
    /// An object reference: `void *` too, because the object's type is the
    /// metadata's, which the call site knows and a `swift.HeapObject *` would
    /// hide behind `obj[1].isa` for its first stored property.
    Obj,
    /// `swift.type_metadata *`.
    Meta,
    /// `swift.MetadataResponse` (the metadata and its state, two words).
    Response,
    /// `swift.BoxPair` (an object and the address of its payload).
    BoxPair,
    /// `char *`.
    Str,
    /// `unsigned long`.
    Size,
    /// `long`.
    Int,
    /// `unsigned int`.
    U32,
    /// `int`.
    I32,
    Bool,
    /// `swift.NSOperatingSystemVersion` (major, minor and patch, three words).
    OsVersion,
}

struct Sig {
    ret: Ty,
    params: &'static [Ty],
}

macro_rules! sig {
    ($ret:ident ( $($p:ident),* )) => {
        Sig { ret: Ty::$ret, params: &[$(Ty::$p),*] }
    };
}

use Ty::*;

/// `(name, signature)`; `_n` variants and the unknown-object family follow
/// their plain forms.
const RUNTIME: &[(&str, Sig)] = &[
    ("swift_retain", sig!(Obj(Obj))),
    ("swift_release", sig!(Void(Obj))),
    ("swift_retain_n", sig!(Obj(Obj, U32))),
    ("swift_release_n", sig!(Void(Obj, U32))),
    ("swift_nonatomic_retain", sig!(Obj(Obj))),
    ("swift_nonatomic_release", sig!(Void(Obj))),
    ("swift_unknownObjectRetain", sig!(Ptr(Ptr))),
    ("swift_unknownObjectRelease", sig!(Void(Ptr))),
    ("swift_unknownObjectRetain_n", sig!(Ptr(Ptr, U32))),
    ("swift_unknownObjectRelease_n", sig!(Void(Ptr, U32))),
    ("swift_bridgeObjectRetain", sig!(Ptr(Ptr))),
    ("swift_bridgeObjectRelease", sig!(Void(Ptr))),
    ("swift_bridgeObjectRetain_n", sig!(Ptr(Ptr, U32))),
    ("swift_bridgeObjectRelease_n", sig!(Void(Ptr, U32))),
    ("swift_unownedRetain", sig!(Obj(Obj))),
    ("swift_unownedRelease", sig!(Void(Obj))),
    ("swift_unownedRetainStrong", sig!(Obj(Obj))),
    ("swift_unownedRetainStrongAndRelease", sig!(Obj(Obj))),
    ("swift_errorRetain", sig!(Ptr(Ptr))),
    ("swift_errorRelease", sig!(Void(Ptr))),
    ("swift_retainCount", sig!(Size(Obj))),
    ("swift_isUniquelyReferenced", sig!(Bool(Obj))),
    ("swift_isUniquelyReferenced_native", sig!(Bool(Obj))),
    ("swift_isUniquelyReferenced_nonNull", sig!(Bool(Obj))),
    ("swift_isUniquelyReferenced_nonNull_native", sig!(Bool(Obj))),
    ("swift_isUniquelyReferencedNonObjC", sig!(Bool(Ptr))),
    ("swift_isUniquelyReferencedNonObjC_nonNull", sig!(Bool(Ptr))),
    (
        "swift_isUniquelyReferenced_nonNull_bridgeObject",
        sig!(Bool(Ptr)),
    ),
    (
        "swift_isUniquelyReferencedNonObjC_nonNull_bridgeObject",
        sig!(Bool(Ptr)),
    ),
    (
        "swift_isEscapingClosureAtFileLocation",
        sig!(Bool(Obj, Str, U32, U32, U32, U32)),
    ),
    ("swift_allocObject", sig!(Obj(Meta, Size, Size))),
    ("swift_initStackObject", sig!(Obj(Meta, Obj))),
    ("swift_initStaticObject", sig!(Obj(Meta, Obj))),
    ("swift_deallocObject", sig!(Void(Obj, Size, Size))),
    (
        "swift_deallocUninitializedObject",
        sig!(Void(Obj, Size, Size)),
    ),
    ("swift_deallocClassInstance", sig!(Void(Obj, Size, Size))),
    (
        "swift_deallocPartialClassInstance",
        sig!(Void(Obj, Meta, Size, Size)),
    ),
    ("swift_setDeallocating", sig!(Void(Obj))),
    ("swift_slowAlloc", sig!(Ptr(Size, Size))),
    ("swift_slowDealloc", sig!(Void(Ptr, Size, Size))),
    ("swift_coroFrameAlloc", sig!(Ptr(Size, Size))),
    ("swift_allocBox", sig!(BoxPair(Meta))),
    ("swift_makeBoxUnique", sig!(BoxPair(Ptr, Meta, Size))),
    ("swift_projectBox", sig!(Ptr(Obj))),
    ("swift_allocError", sig!(BoxPair(Meta, Ptr, Ptr, Bool))),
    ("swift_getErrorValue", sig!(Void(Ptr, Ptr, Ptr))),
    ("swift_willThrowTypedImpl", sig!(Void(Ptr, Meta, Ptr))),
    ("swift_getGenericMetadata", sig!(Response(Size, Ptr, Ptr))),
    ("swift_checkMetadataState", sig!(Response(Size, Meta))),
    ("swift_getSingletonMetadata", sig!(Response(Size, Ptr))),
    ("swift_getForeignTypeMetadata", sig!(Response(Size, Meta))),
    (
        "swift_getOpaqueTypeMetadata2",
        sig!(Response(Size, Ptr, Ptr, U32)),
    ),
    (
        "swift_getAssociatedTypeWitness",
        sig!(Response(Size, Ptr, Meta, Ptr, Ptr)),
    ),
    (
        "swift_getTupleTypeMetadata",
        sig!(Response(Size, Size, Ptr, Str, Ptr)),
    ),
    (
        "swift_getTupleTypeMetadata2",
        sig!(Response(Size, Meta, Meta, Str, Ptr)),
    ),
    (
        "swift_getTupleTypeMetadata3",
        sig!(Response(Size, Meta, Meta, Meta, Str, Ptr)),
    ),
    ("swift_getTupleTypeLayout", sig!(Void(Ptr, Ptr, Size, Ptr))),
    ("swift_getTupleTypeLayout2", sig!(Size(Ptr, Ptr, Ptr))),
    ("swift_getObjCClassMetadata", sig!(Meta(Ptr))),
    ("swift_getObjCClassFromMetadata", sig!(Ptr(Meta))),
    ("swift_getObjCClassFromObject", sig!(Ptr(Obj))),
    ("swift_getInitializedObjCClass", sig!(Ptr(Ptr))),
    ("swift_getObjectType", sig!(Meta(Obj))),
    ("swift_getDynamicType", sig!(Meta(Ptr, Meta, Bool))),
    ("swift_getMetadataKind", sig!(Size(Meta))),
    ("swift_isClassType", sig!(Bool(Meta))),
    ("swift_isOptionalType", sig!(Bool(Meta))),
    ("swift_getMetatypeMetadata", sig!(Meta(Meta))),
    ("swift_getExistentialMetatypeMetadata", sig!(Meta(Meta))),
    (
        "swift_getExistentialTypeMetadata",
        sig!(Meta(Size, Meta, Size, Ptr)),
    ),
    (
        "swift_getExtendedExistentialTypeMetadata",
        sig!(Meta(Ptr, Ptr)),
    ),
    (
        "swift_getFunctionTypeMetadata",
        sig!(Meta(Size, Ptr, Ptr, Meta)),
    ),
    ("swift_getFunctionTypeMetadata0", sig!(Meta(Size, Meta))),
    (
        "swift_getFunctionTypeMetadata1",
        sig!(Meta(Size, Meta, Meta)),
    ),
    (
        "swift_getFunctionTypeMetadata2",
        sig!(Meta(Size, Meta, Meta, Meta)),
    ),
    (
        "swift_getFunctionTypeMetadata3",
        sig!(Meta(Size, Meta, Meta, Meta, Meta)),
    ),
    (
        "swift_getTypeByMangledNameInContext",
        sig!(Meta(Str, Size, Ptr, Ptr)),
    ),
    (
        "swift_getTypeByMangledNameInContext2",
        sig!(Meta(Str, Size, Ptr, Ptr)),
    ),
    (
        "swift_getTypeByMangledNameInContextInMetadataState",
        sig!(Meta(Size, Str, Size, Ptr, Ptr)),
    ),
    (
        "swift_getTypeByMangledNameInContextInMetadataState2",
        sig!(Meta(Size, Str, Size, Ptr, Ptr)),
    ),
    (
        "swift_getTypeByMangledNameInEnvironment",
        sig!(Meta(Str, Size, Ptr, Ptr)),
    ),
    ("swift_conformsToProtocol", sig!(Ptr(Meta, Ptr))),
    ("swift_conformsToProtocol2", sig!(Ptr(Meta, Ptr))),
    ("swift_conformsToProtocolCommon", sig!(Ptr(Meta, Ptr))),
    (
        "swift_getAssociatedConformanceWitness",
        sig!(Ptr(Ptr, Meta, Meta, Ptr, Ptr)),
    ),
    ("swift_getOpaqueTypeConformance", sig!(Ptr(Ptr, Ptr, U32))),
    ("swift_getOpaqueTypeConformance2", sig!(Ptr(Ptr, Ptr, U32))),
    ("swift_dynamicCast", sig!(Bool(Ptr, Ptr, Meta, Meta, Size))),
    ("swift_dynamicCastClass", sig!(Ptr(Ptr, Meta))),
    (
        "swift_dynamicCastClassUnconditional",
        sig!(Ptr(Ptr, Meta, Str, U32, U32)),
    ),
    ("swift_dynamicCastObjCClass", sig!(Ptr(Ptr, Meta))),
    (
        "swift_dynamicCastObjCClassUnconditional",
        sig!(Ptr(Ptr, Meta, Str, U32, U32)),
    ),
    ("swift_dynamicCastUnknownClass", sig!(Ptr(Ptr, Meta))),
    (
        "swift_dynamicCastUnknownClassUnconditional",
        sig!(Ptr(Ptr, Meta, Str, U32, U32)),
    ),
    ("swift_dynamicCastForeignClass", sig!(Ptr(Ptr, Meta))),
    ("swift_dynamicCastMetatype", sig!(Meta(Meta, Meta))),
    (
        "swift_dynamicCastMetatypeUnconditional",
        sig!(Meta(Meta, Meta, Str, U32, U32)),
    ),
    (
        "swift_dynamicCastObjCProtocolConditional",
        sig!(Ptr(Ptr, Size, Ptr)),
    ),
    (
        "swift_dynamicCastObjCProtocolUnconditional",
        sig!(Ptr(Ptr, Size, Ptr, Str, U32, U32)),
    ),
    ("swift_beginAccess", sig!(Void(Ptr, Ptr, Size, Ptr))),
    ("swift_endAccess", sig!(Void(Ptr))),
    ("swift_task_alloc", sig!(Ptr(Size))),
    ("swift_task_dealloc", sig!(Void(Ptr))),
    ("swift_task_getCurrent", sig!(Ptr())),
    ("swift_task_getMainExecutor", sig!(Ptr())),
    ("swift_task_isCurrentExecutor", sig!(Bool(Ptr, Ptr))),
    ("swift_getEnumCaseMultiPayload", sig!(U32(Ptr, Meta))),
    ("swift_storeEnumTagMultiPayload", sig!(Void(Ptr, Meta, U32))),
    (
        "swift_getEnumTagSinglePayloadGeneric",
        sig!(U32(Ptr, U32, Meta, Ptr)),
    ),
    (
        "swift_storeEnumTagSinglePayloadGeneric",
        sig!(Void(Ptr, U32, U32, Meta, Ptr)),
    ),
    ("swift_EnumCaseName", sig!(Str(U32, Meta))),
    ("swift_arrayInitWithCopy", sig!(Void(Ptr, Ptr, Size, Meta))),
    (
        "swift_arrayInitWithTakeFrontToBack",
        sig!(Void(Ptr, Ptr, Size, Meta)),
    ),
    (
        "swift_arrayInitWithTakeBackToFront",
        sig!(Void(Ptr, Ptr, Size, Meta)),
    ),
    (
        "swift_arrayAssignWithCopy",
        sig!(Void(Ptr, Ptr, Size, Meta)),
    ),
    (
        "swift_arrayAssignWithTake",
        sig!(Void(Ptr, Ptr, Size, Meta)),
    ),
    ("swift_arrayDestroy", sig!(Void(Ptr, Size, Meta))),
    (
        "swift_initClassMetadata2",
        sig!(Meta(Meta, Size, Size, Ptr, Ptr)),
    ),
    (
        "swift_updateClassMetadata2",
        sig!(Meta(Meta, Size, Size, Ptr, Ptr)),
    ),
    (
        "swift_initStructMetadata",
        sig!(Void(Meta, Size, Size, Ptr, Ptr)),
    ),
    (
        "swift_initEnumMetadataSingleCase",
        sig!(Void(Meta, Size, Ptr)),
    ),
    (
        "swift_initEnumMetadataSinglePayload",
        sig!(Void(Meta, Size, Ptr, U32)),
    ),
    (
        "swift_initEnumMetadataMultiPayload",
        sig!(Void(Meta, Size, U32, Ptr)),
    ),
    (
        "swift_allocateGenericClassMetadata",
        sig!(Meta(Ptr, Ptr, Ptr)),
    ),
    (
        "swift_allocateGenericValueMetadata",
        sig!(Meta(Ptr, Ptr, Ptr, Size)),
    ),
    ("swift_weakInit", sig!(Ptr(Ptr, Obj))),
    ("swift_weakAssign", sig!(Ptr(Ptr, Obj))),
    ("swift_weakLoadStrong", sig!(Obj(Ptr))),
    ("swift_weakTakeStrong", sig!(Obj(Ptr))),
    ("swift_weakDestroy", sig!(Void(Ptr))),
    ("swift_weakCopyInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_weakTakeInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_weakCopyAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_weakTakeAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakLoadStrong", sig!(Ptr(Ptr))),
    ("swift_unknownObjectWeakTakeStrong", sig!(Ptr(Ptr))),
    ("swift_unknownObjectWeakDestroy", sig!(Void(Ptr))),
    ("swift_unknownObjectWeakCopyInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakTakeInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakCopyAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectWeakTakeAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedLoadStrong", sig!(Ptr(Ptr))),
    ("swift_unknownObjectUnownedTakeStrong", sig!(Ptr(Ptr))),
    ("swift_unknownObjectUnownedDestroy", sig!(Void(Ptr))),
    ("swift_unknownObjectUnownedCopyInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedTakeInit", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedCopyAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_unknownObjectUnownedTakeAssign", sig!(Ptr(Ptr, Ptr))),
    ("swift_defaultActor_initialize", sig!(Void(Ptr))),
    ("swift_defaultActor_destroy", sig!(Void(Ptr))),
    ("swift_defaultActor_deallocate", sig!(Void(Ptr))),
    ("swift_stdlib_isStackAllocationSafe", sig!(Bool(Size, Size))),
    ("swift_stdlib_random", sig!(Void(Ptr, Size))),
    ("swift_getTypeName", sig!(BoxPair(Meta, Bool))),
    ("swift_getMangledTypeName", sig!(BoxPair(Meta))),
    ("swift_reflectionMirror_count", sig!(Int(Ptr, Meta, Meta))),
    ("swift_reflectionMirror_recursiveCount", sig!(Int(Meta))),
    (
        "swift_reflectionMirror_normalizedType",
        sig!(Meta(Ptr, Meta, Meta)),
    ),
    ("swift_lookUpClassMethod", sig!(Ptr(Meta, Ptr, Ptr))),
    ("swift_deletedMethodError", sig!(Void())),
    ("swift_reportError", sig!(Void(U32, Str))),
    ("swift_reportWarning", sig!(Void(U32, Str))),
    (
        "_swift_stdlib_reportFatalError",
        sig!(Void(Str, I32, Str, I32, U32)),
    ),
    (
        "_swift_stdlib_reportFatalErrorInFile",
        sig!(Void(Str, I32, Str, I32, Str, I32, U32, U32)),
    ),
    (
        "_swift_stdlib_reportUnimplementedInitializer",
        sig!(Void(Str, I32, Str, I32, U32)),
    ),
    (
        "_swift_stdlib_reportUnimplementedInitializerInFile",
        sig!(Void(Str, I32, Str, I32, Str, I32, U32, U32, U32)),
    ),
    ("_swift_stdlib_strtold_clocale", sig!(Str(Str, Ptr))),
    ("_swift_stdlib_strtod_clocale", sig!(Str(Str, Ptr))),
    ("_swift_stdlib_strtof_clocale", sig!(Str(Str, Ptr))),
    ("_swift_stdlib_strtof16_clocale", sig!(Str(Str, Ptr))),
    ("_swift_stdlib_immortalize", sig!(Void(Ptr))),
    ("_swift_stdlib_flockfile_stdout", sig!(Void())),
    ("_swift_stdlib_funlockfile_stdout", sig!(Void())),
    ("_swift_stdlib_putc_stderr", sig!(I32(I32))),
    ("_swift_stdlib_getHardwareConcurrency", sig!(Size())),
    ("_swift_stdlib_getCurrentStackBounds", sig!(Bool(Ptr, Ptr))),
    ("_swift_stdlib_isExecutableLinkedOnOrAfter", sig!(Bool(U32))),
    ("_swift_stdlib_fwrite_stdout", sig!(Size(Ptr, Size, Size))),
    ("swift_stdlib_readLine_stdin", sig!(Int(Ptr))),
    ("_swift_stdlib_operatingSystemVersion", sig!(OsVersion())),
    ("_swift_stdlib_threadLocalStorageGet", sig!(Ptr())),
];

/// The shared types, interned once per factory.
struct Types<'a> {
    tf: &'a dyn TypeFactory,
    ptr_size: i32,
}

impl Types<'_> {
    fn void_ptr(&self) -> Option<Rc<Datatype>> {
        let void = self.tf.get_type_void().ok()?;
        self.tf.get_type_pointer(self.ptr_size, void, 1).ok()
    }

    fn shell(&self, name: &str) -> Option<Rc<Datatype>> {
        if let Some(t) = self.tf.find_by_name(name).ok().flatten() {
            return Some(t);
        }
        self.tf.get_type_struct(name).ok()
    }

    fn pointer_to(&self, name: &str) -> Option<Rc<Datatype>> {
        let st = self.shell(name)?;
        self.tf.get_type_pointer(self.ptr_size, st, 1).ok()
    }

    fn pair(&self, name: &str, fields: [(&str, Rc<Datatype>); 2]) -> Option<Rc<Datatype>> {
        if let Some(t) = self.tf.find_by_name(name).ok().flatten() {
            if t.get_size() > 0 {
                return Some(t);
            }
        }
        let shell = self.tf.get_type_struct(name).ok()?;
        let fields = fields
            .into_iter()
            .enumerate()
            .map(|(i, (n, t))| TypeField::new(i as i32, 8 * i as i32, n.to_string(), t))
            .collect();
        self.tf
            .set_fields_struct_raw(&shell, fields, Vec::new(), 16, 8, 0)
            .ok()
    }

    fn metadata_response(&self) -> Option<Rc<Datatype>> {
        let word = self.tf.get_base(8, type_metatype::TYPE_UINT).ok()?;
        self.pair(
            "swift.MetadataResponse",
            [
                ("metadata", self.pointer_to("swift.type_metadata")?),
                ("state", word),
            ],
        )
    }

    fn os_version(&self) -> Option<Rc<Datatype>> {
        let name = "swift.NSOperatingSystemVersion";
        if let Some(t) = self.tf.find_by_name(name).ok().flatten() {
            if t.get_size() > 0 {
                return Some(t);
            }
        }
        let word = self.tf.get_base(8, type_metatype::TYPE_INT).ok()?;
        let shell = self.tf.get_type_struct(name).ok()?;
        let fields = ["majorVersion", "minorVersion", "patchVersion"]
            .into_iter()
            .enumerate()
            .map(|(i, n)| TypeField::new(i as i32, 8 * i as i32, n.to_string(), Rc::clone(&word)))
            .collect();
        self.tf
            .set_fields_struct_raw(&shell, fields, Vec::new(), 24, 8, 0)
            .ok()
    }

    fn box_pair(&self) -> Option<Rc<Datatype>> {
        self.pair(
            "swift.BoxPair",
            [("object", self.void_ptr()?), ("buffer", self.void_ptr()?)],
        )
    }

    fn of(&self, ty: Ty) -> Option<Rc<Datatype>> {
        Some(match ty {
            Void => self.tf.get_type_void().ok()?,
            Ptr => self.void_ptr()?,
            Obj => self.void_ptr()?,
            Meta => self.pointer_to("swift.type_metadata")?,
            Response => self.metadata_response()?,
            BoxPair => self.box_pair()?,
            Str => {
                let ch = self.tf.get_type_char(1).ok()?;
                self.tf.get_type_pointer(self.ptr_size, ch, 1).ok()?
            }
            Size => self.tf.get_base(8, type_metatype::TYPE_UINT).ok()?,
            Int => self.tf.get_base(8, type_metatype::TYPE_INT).ok()?,
            U32 => self.tf.get_base(4, type_metatype::TYPE_UINT).ok()?,
            I32 => self.tf.get_base(4, type_metatype::TYPE_INT).ok()?,
            Bool => self.tf.get_base(1, type_metatype::TYPE_BOOL).ok()?,
            OsVersion => self.os_version()?,
        })
    }
}

/// The prototypes, for the factory of one image.
pub fn prototypes(tf: &dyn TypeFactory, ptr_size: i32) -> Vec<PrototypePieces> {
    let types = Types { tf, ptr_size };
    let mut out = Vec::with_capacity(RUNTIME.len());
    for (name, sig) in RUNTIME {
        let Some(outtype) = types.of(sig.ret) else {
            continue;
        };
        let intypes: Option<Vec<Rc<Datatype>>> = sig.params.iter().map(|&p| types.of(p)).collect();
        let Some(intypes) = intypes else { continue };
        out.push(PrototypePieces {
            name: name.to_string(),
            outtype: Some(outtype),
            innames: vec![String::new(); intypes.len()],
            intypes,
            first_var_arg_slot: -1,
            output_storage: None,
            input_storage: Vec::new(),
            kuna_library_names: None,
        });
    }
    out
}

/// The registers the runtime defines `swift_retain_x<N>` / `swift_release_x<N>`
/// for (`CUSTOM_RR_ENTRYPOINTS_FOREACH_REG`): `x0` is the plain entry point,
/// `x16`/`x17` are call glue, `x18` is reserved, `x29`/`x30` the frame and
/// link registers.
pub const CUSTOM_RR_REGISTERS: [u32; 25] =
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28];

/// `swift_retain_x19` → `(true, 19)`, `swift_release_x8` → `(false, 8)`; `None`
/// for any other name, a register the runtime defines no entry point for included.
pub fn custom_rr_register(name: &str) -> Option<(bool, u32)> {
    let (retains, rest) = match name.strip_prefix("swift_retain_x") {
        Some(rest) => (true, rest),
        None => (false, name.strip_prefix("swift_release_x")?),
    };
    let n: u32 = rest.parse().ok()?;
    CUSTOM_RR_REGISTERS.contains(&n).then_some((retains, n))
}

/// The register-specific retain and release entry points: one `swift_retain_x<N>`
/// (`void *obj @ x<N>`, the object back in `x0`) and one `swift_release_x<N>`
/// (`void` of the same) per register in [`CUSTOM_RR_REGISTERS`].
pub fn custom_rr_prototypes(arch: &Architecture, ptr_size: i32) -> Vec<PrototypePieces> {
    let types = Types { tf: arch.types(), ptr_size };
    let Some(obj) = types.of(Obj) else { return Vec::new() };
    let Some(void) = types.of(Void) else { return Vec::new() };
    let storage = |register: &str| {
        let vn = arch.get_register_varnode(register.as_bytes()).ok()?;
        Some(ParameterPieces {
            addr: Address::new(vn.space?, vn.offset),
            type_: Some(Rc::clone(&obj)),
            flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
        })
    };
    let mut out = Vec::with_capacity(2 * CUSTOM_RR_REGISTERS.len());
    for n in CUSTOM_RR_REGISTERS {
        let Some(input) = storage(&format!("x{n}")) else { continue };
        for retains in [true, false] {
            let family = if retains { "retain" } else { "release" };
            out.push(PrototypePieces {
                name: format!("swift_{family}_x{n}"),
                outtype: Some(if retains { Rc::clone(&obj) } else { Rc::clone(&void) }),
                intypes: vec![Rc::clone(&obj)],
                innames: vec!["obj".to_string()],
                first_var_arg_slot: -1,
                output_storage: if retains { storage("x0") } else { None },
                input_storage: vec![(0, input.clone())],
                kuna_library_names: None,
            });
        }
    }
    out
}

/// P1 pass: the runtime prototypes of a Swift image (always on; registered
/// for a Swift-detected image only).
pub struct SwiftRuntimePass;

impl AnalysisPass for SwiftRuntimePass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "swiftruntime"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        let ptr_size = if ctx.file.is_64() { 8 } else { 4 };
        out.prototypes = prototypes(ctx.arch.types(), ptr_size);
        if ctx.file.architecture() == object::Architecture::Aarch64 && ptr_size == 8 {
            out.prototypes.extend(custom_rr_prototypes(ctx.arch, ptr_size));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_names_are_unique_and_runtime_spelled() {
        let mut names: Vec<&str> = RUNTIME.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        let total = names.len();
        names.dedup();
        assert_eq!(names.len(), total, "a runtime entry point is listed twice");
        assert!(names.iter().all(|n| n.starts_with("swift_") || n.starts_with("_swift_stdlib_")));
    }

    #[test]
    fn custom_rr_names_name_their_register() {
        assert_eq!(custom_rr_register("swift_retain_x19"), Some((true, 19)));
        assert_eq!(custom_rr_register("swift_release_x8"), Some((false, 8)));
        assert_eq!(custom_rr_register("swift_release_x0"), None, "x0 is the plain entry point");
        assert_eq!(custom_rr_register("swift_retain_x16"), None, "call glue");
        assert_eq!(custom_rr_register("swift_retain_x18"), None, "reserved");
        assert_eq!(custom_rr_register("swift_retain_x29"), None, "the frame pointer");
        assert_eq!(custom_rr_register("swift_retain"), None);
        assert_eq!(custom_rr_register("swift_retain_xa"), None);
        assert_eq!(custom_rr_register("swift_bridgeObjectRelease_x20"), None);
        assert_eq!(CUSTOM_RR_REGISTERS.len(), 25);
    }
}
