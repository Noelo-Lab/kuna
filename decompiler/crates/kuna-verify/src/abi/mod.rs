//! What kuna claims about a function's interface, and what the calling
//! convention promises regardless of the claim.

pub mod calls;
pub mod contract;
pub mod divergence;

use std::rc::Rc;

use kuna_console::engine::ConsoleProgram;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces};
use kuna_base::space::spacetype;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

use crate::contract::{CKind, CSizes, CType};
use crate::state::{Env, Extension};
use crate::storage::Storage;
use crate::value::mask;

/// kuna's prototype for the function: its return storage and signedness, its
/// parameters, and the text the report prints.
pub struct Claim {
    pub storage: Option<Storage>,
    pub signed: bool,
    /// The claimed return is a float.
    pub float: bool,
    pub text: String,
    pub params: Vec<(String, Storage)>,
}

fn is_signed(t: &Datatype) -> bool {
    matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT)
}

/// What the function claims to return, and where. A return kuna folded to a
/// constant keeps the storage its model gives the return type.
pub fn claim(prog: &ConsoleProgram, fd: &Funcdata) -> Claim {
    let proto = fd.get_func_proto();
    let out = proto.get_output();
    let out_type = proto.get_output_type();
    let meta = out_type.map(|t| t.get_metatype()).unwrap_or(type_metatype::TYPE_VOID);
    let addr = out.get_address();
    let storage = match meta {
        type_metatype::TYPE_VOID => None,
        _ if !addr.is_invalid() && out.get_size() > 0 => Storage::at(&addr, out.get_size() as u32),
        _ => None,
    };
    let storage = match (storage, out_type) {
        (Some(st), Some(t)) if matches!(st.space.get_type(), spacetype::IPTR_CONSTANT | spacetype::IPTR_INTERNAL) => {
            Some(model_output(prog, fd, t.clone(), st.size).unwrap_or(st))
        }
        (st, _) => st,
    };
    let type_name = |t: Option<&Rc<Datatype>>| {
        t.map(|t| t.get_display_name().to_string()).filter(|n| !n.is_empty()).unwrap_or_else(|| "?".into())
    };
    let mut params = Vec::new();
    let mut decl = Vec::new();
    for i in 0..proto.num_params() {
        let Some(p) = proto.get_param(i) else { continue };
        let pname = if p.get_name().is_empty() { format!("a{i}") } else { p.get_name().to_string() };
        decl.push(format!("{} {}", type_name(p.get_type()), pname));
        if let Some(st) = Storage::at(&p.get_address(), p.get_size() as u32) {
            params.push((pname, st));
        }
    }
    let ret = if storage.is_none() { "void".to_string() } else { type_name(out_type) };
    Claim {
        storage,
        signed: out_type.is_some_and(|t| is_signed(t)),
        float: meta == type_metatype::TYPE_FLOAT,
        text: format!("{} {}({})", ret, fd.get_name(), decl.join(", ")),
        params,
    }
}

/// The C integer sizes the contract reader needs, from the program's types.
/// Whether plain `char` is signed on the target `archid` names
/// (`AARCH64:LE:64:AppleSilicon:default`): the System V ABIs of ARM, AArch64,
/// PowerPC, RISC-V and s390 make it unsigned; Apple's and Windows' ARM ABIs
/// and x86 make it signed. A contract can override it (`-funsigned-char`).
pub fn char_signed(archid: &str) -> bool {
    let mut parts = archid.split(':');
    let processor = parts.next().unwrap_or("");
    let rest: Vec<&str> = parts.collect();
    let signs_on_arm = rest.iter().any(|p| *p == "AppleSilicon" || *p == "windows");
    let unsigned_family = ["ARM", "AARCH64", "PowerPC", "RISCV", "s390"].iter().any(|p| processor.starts_with(p));
    !unsigned_family || signs_on_arm
}

pub fn c_sizes(prog: &ConsoleProgram) -> CSizes {
    let t = prog.arch().types();
    CSizes {
        int: t.get_size_of_int() as u32,
        long: t.get_size_of_long() as u32,
        long_long: t.get_size_of_long_long() as u32,
        pointer: t.get_size_of_pointer() as u32,
        char_signed: char_signed(&prog.arch().archid),
    }
}

/// Where the function's prototype model returns a value of this type: an
/// integer, or a pointer, which some conventions return elsewhere.
pub fn return_storage(prog: &ConsoleProgram, fd: &Funcdata, ty: &CType) -> Result<Storage, String> {
    let arch = prog.arch();
    let types = arch.types();
    let dt = if ty.kind == CKind::Pointer {
        let word = arch.manage().get_default_data_space().map_or(1, |s| s.get_word_size());
        types
            .get_type_void()
            .and_then(|v| types.get_type_pointer(ty.size as i32, v, word))
            .map_err(|e| format!("no {}-byte pointer type: {}", ty.size, e.explain()))?
    } else {
        let meta = match (&ty.kind, ty.signed) {
            (CKind::Boolean, _) => type_metatype::TYPE_BOOL,
            (_, true) => type_metatype::TYPE_INT,
            (_, false) => type_metatype::TYPE_UINT,
        };
        types
            .get_base(ty.size as i32, meta)
            .map_err(|e| format!("no {}-byte scalar type: {}", ty.size, e.explain()))?
    };
    model_output(prog, fd, dt, ty.size).map_err(|e| format!("`{}`: {e}", ty.spelling))
}

/// Where the function's prototype model returns a value of type `dt`.
fn model_output(prog: &ConsoleProgram, fd: &Funcdata, dt: Rc<Datatype>, size: u32) -> Result<Storage, String> {
    let arch = prog.arch();
    let pieces = PrototypePieces { outtype: Some(dt), first_var_arg_slot: -1, ..Default::default() };
    let mut res: Vec<ParameterPieces> = Vec::new();
    fd.get_func_proto()
        .model()
        .assign_parameter_storage(&pieces, &mut res, false, arch.types(), arch.manage())
        .map_err(|e| format!("model assigns no storage: {}", e.explain()))?;
    let p = res.first().ok_or("model assigned no output")?;
    Storage::at(&p.addr, size).ok_or_else(|| "model output has no space".to_string())
}

/// C's conversion of kuna's claimed return to the contract type: extend by the
/// claimed type's signedness, or truncate.
pub fn convert(v: u128, from: u32, signed: bool, to: u32) -> u128 {
    let v = v & mask(from);
    let negative = signed && from > 0 && from < 16 && (v >> (from * 8 - 1)) & 1 == 1;
    let ext = if negative { v | !mask(from) } else { v };
    ext & mask(to)
}

/// The extensions the calling convention promises for the claimed parameters:
/// a 32-bit `int` in a 64-bit RISC-V or MIPS register arrives sign-extended.
/// Inputs that break the promise are not inputs a conforming caller can pass,
/// so both sides start from inputs that keep it. The rule is the one the ABI
/// documents (`narrowext abi`), whatever the run's own `narrowext` value, so a
/// run with the option off is checked against the real convention.
pub fn extensions(fd: &Funcdata, env: &Env, prog: &ConsoleProgram) -> Vec<Extension> {
    use kuna_decomp::kuna_narrowext::{or_model, rules_for, NarrowExtMode};
    let proto = fd.get_func_proto();
    let rule = rules_for(prog.arch(), NarrowExtMode::Abi).input;
    let list = proto.model().input();
    let mut out = Vec::new();
    for i in 0..proto.num_params() {
        let Some(p) = proto.get_param(i) else { continue };
        let addr = p.get_address();
        let Some(space) = addr.get_space() else { continue };
        if space.get_index() != env.register.get_index() {
            continue;
        }
        let size = p.get_size();
        let mut res = kuna_num::pcoderaw::VarnodeData::default();
        let (ext, _) = or_model(rule, list, &addr, size, p.get_type().map(|t| t.as_ref()), &mut res);
        let signed = match ext {
            OpCode::CPUI_INT_SEXT => true,
            OpCode::CPUI_INT_ZEXT => false,
            OpCode::CPUI_PIECE => p.get_type().is_some_and(|t| is_signed(t)),
            _ => continue,
        };
        let same_space = res.space.as_ref().is_some_and(|s| s.get_index() == space.get_index());
        if same_space && res.size > size as u32 {
            out.push(Extension {
                space: space.get_index(),
                container: (res.offset, res.size),
                param: (addr.get_offset(), size as u32),
                signed,
                boolean: false,
            });
        }
    }
    out
}

/// The extension the ABI promises for a callee's return value in a register
/// wider than it, as `narrowext abi` documents it.
pub fn return_extension(env: &Env, proto: &kuna_decomp::fspec::FuncProto) -> Option<Extension> {
    use kuna_decomp::kuna_narrowext::{or_model, rules_for, NarrowExtMode};
    let rule = rules_for(env.prog.arch(), NarrowExtMode::Abi).output;
    let out = proto.get_output();
    let addr = out.get_address();
    let space = addr.get_space()?;
    let size = out.get_size();
    if space.get_index() != env.register.get_index() || size <= 0 {
        return None;
    }
    let mut res = kuna_num::pcoderaw::VarnodeData::default();
    let (ext, _) = or_model(rule, proto.model().output(), &addr, size, out.get_type().map(|t| t.as_ref()), &mut res);
    let boolean = out.get_type().is_some_and(|t| t.get_metatype() == kuna_decomp::dtype::type_metatype::TYPE_BOOL);
    let signed = match ext {
        OpCode::CPUI_INT_SEXT => true,
        OpCode::CPUI_INT_ZEXT => false,
        OpCode::CPUI_PIECE => out.get_type().is_some_and(|t| is_signed(t)),
        _ if boolean => false,
        _ => return None,
    };
    let param = (addr.get_offset(), size as u32);
    let wider = res.space.as_ref().is_some_and(|s| s.get_index() == space.get_index()) && res.size > size as u32;
    (wider || boolean).then(|| Extension {
        space: space.get_index(),
        container: if wider { (res.offset, res.size) } else { param },
        param,
        signed,
        boolean,
    })
}

/// Input registers the body reads that no declared parameter covers (#875's
/// shape). Reported, never a verdict: a custom ABI reads them legitimately.
pub fn unclaimed_inputs(prog: &ConsoleProgram, env: &Env, fd: &Funcdata, claim: &Claim) -> Vec<String> {
    unclaimed_input_storage(prog, env, fd, claim)
        .iter()
        .map(|s| format!("reads {}, which no declared parameter covers", s.name(prog)))
        .collect()
}

/// The registers the body reads at entry that no claimed parameter covers.
pub fn unclaimed_input_storage(prog: &ConsoleProgram, env: &Env, fd: &Funcdata, claim: &Claim) -> Vec<Storage> {
    let params: Vec<Storage> = claim.params.iter().flat_map(|(_, s)| s.pieces(prog)).collect();
    let sp = Storage::new(&env.register, env.sp.0, env.sp.1);
    let mut found: Vec<Storage> = Vec::new();
    for i in 0..fd.bblocks_get_size() {
        for op in fd.bb_ops(fd.bblocks_get_block(i)) {
            let Some(o) = fd.obank().get(op) else { continue };
            for k in 0..o.num_input() {
                let Some(v) = o.get_in(k).and_then(|id| fd.vbank().get(id)) else { continue };
                if !v.is_input() {
                    continue;
                }
                let Some(st) = Storage::at(v.get_addr(), v.get_size() as u32) else { continue };
                if st.space.get_index() != env.register.get_index()
                    || st.overlaps(&sp)
                    || params.iter().any(|p| p.covers(&st))
                    || found.iter().any(|f| f.covers(&st))
                {
                    continue;
                }
                found.push(st);
            }
        }
    }
    found
}

#[cfg(test)]
mod char_tests {
    use super::char_signed;

    #[test]
    fn plain_char_by_target() {
        assert!(char_signed("x86:LE:64:default:gcc"));
        assert!(!char_signed("AARCH64:LE:64:v8A:default"));
        assert!(!char_signed("ARM:LE:32:v8:default"));
        assert!(char_signed("AARCH64:LE:64:AppleSilicon:default"));
        assert!(char_signed("AARCH64:LE:64:v8A:windows"));
        assert!(!char_signed("PowerPC:BE:64:default:default"));
        assert!(!char_signed("RISCV:LE:64:RV64GC:gcc"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_follows_the_claimed_signedness() {
        assert_eq!(convert(0x8000_0000, 4, true, 8), 0xffff_ffff_8000_0000);
        assert_eq!(convert(0x8000_0000, 4, false, 8), 0x8000_0000);
        assert_eq!(convert(0x1_2345_6789, 8, true, 4), 0x2345_6789);
    }
}
