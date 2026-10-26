//! (kuna) The `goreg` option: name the goroutine register (`docs/multilang.md`
//! W4, the Go pack's dedicated-register item).
//!
//! ## The gap it closes
//!
//! Under the register ABI every Go function carries the current goroutine in a
//! pinned register — `R14` on amd64, `x28` on arm64, exactly as the vendored
//! golang `register.info` files state (`<current_goroutine register="R14"/>`)
//! — and the runtime reads through it constantly (`stackguard0` checks the
//! `gomorestack` fold leaves in runtime internals, `g.m`, deferred-panic
//! chains). The cspec lists the register as `unaffected` and no `<pentry>`
//! names it, so kuna sees a value that is never a parameter and never written:
//! every access rendered as a phantom local (`v3; // r14`) read through
//! unexplained arithmetic.
//!
//! With the option on (the default) and the golang cspec active, the register
//! becomes a NAMED, TYPED global: its storage joins the global scope (the
//! cspec `<global><register/>` mechanism, applied kuna-side so the vendored
//! spec stays pristine) and a `g` symbol typed `runtime.g *` is mapped over
//! it, type- and name-locked. A `stackguard0` load then renders
//! `g->stackguard0`, and `g->m`, `g->_defer`, … read like the runtime source.
//!
//! ## The `runtime.g` shell
//!
//! The synthesized-shell pattern (`docs/multilang.md` W3, the `libctypes`
//! precedent): `runtime.g` and `runtime.gobuf` are interned with exactly the
//! fields whose offsets the go1.22 runtime source states for the register-ABI
//! era (`src/runtime/runtime2.go`, 64-bit layout, through `goid` at 0x98;
//! `gobuf` is seven words). Every field is word-sized except the two `u32`s
//! (`atomicstatus`, `stackLock`), so the offsets are the running sum the
//! builder computes; a unit test pins them against the source-stated values.
//! Fields past `goid` vary more across Go releases and are deliberately not
//! modeled — an access past 0xa0 renders as an offset off `g`, which is still
//! anchored and honest.
//!
//! ## Why an env bridge
//!
//! The install runs at architecture build (right after the cspec `<global>`
//! decode), upstream of every per-function `option` command — the
//! `goabi`/`sourcelang` seam, and the same [`GOREG_ENV`] pattern. The
//! `Architecture::go_reg` field exists for catalog visibility (`kuna_live_value`),
//! and flipping the option in a live session bridges the env for a later
//! `load file`. Structurally inert off-Go: the gate requires the golang cspec
//! (`archid` compiler field `golang`), which only `goabi auto` on a
//! Go-detected image composes.

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4};

use crate::dtype::{type_metatype, TypeFactory, TypeField};
use crate::options::env_toggle;

/// Environment variable gating the goroutine-register naming. Absent or any
/// non-off token means **enabled**; `off`/`0`/`false` disables.
pub const GOREG_ENV: &str = "KUNA_GOREG";

/// Whether the goroutine-register naming is enabled for this process.
pub fn goreg_enabled() -> bool {
    env_toggle(GOREG_ENV, true)
}

/// Bridge a `set_kuna_option("goreg", val)` toggle to [`GOREG_ENV`] so a later
/// `load file` in the same process sees the choice.
pub fn set_goreg_env(on: bool) {
    std::env::set_var(GOREG_ENV, if on { "on" } else { "off" });
}

/// The goroutine register for a golang-cspec architecture, by SLEIGH language
/// id — the values the vendored `*.register.info` files state
/// (`<current_goroutine register=…/>`). `None` for every non-golang id and for
/// golang ids with no pinned register (x86-32 Go reaches `g` through TLS).
pub fn goroutine_register(archid: &str) -> Option<&'static str> {
    // The compiler field is the id's last `:` token; only the golang cspec
    // carries these models, and only goabi composes it.
    if !archid.ends_with(":golang") {
        return None;
    }
    if archid.starts_with("x86:LE:64:") {
        Some("R14")
    } else if archid.starts_with("AARCH64:LE:64:") {
        Some("x28")
    } else {
        None
    }
}

/// One verified field of a shell: `(name, size, is_pointer)`. Offsets are the
/// running sum (everything is naturally aligned at these sizes and order).
type ShellField = (&'static str, int4, bool);

/// `runtime.gobuf` — seven words (go1.22 `runtime2.go`).
const GOBUF_FIELDS: &[ShellField] = &[
    ("sp", 8, false),
    ("pc", 8, false),
    ("g", 8, false),
    ("ctxt", 8, true),
    ("ret", 8, false),
    ("lr", 8, false),
    ("bp", 8, false),
];

/// `runtime.g` through `goid` (go1.22 `runtime2.go`, 64-bit): the prefix whose
/// offsets are stated in the source ("offset known to liblink"). `sched` is
/// handled apart (it takes the `runtime.gobuf` shell type).
const G_FIELDS_HEAD: &[ShellField] = &[
    ("stack_lo", 8, false),    // 0x00  stack.lo
    ("stack_hi", 8, false),    // 0x08  stack.hi
    ("stackguard0", 8, false), // 0x10
    ("stackguard1", 8, false), // 0x18
    ("_panic", 8, true),       // 0x20
    ("_defer", 8, true),       // 0x28
    ("m", 8, true),            // 0x30
];
const G_FIELDS_TAIL: &[ShellField] = &[
    ("syscallsp", 8, false),   // 0x70
    ("syscallpc", 8, false),   // 0x78
    ("stktopsp", 8, false),    // 0x80
    ("param", 8, true),        // 0x88
    ("atomicstatus", 4, false), // 0x90
    ("stackLock", 4, false),   // 0x94
    ("goid", 8, false),        // 0x98
];

/// The byte offset `sched` sits at, and the sizes the builder must reproduce.
pub const G_SCHED_OFFSET: int4 = 0x38;
/// `runtime.gobuf`'s size (seven words).
pub const GOBUF_SIZE: int4 = 0x38;
/// The modeled `runtime.g` prefix size (through `goid`, padded to its align).
pub const G_SIZE: int4 = 0xa0;

fn push_fields(
    types: &dyn TypeFactory,
    fields: &mut Vec<TypeField>,
    spec: &[ShellField],
    mut offset: int4,
) -> KunaResult<int4> {
    for (name, size, is_ptr) in spec {
        let ct = if *is_ptr {
            let void = types.get_base(1, type_metatype::TYPE_VOID)?;
            types.get_type_pointer(8, void, 1)?
        } else {
            types.get_base(*size, type_metatype::TYPE_UINT)?
        };
        fields.push(TypeField {
            ident: fields.len() as int4,
            offset,
            name: (*name).to_string(),
            field_type: ct,
        });
        offset += size;
    }
    Ok(offset)
}

/// Intern the `runtime.gobuf` + `runtime.g` shells and return `runtime.g *`.
///
/// Idempotent through the factory's name-keyed interning: a second install of
/// the same layouts resolves to the already-interned types.
pub fn g_pointer_type(types: &dyn TypeFactory) -> KunaResult<std::rc::Rc<crate::dtype::Datatype>> {
    let gobuf_shell = types.get_type_struct("runtime.gobuf")?;
    let mut gb = Vec::new();
    let end = push_fields(types, &mut gb, GOBUF_FIELDS, 0)?;
    debug_assert_eq!(end, GOBUF_SIZE);
    let gobuf = types.set_fields_struct_raw(&gobuf_shell, gb, Vec::new(), GOBUF_SIZE, 8, 0)?;

    let g_shell = types.get_type_struct("runtime.g")?;
    let mut gf = Vec::new();
    let mid = push_fields(types, &mut gf, G_FIELDS_HEAD, 0)?;
    debug_assert_eq!(mid, G_SCHED_OFFSET);
    gf.push(TypeField {
        ident: gf.len() as int4,
        offset: G_SCHED_OFFSET,
        name: "sched".to_string(),
        field_type: gobuf,
    });
    let end = push_fields(types, &mut gf, G_FIELDS_TAIL, G_SCHED_OFFSET + GOBUF_SIZE)?;
    debug_assert_eq!(end, 0x98 + 8);
    let g = types.set_fields_struct_raw(&g_shell, gf, Vec::new(), G_SIZE, 8, 0)?;
    types.get_type_pointer(8, g, 1)
}

/// The flags the mapped `g` symbol locks: the name is the point, and the type
/// is what turns an offset read into a named field.
pub fn g_symbol_flags() -> uint4 {
    crate::varnode::varnode_flags::typelock | crate::varnode::varnode_flags::namelock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goroutine_register_is_golang_gated() {
        assert_eq!(goroutine_register("x86:LE:64:default:golang"), Some("R14"));
        assert_eq!(goroutine_register("AARCH64:LE:64:v8A:golang"), Some("x28"));
        // The C cspec, a non-Go image, never names one.
        assert_eq!(goroutine_register("x86:LE:64:default:gcc"), None);
        // x86-32 Go reaches `g` through TLS: golang cspec, no pinned register.
        assert_eq!(goroutine_register("x86:LE:32:default:golang"), None);
        assert_eq!(goroutine_register(""), None);
    }

    #[test]
    fn shell_offsets_match_the_runtime_source() {
        // The go1.22 runtime2.go 64-bit layout this module's doc derives:
        // running sums must land the liblink-stated offsets.
        let mut off = 0;
        let mut at = std::collections::HashMap::new();
        for (name, size, _) in G_FIELDS_HEAD {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(off, G_SCHED_OFFSET);
        off += GOBUF_SIZE;
        for (name, size, _) in G_FIELDS_TAIL {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(at["stackguard0"], 0x10, "offset known to liblink");
        assert_eq!(at["stackguard1"], 0x18, "offset known to liblink");
        assert_eq!(at["_panic"], 0x20, "offset known to liblink");
        assert_eq!(at["m"], 0x30, "offset known to arm liblink");
        assert_eq!(at["syscallsp"], 0x70);
        assert_eq!(at["atomicstatus"], 0x90);
        assert_eq!(at["goid"], 0x98);
        assert!(off <= G_SIZE, "modeled prefix fits the declared size");
        assert_eq!(GOBUF_FIELDS.len(), 7, "gobuf is seven words");
    }
}
