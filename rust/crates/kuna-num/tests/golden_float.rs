//! GOLDEN GATE for item w1-num-float-multiprec: replay EVERY row of
//! `tests/golden/vectors/float.csv` (emitted by the C++ oracle's
//! `golden float` command, `decompiler/cpp/kuna_goldengen.cc`) through the
//! Rust `FloatFormat` and `cfmt` ports and require a 100% byte-for-byte
//! match.
//!
//! Row formats (see `tests/golden/vectors/README.md`):
//!   dec,size,enc,class,decimal
//!   <unop>,size,a,res                 (neg|abs|sqrt|ceil|floor|round|nan)
//!   <binop>,size,a,b,res              (add|sub|mult|div|equal|less|lessequal)
//!   trunc,size,sizeout,a,res
//!   int2float,sizein,size,a,res
//!   float2float,sizein,sizeout,a,res

use kuna_base::cfmt;
use kuna_num::float::{floatclass, FloatFormat};

/// Total number of data rows in float.csv (3607 lines minus 3 comments).
const EXPECTED_ROWS: usize = 3604;

fn parse_hex(s: &str, line_no: usize) -> u64 {
    let t = s
        .strip_prefix("0x")
        .unwrap_or_else(|| panic!("line {line_no}: bad hex literal {s:?}"));
    u64::from_str_radix(t, 16).unwrap_or_else(|e| panic!("line {line_no}: bad hex {s:?}: {e}"))
}

fn parse_size(s: &str, line_no: usize) -> i32 {
    s.parse()
        .unwrap_or_else(|e| panic!("line {line_no}: bad size {s:?}: {e}"))
}

/// The fixed class labels of the golden generator (goldenFloatClass).
fn class_name(cls: floatclass) -> &'static str {
    match cls {
        floatclass::normalized => "normalized",
        floatclass::infinity => "infinity",
        floatclass::zero => "zero",
        floatclass::nan => "nan",
        floatclass::denormalized => "denormalized",
    }
}

/// goldenHex: 0x-prefixed lower-case hex, no padding.
fn golden_hex(val: u64) -> String {
    format!("0x{val:x}")
}

#[test]
fn golden_float_vectors() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/golden/vectors/float.csv"
    );
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read golden vectors at {path}: {e}"));

    let fmt4 = FloatFormat::new(4);
    let fmt8 = FloatFormat::new(8);
    let by_size = |size: i32, line_no: usize| -> &FloatFormat {
        match size {
            4 => &fmt4,
            8 => &fmt8,
            _ => panic!("line {line_no}: unexpected format size {size}"),
        }
    };

    let mut rows = 0usize;
    for (idx, line) in data.lines().enumerate() {
        let line_no = idx + 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        match f[0] {
            "dec" => {
                assert_eq!(f.len(), 5, "line {line_no}: bad dec row {line:?}");
                let fmt = by_size(parse_size(f[1], line_no), line_no);
                let enc = parse_hex(f[2], line_no);
                let (host, cls) = fmt.get_host_float(enc);
                assert_eq!(
                    class_name(cls),
                    f[3],
                    "line {line_no}: class mismatch for {line:?}"
                );
                // getClass must agree with the class getHostFloat reports.
                assert_eq!(
                    fmt.get_class(enc),
                    cls,
                    "line {line_no}: getClass disagreement for {line:?}"
                );
                assert_eq!(
                    cfmt::ostream_default(host),
                    f[4],
                    "line {line_no}: decimal rendering mismatch for {line:?}"
                );
            }
            "neg" | "abs" | "sqrt" | "ceil" | "floor" | "round" | "nan" => {
                assert_eq!(f.len(), 4, "line {line_no}: bad unop row {line:?}");
                let fmt = by_size(parse_size(f[1], line_no), line_no);
                let a = parse_hex(f[2], line_no);
                let res = match f[0] {
                    "neg" => fmt.op_neg(a),
                    "abs" => fmt.op_abs(a),
                    "sqrt" => fmt.op_sqrt(a),
                    "ceil" => fmt.op_ceil(a),
                    "floor" => fmt.op_floor(a),
                    "round" => fmt.op_round(a),
                    "nan" => fmt.op_nan(a),
                    _ => unreachable!(),
                };
                assert_eq!(
                    golden_hex(res),
                    f[3],
                    "line {line_no}: result mismatch for {line:?}"
                );
            }
            "add" | "sub" | "mult" | "div" | "equal" | "less" | "lessequal" => {
                assert_eq!(f.len(), 5, "line {line_no}: bad binop row {line:?}");
                let fmt = by_size(parse_size(f[1], line_no), line_no);
                let a = parse_hex(f[2], line_no);
                let b = parse_hex(f[3], line_no);
                let res = match f[0] {
                    "add" => fmt.op_add(a, b),
                    "sub" => fmt.op_sub(a, b),
                    "mult" => fmt.op_mult(a, b),
                    "div" => fmt.op_div(a, b),
                    "equal" => fmt.op_equal(a, b),
                    "less" => fmt.op_less(a, b),
                    "lessequal" => fmt.op_less_equal(a, b),
                    _ => unreachable!(),
                };
                assert_eq!(
                    golden_hex(res),
                    f[4],
                    "line {line_no}: result mismatch for {line:?}"
                );
            }
            "trunc" => {
                assert_eq!(f.len(), 5, "line {line_no}: bad trunc row {line:?}");
                let fmt = by_size(parse_size(f[1], line_no), line_no);
                let sizeout = parse_size(f[2], line_no);
                let a = parse_hex(f[3], line_no);
                assert_eq!(
                    golden_hex(fmt.op_trunc(a, sizeout)),
                    f[4],
                    "line {line_no}: result mismatch for {line:?}"
                );
            }
            "int2float" => {
                assert_eq!(f.len(), 5, "line {line_no}: bad int2float row {line:?}");
                let sizein = parse_size(f[1], line_no);
                let fmt = by_size(parse_size(f[2], line_no), line_no);
                let a = parse_hex(f[3], line_no);
                assert_eq!(
                    golden_hex(fmt.op_int2float(a, sizein)),
                    f[4],
                    "line {line_no}: result mismatch for {line:?}"
                );
            }
            "float2float" => {
                assert_eq!(f.len(), 5, "line {line_no}: bad float2float row {line:?}");
                let fmt_in = by_size(parse_size(f[1], line_no), line_no);
                let fmt_out = by_size(parse_size(f[2], line_no), line_no);
                let a = parse_hex(f[3], line_no);
                assert_eq!(
                    golden_hex(fmt_in.op_float2float(a, fmt_out)),
                    f[4],
                    "line {line_no}: result mismatch for {line:?}"
                );
            }
            other => panic!("line {line_no}: unknown row kind {other:?}"),
        }
        rows += 1;
    }
    assert_eq!(
        rows, EXPECTED_ROWS,
        "replayed row count diverged from the pinned vector size"
    );
}
