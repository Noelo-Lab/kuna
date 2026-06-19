//! WS3 golden test for [`MacroBuilder`] (the p-code macro-expansion
//! `PcodeBuilder`).
//!
//! `tests/golden/macrobuilder_expand.txt` is a golden op-template dump produced
//! by an instrumented /tmp copy of the C++ compiler tree (`decompiler/cpp/**`
//! stays byte-untouched): a dumper (`/tmp/kunacpp/macrodump.cc`, recorded in
//! `docs/rust-port/sleigh-compiler/ws3-pcode-actions.md`) builds a representative
//! macro body (`ConstructTpl` of `OpTpl`s referencing macro handle-parameters),
//! a `MACROBUILD` invocation supplying two handle args, runs the real C++
//! `MacroBuilder::build`, and dumps the resulting `OpTpl` list before and after
//! expansion.  This test builds the *identical* templates, runs the Rust
//! [`MacroBuilder`], and asserts a byte-identical dump.
//!
//! Coverage (every MacroBuilder code path):
//! - plain handle substitution (`transferOp` -> `VarnodeTpl::transfer`);
//! - `setLabel` index adjustment by `labelbase` (LABELBUILD / PTRADD);
//! - `dump`'s relative-operand (label) adjustment by `labelbase`;
//! - the truncation -> `CPUI_SUBPIECE` insertion path (`getUniqueAddr`/
//!   `getUniqueSpace`/`getConstantSpace` via the [`CompilerHost`] callbacks).

use std::cell::RefCell;
use std::fmt::Write as _;
use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::pcodecompile::{Location, PcodeCompileSymbol};
use kuna_sleigh::semantics::{
    ConstTpl, ConstType, ConstructTpl, OpTpl, PcodeBuilder, VField, VarnodeTpl,
};

use kuna_slacomp::pcodecompile_actions::{CompilerHost, MacroBuilder};

const MAX_UNIQUE_SIZE: u32 = 256; // C++ SleighBase::MAX_UNIQUE_SIZE

/// A minimal [`CompilerHost`] for the test: owns the const/unique spaces and a
/// unique-base counter mirroring `SleighCompile::getUniqueAddr`.
struct TestHost {
    constant: Rc<AddrSpace>,
    unique: Rc<AddrSpace>,
    unique_base: RefCell<u32>,
}

impl TestHost {
    fn new() -> TestHost {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        // An "other" placeholder so the unique space lands at index 2 (const=0,
        // other=1, unique=2), matching the C++ predefinedSymbols order; space
        // identity in the dump is by name, not index.
        let other = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "OTHER",
            false,
            8,
            1,
            1,
            addrspace_flags::hasphysical,
            0,
            0,
        );
        manager.insert_space(Rc::new(other)).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(2, 0, false)))
            .unwrap();
        TestHost {
            constant: Rc::clone(manager.get_constant_space().unwrap()),
            unique: Rc::clone(manager.get_space_by_name("unique").unwrap()),
            unique_base: RefCell::new(0),
        }
    }
}

impl CompilerHost for TestHost {
    fn get_unique_addr(&mut self) -> u32 {
        let base = *self.unique_base.borrow();
        *self.unique_base.borrow_mut() = base + MAX_UNIQUE_SIZE;
        base
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.unique)
    }
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.constant)
    }
    fn get_location(&self, _symbol_name: &[u8]) -> Option<Location> {
        None
    }
    fn add_symbol(&mut self, _sym: PcodeCompileSymbol) {}
    fn report_error(&mut self, _loc: Option<&Location>, msg: &str) {
        panic!("unexpected MacroBuilder error: {msg}");
    }
    fn report_warning(&mut self, _loc: Option<&Location>, _msg: &str) {}
}

// ---- dump helpers (byte-identical format to /tmp/kunacpp/macrodump.cc) ------

fn consttype_name(t: ConstType) -> &'static str {
    match t {
        ConstType::Real => "real",
        ConstType::Handle => "handle",
        ConstType::JStart => "j_start",
        ConstType::JNext => "j_next",
        ConstType::JNext2 => "j_next2",
        ConstType::JCurspace => "j_curspace",
        ConstType::JCurspaceSize => "j_curspace_size",
        ConstType::Spaceid => "spaceid",
        ConstType::JRelative => "j_relative",
        ConstType::JFlowref => "j_flowref",
        ConstType::JFlowrefSize => "j_flowref_size",
        ConstType::JFlowdest => "j_flowdest",
        ConstType::JFlowdestSize => "j_flowdest_size",
    }
}

fn vfield_name(f: VField) -> &'static str {
    match f {
        VField::VSpace => "v_space",
        VField::VOffset => "v_offset",
        VField::VSize => "v_size",
        VField::VOffsetPlus => "v_offset_plus",
    }
}

fn dump_const(s: &mut String, c: &ConstTpl) {
    let t = c.get_type();
    s.push_str(consttype_name(t));
    match t {
        ConstType::Spaceid => {
            write!(s, "(space={})", c.get_space().get_name()).unwrap();
        }
        ConstType::Handle => {
            write!(
                s,
                "(hi={},sel={},plus=0x{:x})",
                c.get_handle_index(),
                vfield_name(c.get_select()),
                c.get_real()
            )
            .unwrap();
        }
        _ => {
            write!(s, "(0x{:x})", c.get_real()).unwrap();
        }
    }
}

fn dump_vn(s: &mut String, tag: &str, vn: Option<&VarnodeTpl>) {
    write!(s, "    {tag}: ").unwrap();
    match vn {
        None => s.push_str("<null>\n"),
        Some(vn) => {
            s.push_str("space=");
            dump_const(s, vn.get_space());
            s.push_str(" off=");
            dump_const(s, vn.get_offset());
            s.push_str(" sz=");
            dump_const(s, vn.get_size());
            s.push('\n');
        }
    }
}

fn dump_opvec(s: &mut String, title: &str, ops: &[OpTpl]) {
    writeln!(s, "=== {title} ({} ops) ===", ops.len()).unwrap();
    for (i, op) in ops.iter().enumerate() {
        writeln!(s, "  op[{i}] opcode={}", op.get_opcode() as i32).unwrap();
        dump_vn(s, "out", op.get_out());
        for j in 0..op.num_input() {
            dump_vn(s, &format!("in[{j}]"), Some(op.get_in(j)));
        }
    }
}

// ---- template builders (mirror macrodump.cc) --------------------------------

fn handle_vn(hi: i32) -> VarnodeTpl {
    VarnodeTpl::new(
        ConstTpl::new_handle(hi, VField::VSpace),
        ConstTpl::new_handle(hi, VField::VOffset),
        ConstTpl::new_handle(hi, VField::VSize),
    )
}

fn handle_trunc_vn(hi: i32, plus: u64, sz: u64) -> VarnodeTpl {
    VarnodeTpl::new(
        ConstTpl::new_handle(hi, VField::VSpace),
        ConstTpl::new_handle_plus(hi, VField::VOffsetPlus, plus),
        ConstTpl::new_real(ConstType::Real, sz),
    )
}

fn const_vn(cspace: &Rc<AddrSpace>, off: u64, sz: u64) -> VarnodeTpl {
    VarnodeTpl::new(
        ConstTpl::new_space(Rc::clone(cspace)),
        ConstTpl::new_real(ConstType::Real, off),
        ConstTpl::new_real(ConstType::Real, sz),
    )
}

/// The macro body: identical op sequence to `macrodump.cc`.
fn macro_body(cspace: &Rc<AddrSpace>, uniq: &Rc<AddrSpace>) -> Vec<OpTpl> {
    let mut body: Vec<OpTpl> = Vec::new();

    // op0: COPY out=param0 in=param1
    let mut op0 = OpTpl::new(OpCode::CPUI_COPY);
    op0.set_output(handle_vn(0));
    op0.add_input(handle_vn(1));
    body.push(op0);

    // op1: INT_ADD out=param0 in0=param0 in1=<const 1>
    let mut op1 = OpTpl::new(OpCode::CPUI_INT_ADD);
    op1.set_output(handle_vn(0));
    op1.add_input(handle_vn(0));
    op1.add_input(const_vn(cspace, 1, 4));
    body.push(op1);

    // op2: LABELBUILD (PTRADD) with label index 0
    let mut op2 = OpTpl::new(OpCode::CPUI_PTRADD);
    op2.add_input(const_vn(cspace, 0, 4));
    body.push(op2);

    // op3: BRANCH to a *relative* label 0 (dump's relative adjust)
    let mut op3 = OpTpl::new(OpCode::CPUI_BRANCH);
    op3.add_input(VarnodeTpl::new(
        ConstTpl::new_space(Rc::clone(cspace)),
        ConstTpl::new_real(ConstType::JRelative, 0),
        ConstTpl::new_real(ConstType::Real, 8),
    ));
    body.push(op3);

    // op4: COPY out=<unique 0x500,sz2> in=param1[trunc +2, sz2] (subpiece path)
    let mut op4 = OpTpl::new(OpCode::CPUI_COPY);
    op4.set_output(VarnodeTpl::new(
        ConstTpl::new_space(Rc::clone(uniq)),
        ConstTpl::new_real(ConstType::Real, 0x500),
        ConstTpl::new_real(ConstType::Real, 2),
    ));
    op4.add_input(handle_trunc_vn(1, 2, 2));
    body.push(op4);

    body
}

#[test]
fn macrobuilder_expand_matches_golden() {
    let golden = include_str!("golden/macrobuilder_expand.txt");
    let mut host = TestHost::new();
    let cspace = host.get_constant_space();
    let uniq = host.get_unique_space();

    // The macro body (cloned: one copy for the pre-dump, one for the ConstructTpl
    // that `build` walks -- the C++ owns the body via the ConstructTpl).
    let body = macro_body(&cspace, &uniq);

    let mut pre = String::new();
    dump_opvec(&mut pre, "MACRO BODY (pre-expansion)", &body);

    // Build the ConstructTpl; `add_op_list` counts the LABELBUILD -> numlabels=1
    // (matching the C++ `setNumLabels(1)`).
    let mut macro_construct = ConstructTpl::new();
    assert!(macro_construct.add_op_list(body));
    assert_eq!(macro_construct.num_labels(), 1);

    // The MACROBUILD invocation: input0 = macro index, then the actual args.
    // Only inputs 1.. become handle params (via set_macro_op); the macroop
    // itself is not dumped, so its opcode is irrelevant to MacroBuilder.
    let mut macroop = OpTpl::new(OpCode::CPUI_CAST); // MACROBUILD = CPUI_CAST
    macroop.add_input(const_vn(&cspace, 0, 4)); // macro index 0
    macroop.add_input(VarnodeTpl::new(
        ConstTpl::new_space(Rc::clone(&uniq)),
        ConstTpl::new_real(ConstType::Real, 0x100),
        ConstTpl::new_real(ConstType::Real, 4),
    )); // arg0 -> param0
    macroop.add_input(VarnodeTpl::new(
        ConstTpl::new_space(Rc::clone(&uniq)),
        ConstTpl::new_real(ConstType::Real, 0x200),
        ConstTpl::new_real(ConstType::Real, 4),
    )); // arg1 -> param1

    // Run the expansion exactly as SleighCompile::expandMacros does.
    let labelbase: u32 = 3; // pretend the enclosing constructor already has 3 labels
    let mut outvec: Vec<OpTpl> = Vec::new();
    {
        let mut builder = MacroBuilder::new(&mut host, &mut outvec, labelbase);
        builder.set_macro_op(&macroop).unwrap();
        builder.build(Some(&macro_construct), -1).unwrap();
        assert!(!builder.has_error(), "MacroBuilder reported an error");
    }

    let mut post = String::new();
    dump_opvec(&mut post, "EXPANSION (post MacroBuilder::build)", &outvec);

    let mut got = String::new();
    got.push_str(&pre);
    got.push_str(&post);
    got.push_str("haserror=0\n");
    writeln!(got, "labelbase_used={labelbase}").unwrap();

    assert_eq!(got, golden, "\n--- GOT ---\n{got}\n--- WANT ---\n{golden}");
}
