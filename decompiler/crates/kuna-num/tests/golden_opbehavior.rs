//! GOLDEN GATE for item w1-num-pcode-semantics: replay EVERY row of
//! `tests/golden/vectors/opbehavior.csv` (emitted by the C++ oracle's
//! `golden opbehavior` command, `decompiler/cpp/kuna_goldengen.cc`).
//!
//! The test transcribes the generator loop itself (`IfcKunaGoldenOpbehavior::
//! execute`: behavior-table iteration in opcode enum order, the fixed
//! size/input matrices, the ERR mapping, the UB-2 TRAP pre-detection) and
//! requires the regenerated text to match the pinned vector file
//! byte-for-byte.  That subsumes the row-replay contract:
//! result rows must match exactly, ERR rows must produce an error, TRAP rows
//! must produce an error without panicking (asserted explicitly before
//! emitting the literal).
//!
//! The C++ generator runs against the pinned x86:LE:64 architecture only to
//! reach `Translate::getFloatFormat`; sleigh defines no custom float formats,
//! so that resolves to `Translate::setDefaultFloatFormats` — `FloatFormat(4)`
//! and `FloatFormat(8)` — reproduced here by a fixed provider (sizes 1 and 2
//! have no format and fall back to ERR, as the vector README pins).

use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_num::float::FloatFormat;
use kuna_num::opbehavior::{register_instructions, FloatFormatProvider, OpBehavior};
use kuna_num::opcodes::{get_opname, OpCode};

/// The sizes (in bytes) every vector matrix iterates over, in fixed order
const GOLDEN_SIZES: [i32; 4] = [1, 2, 4, 8];

/// The default IEEE 754 formats of the pinned oracle architecture.
struct DefaultFloatFormats {
    fmt4: FloatFormat,
    fmt8: FloatFormat,
}

impl FloatFormatProvider for DefaultFloatFormats {
    fn get_float_format(&self, size: i32) -> Option<&FloatFormat> {
        match size {
            4 => Some(&self.fmt4),
            8 => Some(&self.fmt8),
            _ => None,
        }
    }
}

/// goldenHex: 0x-prefixed lower-case hex, no padding.
fn golden_hex(val: u64) -> String {
    format!("0x{val:x}")
}

/// goldenOpName: the C++ generator names ZPULL/SPULL itself because the
/// upstream get_opname table is stale there (UB-1); the Rust table already
/// carries the canonical names, so this is exactly get_opname.
fn golden_op_name(opc: OpCode) -> &'static str {
    get_opname(opc)
}

/// goldenInputs: the fixed edge-input list for a given input size.
/// Positional (index) order is part of the vector contract.
fn golden_inputs(size: i32) -> Vec<u64> {
    let mask = calc_mask(size);
    vec![
        0,
        1,
        2,
        mask >> 1,            // sign bit - 1 (max signed)
        (mask >> 1) + 1,      // sign bit (min signed)
        mask,                 // all ones for size
        0xdeadbeefdeadbeef_u64 & mask, // fixed odd constant 1
        0x123456789abcdef1_u64 & mask, // fixed odd constant 2
    ]
}

/// goldenEvalUnary: evaluate one unary cell, mapping any evaluation failure
/// to "ERR" (the C++ catches LowlevelError, which covers EvaluationError and
/// the base-class throw — every KunaError our behaviors return).
fn golden_eval_unary(beh: &dyn OpBehavior, sizeout: i32, sizein: i32, in0: u64) -> String {
    match beh.evaluate_unary(sizeout, sizein, in0) {
        Ok(val) => golden_hex(val),
        Err(_) => "ERR".to_string(),
    }
}

/// goldenEvalBinary: evaluate one binary cell.  The C++ generator never
/// evaluates the INT64_MIN / -1 cell of INT_SDIV / INT_SREM (host SIGFPE,
/// UB-2) and emits "TRAP"; the Rust port CAN evaluate it and must produce an
/// error without panicking — asserted here, then the pinned literal is
/// emitted.  Likewise SPULL is short-circuited to "ERR" in C++ (the throw
/// path would read past the stale name table); the Rust evaluation must
/// error.
fn golden_eval_binary(beh: &dyn OpBehavior, sizeout: i32, sizein: i32, in0: u64, in1: u64) -> String {
    let opc = beh.get_opcode();
    if (opc == OpCode::CPUI_INT_SDIV || opc == OpCode::CPUI_INT_SREM)
        && sizein == 8
        && in0 == 0x8000000000000000
        && in1 == calc_mask(8)
    {
        // UB-2 gate: TRAP == error, and no panic.
        assert!(
            beh.evaluate_binary(sizeout, sizein, in0, in1).is_err(),
            "UB-2 cell of {opc:?} must evaluate to an error"
        );
        return "TRAP".to_string();
    }
    if opc == OpCode::CPUI_SPULL {
        assert!(
            beh.evaluate_binary(sizeout, sizein, in0, in1).is_err(),
            "SPULL evaluation must error (base-class unimplemented path)"
        );
        return "ERR".to_string();
    }
    match beh.evaluate_binary(sizeout, sizein, in0, in1) {
        Ok(val) => golden_hex(val),
        Err(_) => "ERR".to_string(),
    }
}

/// Transcription of IfcKunaGoldenOpbehavior::execute, producing the CSV
/// lines (including the three header comments).
fn generate_lines() -> Vec<String> {
    let trans: Rc<dyn FloatFormatProvider> =
        Rc::new(DefaultFloatFormats { fmt4: FloatFormat::new(4), fmt8: FloatFormat::new(8) });
    let mut inst: Vec<Option<Rc<dyn OpBehavior>>> = Vec::new();
    register_instructions(&mut inst, &trans);

    let mut lines: Vec<String> = Vec::new();
    lines.push(
        "# golden opbehavior vectors (kuna_goldengen.cc; see tests/golden/vectors/README.md)"
            .to_string(),
    );
    lines.push(
        "# binary rows: op,sizein,sizeout,in0,in1,result   unary rows: op,sizein,sizeout,in0,result"
            .to_string(),
    );
    lines.push("# result ERR = evaluation threw (EvaluationError or unimplemented)".to_string());

    for slot in &inst {
        // (C++ iterates indices 0..inst.size() and skips nulls)
        let beh = match slot {
            Some(beh) => beh,
            None => continue,
        };
        if beh.is_special() {
            continue; // No direct unary/binary evaluation semantics
        }
        let opname = golden_op_name(beh.get_opcode());
        if beh.is_unary() {
            for &sizein in GOLDEN_SIZES.iter() {
                let in0vals = golden_inputs(sizein);
                for &sizeout in GOLDEN_SIZES.iter() {
                    for &in0 in &in0vals {
                        lines.push(format!(
                            "{opname},{sizein},{sizeout},{},{}",
                            golden_hex(in0),
                            golden_eval_unary(beh.as_ref(), sizeout, sizein, in0)
                        ));
                    }
                }
            }
        } else {
            for &sizein in GOLDEN_SIZES.iter() {
                let in0vals = golden_inputs(sizein);
                let in1vals = in0vals.clone();
                for j in 0..2 {
                    let sizeout = if j == 0 { sizein } else { 1 };
                    if j == 1 && sizein == 1 {
                        continue; // (1,1) already emitted
                    }
                    for &in0 in &in0vals {
                        for &in1 in &in1vals {
                            lines.push(format!(
                                "{opname},{sizein},{sizeout},{},{},{}",
                                golden_hex(in0),
                                golden_hex(in1),
                                golden_eval_binary(beh.as_ref(), sizeout, sizein, in0, in1)
                            ));
                        }
                    }
                }
            }
        }
    }
    lines
}

#[test]
fn golden_opbehavior_vectors() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/golden/vectors/opbehavior.csv"
    );
    let data = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read golden vectors at {path}: {e}"));

    let expected = generate_lines();

    // Per-line comparison first, for a meaningful failure message.
    let mut file_lines = 0usize;
    for (idx, (file_line, gen_line)) in data.lines().zip(expected.iter()).enumerate() {
        assert_eq!(
            gen_line,
            file_line,
            "line {} diverges from the pinned vector",
            idx + 1
        );
        file_lines += 1;
    }
    assert_eq!(
        data.lines().count(),
        expected.len(),
        "row count diverged from the pinned vector size"
    );
    assert_eq!(file_lines, expected.len());
    // 3 header lines + 20224 evaluation rows (the pinned 20227-line vector).
    assert_eq!(expected.len(), 20227);

    // Byte-for-byte: the generator writes '\n' after every line.
    let regenerated: String =
        expected.iter().map(|l| format!("{l}\n")).collect();
    assert_eq!(regenerated, data, "regenerated vector text is not byte-identical");
}
