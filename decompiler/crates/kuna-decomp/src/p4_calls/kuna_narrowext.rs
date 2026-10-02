//! (kuna) `narrowext`: the extension a calling convention gives an integer
//! narrower than the register that carries it, as an argument or a return value.
//!
//! A compiler spec states one extension per register entry, which cannot say
//! what the RISC-V and LoongArch procedure-call standards say: an integer
//! narrower than 32 bits is extended by the sign of its type, and a 32-bit one
//! is sign-extended to a 64-bit register whatever its sign.  The RISC-V specs
//! state `zero` and the LoongArch ones nothing, so a signed result reads as a
//! large positive one, or the rest of its register as an unassigned piece.
//! MIPS and Apple arm64 compilers extend the same way (to 32 bits only on
//! Apple), but their ABI documents state it for arguments alone, so those two
//! take the rule only under `compiler`.
use crate::{
    dtype::{type_class, type_metatype, Datatype},
    fspec::ParamListStandard,
    infra::architecture::Architecture,
};
use kuna_base::address::Address;
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};

/// The value of `narrowext`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NarrowExtMode {
    /// The compiler spec's extension everywhere.
    Off,
    /// The rule wherever an ABI document states it.
    #[default]
    Abi,
    /// `Abi`, and the rule MIPS and Apple arm64 compilers follow.
    Compiler,
}

impl NarrowExtMode {
    pub fn as_str(self) -> &'static str {
        match self {
            NarrowExtMode::Off => "off",
            NarrowExtMode::Abi => "abi",
            NarrowExtMode::Compiler => "compiler",
        }
    }
}

/// The `narrowext` option parser.
pub struct OptionNarrowExt;

impl OptionNarrowExt {
    pub const NAME: &'static str = "narrowext";

    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(NarrowExtMode, String)> {
        let mode = match p1 {
            "off" => NarrowExtMode::Off,
            "abi" => NarrowExtMode::Abi,
            "compiler" => NarrowExtMode::Compiler,
            other => {
                return Err(kuna_base::error::KunaError::parse(format!(
                    "Unknown narrowext value: {other} (expected off|abi|compiler)"
                )))
            }
        };
        Ok((mode, format!("Narrow integer extension set to {}", mode.as_str())))
    }
}

/// How a narrow integer fills its register: extended by the sign of its type to
/// 32 bits, then sign-extended to the whole register (`Register`), or with the
/// bits above 32 left unspecified (`Word`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Widen {
    Register,
    Word,
}

/// The widening an image's arguments and return values follow, if known.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rules {
    pub input: Option<Widen>,
    pub output: Option<Widen>,
}

/// The rules for `arch` under its `narrowext` value.
pub fn rules(arch: &Architecture) -> Rules {
    let id = arch.archid.as_str();
    let both = |w| Rules { input: Some(w), output: Some(w) };
    match arch.narrow_ext {
        NarrowExtMode::Off => Rules::default(),
        _ if id.starts_with("RISCV:") || id.starts_with("Loongarch:") => both(Widen::Register),
        NarrowExtMode::Abi => Rules::default(),
        NarrowExtMode::Compiler if id.starts_with("MIPS:") => both(Widen::Register),
        NarrowExtMode::Compiler if is_apple_arm64(arch) => both(Widen::Word),
        NarrowExtMode::Compiler => Rules::default(),
    }
}

fn is_apple_arm64(arch: &Architecture) -> bool {
    arch.archid.starts_with("AARCH64:LE:64:AppleSilicon:")
        || (arch.archid.starts_with("AARCH64:")
            && arch
                .translate()
                .loader_rc()
                .try_borrow()
                .is_ok_and(|loader| loader.callee_extends_returns() == Some(true)))
}

/// Whether `ty` is an integer whose sign is known; plain `char` is not, since
/// its sign is the platform's and DWARF folds every character type into it.
fn signed(ty: &Datatype) -> Option<bool> {
    if ty.is_char_print() {
        return None;
    }
    match ty.get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT => Some(true),
        type_metatype::TYPE_UINT | type_metatype::TYPE_ENUM_UINT | type_metatype::TYPE_BOOL => {
            Some(false)
        }
        _ => None,
    }
}

/// The extension `widen` gives a `size`-byte integer of type `ty` in the low
/// bytes of a register entry of `list`, and the storage it fills, or `None`
/// where the rule says nothing.
pub fn extension(
    widen: Widen,
    list: &ParamListStandard,
    addr: &Address,
    size: i32,
    ty: &Datatype,
    res: &mut VarnodeData,
) -> Option<OpCode> {
    let signed = signed(ty)?;
    let entry = list.get_entry().iter().find(|e| {
        e.get_min_size() <= size
            && e.get_align() == 0
            && e.get_join_record().is_none()
            && e.get_type() != type_class::TYPECLASS_FLOAT
            && e.justified_contain(addr, size) == 0
    })?;
    let register = entry.get_size();
    let by_sign = if signed { OpCode::CPUI_INT_SEXT } else { OpCode::CPUI_INT_ZEXT };
    let (op, width) = match widen {
        _ if size >= register => return None,
        Widen::Register if size < 4 => (by_sign, register),
        Widen::Register if size == 4 => (OpCode::CPUI_INT_SEXT, register),
        Widen::Word if size < 4 => (by_sign, 4),
        _ => return None,
    };
    let space = entry.get_space();
    let skip = if space.is_big_endian() { register - width } else { 0 };
    res.space = Some(space.clone());
    res.offset = entry.get_base() + skip as u64;
    res.size = width as u32;
    Some(op)
}

/// [`extension`] under `rule`, falling back to `fallback` (the model's own
/// answer) where the rule says nothing.
pub fn or_model(
    rule: Option<Widen>,
    list: &ParamListStandard,
    addr: &Address,
    size: i32,
    ty: Option<&Datatype>,
    res: &mut VarnodeData,
) -> OpCode {
    rule.zip(ty)
        .and_then(|(w, ty)| extension(w, list, addr, size, ty, res))
        .unwrap_or_else(|| list.assumed_extension(addr, size, res))
}

