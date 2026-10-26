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

/// Environment variable carrying the image's Go toolchain version
/// (`"<maj>.<min>"`) into the architecture build — set by the console
/// bootstrap from the build-info blob (`kuna-analysis loader/gobuildinfo`)
/// BEFORE the engine is built, because the shell install runs inside the
/// cspec decode, upstream of every per-image fact write. Absent or
/// unparsable means the go1.22 register-ABI baseline.
pub const GOVERSION_ENV: &str = "KUNA_GOVERSION";

/// The release-dependent pieces of the runtime layouts: go1.23 inserted
/// `g.syscallbp`, go1.25 dropped `gobuf.ret` (verified against the vendored
/// go1.22.10 and go1.25.3 sources; 1.23/1.24 carry `syscallbp` with the
/// seven-word gobuf).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GoLayout {
    /// `runtime.gobuf`'s word count: 7 through go1.24, 6 from go1.25.
    pub gobuf_words: int4,
    /// `g.syscallbp` present (go1.23+).
    pub syscallbp: bool,
}

impl GoLayout {
    /// The go1.22 register-ABI baseline (also the unknown-version fallback).
    pub fn baseline() -> GoLayout {
        GoLayout { gobuf_words: 7, syscallbp: false }
    }

    /// The layout a toolchain version implies.
    pub fn from_version(maj: u16, min: u16) -> GoLayout {
        GoLayout {
            gobuf_words: if (maj, min) >= (1, 25) { 6 } else { 7 },
            syscallbp: (maj, min) >= (1, 23),
        }
    }

    /// The layout [`GOVERSION_ENV`] states, else the baseline.
    pub fn from_env() -> GoLayout {
        let Ok(v) = std::env::var(GOVERSION_ENV) else { return GoLayout::baseline() };
        let mut it = v.trim().split('.');
        match (it.next().and_then(|t| t.parse().ok()), it.next().and_then(|t| t.parse().ok())) {
            (Some(maj), Some(min)) => GoLayout::from_version(maj, min),
            _ => GoLayout::baseline(),
        }
    }

    /// `runtime.gobuf`'s byte size under this layout.
    pub fn gobuf_size(self) -> int4 {
        self.gobuf_words * 8
    }
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

/// `runtime.gobuf` through go1.24 — seven words (go1.22 `runtime2.go`).
const GOBUF_FIELDS: &[ShellField] = &[
    ("sp", 8, false),
    ("pc", 8, false),
    ("g", 8, false),
    ("ctxt", 8, true),
    ("ret", 8, false),
    ("lr", 8, false),
    ("bp", 8, false),
];
/// `runtime.gobuf` from go1.25 — `ret` dropped (go1.25 `runtime2.go`).
const GOBUF_FIELDS_125: &[ShellField] = &[
    ("sp", 8, false),
    ("pc", 8, false),
    ("g", 8, false),
    ("ctxt", 8, true),
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
    ("syscallsp", 8, false),   // 0x70 under the baseline layout
    ("syscallpc", 8, false),
    // `syscallbp` is inserted here from go1.23 (layout-dependent).
    ("stktopsp", 8, false),
    ("param", 8, true),
    ("atomicstatus", 4, false),
    ("stackLock", 4, false),
    ("goid", 8, false),
];
/// The `syscallbp` word go1.23 inserted after `syscallpc`.
const G_SYSCALLBP: ShellField = ("syscallbp", 8, false);

/// The byte offset `sched` sits at, and the sizes the builder must reproduce.
pub const G_SCHED_OFFSET: int4 = 0x38;
/// `runtime.gobuf`'s size (seven words).
pub const GOBUF_SIZE: int4 = 0x38;
/// The modeled `runtime.g` prefix size (through `goid`, padded to its align).
pub const G_SIZE: int4 = 0xa0;

/// `runtime.gsignalStack` (`signal_unix.go`, identical in go1.22 and go1.25):
/// the embedded stack bounds plus guards the signal g borrows.
const GSIGSTACK_FIELDS: &[ShellField] = &[
    ("stack_lo", 8, false),
    ("stack_hi", 8, false),
    ("stackguard0", 8, false),
    ("stackguard1", 8, false),
    ("stktopsp", 8, false),
];
/// `runtime.gsignalStack`'s size (five words).
pub const GSIGSTACK_SIZE: int4 = 0x28;

/// `runtime.m` head (`runtime2.go`, 64-bit; field list and order identical in
/// go1.22 and go1.25 — 1.25 merely drops the explicit pad `_ uint32`, which
/// natural alignment reinserts). `g0`/`gsignal` are goroutine pointers but are
/// typed `void *` here: `runtime.g` is completed only after `runtime.m` (the
/// `g.m` field needs the finished m), and a pointer taken to a still-incomplete
/// composite would keep referring to the pre-completion type through the
/// factory's clone-and-re-key completion. The names are the readability win.
const M_FIELDS_HEAD: &[ShellField] = &[
    ("g0", 8, true),     // 0x00  *g, scheduling stack
    // 0x08 `morebuf` (a gobuf) is pushed apart, like g's `sched`.
];
/// `runtime.m` after `morebuf`: `divmod` ("known to liblink") then the
/// debugger-visible block. `goSigStack` is pushed apart (its own shell).
const M_FIELDS_MID: &[ShellField] = &[
    ("divmod", 4, false),  // 0x40  arm div/mod denominator, known to liblink
    ("pad_44", 4, false),  // 0x44  alignment (explicit `_ uint32` in go1.22)
    ("procid", 8, false),  // 0x48
    ("gsignal", 8, true),  // 0x50  *g
];
/// `runtime.m` tail after `goSigStack`: verified identical through
/// `profilehz` on go1.22.10 and go1.25.3, linux/amd64 (`sigmask` is the
/// linux `[2]uint32`; a PE Go binary's `sigset` is empty, shifting everything
/// from `tls` on — the shell is a linux/darwin-era reading aid, and `goreg
/// off` removes it entirely).
const M_FIELDS_TAIL: &[ShellField] = &[
    ("sigmask", 8, false),      // 0x80
    // 0x88 `tls` ([6]uintptr) is pushed apart as an array.
    ("mstartfn", 8, true),      // 0xb8
    ("curg", 8, true),          // 0xc0  *g, the running goroutine
    ("caughtsig", 8, false),    // 0xc8  guintptr (an integer, not a pointer)
    ("p", 8, false),            // 0xd0  puintptr
    ("nextp", 8, false),        // 0xd8  puintptr
    ("oldp", 8, false),         // 0xe0  puintptr
    ("id", 8, false),           // 0xe8  int64
    ("mallocing", 4, false),    // 0xf0
    ("throwing", 4, false),     // 0xf4
    ("preemptoff_ptr", 8, false), // 0xf8  string header
    ("preemptoff_len", 8, false), // 0x100
    ("locks", 4, false),        // 0x108
    ("dying", 4, false),        // 0x10c
    ("profilehz", 4, false),    // 0x110
];
/// `morebuf`'s offset inside `runtime.m`.
pub const M_MOREBUF_OFFSET: int4 = 0x08;
/// `goSigStack`'s offset inside `runtime.m`.
pub const M_GSIGSTACK_OFFSET: int4 = 0x58;
/// `tls`'s offset inside `runtime.m` ([6]uintptr).
pub const M_TLS_OFFSET: int4 = 0x88;
/// The modeled `runtime.m` prefix size (through `profilehz`, padded).
pub const M_SIZE: int4 = 0x118;

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

/// Intern the `runtime.gsignalStack` + `runtime.m` shells and return
/// `runtime.m *`. Built before `runtime.g` so g's `m` field can carry the
/// completed type (see the pointer note on [`M_FIELDS_HEAD`]).
fn m_pointer_type(
    types: &dyn TypeFactory,
    gobuf: std::rc::Rc<crate::dtype::Datatype>,
    layout: GoLayout,
) -> KunaResult<std::rc::Rc<crate::dtype::Datatype>> {
    let gobuf_size = layout.gobuf_size();
    let sig_shell = types.get_type_struct("runtime.gsignalStack")?;
    let mut sf = Vec::new();
    let end = push_fields(types, &mut sf, GSIGSTACK_FIELDS, 0)?;
    debug_assert_eq!(end, GSIGSTACK_SIZE);
    let gsig = types.set_fields_struct_raw(&sig_shell, sf, Vec::new(), GSIGSTACK_SIZE, 8, 0)?;

    let m_shell = types.get_type_struct("runtime.m")?;
    let mut mf = Vec::new();
    let at = push_fields(types, &mut mf, M_FIELDS_HEAD, 0)?;
    debug_assert_eq!(at, M_MOREBUF_OFFSET);
    mf.push(TypeField {
        ident: mf.len() as int4,
        offset: M_MOREBUF_OFFSET,
        name: "morebuf".to_string(),
        field_type: gobuf,
    });
    let at = push_fields(types, &mut mf, M_FIELDS_MID, M_MOREBUF_OFFSET + gobuf_size)?;
    debug_assert_eq!(at, M_GSIGSTACK_OFFSET + gobuf_size - GOBUF_SIZE);
    mf.push(TypeField {
        ident: mf.len() as int4,
        offset: at,
        name: "goSigStack".to_string(),
        field_type: gsig,
    });
    let mut mid_tail = Vec::new();
    let at = push_fields(types, &mut mid_tail, &M_FIELDS_TAIL[..1], at + GSIGSTACK_SIZE)?;
    for mut f in mid_tail {
        f.ident = mf.len() as int4;
        mf.push(f);
    }
    debug_assert_eq!(at, M_TLS_OFFSET + gobuf_size - GOBUF_SIZE);
    let word = types.get_base(8, type_metatype::TYPE_UINT)?;
    let tls = types.get_type_array(6, word)?;
    mf.push(TypeField {
        ident: mf.len() as int4,
        offset: at,
        name: "tls".to_string(),
        field_type: tls,
    });
    let mut tail = Vec::new();
    let end = push_fields(types, &mut tail, &M_FIELDS_TAIL[1..], at + 6 * 8)?;
    for mut f in tail {
        f.ident = mf.len() as int4;
        mf.push(f);
    }
    debug_assert_eq!(end, 0x110 + 4 + gobuf_size - GOBUF_SIZE);
    let m_size = (end + 7) & !7;
    let m = types.set_fields_struct_raw(&m_shell, mf, Vec::new(), m_size, 8, 0)?;
    types.get_type_pointer(8, m, 1)
}

/// Intern the `runtime.gobuf` + `runtime.m` + `runtime.g` shells and return
/// `runtime.g *`.
///
/// Idempotent through the factory's name-keyed interning: a second install of
/// the same layouts resolves to the already-interned types.
pub fn g_pointer_type(
    types: &dyn TypeFactory,
    layout: GoLayout,
) -> KunaResult<std::rc::Rc<crate::dtype::Datatype>> {
    let gobuf_shell = types.get_type_struct("runtime.gobuf")?;
    let mut gb = Vec::new();
    let table = if layout.gobuf_words == 6 { GOBUF_FIELDS_125 } else { GOBUF_FIELDS };
    let end = push_fields(types, &mut gb, table, 0)?;
    debug_assert_eq!(end, layout.gobuf_size());
    let gobuf =
        types.set_fields_struct_raw(&gobuf_shell, gb, Vec::new(), layout.gobuf_size(), 8, 0)?;

    let m_ptr = m_pointer_type(types, std::rc::Rc::clone(&gobuf), layout)?;

    let g_shell = types.get_type_struct("runtime.g")?;
    let mut gf = Vec::new();
    let mid = push_fields(types, &mut gf, G_FIELDS_HEAD, 0)?;
    debug_assert_eq!(mid, G_SCHED_OFFSET);
    // `g.m` carries the completed `runtime.m *`, so `g->m->curg`-style chains
    // render named end to end.
    if let Some(mf) = gf.iter_mut().find(|f| f.name == "m") {
        mf.field_type = m_ptr;
    }
    gf.push(TypeField {
        ident: gf.len() as int4,
        offset: G_SCHED_OFFSET,
        name: "sched".to_string(),
        field_type: gobuf,
    });
    let at =
        push_fields(types, &mut gf, &G_FIELDS_TAIL[..2], G_SCHED_OFFSET + layout.gobuf_size())?;
    let at = if layout.syscallbp {
        push_fields(types, &mut gf, &[G_SYSCALLBP], at)?
    } else {
        at
    };
    let end = push_fields(types, &mut gf, &G_FIELDS_TAIL[2..], at)?;
    let g_size = (end + 7) & !7;
    debug_assert!(end <= g_size && g_size <= G_SIZE + 8);
    let g = types.set_fields_struct_raw(&g_shell, gf, Vec::new(), g_size, 8, 0)?;
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

    #[test]
    fn m_shell_offsets_match_the_runtime_source() {
        // runtime2.go `struct m`, 64-bit linux: the field list and order are
        // identical in go1.22.10 and go1.25.3 (verified against both vendored
        // GOROOTs; 1.25 drops the explicit `_ uint32` pad that alignment
        // reinserts). Running sums must land the source-derived offsets.
        let mut at = std::collections::HashMap::new();
        let mut off = 0;
        for (name, size, _) in M_FIELDS_HEAD {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(off, M_MOREBUF_OFFSET);
        at.insert("morebuf", off);
        off += GOBUF_SIZE;
        for (name, size, _) in M_FIELDS_MID {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(off, M_GSIGSTACK_OFFSET);
        at.insert("goSigStack", off);
        off += GSIGSTACK_SIZE;
        for (name, size, _) in &M_FIELDS_TAIL[..1] {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(off, M_TLS_OFFSET);
        at.insert("tls", off);
        off += 6 * 8;
        for (name, size, _) in &M_FIELDS_TAIL[1..] {
            at.insert(*name, off);
            off += size;
        }
        assert_eq!(at["g0"], 0x0);
        assert_eq!(at["divmod"], 0x40, "known to liblink");
        assert_eq!(at["procid"], 0x48);
        assert_eq!(at["gsignal"], 0x50);
        assert_eq!(at["sigmask"], 0x80, "linux [2]uint32 sigset");
        assert_eq!(at["mstartfn"], 0xb8);
        assert_eq!(at["curg"], 0xc0);
        assert_eq!(at["p"], 0xd0);
        assert_eq!(at["id"], 0xe8);
        assert_eq!(at["locks"], 0x108);
        assert!(off <= M_SIZE, "modeled prefix fits the declared size");
        // gsignalStack is five words in both sources (signal_unix.go).
        assert_eq!(GSIGSTACK_FIELDS.len() as i32 * 8, GSIGSTACK_SIZE);
    }

    #[test]
    fn layouts_follow_the_release_drift() {
        // go1.22: seven-word gobuf, no syscallbp (the baseline).
        assert_eq!(GoLayout::from_version(1, 22), GoLayout::baseline());
        // go1.23/1.24: syscallbp inserted, gobuf still seven words.
        assert_eq!(GoLayout::from_version(1, 23), GoLayout { gobuf_words: 7, syscallbp: true });
        assert_eq!(GoLayout::from_version(1, 24), GoLayout { gobuf_words: 7, syscallbp: true });
        // go1.25: gobuf.ret dropped.
        assert_eq!(GoLayout::from_version(1, 25), GoLayout { gobuf_words: 6, syscallbp: true });
        assert_eq!(GoLayout::from_version(1, 25).gobuf_size(), 0x30);
        // Unknown / future majors stay conservative only in the fields they
        // state; the env fallback is the baseline.
        assert_eq!(GoLayout::baseline().gobuf_size(), GOBUF_SIZE);
        // The 1.25 g tail keeps the liblink-stable anchors: with the six-word
        // gobuf and syscallbp inserted, stktopsp..goid land back on the
        // baseline offsets (0x80..0x98) and only syscallsp/pc move.
        let l = GoLayout::from_version(1, 25);
        let sched_end = G_SCHED_OFFSET + l.gobuf_size(); // 0x68
        assert_eq!(sched_end, 0x68);
        let stktopsp = sched_end + 8 + 8 + 8; // syscallsp, syscallpc, syscallbp
        assert_eq!(stktopsp, 0x80);
        // And the 1.25 m: curg shifts to 0xb8 (one word below the baseline).
        assert_eq!(0xc0 + l.gobuf_size() - GOBUF_SIZE, 0xb8);
    }
}
