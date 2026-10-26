//! (kuna) Swift symbol demangling — a bounded, name-only subset
//! (`docs/multilang.md`, the Swift pack's first prerequisite).
//!
//! ## Why a port and not a crate
//!
//! No Swift analog of [`rustc_demangle`]/[`cpp_demangle`] exists: the
//! Vector35 `swift-demangler` crate builds the real C++ demangler through
//! cmake (a native-toolchain dependency this workspace does not carry, and its
//! floor rustc exceeds this repo's), and the pure-Rust alternatives are
//! single-author 0.x crates whose coverage is unvetted — for symbol RENAMING a
//! wrong answer is strictly worse than none. So this is the repo's usual move
//! (the MSVC signature recovery precedent): port the subset kuna needs, with a
//! **parse-or-passthrough** contract — every name this module cannot derive
//! with certainty comes back `None` and the symbol keeps its mangled spelling.
//!
//! ## The subset (grammar per `swift/docs/ABI/Mangling.rst`, hand-derived)
//!
//! Accepted shapes, v5 (`$s`, `$S`, Mach-O `_$s`/`_$S`) and the Swift-4 `_T0`
//! spelling of the same body grammar:
//!
//! - a **context chain**: length-prefixed ASCII identifiers, each optionally
//!   kinded `V`/`C`/`O` (struct/class/enum; the stdlib module contraction `s`
//!   reads as `Swift`), e.g. `4main5PointV` → `main.Point`;
//! - a bare nominal reference, or one carrying a recognized **global marker**
//!   (`N` type metadata, `Mn` nominal descriptor, `Ma` metadata accessor,
//!   `Mf` full metadata, `Mm` metaclass, `WV` value witness table), rendered
//!   as a dotted suffix: `$s4main5PointVN` → `main.Point.type_metadata`;
//! - a **function**: declaration name, then an argument-label run, then an
//!   opaque-but-validated signature ending in `F` (`Z` static marker
//!   tolerated), rendered `main.greet(_:for:)` — labels are kept only when
//!   the whole label run parsed as plain identifiers/`_` slots **and** the
//!   signature region that follows starts with a known type-opening character;
//!   a label-less function additionally requires a digit-free signature, since
//!   a digit there could open an un-labeled custom parameter type whose path
//!   components would be indistinguishable from labels;
//! - **accessors** `vg`/`vs` (`x.getter`/`x.setter`), **constructors**
//!   `fC`/`fc` (the captured head of the label run is really the first label;
//!   the name is `init`), **destructors** `fd`/`fD` (`deinit`).
//!
//! Everything else — punycode identifiers (`00…`), generic signatures whose
//! shapes fall outside the above, protocol witnesses/thunks (`TW`, `TA`, …),
//! specializations (`Tf…`) — passes through unchanged. The pack proper widens
//! coverage; this module's invariant is that what it does return is right.
//!
//! The corpus in the unit tests is hand-derived from the mangling grammar
//! (each case cites its production) rather than copied from the Swift
//! project's test suite, so no third-party fixture text is vendored.

/// The renderings of the recognized global markers (suffix → dotted tag).
const GLOBAL_MARKERS: [(&str, &str); 6] = [
    ("N", "type_metadata"),
    ("Mn", "nominal_descriptor"),
    ("Ma", "metadata_accessor"),
    ("Mf", "full_metadata"),
    ("Mm", "metaclass"),
    ("WV", "value_witness_table"),
];

/// Characters that may open a function-signature region in the shapes this
/// subset accepts: the empty tuple / no-label marker `y`, a known-type
/// shortcut `S…`, the stdlib contraction `s`, a substitution backreference
/// `A…`, or a generic parameter `x`.
fn opens_signature(b: u8) -> bool {
    matches!(b, b'y' | b'S' | b's' | b'A' | b'x')
}

/// Is every byte legal inside a mangled body (conservative alphabet)?
fn mangle_alphabet(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$')
}

/// Strip the Swift prefix, normalizing the Mach-O platform underscore and the
/// Swift-4 `_T0` spelling onto one body grammar. `None` when `raw` does not
/// carry a Swift prefix at all.
fn strip_prefix(raw: &str) -> Option<&str> {
    let bare = raw.strip_prefix('_').unwrap_or(raw);
    bare.strip_prefix("$s")
        .or_else(|| bare.strip_prefix("$S"))
        .or_else(|| bare.strip_prefix("_T0"))
        .or_else(|| raw.strip_prefix("_T0"))
}

/// One parsed path component.
struct Component {
    name: String,
    /// `true` when the component carried a nominal kind (`V`/`C`/`O`) — i.e.
    /// it is a context the entity lives in, not the entity's own name.
    kinded: bool,
}

/// Parse a length-prefixed ASCII identifier at `s`; returns `(ident, rest)`.
/// Refuses punycode (`0`-led counts) and any byte outside the identifier set.
fn identifier(s: &str) -> Option<(&str, &str)> {
    let digits = s.bytes().take_while(|b| b.is_ascii_digit()).count();
    if digits == 0 || s.as_bytes()[0] == b'0' {
        return None;
    }
    let len: usize = s[..digits].parse().ok()?;
    let rest = &s[digits..];
    if len == 0 || rest.len() < len {
        return None;
    }
    let (ident, rest) = rest.split_at(len);
    ident
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        .then_some((ident, rest))
}

/// Demangle `raw` to kuna's **name-only** form (`module.Type.member(...)`),
/// or `None` for any name outside the subset — never a guess.
pub fn demangle_swift_name(raw: &str) -> Option<String> {
    let body = strip_prefix(raw)?;
    if body.is_empty() || !mangle_alphabet(body) {
        return None;
    }

    // The context chain. Grammar order: the MODULE identifier (never kinded —
    // modules carry no nominal letter), then zero or more `V`/`C`/`O`-kinded
    // nominal contexts, then the entity's own (unkinded) declaration name.
    // The stdlib contraction `s` stands in for the module `Swift`.
    let mut rest = body;
    let mut parts: Vec<Component> = Vec::new();
    if let Some(r) = rest.strip_prefix('s') {
        // Only a module position reads the contraction, and only when an
        // identifier follows (`$ss5print…`).
        if r.as_bytes().first().is_some_and(|b| b.is_ascii_digit()) {
            parts.push(Component { name: "Swift".into(), kinded: true });
            rest = r;
        }
    } else if let Some((module, r)) = identifier(rest) {
        parts.push(Component { name: module.into(), kinded: true });
        rest = r;
    } else {
        return None;
    }
    while let Some((ident, mut r)) = identifier(rest) {
        let mut kinded = false;
        if let Some(b) = r.as_bytes().first() {
            if matches!(b, b'V' | b'C' | b'O') {
                kinded = true;
                r = &r[1..];
            }
        }
        parts.push(Component { name: ident.into(), kinded });
        rest = r;
        if !kinded {
            break; // the declaration name ends the chain
        }
    }
    // A lone module (or the bare contraction) names nothing in this subset.
    if parts.len() < 2 {
        return None;
    }

    let path = |ps: &[Component]| ps.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(".");

    // Pure nominal reference, bare or with a recognized global marker.
    let all_kinded = parts.last().map(|c| c.kinded).unwrap_or(false);
    if all_kinded {
        if rest.is_empty() {
            return Some(path(&parts));
        }
        if let Some((_, tag)) = GLOBAL_MARKERS.iter().find(|(m, _)| *m == rest) {
            return Some(format!("{}.{}", path(&parts), tag));
        }
        // A destructor hangs directly off its nominal context (no declaration
        // name of its own): `…CfD` → `….deinit`.
        if rest == "fD" || rest == "fd" {
            return Some(format!("{}.deinit", path(&parts)));
        }
        // A kinded chain followed by unrecognized machinery (witnesses,
        // conformances, …): out of subset.
        return None;
    }

    // The last component is a declaration name; what follows is the label run
    // plus a signature, or accessor/constructor machinery.
    let mut labels: Vec<String> = Vec::new();
    loop {
        if let Some(r) = rest.strip_prefix('_') {
            // `_` is a missing-label slot only while label-run material (more
            // labels or a signature opener) follows; a trailing `_` is not.
            if r.as_bytes().first().is_some_and(|&b| b.is_ascii_digit() || opens_signature(b)) {
                labels.push("_".into());
                rest = r;
                continue;
            }
        }
        if let Some((ident, r)) = identifier(rest) {
            labels.push(ident.into());
            rest = r;
            continue;
        }
        break;
    }

    // Accessors: name + digit-free type + `vg`/`vs`.
    if labels.is_empty() {
        for (suffix, tag) in [("vg", "getter"), ("vs", "setter")] {
            if let Some(sig) = rest.strip_suffix(suffix) {
                if !sig.bytes().any(|b| b.is_ascii_digit())
                    && sig.as_bytes().first().copied().map(opens_signature).unwrap_or(false)
                {
                    return Some(format!("{}.{}", path(&parts), tag));
                }
            }
        }
    }

    // Constructors / destructors: the captured declaration name was really the
    // first label (an init has no declName of its own).
    for (suffix, name, keep_labels) in
        [("fC", "init", true), ("fc", "init", true), ("fD", "deinit", false), ("fd", "deinit", false)]
    {
        if let Some(sig) = rest.strip_suffix(suffix) {
            let ctx = &parts[..parts.len() - 1];
            if ctx.is_empty() || !ctx.last().unwrap().kinded {
                return None; // an init outside a nominal context is not ours
            }
            if !sig.bytes().any(|b| b.is_ascii_digit()) {
                let mut ls: Vec<String> = Vec::new();
                if keep_labels {
                    ls.push(parts.last().unwrap().name.clone());
                    ls.extend(labels.iter().cloned());
                    return Some(format!(
                        "{}.{}({})",
                        path(ctx),
                        name,
                        ls.iter().map(|l| format!("{l}:")).collect::<String>()
                    ));
                }
                return Some(format!("{}.{}", path(ctx), name));
            }
            return None;
        }
    }

    // Plain function: validated signature ending in `F` (optional `Z` static).
    let sig = rest.strip_suffix('Z').unwrap_or(rest);
    let Some(sig) = sig.strip_suffix('F') else { return None };
    let sig_ok = if labels.is_empty() {
        // No labels parsed: a digit anywhere could be an un-labeled custom
        // parameter type whose path components are indistinguishable from
        // labels, so only digit-free signatures are accepted.
        !sig.bytes().any(|b| b.is_ascii_digit())
    } else {
        // Labels parsed: the run ended at a signature opener by construction;
        // repeat counts and substitution indices inside the signature are fine.
        true
    };
    if !sig_ok || !sig.as_bytes().first().copied().map(opens_signature).unwrap_or(false) {
        return None;
    }
    if labels.is_empty() {
        return Some(path(&parts));
    }
    Some(format!(
        "{}({})",
        path(&parts),
        labels.iter().map(|l| format!("{l}:")).collect::<String>()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hand-derived corpus; each case cites the grammar production it uses
    /// (`swift/docs/ABI/Mangling.rst`).
    #[test]
    fn subset_demangles_the_common_shapes() {
        for (raw, want) in [
            // global ::= entity; plain function, empty signature `yyF`.
            ("$s4main5helloyyF", "main.hello"),
            ("_$s4main5helloyyF", "main.hello"),   // Mach-O spelling
            ("_T04main5helloyyF", "main.hello"),   // Swift-4 prefix
            ("__T04main5helloyyF", "main.hello"),  // Mach-O Swift-4
            ("$S4main5helloyyF", "main.hello"),    // 4.2 uppercase prefix
            // static function: trailing `Z`.
            ("$s4main5helloyyFZ", "main.hello"),
            // stdlib contraction + label run + substitution-bearing signature.
            (
                "$ss5print_9separator10terminatoryypd_S2StF",
                "Swift.print(_:separator:terminator:)",
            ),
            // nominal references, bare and with global markers.
            ("$s4main5PointV", "main.Point"),
            ("$s4main5PointVN", "main.Point.type_metadata"),
            ("$s4main5PointVMn", "main.Point.nominal_descriptor"),
            ("$s4main5PointVMa", "main.Point.metadata_accessor"),
            ("$s4main5PointVWV", "main.Point.value_witness_table"),
            ("$s4main7MyClassC", "main.MyClass"),
            ("$s4main4KindO", "main.Kind"),
            // nested nominal context + method with labels.
            ("$s4main5PointV4move2byySi_tF", "main.Point.move(by:)"),
            // accessor: Point.x getter (`Sivg`).
            ("$s4main5PointV1xSivg", "main.Point.x.getter"),
            ("$s4main5PointV1xSivs", "main.Point.x.setter"),
            // constructor: the head of the label run is the first label.
            ("$s4main5PointV1x1yACyycfC", "main.Point.init(x:y:)"),
            // destructor.
            ("$s4main7MyClassCfD", "main.MyClass.deinit"),
        ] {
            assert_eq!(
                demangle_swift_name(raw).as_deref(),
                Some(want),
                "{raw} demangles in-subset"
            );
        }
    }

    /// Parse-or-passthrough: everything outside the subset answers `None`,
    /// byte-identical passthrough being the caller's contract.
    #[test]
    fn out_of_subset_names_pass_through() {
        for raw in [
            // not Swift at all
            "main",
            "_ZN4core9panicking5panic17h0123456789abcdefE",
            "$LN3",
            "_RNvCs1234_4core5panic",
            // punycode identifier (0-led count)
            "$s4main00abcyyF",
            // protocol witness / thunk machinery
            "$s4main5PointVAA8MovableAAWP",
            "$s4main5helloyyFTW",
            // function-signature specialization (digits past the terminal)
            "$s4main5helloyyFTf4d_n",
            // label-less function whose signature carries digits (could be an
            // un-labeled custom parameter type: refuse, never guess)
            "$s4main3useyy4main5OtherVF",
            // truncated / malformed
            "$s4main",
            "$s",
            "$s99zz",
            // kinded chain followed by unrecognized machinery
            "$s4main5PointVMc",
        ] {
            assert_eq!(demangle_swift_name(raw), None, "{raw} must pass through");
        }
    }

    /// The identifier scanner refuses punycode and oversize counts outright.
    #[test]
    fn identifier_scanner_bounds() {
        assert_eq!(identifier("4mainrest"), Some(("main", "rest")));
        assert!(identifier("04ab").is_none(), "punycode counts refuse");
        assert!(identifier("9ab").is_none(), "oversize count refuses");
        assert!(identifier("xmain").is_none(), "no count, no identifier");
    }
}
