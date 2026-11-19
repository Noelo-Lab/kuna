//! (kuna) Prototypes for the Objective-C runtime's entry points an image imports
//! — `objc_alloc_init(Class)`, `id objc_retain(id)`, `objc_storeStrong(id *, id)`,
//! … — so a call through an import stub carries its arguments and result (an
//! untyped stub drops them: `v = objc_retainAutoreleasedReturnValue();`), and for
//! clang's outlined arm64 `objc_retain_x<N>` / `objc_release_x<N>`, which take
//! the object in `x<N>` and return a retain in `x0`, outside every calling
//! convention.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::Datatype;
use kuna_decomp::fspec::{parameter_pieces_flags, ParameterPieces, PrototypePieces};
use object::read::{Object, ObjectSymbol};

use super::encoding::{decode_ivar_type, TypeCtx};

/// `(name, return encoding, argument encodings)` of the runtime entry points
/// whose shape is fixed by objc4's `runtime.h` / `objc-internal.h`.
const RUNTIME: &[(&str, &str, &[&str])] = &[
    ("objc_retain", "@", &["@"]),
    ("objc_release", "v", &["@"]),
    ("objc_autorelease", "@", &["@"]),
    ("objc_retainAutorelease", "@", &["@"]),
    ("objc_retainAutoreleaseReturnValue", "@", &["@"]),
    ("objc_autoreleaseReturnValue", "@", &["@"]),
    ("objc_retainAutoreleasedReturnValue", "@", &["@"]),
    ("objc_unsafeClaimAutoreleasedReturnValue", "@", &["@"]),
    ("objc_claimAutoreleasedReturnValue", "@", &["@"]),
    ("objc_retainBlock", "@", &["@"]),
    ("objc_storeStrong", "v", &["^@", "@"]),
    ("objc_storeWeak", "@", &["^@", "@"]),
    ("objc_loadWeak", "@", &["^@"]),
    ("objc_loadWeakRetained", "@", &["^@"]),
    ("objc_initWeak", "@", &["^@", "@"]),
    ("objc_destroyWeak", "v", &["^@"]),
    ("objc_copyWeak", "v", &["^@", "^@"]),
    ("objc_moveWeak", "v", &["^@", "^@"]),
    ("objc_alloc", "@", &["#"]),
    ("objc_alloc_init", "@", &["#"]),
    ("objc_allocWithZone", "@", &["#"]),
    ("objc_opt_new", "@", &["#"]),
    ("objc_opt_class", "#", &["@"]),
    ("objc_opt_self", "@", &["@"]),
    ("objc_opt_isKindOfClass", "B", &["@", "#"]),
    ("objc_opt_respondsToSelector", "B", &["@", ":"]),
    ("objc_getClass", "#", &["*"]),
    ("objc_lookUpClass", "#", &["*"]),
    ("objc_getMetaClass", "#", &["*"]),
    ("object_getClass", "#", &["@"]),
    ("sel_registerName", ":", &["*"]),
    ("sel_getName", "*", &[":"]),
    ("class_getName", "*", &["#"]),
    ("class_getInstanceMethod", "^v", &["#", ":"]),
    ("class_respondsToSelector", "B", &["#", ":"]),
    ("objc_enumerationMutation", "v", &["@"]),
    ("objc_sync_enter", "i", &["@"]),
    ("objc_sync_exit", "i", &["@"]),
    ("objc_begin_catch", "^v", &["^v"]),
    ("objc_end_catch", "v", &[]),
    ("objc_exception_throw", "v", &["@"]),
    ("objc_terminate", "v", &[]),
    ("objc_autoreleasePoolPush", "^v", &[]),
    ("objc_autoreleasePoolPop", "v", &["^v"]),
    ("objc_setProperty_nonatomic_copy", "v", &["@", ":", "@", "q"]),
    ("objc_setProperty_nonatomic", "v", &["@", ":", "@", "q"]),
    ("objc_setProperty_atomic", "v", &["@", ":", "@", "q"]),
    ("objc_setProperty_atomic_copy", "v", &["@", ":", "@", "q"]),
    ("objc_getProperty", "@", &["@", ":", "q", "B"]),
    ("objc_copyStruct", "v", &["^v", "^v", "q", "B", "B"]),
];

const ARG_NAMES: [&str; 5] = ["obj", "a1", "a2", "a3", "a4"];

fn pieces(name: &str, ret: &str, args: &[&str], tctx: &TypeCtx) -> Option<PrototypePieces> {
    let outtype = decode_ivar_type(ret, tctx)?;
    let mut p = PrototypePieces {
        name: name.to_string(),
        outtype: Some(outtype),
        first_var_arg_slot: -1,
        ..Default::default()
    };
    for (i, a) in args.iter().enumerate() {
        p.intypes.push(decode_ivar_type(a, tctx)?);
        p.innames.push(ARG_NAMES.get(i).unwrap_or(&"a").to_string());
    }
    Some(p)
}

/// The prototypes, by entry address, for every runtime entry point `file`
/// imports — on the `__stubs` entry a call targets and on the pointer slot it
/// reads — and for every `objc_retain_x<N>` / `objc_release_x<N>`, imported
/// (libobjc exports them since objc4-838) or outlined into the image itself.
pub fn prototypes(file: &object::File, bytes: &[u8], arch: &Architecture, tctx: &TypeCtx) -> Vec<(u64, PrototypePieces)> {
    let mut out = Vec::new();
    let arm64 = file.architecture() == object::Architecture::Aarch64;
    for imp in crate::loader::macho_stubs::resolve_macho_imports(file, bytes) {
        if imp.kind != crate::loader::format::ImportSymKind::Import {
            continue;
        }
        let Ok(n) = std::str::from_utf8(&imp.name) else { continue };
        if let Some((name, ret, args)) = RUNTIME.iter().find(|(name, _, _)| *name == n) {
            out.extend(pieces(name, ret, args, tctx).map(|p| (imp.addr, p)));
        } else if let Some(reg) = outlined_register(n).filter(|_| arm64) {
            out.extend(outlined(n, reg, arch, tctx).map(|p| (imp.addr, p)));
        }
    }
    if arm64 {
        for sym in file.symbols() {
            let Ok(n) = sym.name() else { continue };
            let n = n.strip_prefix('_').unwrap_or(n);
            if let Some(reg) = outlined_register(n) {
                out.extend(outlined(n, reg, arch, tctx).map(|p| (sym.address(), p)));
            }
        }
    }
    out
}

/// `objc_retain_x19` → `x19`; `None` for any other name.
pub fn outlined_register(name: &str) -> Option<&str> {
    let rest = name.strip_prefix("objc_retain_").or_else(|| name.strip_prefix("objc_release_"))?;
    let n: u32 = rest.strip_prefix('x')?.parse().ok()?;
    (n <= 28).then_some(rest)
}

/// `id objc_retain_x<N>(id obj @ x<N>)` returning the object in `x0`, while `x<N>`
/// keeps it; a release has no result but reads the same register.
fn outlined(name: &str, reg: &str, arch: &Architecture, tctx: &TypeCtx) -> Option<PrototypePieces> {
    let obj: Rc<Datatype> = decode_ivar_type("@", tctx)?;
    let storage = |register: &str| {
        let vn = arch.get_register_varnode(register.as_bytes()).ok()?;
        let space = vn.space?;
        Some(ParameterPieces {
            addr: Address::new(Rc::clone(&space), vn.offset),
            type_: Some(Rc::clone(&obj)),
            flags: parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::NAMELOCK,
        })
    };
    let retains = name.starts_with("objc_retain_");
    let mut p = PrototypePieces {
        name: name.to_string(),
        outtype: Some(if retains { Rc::clone(&obj) } else { decode_ivar_type("v", tctx)? }),
        intypes: vec![Rc::clone(&obj)],
        innames: vec!["obj".to_string()],
        first_var_arg_slot: -1,
        ..Default::default()
    };
    p.input_storage.push((0, storage(reg)?));
    if retains {
        p.output_storage = Some(storage("x0")?);
    }
    Some(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outlined_names_name_their_register() {
        assert_eq!(outlined_register("objc_retain_x19"), Some("x19"));
        assert_eq!(outlined_register("objc_release_x0"), Some("x0"));
        assert_eq!(outlined_register("objc_retain_x29"), None, "the frame pointer is never used");
        assert_eq!(outlined_register("objc_retain"), None);
        assert_eq!(outlined_register("objc_retain_xa"), None);
    }

    #[test]
    fn runtime_table_has_no_duplicate_and_decodable_shapes() {
        let mut seen = std::collections::HashSet::new();
        for (name, ret, args) in RUNTIME {
            assert!(seen.insert(*name), "{name} listed twice");
            for e in std::iter::once(ret).chain(args.iter()) {
                assert!(matches!(e.as_bytes()[0], b'@' | b'#' | b':' | b'*' | b'v' | b'B' | b'i' | b'q' | b'^'), "{name}: {e}");
            }
        }
    }
}
