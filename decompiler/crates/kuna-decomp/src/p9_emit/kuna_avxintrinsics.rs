//! (kuna `avxintrinsics`) Intel intrinsic names for the SLEIGH AVX user-ops.
//!
//! The x86 SLEIGH spec lifts most AVX2, FMA and AVX-512 instructions to opaque
//! user-ops named after the mnemonic and its extension (`vpaddd_avx2`,
//! `vfmadd132ps_fma`, `vaddps_avx512f`). This maps each one to the Intel
//! intrinsic a programmer writes (`_mm256_add_epi32`, `_mm256_fmadd_ps`,
//! `_mm512_add_ps`) and the order its arguments go in. The vector width comes
//! from the user-op's result size (16, 32 or 64 bytes). The FMA forms are the
//! one reordering: `vfmadd132ps(a, b, c)` computes `a*c + b`, so it prints as
//! `_mm256_fmadd_ps(a, c, b)`. The `lanefuse` masked forms (`<name>_mask`,
//! `<name>_maskz`) take the intrinsic's `_mask_`/`_maskz_` infix and its
//! leading source and mask arguments.

/// The intrinsic name and the order of the user-op's arguments (0-based,
/// after the user-op id) for the user-op `name` whose result is `size` bytes.
pub fn intrinsic(name: &str, size: i32) -> Option<(String, Vec<usize>)> {
    // The `lanefuse` masked forms: `(src, k, operands...)` / `(k, operands...)`.
    if let Some(base) = name.strip_suffix("_maskz") {
        let (intr, order) = intrinsic(base, size)?;
        let (head, tail) = intr.split_once('_').and_then(|(_, r)| r.split_once('_'))?;
        let named = format!("_{head}_maskz_{tail}");
        return Some((named, std::iter::once(0).chain(order.into_iter().map(|a| a + 1)).collect()));
    }
    if let Some(base) = name.strip_suffix("_mask") {
        let (intr, order) = intrinsic(base, size)?;
        let (head, tail) = intr.split_once('_').and_then(|(_, r)| r.split_once('_'))?;
        let named = format!("_{head}_mask_{tail}");
        return Some((named, [0, 1].into_iter().chain(order.into_iter().map(|a| a + 2)).collect()));
    }
    let (mnem, ext) = name.rsplit_once('_')?;
    if !matches!(ext, "avx" | "avx2" | "fma" | "avx512f" | "avx512bw" | "avx512dq" | "avx512vl" | "avx512cd") {
        return None;
    }
    let w = match size {
        16 => "",
        32 => "256",
        64 => "512",
        _ => return None,
    };
    let si = match size {
        16 => "si128",
        32 => "si256",
        _ => "si512",
    };
    let mm = |op: &str, ty: &str| format!("_mm{w}_{op}_{ty}");
    let ident = |n: usize| (0..n).collect::<Vec<_>>();
    if let Some(rest) = fma(mnem) {
        return Some(rest_fma(rest, w));
    }
    let m = mnem.strip_prefix('v')?;
    // Packed float: add/sub/mul/div/min/max/sqrt/and/or/xor/andn + ps|pd.
    for (op, intr) in [
        ("add", "add"),
        ("sub", "sub"),
        ("mul", "mul"),
        ("div", "div"),
        ("min", "min"),
        ("max", "max"),
        ("sqrt", "sqrt"),
        ("andn", "andnot"),
        ("and", "and"),
        ("or", "or"),
        ("xor", "xor"),
        ("addsub", "addsub"),
        ("hadd", "hadd"),
        ("hsub", "hsub"),
        ("rcp", "rcp"),
        ("rsqrt", "rsqrt"),
        ("unpckl", "unpacklo"),
        ("unpckh", "unpackhi"),
        ("shuf", "shuffle"),
        ("blendv", "blendv"),
        ("blend", "blend"),
    ] {
        for ty in ["ps", "pd"] {
            if m == format!("{op}{ty}") {
                let n = if matches!(op, "sqrt" | "rcp" | "rsqrt") { 1 } else if matches!(op, "blendv" | "blend" | "shuf") { 3 } else { 2 };
                return Some((mm(intr, ty), ident(n)));
            }
        }
    }
    // Packed integer.
    let p = m.strip_prefix('p')?;
    let elem = |c: char| match c {
        'b' => Some("8"),
        'w' => Some("16"),
        'd' => Some("32"),
        'q' => Some("64"),
        _ => None,
    };
    let split = |pre: &str| -> Option<&str> { p.strip_prefix(pre).and_then(|r| (r.len() == 1).then_some(r)) };
    let bits = |r: &str| elem(r.chars().next()?);
    for (pre, op, ty) in [
        ("addus", "adds", "epu"),
        ("subus", "subs", "epu"),
        ("adds", "adds", "epi"),
        ("subs", "subs", "epi"),
        ("add", "add", "epi"),
        ("sub", "sub", "epi"),
        ("maxu", "max", "epu"),
        ("maxs", "max", "epi"),
        ("minu", "min", "epu"),
        ("mins", "min", "epi"),
        ("cmpeq", "cmpeq", "epi"),
        ("cmpgt", "cmpgt", "epi"),
        ("avg", "avg", "epu"),
        ("abs", "abs", "epi"),
        ("sign", "sign", "epi"),
        ("sllv", "sllv", "epi"),
        ("srlv", "srlv", "epi"),
        ("srav", "srav", "epi"),
    ] {
        if let Some(r) = split(pre) {
            let b = bits(r)?;
            let n = if op == "abs" { 1 } else { 2 };
            return Some((mm(op, &format!("{ty}{b}")), ident(n)));
        }
    }
    let named = match p {
        "unpcklbw" => mm("unpacklo", "epi8"),
        "unpcklwd" => mm("unpacklo", "epi16"),
        "unpckldq" => mm("unpacklo", "epi32"),
        "unpcklqdq" => mm("unpacklo", "epi64"),
        "unpckhbw" => mm("unpackhi", "epi8"),
        "unpckhwd" => mm("unpackhi", "epi16"),
        "unpckhdq" => mm("unpackhi", "epi32"),
        "unpckhqdq" => mm("unpackhi", "epi64"),
        "and" | "andd" | "andq" => mm("and", si),
        "andn" | "andnd" | "andnq" => mm("andnot", si),
        "or" | "ord" | "orq" => mm("or", si),
        "xor" | "xord" | "xorq" => mm("xor", si),
        "mullw" => mm("mullo", "epi16"),
        "mulld" => mm("mullo", "epi32"),
        "mullq" => mm("mullo", "epi64"),
        "mulhw" => mm("mulhi", "epi16"),
        "mulhuw" => mm("mulhi", "epu16"),
        "mulhrsw" => mm("mulhrs", "epi16"),
        "muludq" => mm("mul", "epu32"),
        "muldq" => mm("mul", "epi32"),
        "maddwd" => mm("madd", "epi16"),
        "maddubsw" => mm("maddubs", "epi16"),
        "sadbw" => mm("sad", "epu8"),
        "shufb" => mm("shuffle", "epi8"),
        "shufd" => mm("shuffle", "epi32"),
        "shufhw" => mm("shufflehi", "epi16"),
        "shuflw" => mm("shufflelo", "epi16"),
        "alignr" => mm("alignr", "epi8"),
        "blendvb" => mm("blendv", "epi8"),
        "blendw" => mm("blend", "epi16"),
        "blendd" => mm("blend", "epi32"),
        "packsswb" => mm("packs", "epi16"),
        "packssdw" => mm("packs", "epi32"),
        "packuswb" => mm("packus", "epi16"),
        "packusdw" => mm("packus", "epi32"),
        "haddw" => mm("hadd", "epi16"),
        "haddd" => mm("hadd", "epi32"),
        "hsubw" => mm("hsub", "epi16"),
        "hsubd" => mm("hsub", "epi32"),
        _ => return None,
    };
    let n = match p {
        "shufd" | "shufhw" | "shuflw" => 2,
        "alignr" | "blendvb" | "blendw" | "blendd" => 3,
        _ => 2,
    };
    Some((named, ident(n)))
}

/// `(kind, form, type)` of an FMA mnemonic such as `vfnmsub231ps`.
fn fma(mnem: &str) -> Option<(&str, &str, &str)> {
    let rest = mnem.strip_prefix("vf")?;
    for kind in ["maddsub", "msubadd", "nmadd", "nmsub", "madd", "msub"] {
        if let Some(r) = rest.strip_prefix(kind) {
            if r.len() == 5 && matches!(&r[..3], "132" | "213" | "231") && matches!(&r[3..], "ps" | "pd" | "ss" | "sd") {
                return Some((kind, &r[..3], &r[3..]));
            }
        }
    }
    None
}

/// `vf<kind><form><ty>(a, b, c)`: 132 is `a*c + b`, 213 is `b*a + c` and 231 is
/// `b*c + a`, while `_mm_f<kind>_<ty>(x, y, z)` is `x*y + z`.
fn rest_fma((kind, form, ty): (&str, &str, &str), w: &str) -> (String, Vec<usize>) {
    let order = match form {
        "132" => vec![0, 2, 1],
        "213" => vec![1, 0, 2],
        _ => vec![1, 2, 0],
    };
    let w = if matches!(ty, "ss" | "sd") { "" } else { w };
    (format!("_mm{w}_f{kind}_{ty}"), order)
}

#[cfg(test)]
mod tests;
