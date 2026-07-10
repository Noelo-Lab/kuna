//! Port of `decompiler/cpp/pcodecompile.{hh,cc}` (item
//! `w2-sleigh-semantics`): the p-code expression/statement builder shared
//! by the SLEIGH compiler and the runtime snippet parser.
//!
//! At runtime this machinery is driven by `pcodeparse` (`PcodeSnippet`,
//! a later item) to compile p-code **injection snippets**; the other C++
//! subclass (`SleighCompile`) belongs to the unported compiler (LOSS-001).
//!
//! ## Shape of the port
//!
//! - C++ `PcodeCompile` is an abstract class with data members; the port is
//!   the [`PcodeCompile`] trait: the C++ pure virtuals (`allocateTemp`,
//!   `addSymbol`, `getLocation`, `reportError`, `reportWarning`) and
//!   accessors for the C++ data members (`defaultspace`, `constantspace`,
//!   `uniqspace`, `local_labelcount`, `enforceLocalKey`) are required
//!   methods; every concrete C++ method body is a provided method
//!   transcribed from pcodecompile.cc.
//! - [`ExprTree`] owns its ops/output by value (`Vec<OpTpl>` /
//!   `Option<VarnodeTpl>`); the C++ null-vs-empty `ops` pointer distinction
//!   only ever exists transiently and is collapsed to the empty `Vec`.
//!   Methods that C++ passes/frees raw pointers through take and return
//!   `ExprTree` by value.
//! - C++ overloads split by arity: `createOp` -> [`PcodeCompile::create_op`]
//!   (unary) / [`PcodeCompile::create_op2`] (binary), `createOpNoOut` ->
//!   [`PcodeCompile::create_op_no_out`] / [`PcodeCompile::create_op_no_out2`].
//! - `LabelSymbol` lives in C++ slghsymbol.hh but was deliberately not
//!   ported by the symbol wave (its only consumers are the compiler and
//!   this module); it is defined here as [`LabelSymbol`], shared as
//!   `Rc<LabelSymbol>` with `Cell` interior mutability for the two fields
//!   C++ mutates through shared pointers (`isplaced`, `refcount`).
//! - `addSymbol(SleighSymbol *)` receives either a freshly built
//!   `VarnodeSymbol` or a `LabelSymbol`; the port's
//!   [`PcodeCompileSymbol`] enum carries exactly those two cases.
//! - `getLocation(SleighSymbol *)` is keyed by the symbol **name** in the
//!   port ([`PcodeCompile::get_location`]): the only runtime implementor
//!   (`PcodeSnippet`) returns null unconditionally, and the pointer-keyed
//!   map belongs to the unported compiler.
//! - `createBitRange(SpecificSymbol *sym,...)` takes the already-resolved
//!   `sym->getVarnode()` plus the symbol name
//!   ([`PcodeCompile::create_bit_range`]): the `getVarnode()` virtuals were
//!   deferred by the symbol wave, and the caller (the pcodeparse item)
//!   resolves them.
//! - The C++ statics `force_size`/`matchSize`/`fillinZero`/`propagateSize`
//!   are free functions.  C++ aliases a target `VarnodeTpl *` *into* the op
//!   vector it simultaneously mutates; the port expresses the same
//!   algorithm with an explicit target slot (`(op_index, slot)`,
//!   [`match_size`]/[`fillin_zero`]) — the propagation loop still visits
//!   the target's own slot exactly as C++ does (a same-size no-op).

use std::cell::Cell;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;
use kuna_num::opcodes::OpCode;

use crate::semantics::{ConstTpl, ConstType, ConstructTpl, OpTpl, VField, VarnodeTpl, LABELBUILD};
use crate::slghsymbol::{SleighSymbol, UserOpSymbol};

// ---------------------------------------------------------------------------
// Location
// ---------------------------------------------------------------------------

/// C++ `Location`: a source file / line number pair for error reporting.
#[derive(Debug, Clone, Default)]
pub struct Location {
    filename: Vec<u8>,
    lineno: i32,
}

impl Location {
    /// C++ `Location(const string &fname,const int4 line)`.
    pub fn new(fname: &[u8], line: i32) -> Location {
        Location {
            filename: fname.to_vec(),
            lineno: line,
        }
    }

    /// C++ `getFilename`.
    pub fn get_filename(&self) -> &[u8] {
        &self.filename
    }

    /// C++ `getLineno`.
    pub fn get_lineno(&self) -> i32 {
        self.lineno
    }

    /// C++ `format`: `"<filename>:<lineno>"`.
    pub fn format(&self) -> String {
        format!(
            "{}:{}",
            String::from_utf8_lossy(&self.filename),
            self.lineno
        )
    }
}

// ---------------------------------------------------------------------------
// StarQuality / LabelSymbol
// ---------------------------------------------------------------------------

/// C++ `StarQuality`: the parsed `*[space]:size` qualifier of a
/// load/store.
#[derive(Debug, Clone)]
pub struct StarQuality {
    /// The space id template (C++ `ConstTpl id`).
    pub id: ConstTpl,
    /// The explicit size, 0 if unspecified (C++ `uint4 size`).
    pub size: u32,
}

/// C++ `LabelSymbol` (slghsymbol.hh): a branch label within a p-code
/// snippet.  Defined here because the symbol wave deliberately skipped it
/// (compiler-side class; this module and the snippet parser are its only
/// runtime consumers).  Shared as `Rc<LabelSymbol>` mirroring the C++
/// aliasing between the local scope and grammar values; the two fields C++
/// mutates through that shared pointer use `Cell`.
#[derive(Debug, Default)]
pub struct LabelSymbol {
    /// Symbol name (C++ `SleighSymbol::name`).
    name: Vec<u8>,
    /// Local 1-up index of label (C++ `uint4 index`).
    index: u32,
    /// Has the label been placed (not just referenced)?
    isplaced: Cell<bool>,
    /// Number of references to this label (C++ `uint4 refcount`).
    refcount: Cell<u32>,
}

impl LabelSymbol {
    /// C++ `LabelSymbol(const string &nm,uint4 i)`.
    pub fn new(nm: &[u8], i: u32) -> LabelSymbol {
        LabelSymbol {
            name: nm.to_vec(),
            index: i,
            isplaced: Cell::new(false),
            refcount: Cell::new(0),
        }
    }

    /// C++ `SleighSymbol::getName`.
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    /// C++ `getIndex`.
    pub fn get_index(&self) -> u32 {
        self.index
    }

    /// C++ `incrementRefCount`.
    pub fn increment_ref_count(&self) {
        self.refcount.set(self.refcount.get().wadd(1));
    }

    /// C++ `getRefCount`.
    pub fn get_ref_count(&self) -> u32 {
        self.refcount.get()
    }

    /// C++ `setPlaced`.
    pub fn set_placed(&self) {
        self.isplaced.set(true);
    }

    /// C++ `isPlaced`.
    pub fn is_placed(&self) -> bool {
        self.isplaced.get()
    }
}

/// The payload of [`PcodeCompile::add_symbol`] (C++ `addSymbol(SleighSymbol
/// *)`): the builder creates either `VarnodeSymbol`s (as [`SleighSymbol`])
/// or snippet-local [`LabelSymbol`]s.
#[derive(Debug)]
pub enum PcodeCompileSymbol {
    /// A `SleighSymbol` (the `VarnodeSymbol`s `newOutput` /
    /// `newLocalDefinition` create).
    Sleigh(Box<SleighSymbol>),
    /// A snippet-local branch label created by `defineLabel`.
    Label(Rc<LabelSymbol>),
}

impl PcodeCompileSymbol {
    /// The symbol's name (C++ `SleighSymbol::getName`, used by implementors
    /// to key their scope).
    pub fn get_name(&self) -> &[u8] {
        match self {
            PcodeCompileSymbol::Sleigh(sym) => sym.get_name(),
            PcodeCompileSymbol::Label(lab) => lab.get_name(),
        }
    }
}

// ---------------------------------------------------------------------------
// ExprTree
// ---------------------------------------------------------------------------

/// C++ `ExprTree`: a flattened expression tree — the ops making up the
/// expression plus the output varnode (a COPY of the last op's output when
/// that op has one).
#[derive(Debug, Default)]
pub struct ExprTree {
    /// C++ `vector<OpTpl *> *ops` (the null pointer state collapses to the
    /// empty vector; see module docs).
    pub(crate) ops: Vec<OpTpl>,
    /// C++ `VarnodeTpl *outvn`.
    pub(crate) outvn: Option<VarnodeTpl>,
}

impl ExprTree {
    /// C++ `ExprTree(VarnodeTpl *vn)`.
    pub fn new(vn: VarnodeTpl) -> ExprTree {
        ExprTree {
            ops: Vec::new(),
            outvn: Some(vn),
        }
    }

    /// C++ `ExprTree(OpTpl *op)`: `outvn` becomes a COPY of the op's
    /// output, if it has one.
    pub fn new_from_op(op: OpTpl) -> ExprTree {
        let outvn = op.get_out().cloned();
        ExprTree {
            ops: vec![op],
            outvn,
        }
    }

    /// C++ `setOutput`: force the output of the expression to be `newout`.
    /// If the original output is named, this requires an extra COPY op.
    pub fn set_output(&mut self, newout: VarnodeTpl) -> KunaResult<()> {
        let outvn = match self.outvn.take() {
            None => return Err(KunaError::sleigh("Expression has no output")),
            Some(vn) => vn,
        };
        if outvn.is_unnamed() {
            // (C++ deletes outvn)
            let op = self
                .ops
                .last_mut()
                .expect("ExprTree::setOutput on an opless expression (C++ ops->back() is UB)");
            op.clear_output();
            op.set_output(newout.clone());
        } else {
            let mut op = OpTpl::new(OpCode::CPUI_COPY);
            op.add_input(outvn);
            op.set_output(newout.clone());
            self.ops.push(op);
        }
        // C++ `outvn = new VarnodeTpl(*newout)`
        self.outvn = Some(newout);
        Ok(())
    }

    /// C++ `getOut` (nullable).
    pub fn get_out(&self) -> Option<&VarnodeTpl> {
        self.outvn.as_ref()
    }

    /// C++ `getSize` (C++ dereferences `outvn` unconditionally).
    pub fn get_size(&self) -> &ConstTpl {
        self.outvn
            .as_ref()
            .expect("ExprTree::getSize on an expression without output (C++ dereferences null)")
            .get_size()
    }

    /// C++ static `appendParams`: create an op with the entire list of
    /// expression inputs.
    pub fn append_params(mut op: OpTpl, params: Vec<ExprTree>) -> Vec<OpTpl> {
        let mut res: Vec<OpTpl> = Vec::new();

        for mut param in params {
            res.append(&mut param.ops);
            op.add_input(param.outvn.take().expect(
                "appendParams: parameter expression has no output (C++ would push a null input)",
            ));
        }
        res.push(op);
        res
    }

    /// C++ static `toVector`: grab the op vector and delete the output
    /// expression.
    pub fn to_vector(expr: ExprTree) -> Vec<OpTpl> {
        expr.ops
    }
}

// ---------------------------------------------------------------------------
// size propagation statics (C++ PcodeCompile static members)
// ---------------------------------------------------------------------------

/// Identifies a varnode slot within an op for in-vector size forcing
/// (the port's stand-in for the C++ `VarnodeTpl *` aliased into `ops`).
#[derive(Debug, Clone, Copy)]
enum VtSlot {
    /// The op's output.
    Out,
    /// Input slot `i` (C++ `int4`).
    In(i32),
}

/// The propagation loop of C++ `PcodeCompile::force_size`: set `size` on
/// every local-temp varnode in `ops` whose offset matches `vt_offset`.
fn force_size_propagate(
    vt_offset: &ConstTpl,
    size: &ConstTpl,
    ops: &mut [OpTpl],
) -> KunaResult<()> {
    for op in ops.iter_mut() {
        if let Some(vn) = op.get_out_mut() {
            if vn.is_local_temp() && *vn.get_offset() == *vt_offset {
                if size.get_type() == ConstType::Real
                    && vn.get_size().get_type() == ConstType::Real
                    && vn.get_size().get_real() != 0
                    && vn.get_size().get_real() != size.get_real()
                {
                    return Err(KunaError::sleigh("Localtemp size mismatch"));
                }
                vn.set_size(size.clone());
            }
        }
        for j in 0..op.num_input() {
            let vn = op.get_in_mut(j);
            if vn.is_local_temp() && *vn.get_offset() == *vt_offset {
                if size.get_type() == ConstType::Real
                    && vn.get_size().get_type() == ConstType::Real
                    && vn.get_size().get_real() != 0
                    && vn.get_size().get_real() != size.get_real()
                {
                    return Err(KunaError::sleigh("Localtemp size mismatch"));
                }
                vn.set_size(size.clone());
            }
        }
    }
    Ok(())
}

/// C++ static `PcodeCompile::force_size` for a target varnode living
/// *outside* `ops` (e.g. an `ExprTree::outvn`): if `vt` has no size yet,
/// set it, and if `vt` is a local temporary propagate the size to its other
/// uses in `ops`.
pub fn force_size(vt: &mut VarnodeTpl, size: &ConstTpl, ops: &mut [OpTpl]) -> KunaResult<()> {
    if vt.get_size().get_type() != ConstType::Real || vt.get_size().get_real() != 0 {
        return Ok(()); // Size already exists
    }
    vt.set_size(size.clone());
    if !vt.is_local_temp() {
        return Ok(());
    }
    // If the variable is a local temporary the size may need to be
    // propagated to the various uses of the variable
    let offset = vt.get_offset().clone();
    force_size_propagate(&offset, size, ops)
}

/// C++ `force_size` where the target varnode is `ops[op_index]`'s `slot`
/// (C++ aliases a pointer into the vector; the propagation loop revisiting
/// the target slot re-sets the same size, a no-op, exactly as in C++).
fn force_size_at(
    ops: &mut [OpTpl],
    op_index: usize,
    slot: VtSlot,
    size: &ConstTpl,
) -> KunaResult<()> {
    let offset = {
        let op = &mut ops[op_index];
        let vt = match slot {
            VtSlot::Out => op
                .get_out_mut()
                .expect("force_size target output is null (C++ dereferences null)"),
            VtSlot::In(j) => op.get_in_mut(j),
        };
        if vt.get_size().get_type() != ConstType::Real || vt.get_size().get_real() != 0 {
            return Ok(()); // Size already exists
        }
        vt.set_size(size.clone());
        if !vt.is_local_temp() {
            return Ok(());
        }
        vt.get_offset().clone()
    };
    force_size_propagate(&offset, size, ops)
}

/// C++ static `PcodeCompile::matchSize`: find something to fill in the
/// zero-size varnode at slot `j` (-1 = output) of `ops[op_index]`; don't
/// consider the output as a source if `inputonly`.  (The C++ signature
/// passes the op by aliased pointer; the port passes its index.)
pub fn match_size(j: i32, op_index: usize, inputonly: bool, ops: &mut [OpTpl]) -> KunaResult<()> {
    // read-only scan for a matching nonzero-size varnode
    let matched: Option<ConstTpl> = {
        let op = &ops[op_index];
        let mut found: Option<&VarnodeTpl> = None;
        if !inputonly {
            if let Some(out) = op.get_out() {
                if !out.is_zero_size() {
                    found = Some(out);
                }
            }
        }
        let inputsize = op.num_input();
        for i in 0..inputsize {
            if found.is_some() {
                break;
            }
            if op.get_in(i).is_zero_size() {
                continue;
            }
            found = Some(op.get_in(i));
        }
        found.map(|vn| vn.get_size().clone())
    };
    if let Some(size) = matched {
        let slot = if j == -1 { VtSlot::Out } else { VtSlot::In(j) };
        force_size_at(ops, op_index, slot, &size)?;
    }
    Ok(())
}

/// C++ static `PcodeCompile::fillinZero`: try to get rid of zero-size
/// varnodes in `ops[op_index]`.  Right now this is written assuming
/// operands for the constructor are built before any other pcode in the
/// constructor is generated.  (The C++ signature passes the op by aliased
/// pointer; the port passes its index.)
pub fn fillin_zero(op_index: usize, ops: &mut [OpTpl]) -> KunaResult<()> {
    let opc = ops[op_index].get_opcode();
    match opc {
        // Instructions where all inputs and output are same size
        OpCode::CPUI_COPY
        | OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_INT_2COMP
        | OpCode::CPUI_INT_NEGATE
        | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_AND
        | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_DIV
        | OpCode::CPUI_INT_SDIV
        | OpCode::CPUI_INT_REM
        | OpCode::CPUI_INT_SREM
        | OpCode::CPUI_FLOAT_ADD
        | OpCode::CPUI_FLOAT_DIV
        | OpCode::CPUI_FLOAT_MULT
        | OpCode::CPUI_FLOAT_SUB
        | OpCode::CPUI_FLOAT_NEG
        | OpCode::CPUI_FLOAT_ABS
        | OpCode::CPUI_FLOAT_SQRT
        | OpCode::CPUI_FLOAT_CEIL
        | OpCode::CPUI_FLOAT_FLOOR
        | OpCode::CPUI_FLOAT_ROUND => {
            if ops[op_index].get_out().is_some_and(|o| o.is_zero_size()) {
                match_size(-1, op_index, false, ops)?;
            }
            let inputsize = ops[op_index].num_input();
            for i in 0..inputsize {
                if ops[op_index].get_in(i).is_zero_size() {
                    match_size(i, op_index, false, ops)?;
                }
            }
        }
        // Instructions with bool output
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_CARRY
        | OpCode::CPUI_INT_SCARRY
        | OpCode::CPUI_INT_SBORROW
        | OpCode::CPUI_FLOAT_EQUAL
        | OpCode::CPUI_FLOAT_NOTEQUAL
        | OpCode::CPUI_FLOAT_LESS
        | OpCode::CPUI_FLOAT_LESSEQUAL
        | OpCode::CPUI_FLOAT_NAN
        | OpCode::CPUI_BOOL_NEGATE
        | OpCode::CPUI_BOOL_XOR
        | OpCode::CPUI_BOOL_AND
        | OpCode::CPUI_BOOL_OR => {
            // (C++ dereferences getOut() unconditionally here)
            if ops[op_index]
                .get_out()
                .expect("bool-output op without output (C++ dereferences null)")
                .is_zero_size()
            {
                force_size_at(
                    ops,
                    op_index,
                    VtSlot::Out,
                    &ConstTpl::new_real(ConstType::Real, 1),
                )?;
            }
            let inputsize = ops[op_index].num_input();
            for i in 0..inputsize {
                if ops[op_index].get_in(i).is_zero_size() {
                    match_size(i, op_index, true, ops)?;
                }
            }
        }
        // The shift amount does not necessarily have to be the same size,
        // but if no size is specified, assume it is the same size
        OpCode::CPUI_INT_LEFT
        | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_SUBPIECE => {
            if opc != OpCode::CPUI_SUBPIECE {
                let out_zero = ops[op_index]
                    .get_out()
                    .expect("shift op without output (C++ dereferences null)")
                    .is_zero_size();
                if out_zero {
                    if !ops[op_index].get_in(0).is_zero_size() {
                        let size = ops[op_index].get_in(0).get_size().clone();
                        force_size_at(ops, op_index, VtSlot::Out, &size)?;
                    }
                } else if ops[op_index].get_in(0).is_zero_size() {
                    let size = ops[op_index]
                        .get_out()
                        .expect("shift op without output (C++ dereferences null)")
                        .get_size()
                        .clone();
                    force_size_at(ops, op_index, VtSlot::In(0), &size)?;
                }
            }
            // C++ fallthru to subpiece constant check
            if ops[op_index].get_in(1).is_zero_size() {
                force_size_at(
                    ops,
                    op_index,
                    VtSlot::In(1),
                    &ConstTpl::new_real(ConstType::Real, 4),
                )?;
            }
        }
        OpCode::CPUI_CPOOLREF => {
            let out_zero = ops[op_index]
                .get_out()
                .expect("cpoolref op without output (C++ dereferences null)")
                .is_zero_size();
            if out_zero && !ops[op_index].get_in(0).is_zero_size() {
                let size = ops[op_index].get_in(0).get_size().clone();
                force_size_at(ops, op_index, VtSlot::Out, &size)?;
            }
            let out_zero = ops[op_index]
                .get_out()
                .expect("cpoolref op without output (C++ dereferences null)")
                .is_zero_size();
            if ops[op_index].get_in(0).is_zero_size() && !out_zero {
                let size = ops[op_index]
                    .get_out()
                    .expect("cpoolref op without output (C++ dereferences null)")
                    .get_size()
                    .clone();
                force_size_at(ops, op_index, VtSlot::In(0), &size)?;
            }
            for i in 1..ops[op_index].num_input() {
                if ops[op_index].get_in(i).is_zero_size() {
                    // sizeof(uintb) == 8
                    force_size_at(
                        ops,
                        op_index,
                        VtSlot::In(i),
                        &ConstTpl::new_real(ConstType::Real, 8),
                    )?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

/// C++ static `PcodeCompile::propagateSize`: fill in sizes for varnodes
/// with size 0; returns false if some zero-size varnode could not be filled
/// in, true otherwise.
pub fn propagate_size(ct: &mut ConstructTpl) -> KunaResult<bool> {
    let ops = ct.get_opvec_mut();
    let mut zerovec: Vec<usize> = Vec::new();
    for i in 0..ops.len() {
        if ops[i].is_zero_size() {
            fillin_zero(i, ops)?;
            if ops[i].is_zero_size() {
                zerovec.push(i);
            }
        }
    }
    let mut lastsize = zerovec.len() + 1;
    while zerovec.len() < lastsize {
        lastsize = zerovec.len();
        let mut zerovec2: Vec<usize> = Vec::new();
        for &i in &zerovec {
            fillin_zero(i, ops)?;
            if ops[i].is_zero_size() {
                zerovec2.push(i);
            }
        }
        zerovec = zerovec2;
    }
    Ok(lastsize == 0)
}

// ---------------------------------------------------------------------------
// PcodeCompile
// ---------------------------------------------------------------------------

/// C++ `PcodeCompile`: the expression/statement builder.  See the module
/// docs for the trait-shape mapping.  Initialize the backing state to the
/// C++ constructor values: null spaces, `local_labelcount = 0`,
/// `enforceLocalKey = false`.
pub trait PcodeCompile {
    // -- C++ data members (backed by the implementor) -------------------------

    /// C++ `getDefaultSpace` (the `defaultspace` member; `None` is the C++
    /// null initial value).
    fn get_default_space(&self) -> Option<Rc<AddrSpace>>;
    /// C++ `setDefaultSpace`.
    fn set_default_space(&mut self, spc: Rc<AddrSpace>);
    /// C++ `getConstantSpace` (the `constantspace` member).
    fn get_constant_space(&self) -> Option<Rc<AddrSpace>>;
    /// C++ `setConstantSpace`.
    fn set_constant_space(&mut self, spc: Rc<AddrSpace>);
    /// The C++ `uniqspace` member (no C++ getter; used by
    /// `buildTemporary`).
    fn get_unique_space(&self) -> Option<Rc<AddrSpace>>;
    /// C++ `setUniqueSpace`.
    fn set_unique_space(&mut self, spc: Rc<AddrSpace>);
    /// The C++ `enforceLocalKey` member.
    fn is_enforce_local_key(&self) -> bool;
    /// C++ `setEnforceLocalKey`.
    fn set_enforce_local_key(&mut self, val: bool);
    /// The C++ `local_labelcount` member.
    fn local_label_count(&self) -> u32;
    /// Setter for the C++ `local_labelcount` member.
    fn set_local_label_count(&mut self, val: u32);

    // -- C++ pure virtuals -----------------------------------------------------

    /// C++ pure virtual `allocateTemp`.
    fn allocate_temp(&mut self) -> u32;
    /// C++ pure virtual `addSymbol`.
    fn add_symbol(&mut self, sym: PcodeCompileSymbol);
    /// C++ pure virtual `getLocation(SleighSymbol *)`, keyed by symbol name
    /// in the port (see module docs).
    fn get_location(&self, symbol_name: &[u8]) -> Option<Location>;
    /// C++ pure virtual `reportError`.
    fn report_error(&mut self, loc: Option<&Location>, msg: &str);
    /// C++ pure virtual `reportWarning`.
    fn report_warning(&mut self, loc: Option<&Location>, msg: &str);

    // -- concrete C++ methods (pcodecompile.cc) ---------------------------------

    /// C++ `resetLabelCount`.
    fn reset_label_count(&mut self) {
        self.set_local_label_count(0);
    }

    /// C++ `buildTemporary`: build a temporary variable (with zerosize).
    fn build_temporary(&mut self) -> VarnodeTpl {
        let uniqspace = self
            .get_unique_space()
            .expect("PcodeCompile unique space not set (C++ would store a null space)");
        let temp = self.allocate_temp();
        let mut res = VarnodeTpl::new(
            ConstTpl::new_space(uniqspace),
            ConstTpl::new_real(ConstType::Real, u64::from(temp)),
            ConstTpl::new_real(ConstType::Real, 0),
        );
        res.set_unnamed(true);
        res
    }

    /// C++ `defineLabel`: create a label symbol and add it to the local
    /// scope.
    fn define_label(&mut self, name: &[u8]) -> Rc<LabelSymbol> {
        let count = self.local_label_count();
        let labsym = Rc::new(LabelSymbol::new(name, count));
        self.set_local_label_count(count.wadd(1)); // C++ local_labelcount++
        self.add_symbol(PcodeCompileSymbol::Label(Rc::clone(&labsym)));
        labsym
    }

    /// C++ `placeLabel`: create the placeholder OpTpl for a label.
    fn place_label(&mut self, labsym: &Rc<LabelSymbol>) -> Vec<OpTpl> {
        if labsym.is_placed() {
            let loc = self.get_location(labsym.get_name());
            let msg = format!(
                "Label '{}' is placed more than once",
                String::from_utf8_lossy(labsym.get_name())
            );
            self.report_error(loc.as_ref(), &msg);
        }
        labsym.set_placed();
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let mut res = Vec::new();
        let mut op = OpTpl::new(LABELBUILD);
        let idvn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            ConstTpl::new_real(ConstType::Real, u64::from(labsym.get_index())),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        op.add_input(idvn);
        res.push(op);
        res
    }

    /// C++ `newOutput`: assign `rhs` to a fresh named temporary, define the
    /// symbol, and return the statement ops (C++ default `size = 0`).
    fn new_output(
        &mut self,
        uses_local_key: bool,
        mut rhs: ExprTree,
        varname: &[u8],
        size: u32,
    ) -> KunaResult<Vec<OpTpl>> {
        let mut tmpvn = self.build_temporary();
        if size != 0 {
            // Size was explicitly specified
            tmpvn.set_size(ConstTpl::new_real(ConstType::Real, u64::from(size)));
        } else if rhs.get_size().get_type() == ConstType::Real && rhs.get_size().get_real() != 0 {
            // Inherit size from unnamed expression result.  Only inherit if
            // the size is real, otherwise we cannot build the VarnodeSymbol
            // with a placeholder constant.
            tmpvn.set_size(rhs.get_size().clone());
        }
        // capture the symbol triple before tmpvn moves into the expression
        // (C++ keeps reading through the tmpvn pointer after setOutput)
        let sym_space = tmpvn.get_space().get_space();
        let sym_offset = tmpvn.get_offset().get_real();
        // cast: C++ implicit uintb -> int4 (VarnodeSymbol size parameter)
        let sym_size = tmpvn.get_size().get_real() as i32;
        rhs.set_output(tmpvn)?;
        // Create new symbol regardless
        let sym = SleighSymbol::new_varnode(varname, sym_space, sym_offset, sym_size);
        self.add_symbol(PcodeCompileSymbol::Sleigh(Box::new(sym)));
        if !uses_local_key && self.is_enforce_local_key() {
            let loc = self.get_location(varname);
            let msg = format!(
                "Must use 'local' keyword to define symbol '{}'",
                String::from_utf8_lossy(varname)
            );
            self.report_error(loc.as_ref(), &msg);
        }
        Ok(ExprTree::to_vector(rhs))
    }

    /// C++ `newLocalDefinition`: create a new temporary symbol (without
    /// generating any pcode; C++ default `size = 0`).
    fn new_local_definition(&mut self, varname: &[u8], size: u32) {
        let uniqspace = self
            .get_unique_space()
            .expect("PcodeCompile unique space not set (C++ would store a null space)");
        let temp = self.allocate_temp();
        let sym = SleighSymbol::new_varnode(
            varname,
            uniqspace,
            u64::from(temp),
            size as i32, // cast: C++ implicit uint4 -> int4 (VarnodeSymbol size)
        );
        self.add_symbol(PcodeCompileSymbol::Sleigh(Box::new(sym)));
    }

    /// C++ `createOp(OpCode,ExprTree *)` (unary): new expression performing
    /// `opc` on `vn`; frees (consumes) the input expression.
    fn create_op(&mut self, opc: OpCode, mut vn: ExprTree) -> ExprTree {
        let outvn = self.build_temporary();
        let mut op = OpTpl::new(opc);
        op.add_input(
            vn.outvn
                .take()
                .expect("createOp: input expression has no output (C++ would push a null input)"),
        );
        op.set_output(outvn.clone());
        vn.ops.push(op);
        vn.outvn = Some(outvn); // C++ `new VarnodeTpl(*outvn)`
        vn
    }

    /// C++ `createOp(OpCode,ExprTree *,ExprTree *)` (binary): new
    /// expression performing `opc` on `vn1`, `vn2`; frees (consumes) the
    /// input expressions.
    fn create_op2(&mut self, opc: OpCode, mut vn1: ExprTree, mut vn2: ExprTree) -> ExprTree {
        let outvn = self.build_temporary();
        vn1.ops.append(&mut vn2.ops);
        let mut op = OpTpl::new(opc);
        op.add_input(
            vn1.outvn
                .take()
                .expect("createOp: input expression has no output (C++ would push a null input)"),
        );
        op.add_input(
            vn2.outvn
                .take()
                .expect("createOp: input expression has no output (C++ would push a null input)"),
        );
        op.set_output(outvn.clone());
        vn1.ops.push(op);
        vn1.outvn = Some(outvn); // C++ `new VarnodeTpl(*outvn)`
        vn1
    }

    /// C++ `createOpOut`: an op with explicit output and two inputs.
    fn create_op_out(
        &mut self,
        outvn: VarnodeTpl,
        opc: OpCode,
        mut vn1: ExprTree,
        mut vn2: ExprTree,
    ) -> ExprTree {
        vn1.ops.append(&mut vn2.ops);
        let mut op = OpTpl::new(opc);
        op.add_input(
            vn1.outvn.take().expect(
                "createOpOut: input expression has no output (C++ would push a null input)",
            ),
        );
        op.add_input(
            vn2.outvn.take().expect(
                "createOpOut: input expression has no output (C++ would push a null input)",
            ),
        );
        op.set_output(outvn.clone());
        vn1.ops.push(op);
        vn1.outvn = Some(outvn); // C++ `new VarnodeTpl(*outvn)`
        vn1
    }

    /// C++ `createOpOutUnary`: an op with explicit output and one input.
    fn create_op_out_unary(
        &mut self,
        outvn: VarnodeTpl,
        opc: OpCode,
        mut vn: ExprTree,
    ) -> ExprTree {
        let mut op = OpTpl::new(opc);
        op.add_input(vn.outvn.take().expect(
            "createOpOutUnary: input expression has no output (C++ would push a null input)",
        ));
        op.set_output(outvn.clone());
        vn.ops.push(op);
        vn.outvn = Some(outvn); // C++ `new VarnodeTpl(*outvn)`
        vn
    }

    /// C++ `createOpNoOut(OpCode,ExprTree *)` (unary): statement op with
    /// `opc` and single input; frees (consumes) the input expression.
    fn create_op_no_out(&mut self, opc: OpCode, mut vn: ExprTree) -> Vec<OpTpl> {
        let mut op = OpTpl::new(opc);
        op.add_input(
            vn.outvn.take().expect(
                "createOpNoOut: input expression has no output (C++ would push a null input)",
            ),
        );
        // There is no longer an output to this expression
        let mut res = std::mem::take(&mut vn.ops);
        res.push(op);
        res
    }

    /// C++ `createOpNoOut(OpCode,ExprTree *,ExprTree *)` (binary).
    fn create_op_no_out2(
        &mut self,
        opc: OpCode,
        mut vn1: ExprTree,
        mut vn2: ExprTree,
    ) -> Vec<OpTpl> {
        let mut res = std::mem::take(&mut vn1.ops);
        res.append(&mut vn2.ops);
        let mut op = OpTpl::new(opc);
        op.add_input(
            vn1.outvn.take().expect(
                "createOpNoOut: input expression has no output (C++ would push a null input)",
            ),
        );
        op.add_input(
            vn2.outvn.take().expect(
                "createOpNoOut: input expression has no output (C++ would push a null input)",
            ),
        );
        res.push(op);
        res
    }

    /// C++ `createOpConst`: statement op whose single input is a constant.
    fn create_op_const(&mut self, opc: OpCode, val: u64) -> Vec<OpTpl> {
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            ConstTpl::new_real(ConstType::Real, val),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let mut res = Vec::new();
        let mut op = OpTpl::new(opc);
        op.add_input(vn);
        res.push(op);
        res
    }

    /// C++ `createLoad`: new load expression; frees (consumes) the pointer
    /// expression and the qualifier.
    fn create_load(&mut self, qual: StarQuality, mut ptr: ExprTree) -> KunaResult<ExprTree> {
        let outvn = self.build_temporary();
        let mut op = OpTpl::new(OpCode::CPUI_LOAD);
        // The first varnode input to the load is a constant reference to the
        // AddrSpace being loaded from.  Internally, we really store the
        // pointer to the AddrSpace as the reference, but this isn't platform
        // independent.  So officially, we assume that the constant reference
        // will be the AddrSpace index.  (Upstream comment; the emitted size
        // below is 8, as in the C++ code.)
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let spcvn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            qual.id.clone(),
            ConstTpl::new_real(ConstType::Real, 8),
        );
        op.add_input(spcvn);
        op.add_input(
            ptr.outvn
                .take()
                .expect("createLoad: pointer expression has no output (C++ would push null)"),
        );
        op.set_output(outvn);
        ptr.ops.push(op);
        if qual.size > 0 {
            // C++ force_size(outvn, ...): the output varnode lives in the op
            // just pushed
            let last = ptr.ops.len() - 1;
            force_size_at(
                &mut ptr.ops,
                last,
                VtSlot::Out,
                &ConstTpl::new_real(ConstType::Real, u64::from(qual.size)),
            )?;
        }
        // re-copy the output varnode into outvn (after the force_size)
        ptr.outvn = ptr.ops.last().and_then(|op| op.get_out()).cloned();
        Ok(ptr)
    }

    /// C++ `createStore`: frees (consumes) the qualifier and both
    /// expressions.
    fn create_store(
        &mut self,
        qual: StarQuality,
        mut ptr: ExprTree,
        mut val: ExprTree,
    ) -> KunaResult<Vec<OpTpl>> {
        let mut res = std::mem::take(&mut ptr.ops);
        res.append(&mut val.ops);
        let mut op = OpTpl::new(OpCode::CPUI_STORE);
        // (same constant-space-reference comment as createLoad)
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let spcvn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            qual.id.clone(),
            ConstTpl::new_real(ConstType::Real, 8),
        );
        op.add_input(spcvn);
        op.add_input(
            ptr.outvn
                .take()
                .expect("createStore: pointer expression has no output (C++ would push null)"),
        );
        op.add_input(
            val.outvn
                .take()
                .expect("createStore: value expression has no output (C++ would push null)"),
        );
        res.push(op);
        // C++ force_size(val->outvn,...) — val's output is input slot 2 of
        // the op just pushed (unconditional, unlike createLoad)
        let last = res.len() - 1;
        force_size_at(
            &mut res,
            last,
            VtSlot::In(2),
            &ConstTpl::new_real(ConstType::Real, u64::from(qual.size)),
        )?;
        Ok(res)
    }

    /// C++ `createUserOp`: a user-defined pcode op with an output.
    fn create_user_op(&mut self, sym: &UserOpSymbol, param: Vec<ExprTree>) -> ExprTree {
        let outvn = self.build_temporary();
        let mut ops = self.create_user_op_no_out(sym, param);
        ops.last_mut()
            .expect("createUserOpNoOut returned no ops (internal invariant)")
            .set_output(outvn.clone());
        ExprTree {
            ops,
            outvn: Some(outvn), // C++ `new VarnodeTpl(*outvn)`
        }
    }

    /// C++ `createUserOpNoOut`.
    fn create_user_op_no_out(&mut self, sym: &UserOpSymbol, param: Vec<ExprTree>) -> Vec<OpTpl> {
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let mut op = OpTpl::new(OpCode::CPUI_CALLOTHER);
        let vn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            ConstTpl::new_real(ConstType::Real, u64::from(sym.get_index())),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        op.add_input(vn);
        ExprTree::append_params(op, param)
    }

    /// C++ `createVariadic`.
    fn create_variadic(&mut self, opc: OpCode, param: Vec<ExprTree>) -> ExprTree {
        let outvn = self.build_temporary();
        let op = OpTpl::new(opc);
        let mut ops = ExprTree::append_params(op, param);
        ops.last_mut()
            .expect("appendParams returned no ops (internal invariant)")
            .set_output(outvn.clone());
        ExprTree {
            ops,
            outvn: Some(outvn), // C++ `new VarnodeTpl(*outvn)`
        }
    }

    /// C++ `appendOp`: take the output of `res`, combine with a constant
    /// using `opc`, leaving the result as `res`'s new output.
    fn append_op(&mut self, opc: OpCode, res: &mut ExprTree, constval: u64, constsz: i32) {
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        let mut op = OpTpl::new(opc);
        let constvn = VarnodeTpl::new(
            ConstTpl::new_space(constantspace),
            ConstTpl::new_real(ConstType::Real, constval),
            // cast: C++ implicit int4 -> uintb sign extension
            ConstTpl::new_real(ConstType::Real, i64::from(constsz) as u64),
        );
        let outvn = self.build_temporary();
        op.add_input(
            res.outvn
                .take()
                .expect("appendOp: expression has no output (C++ would push a null input)"),
        );
        op.add_input(constvn);
        op.set_output(outvn.clone());
        res.ops.push(op);
        res.outvn = Some(outvn); // C++ `new VarnodeTpl(*outvn)`
    }

    /// C++ `buildTruncatedVarnode`: build a truncated form of `basevn`
    /// matching the bitrange `[bitoffset, numbits]` if possible using just
    /// ConstTpl mechanics, otherwise return `None` (C++ null).
    fn build_truncated_varnode(
        &self,
        basevn: &VarnodeTpl,
        bitoffset: u32,
        numbits: u32,
    ) -> KunaResult<Option<VarnodeTpl>> {
        let byteoffset = bitoffset / 8; // Convert to byte units
        let numbytes = numbits / 8;
        let mut fullsz: u64 = 0;
        if basevn.get_size().get_type() == ConstType::Real {
            // If we know the size of base, make sure the bit range is in
            // bounds
            fullsz = basevn.get_size().get_real();
            if fullsz == 0 {
                return Ok(None);
            }
            // C++ adds the two uint4s first (can wrap) then widens for the
            // uintb comparison
            if u64::from(byteoffset.wadd(numbytes)) > fullsz {
                return Err(KunaError::sleigh("Requested bit range out of bounds"));
            }
        }

        if !bitoffset.is_multiple_of(8) {
            return Ok(None);
        }
        if !numbits.is_multiple_of(8) {
            return Ok(None);
        }

        let offset_type = basevn.get_offset().get_type();
        if offset_type != ConstType::Real && offset_type != ConstType::Handle {
            return Ok(None);
        }

        let specialoff: ConstTpl = if offset_type == ConstType::Handle {
            // We put in the correct adjustment to offset assuming things are
            // little endian.  We defer the correct big endian calculation
            // until after the consistency check because we need to know the
            // subtable export sizes.
            ConstTpl::new_handle_plus(
                basevn.get_offset().get_handle_index(),
                VField::VOffsetPlus,
                u64::from(byteoffset),
            )
        } else {
            if basevn.get_size().get_type() != ConstType::Real {
                return Err(KunaError::sleigh("Could not construct requested bit range"));
            }
            let defaultspace = self
                .get_default_space()
                .expect("PcodeCompile default space not set (C++ dereferences null)");
            let plus: u64 = if defaultspace.is_big_endian() {
                fullsz.wsub(u64::from(byteoffset.wadd(numbytes)))
            } else {
                u64::from(byteoffset)
            };
            ConstTpl::new_real(ConstType::Real, basevn.get_offset().get_real().wadd(plus))
        };
        let res = VarnodeTpl::new(
            basevn.get_space().clone(),
            specialoff,
            ConstTpl::new_real(ConstType::Real, u64::from(numbytes)),
        );
        Ok(Some(res))
    }

    /// C++ `assignBitRange`: create an expression assigning the rhs to a
    /// bitrange within the symbol varnode `vn`.
    fn assign_bit_range(
        &mut self,
        vn: VarnodeTpl,
        bitoffset: u32,
        numbits: u32,
        mut rhs: ExprTree,
    ) -> KunaResult<Vec<OpTpl>> {
        let mut errmsg = String::new();
        if numbits == 0 {
            errmsg = "Size of bitrange is zero".to_string();
        }
        let smallsize = numbits.wadd(7) / 8; // Size of input (output of rhs)
        let shiftneeded = bitoffset != 0;
        let mut zextneeded = true;
        let mut mask: u64 = 2;
        // (C++ shifts are UB for counts >= 64 / numbits == 0; the wrapping
        // helpers wrap the counts instead)
        mask = !(mask.wshl(numbits.wsub(1)).wsub(1).wshl(bitoffset));

        if vn.get_size().get_type() == ConstType::Real {
            // If we know the size of the bitranged varnode, we can do some
            // immediate checks, and possibly simplify things
            let mut symsize = vn.get_size().get_real() as u32; // cast: C++ implicit uintb -> uint4
            if symsize > 0 {
                zextneeded = symsize > smallsize;
            }
            symsize = symsize.wmul(8); // Convert to number of bits
            if bitoffset >= symsize || bitoffset.wadd(numbits) > symsize {
                errmsg = "Assigned bitrange is bad".to_string();
            } else if bitoffset == 0 && numbits == symsize {
                errmsg = "Assigning to bitrange is superfluous".to_string();
            }
        }

        if !errmsg.is_empty() {
            // Was there an error condition: report it and pass through the
            // old expression (C++ deletes vn and rhs)
            self.report_error(None, &errmsg);
            return Ok(std::mem::take(&mut rhs.ops));
        }

        // We know what the size of the input has to be
        {
            let rhs_ops = &mut rhs.ops;
            let rhs_out = rhs
                .outvn
                .as_mut()
                .expect("assignBitRange: rhs expression has no output (C++ dereferences null)");
            force_size(
                rhs_out,
                &ConstTpl::new_real(ConstType::Real, u64::from(smallsize)),
                rhs_ops,
            )?;
        }

        let finalout = self.build_truncated_varnode(&vn, bitoffset, numbits)?;
        let mut res = if let Some(finalout) = finalout {
            // (C++ deletes vn — don't keep the original varnode object)
            self.create_op_out_unary(finalout, OpCode::CPUI_COPY, rhs)
        } else {
            if bitoffset.wadd(numbits) > 64 {
                errmsg = "Assigned bitrange extends past first 64 bits".to_string();
            }
            // C++ reads vn again (unmutated) for finalout after building the
            // AND chain; clone up front
            let finalout = vn.clone();
            let mut lhs = ExprTree::new(vn);
            self.append_op(OpCode::CPUI_INT_AND, &mut lhs, mask, 0);
            if zextneeded {
                rhs = self.create_op(OpCode::CPUI_INT_ZEXT, rhs);
            }
            if shiftneeded {
                self.append_op(OpCode::CPUI_INT_LEFT, &mut rhs, u64::from(bitoffset), 4);
            }
            self.create_op_out(finalout, OpCode::CPUI_INT_OR, lhs, rhs)
        };
        if !errmsg.is_empty() {
            self.report_error(None, &errmsg);
        }
        Ok(std::mem::take(&mut res.ops))
    }

    /// C++ `createBitRange(SpecificSymbol *sym,...)`: create an expression
    /// computing the indicated bitrange of the symbol.  The result is
    /// truncated to the smallest byte size that can contain the indicated
    /// number of bits, with the desired bits shifted all the way to the
    /// right.  The port takes the already-resolved `sym->getVarnode()` as
    /// `vn` and `sym->getName()` as `sym_name` (see module docs).
    fn create_bit_range(
        &mut self,
        mut vn: VarnodeTpl,
        sym_name: &[u8],
        bitoffset: u32,
        numbits: u32,
    ) -> KunaResult<ExprTree> {
        let mut bitoffset = bitoffset;
        let mut errmsg = String::new();
        if numbits == 0 {
            errmsg = "Size of bitrange is zero".to_string();
        }
        let finalsize = numbits.wadd(7) / 8; // Round up to nearest byte size
        let mut truncshift: u32 = 0;
        let mut maskneeded = !numbits.is_multiple_of(8);
        let mut truncneeded = true;

        // Special case where we can set the size, without invoking a
        // truncation operator
        if errmsg.is_empty()
            && bitoffset == 0
            && !maskneeded
            && vn.get_space().get_type() == ConstType::Handle
            && vn.is_zero_size()
        {
            vn.set_size(ConstTpl::new_real(ConstType::Real, u64::from(finalsize)));
            return Ok(ExprTree::new(vn));
        }

        if errmsg.is_empty() {
            let truncvn = self.build_truncated_varnode(&vn, bitoffset, numbits)?;
            if let Some(truncvn) = truncvn {
                // If we are able to construct a simple truncated varnode,
                // return just the varnode as an expression (C++ deletes vn)
                return Ok(ExprTree::new(truncvn));
            }
        }

        if vn.get_size().get_type() == ConstType::Real {
            // If we know the size of the input varnode, we can do some
            // immediate checks, and possibly simplify things
            let mut insize = vn.get_size().get_real() as u32; // cast: C++ implicit uintb -> uint4
            if insize > 0 {
                truncneeded = finalsize < insize;
                insize = insize.wmul(8); // Convert to number of bits
                if bitoffset >= insize || bitoffset.wadd(numbits) > insize {
                    errmsg = "Bitrange is bad".to_string();
                }
                if maskneeded && bitoffset.wadd(numbits) == insize {
                    maskneeded = false;
                }
            }
        }

        let mut mask: u64 = 2;
        mask = mask.wshl(numbits.wsub(1)).wsub(1);

        if truncneeded && bitoffset.is_multiple_of(8) {
            truncshift = bitoffset / 8;
            bitoffset = 0;
        }

        if bitoffset == 0 && !truncneeded && !maskneeded {
            errmsg = "Superfluous bitrange".to_string();
        }

        if maskneeded && finalsize > 8 {
            errmsg = format!(
                "Illegal masked bitrange producing varnode larger than 64 bits: {}",
                String::from_utf8_lossy(sym_name)
            );
        }

        let mut res = ExprTree::new(vn);

        if !errmsg.is_empty() {
            // Check for error condition
            let loc = self.get_location(sym_name);
            self.report_error(loc.as_ref(), &errmsg);
            return Ok(res);
        }

        if bitoffset != 0 {
            self.append_op(OpCode::CPUI_INT_RIGHT, &mut res, u64::from(bitoffset), 4);
        }
        if truncneeded {
            self.append_op(OpCode::CPUI_SUBPIECE, &mut res, u64::from(truncshift), 4);
        }
        if maskneeded {
            self.append_op(
                OpCode::CPUI_INT_AND,
                &mut res,
                mask,
                finalsize as i32, // cast: C++ implicit uint4 -> int4 (constsz parameter)
            );
        }
        {
            let res_ops = &mut res.ops;
            let res_out = res
                .outvn
                .as_mut()
                .expect("createBitRange: expression has no output (C++ dereferences null)");
            force_size(
                res_out,
                &ConstTpl::new_real(ConstType::Real, u64::from(finalsize)),
                res_ops,
            )?;
        }
        Ok(res)
    }

    /// C++ `addressOf`: produce the constant varnode that is the offset
    /// portion of varnode `var` (C++ default `size = 0`); consumes `var`.
    fn address_of(&self, var: VarnodeTpl, size: u32) -> VarnodeTpl {
        let mut size = size;
        if size == 0 {
            // If no size specified
            if var.get_space().get_type() == ConstType::Spaceid {
                // Look to the particular space to see if it has a standard
                // address size
                let spc = var.get_space().get_space();
                size = spc.get_addr_size();
            }
        }
        let constantspace = self
            .get_constant_space()
            .expect("PcodeCompile constant space not set (C++ would store a null space)");
        if var.get_offset().get_type() == ConstType::Real
            && var.get_space().get_type() == ConstType::Spaceid
        {
            let spc = var.get_space().get_space();
            let off = AddrSpace::byte_to_address(var.get_offset().get_real(), spc.get_word_size());
            VarnodeTpl::new(
                ConstTpl::new_space(constantspace),
                ConstTpl::new_real(ConstType::Real, off),
                ConstTpl::new_real(ConstType::Real, u64::from(size)),
            )
        } else {
            VarnodeTpl::new(
                ConstTpl::new_space(constantspace),
                var.get_offset().clone(),
                ConstTpl::new_real(ConstType::Real, u64::from(size)),
            )
        }
        // (C++ deletes var; the by-value parameter drops here)
    }
}

#[cfg(test)]
mod tests {
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use super::*;
    use crate::semantics::HandleTpl;
    use crate::slghsymbol::SymbolType;

    // -- fixture: a synthetic PcodeSnippet-like implementor ---------------------

    fn test_manager(big_endian: bool) -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            big_endian,
            4,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(ram)).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(2, 0, big_endian)))
            .unwrap();
        manager
    }

    struct TestCompile {
        defaultspace: Option<Rc<AddrSpace>>,
        constantspace: Option<Rc<AddrSpace>>,
        uniqspace: Option<Rc<AddrSpace>>,
        local_labelcount: u32,
        enforce_local_key: bool,
        tempbase: u32,
        symbols: Vec<PcodeCompileSymbol>,
        errors: Vec<String>,
    }

    impl TestCompile {
        fn new(manager: &AddrSpaceManager) -> TestCompile {
            TestCompile {
                defaultspace: manager.get_space_by_name("ram").cloned(),
                constantspace: manager.get_constant_space().cloned(),
                uniqspace: manager.get_space_by_name("unique").cloned(),
                local_labelcount: 0,
                enforce_local_key: false,
                tempbase: 0x100,
                symbols: Vec::new(),
                errors: Vec::new(),
            }
        }
    }

    impl PcodeCompile for TestCompile {
        fn get_default_space(&self) -> Option<Rc<AddrSpace>> {
            self.defaultspace.clone()
        }
        fn set_default_space(&mut self, spc: Rc<AddrSpace>) {
            self.defaultspace = Some(spc);
        }
        fn get_constant_space(&self) -> Option<Rc<AddrSpace>> {
            self.constantspace.clone()
        }
        fn set_constant_space(&mut self, spc: Rc<AddrSpace>) {
            self.constantspace = Some(spc);
        }
        fn get_unique_space(&self) -> Option<Rc<AddrSpace>> {
            self.uniqspace.clone()
        }
        fn set_unique_space(&mut self, spc: Rc<AddrSpace>) {
            self.uniqspace = Some(spc);
        }
        fn is_enforce_local_key(&self) -> bool {
            self.enforce_local_key
        }
        fn set_enforce_local_key(&mut self, val: bool) {
            self.enforce_local_key = val;
        }
        fn local_label_count(&self) -> u32 {
            self.local_labelcount
        }
        fn set_local_label_count(&mut self, val: u32) {
            self.local_labelcount = val;
        }
        fn allocate_temp(&mut self) -> u32 {
            // PcodeSnippet-style: tempbase advances per temporary
            let res = self.tempbase;
            self.tempbase = self.tempbase.wadd(16);
            res
        }
        fn add_symbol(&mut self, sym: PcodeCompileSymbol) {
            self.symbols.push(sym);
        }
        fn get_location(&self, _symbol_name: &[u8]) -> Option<Location> {
            None // PcodeSnippet::getLocation returns null
        }
        fn report_error(&mut self, _loc: Option<&Location>, msg: &str) {
            self.errors.push(msg.to_string());
        }
        fn report_warning(&mut self, _loc: Option<&Location>, _msg: &str) {}
    }

    fn ram_vn(manager: &AddrSpaceManager, off: u64, size: u64) -> VarnodeTpl {
        VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(manager.get_space_by_name("ram").unwrap())),
            ConstTpl::new_real(ConstType::Real, off),
            ConstTpl::new_real(ConstType::Real, size),
        )
    }

    // -- ExprTree ---------------------------------------------------------------

    #[test]
    fn pcodecompile_exprtree_set_output_named_inserts_copy() {
        let manager = test_manager(false);
        // a named varnode expression: setOutput must add a COPY
        let mut tree = ExprTree::new(ram_vn(&manager, 0x42, 4));
        tree.set_output(ram_vn(&manager, 0x80, 4)).unwrap();
        assert_eq!(tree.ops.len(), 1);
        assert_eq!(tree.ops[0].get_opcode(), OpCode::CPUI_COPY);
        assert_eq!(tree.ops[0].get_in(0).get_offset().get_real(), 0x42);
        assert_eq!(tree.ops[0].get_out().unwrap().get_offset().get_real(), 0x80);
        assert_eq!(tree.get_out().unwrap().get_offset().get_real(), 0x80);
    }

    #[test]
    fn pcodecompile_exprtree_set_output_unnamed_replaces_last_output() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        // INT_ADD of two named varnodes: output is an unnamed temp
        let mut tree = comp.create_op2(
            OpCode::CPUI_INT_ADD,
            ExprTree::new(ram_vn(&manager, 0, 4)),
            ExprTree::new(ram_vn(&manager, 4, 4)),
        );
        tree.set_output(ram_vn(&manager, 0x80, 4)).unwrap();
        // no COPY inserted: the ADD's output was replaced in place
        assert_eq!(tree.ops.len(), 1);
        assert_eq!(tree.ops[0].get_opcode(), OpCode::CPUI_INT_ADD);
        assert_eq!(tree.ops[0].get_out().unwrap().get_offset().get_real(), 0x80);

        // no output at all is the C++ SleighError
        let mut empty = ExprTree::default();
        let err = empty.set_output(ram_vn(&manager, 0, 4)).unwrap_err();
        assert_eq!(err.explain(), "Expression has no output");
    }

    // -- createOp* --------------------------------------------------------------

    #[test]
    fn pcodecompile_create_op_unary_and_binary() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);

        let neg = comp.create_op(
            OpCode::CPUI_INT_2COMP,
            ExprTree::new(ram_vn(&manager, 0, 4)),
        );
        assert_eq!(neg.ops.len(), 1);
        assert_eq!(neg.ops[0].get_opcode(), OpCode::CPUI_INT_2COMP);
        let out = neg.get_out().unwrap();
        assert!(out.is_unnamed());
        assert!(out.get_space().is_unique_space());
        assert_eq!(out.get_offset().get_real(), 0x100);

        // binary: vn2's ops are spliced after vn1's, then the op appended
        let lhs = comp.create_op(
            OpCode::CPUI_INT_NEGATE,
            ExprTree::new(ram_vn(&manager, 0, 4)),
        );
        let rhs = comp.create_op(
            OpCode::CPUI_INT_NEGATE,
            ExprTree::new(ram_vn(&manager, 4, 4)),
        );
        let lhs_out = lhs.get_out().unwrap().get_offset().get_real();
        let rhs_out = rhs.get_out().unwrap().get_offset().get_real();
        let add = comp.create_op2(OpCode::CPUI_INT_ADD, lhs, rhs);
        assert_eq!(add.ops.len(), 3);
        assert_eq!(add.ops[0].get_opcode(), OpCode::CPUI_INT_NEGATE);
        assert_eq!(add.ops[1].get_opcode(), OpCode::CPUI_INT_NEGATE);
        assert_eq!(add.ops[2].get_opcode(), OpCode::CPUI_INT_ADD);
        assert_eq!(add.ops[2].get_in(0).get_offset().get_real(), lhs_out);
        assert_eq!(add.ops[2].get_in(1).get_offset().get_real(), rhs_out);
    }

    #[test]
    fn pcodecompile_create_op_no_out_orders_inputs() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let cond = ExprTree::new(ram_vn(&manager, 0, 1));
        let dest = ExprTree::new(ram_vn(&manager, 0x40, 4));
        // CBRANCH(dest, cond) as the grammar builds it
        let ops = comp.create_op_no_out2(OpCode::CPUI_CBRANCH, dest, cond);
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].get_opcode(), OpCode::CPUI_CBRANCH);
        assert_eq!(ops[0].num_input(), 2);
        assert_eq!(ops[0].get_in(0).get_offset().get_real(), 0x40);
        assert_eq!(ops[0].get_in(1).get_offset().get_real(), 0);
        assert!(ops[0].get_out().is_none());

        let ops1 = comp.create_op_no_out(
            OpCode::CPUI_BRANCHIND,
            ExprTree::new(ram_vn(&manager, 8, 4)),
        );
        assert_eq!(ops1.len(), 1);
        assert_eq!(ops1[0].num_input(), 1);
    }

    #[test]
    fn pcodecompile_create_op_const() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let ops = comp.create_op_const(OpCode::CPUI_RETURN, 1);
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].get_opcode(), OpCode::CPUI_RETURN);
        assert!(ops[0].get_in(0).get_space().is_const_space());
        assert_eq!(ops[0].get_in(0).get_offset().get_real(), 1);
        assert_eq!(ops[0].get_in(0).get_size().get_real(), 4);
    }

    // -- load/store ---------------------------------------------------------------

    #[test]
    fn pcodecompile_create_load_forces_output_size() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let qual = StarQuality {
            id: ConstTpl::new_space(ram),
            size: 4,
        };
        let load = comp
            .create_load(qual, ExprTree::new(ram_vn(&manager, 0x20, 4)))
            .unwrap();
        assert_eq!(load.ops.len(), 1);
        let op = &load.ops[0];
        assert_eq!(op.get_opcode(), OpCode::CPUI_LOAD);
        assert_eq!(op.num_input(), 2);
        // input 0: constant-space reference to the loaded space, size 8
        assert!(op.get_in(0).get_space().is_const_space());
        assert_eq!(op.get_in(0).get_size().get_real(), 8);
        // input 1: the pointer
        assert_eq!(op.get_in(1).get_offset().get_real(), 0x20);
        // output: temp forced to the qualifier size, and the ExprTree's
        // outvn copy reflects the forced size
        assert_eq!(op.get_out().unwrap().get_size().get_real(), 4);
        assert_eq!(load.get_out().unwrap().get_size().get_real(), 4);
    }

    #[test]
    fn pcodecompile_create_store_op_shape() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let qual = StarQuality {
            id: ConstTpl::new_space(ram),
            size: 4,
        };
        // value is an unnamed zero-size temp so the force_size shows up
        let val = comp.create_op(
            OpCode::CPUI_INT_NEGATE,
            ExprTree::new(ram_vn(&manager, 4, 0)),
        );
        let ops = comp
            .create_store(qual, ExprTree::new(ram_vn(&manager, 0x20, 4)), val)
            .unwrap();
        assert_eq!(ops.len(), 2);
        let st = &ops[1];
        assert_eq!(st.get_opcode(), OpCode::CPUI_STORE);
        assert_eq!(st.num_input(), 3);
        assert!(st.get_in(0).get_space().is_const_space());
        assert_eq!(st.get_in(1).get_offset().get_real(), 0x20);
        // the stored value's size was forced to the qualifier size and
        // propagated to the temp's defining op
        assert_eq!(st.get_in(2).get_size().get_real(), 4);
        assert_eq!(ops[0].get_out().unwrap().get_size().get_real(), 4);
    }

    // -- user ops / variadic --------------------------------------------------------

    #[test]
    fn pcodecompile_create_user_op_no_out_and_with_out() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let mut sym = UserOpSymbol::default();
        sym.set_index(7);

        let params = vec![
            ExprTree::new(ram_vn(&manager, 0, 4)),
            ExprTree::new(ram_vn(&manager, 4, 4)),
        ];
        let ops = comp.create_user_op_no_out(&sym, params);
        assert_eq!(ops.len(), 1);
        let op = &ops[0];
        assert_eq!(op.get_opcode(), OpCode::CPUI_CALLOTHER);
        assert_eq!(op.num_input(), 3);
        // input 0: the userop index constant
        assert!(op.get_in(0).get_space().is_const_space());
        assert_eq!(op.get_in(0).get_offset().get_real(), 7);
        assert_eq!(op.get_in(0).get_size().get_real(), 4);
        assert_eq!(op.get_in(1).get_offset().get_real(), 0);
        assert_eq!(op.get_in(2).get_offset().get_real(), 4);
        assert!(op.get_out().is_none());

        let tree = comp.create_user_op(&sym, vec![ExprTree::new(ram_vn(&manager, 8, 4))]);
        assert_eq!(tree.ops.len(), 1);
        assert!(tree.ops[0].get_out().is_some());
        assert_eq!(
            tree.get_out().unwrap().get_offset().get_real(),
            tree.ops[0].get_out().unwrap().get_offset().get_real()
        );
    }

    #[test]
    fn pcodecompile_create_variadic_splices_param_ops() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let p0 = comp.create_op(
            OpCode::CPUI_INT_NEGATE,
            ExprTree::new(ram_vn(&manager, 0, 4)),
        );
        let p1 = ExprTree::new(ram_vn(&manager, 4, 4));
        let tree = comp.create_variadic(OpCode::CPUI_PIECE, vec![p0, p1]);
        assert_eq!(tree.ops.len(), 2);
        assert_eq!(tree.ops[0].get_opcode(), OpCode::CPUI_INT_NEGATE);
        assert_eq!(tree.ops[1].get_opcode(), OpCode::CPUI_PIECE);
        assert_eq!(tree.ops[1].num_input(), 2);
        assert!(tree.ops[1].get_out().is_some());
    }

    // -- labels -------------------------------------------------------------------

    #[test]
    fn pcodecompile_define_and_place_label() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let lab = comp.define_label(b"skip");
        assert_eq!(lab.get_index(), 0);
        assert_eq!(comp.local_label_count(), 1);
        assert_eq!(comp.symbols.len(), 1);
        assert_eq!(comp.symbols[0].get_name(), b"skip");
        assert!(!lab.is_placed());

        let ops = comp.place_label(&lab);
        assert!(lab.is_placed());
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].get_opcode(), LABELBUILD);
        assert!(ops[0].get_in(0).get_space().is_const_space());
        assert_eq!(ops[0].get_in(0).get_offset().get_real(), 0);
        assert!(comp.errors.is_empty());

        // placing again reports the C++ error text
        let _ = comp.place_label(&lab);
        assert_eq!(
            comp.errors,
            vec!["Label 'skip' is placed more than once".to_string()]
        );

        // refcount bookkeeping used by the grammar
        lab.increment_ref_count();
        lab.increment_ref_count();
        assert_eq!(lab.get_ref_count(), 2);

        // a second label gets the next index
        let lab2 = comp.define_label(b"next");
        assert_eq!(lab2.get_index(), 1);
    }

    // -- newOutput / newLocalDefinition ----------------------------------------------

    #[test]
    fn pcodecompile_new_output_inherits_size_and_defines_symbol() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        let rhs = ExprTree::new(ram_vn(&manager, 0x42, 4));
        let ops = comp.new_output(true, rhs, b"tmp", 0).unwrap();
        // named rhs output: a COPY into the new temp
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].get_opcode(), OpCode::CPUI_COPY);
        let out = ops[0].get_out().unwrap();
        assert!(out.get_space().is_unique_space());
        // size inherited from rhs
        assert_eq!(out.get_size().get_real(), 4);
        // the VarnodeSymbol was created
        assert_eq!(comp.symbols.len(), 1);
        match &comp.symbols[0] {
            PcodeCompileSymbol::Sleigh(sym) => {
                assert_eq!(sym.get_name(), b"tmp");
                assert_eq!(sym.get_type(), SymbolType::Varnode);
            }
            PcodeCompileSymbol::Label(_) => panic!("expected a varnode symbol"),
        }

        // enforceLocalKey error path
        comp.set_enforce_local_key(true);
        let rhs2 = ExprTree::new(ram_vn(&manager, 0x42, 4));
        let _ = comp.new_output(false, rhs2, b"bad", 0).unwrap();
        assert_eq!(
            comp.errors,
            vec!["Must use 'local' keyword to define symbol 'bad'".to_string()]
        );
    }

    #[test]
    fn pcodecompile_new_local_definition() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        comp.new_local_definition(b"loc", 2);
        assert_eq!(comp.symbols.len(), 1);
        assert_eq!(comp.symbols[0].get_name(), b"loc");
    }

    // -- bit ranges ---------------------------------------------------------------

    #[test]
    fn pcodecompile_build_truncated_varnode() {
        let manager = test_manager(false);
        let comp = TestCompile::new(&manager);
        // little-endian real offset: plus = byteoffset
        let vn = ram_vn(&manager, 0x100, 8);
        let trunc = comp.build_truncated_varnode(&vn, 16, 16).unwrap().unwrap();
        assert_eq!(trunc.get_offset().get_real(), 0x102);
        assert_eq!(trunc.get_size().get_real(), 2);

        // big-endian flips the offset
        let manager_be = test_manager(true);
        let comp_be = TestCompile::new(&manager_be);
        let vn_be = {
            let mut v = ram_vn(&manager_be, 0x100, 8);
            v.set_size(ConstTpl::new_real(ConstType::Real, 8));
            v
        };
        let trunc_be = comp_be
            .build_truncated_varnode(&vn_be, 16, 16)
            .unwrap()
            .unwrap();
        assert_eq!(trunc_be.get_offset().get_real(), 0x100 + (8 - (2 + 2)));

        // non-byte-aligned ranges are not constructible
        assert!(comp.build_truncated_varnode(&vn, 3, 8).unwrap().is_none());
        assert!(comp.build_truncated_varnode(&vn, 8, 3).unwrap().is_none());

        // handle offsets defer via v_offset_plus
        let hvn = VarnodeTpl::new_from_handle(2, false);
        let htrunc = comp.build_truncated_varnode(&hvn, 8, 8).unwrap().unwrap();
        assert_eq!(htrunc.get_offset().get_type(), ConstType::Handle);
        assert_eq!(htrunc.get_offset().get_select(), VField::VOffsetPlus);
        assert_eq!(htrunc.get_offset().get_handle_index(), 2);
        assert_eq!(htrunc.get_offset().get_real(), 1);

        // out-of-bounds range throws the C++ SleighError
        let err = comp.build_truncated_varnode(&vn, 32, 64).unwrap_err();
        assert_eq!(err.explain(), "Requested bit range out of bounds");
    }

    #[test]
    fn pcodecompile_assign_bit_range_truncated_path() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        // byte-aligned assignment into an 8-byte varnode: COPY to the
        // truncated target
        let vn = ram_vn(&manager, 0x100, 8);
        let rhs = ExprTree::new(ram_vn(&manager, 0x200, 2));
        let ops = comp.assign_bit_range(vn, 16, 16, rhs).unwrap();
        assert!(comp.errors.is_empty());
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].get_opcode(), OpCode::CPUI_COPY);
        let out = ops[0].get_out().unwrap();
        assert_eq!(out.get_offset().get_real(), 0x102);
        assert_eq!(out.get_size().get_real(), 2);
    }

    #[test]
    fn pcodecompile_assign_bit_range_mask_path() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        // non-byte-aligned: AND/ZEXT/SHIFT/OR sequence
        let vn = ram_vn(&manager, 0x100, 4);
        let rhs = ExprTree::new(ram_vn(&manager, 0x200, 1));
        let ops = comp.assign_bit_range(vn, 3, 5, rhs).unwrap();
        assert!(comp.errors.is_empty());
        let opcodes: Vec<OpCode> = ops.iter().map(|op| op.get_opcode()).collect();
        assert_eq!(
            opcodes,
            vec![
                OpCode::CPUI_INT_AND,
                OpCode::CPUI_INT_ZEXT,
                OpCode::CPUI_INT_LEFT,
                OpCode::CPUI_INT_OR,
            ]
        );
        // the AND mask clears bits [3,8)
        let mask = !(((2u64 << 4) - 1) << 3);
        assert_eq!(ops[0].get_in(1).get_offset().get_real(), mask);
        // the final OR writes back into the full varnode
        let out = ops[3].get_out().unwrap();
        assert_eq!(out.get_offset().get_real(), 0x100);
        assert_eq!(out.get_size().get_real(), 4);

        // error path: zero-size bitrange passes through the rhs ops
        let vn2 = ram_vn(&manager, 0x100, 4);
        let rhs2 = ExprTree::new(ram_vn(&manager, 0x200, 1));
        let ops2 = comp.assign_bit_range(vn2, 0, 0, rhs2).unwrap();
        assert!(ops2.is_empty());
        assert_eq!(comp.errors, vec!["Size of bitrange is zero".to_string()]);
    }

    #[test]
    fn pcodecompile_create_bit_range_paths() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);

        // zero-size handle varnode with byte-aligned range from 0: just set
        // the size
        let hvn = VarnodeTpl::new_from_handle(0, true);
        let res = comp.create_bit_range(hvn, b"opnd", 0, 16).unwrap();
        assert!(res.ops.is_empty());
        assert_eq!(res.get_out().unwrap().get_size().get_real(), 2);

        // truncated-varnode path
        let vn = ram_vn(&manager, 0x100, 8);
        let res2 = comp.create_bit_range(vn, b"reg", 16, 16).unwrap();
        assert!(res2.ops.is_empty());
        assert_eq!(res2.get_out().unwrap().get_offset().get_real(), 0x102);

        // shift+mask path: bits [3,8) of a 4-byte register
        let vn3 = ram_vn(&manager, 0x100, 4);
        let res3 = comp.create_bit_range(vn3, b"reg", 3, 5).unwrap();
        assert!(comp.errors.is_empty());
        let opcodes: Vec<OpCode> = res3.ops.iter().map(|op| op.get_opcode()).collect();
        assert_eq!(
            opcodes,
            vec![
                OpCode::CPUI_INT_RIGHT,
                OpCode::CPUI_SUBPIECE,
                OpCode::CPUI_INT_AND,
            ]
        );
        // INT_RIGHT by the bitoffset, SUBPIECE by 0 (byte shift), AND with
        // the 5-bit mask
        assert_eq!(res3.ops[0].get_in(1).get_offset().get_real(), 3);
        assert_eq!(res3.ops[1].get_in(1).get_offset().get_real(), 0);
        assert_eq!(res3.ops[2].get_in(1).get_offset().get_real(), 0x1f);
        // final size forced to 1 byte
        assert_eq!(res3.get_out().unwrap().get_size().get_real(), 1);

        // superfluous bitrange is an error: needs a varnode the truncated
        // path cannot handle (non-real, non-handle offset), full range
        let vn4 = VarnodeTpl::new(
            ConstTpl::new_space(Rc::clone(manager.get_space_by_name("ram").unwrap())),
            ConstTpl::new_type(ConstType::JStart),
            ConstTpl::new_real(ConstType::Real, 4),
        );
        let _ = comp.create_bit_range(vn4, b"reg", 0, 32).unwrap();
        assert_eq!(comp.errors, vec!["Superfluous bitrange".to_string()]);
    }

    // -- addressOf ----------------------------------------------------------------

    #[test]
    fn pcodecompile_address_of() {
        let manager = test_manager(false);
        let comp = TestCompile::new(&manager);
        // real offset in a spaceid varnode: offset surfaces as a constant,
        // with the space's address size when none specified
        let vn = ram_vn(&manager, 0x42, 4);
        let res = comp.address_of(vn, 0);
        assert!(res.get_space().is_const_space());
        assert_eq!(res.get_offset().get_real(), 0x42);
        assert_eq!(res.get_size().get_real(), 4);

        // non-real offset passes the offset template through
        let hvn = VarnodeTpl::new_from_handle(1, false);
        let res2 = comp.address_of(hvn, 2);
        assert!(res2.get_space().is_const_space());
        assert_eq!(res2.get_offset().get_type(), ConstType::Handle);
        assert_eq!(res2.get_size().get_real(), 2);
    }

    // -- size propagation -----------------------------------------------------------

    #[test]
    fn pcodecompile_propagate_size_fills_temps() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        // tmp = ram(0):4 + ram(4):4 ; ram(8):4 = tmp — temp sizes start 0
        let mut add = comp.create_op2(
            OpCode::CPUI_INT_ADD,
            ExprTree::new(ram_vn(&manager, 0, 4)),
            ExprTree::new(ram_vn(&manager, 4, 4)),
        );
        let temp_out = add.outvn.take().unwrap();
        let mut ops = ExprTree::to_vector(add);
        let mut copy = OpTpl::new(OpCode::CPUI_COPY);
        copy.add_input(temp_out);
        copy.set_output(ram_vn(&manager, 8, 4));
        ops.push(copy);
        let mut ct = ConstructTpl::new();
        assert!(ct.add_op_list(ops));
        assert!(propagate_size(&mut ct).unwrap());
        // the temp's size was filled in everywhere
        assert_eq!(
            ct.get_opvec()[0].get_out().unwrap().get_size().get_real(),
            4
        );
        assert_eq!(ct.get_opvec()[1].get_in(0).get_size().get_real(), 4);

        // a template whose zero sizes cannot be resolved returns false
        let mut ct2 = ConstructTpl::new();
        let mut comp2 = TestCompile::new(&manager);
        let lonely = comp2.create_op2(
            OpCode::CPUI_INT_ADD,
            ExprTree::new(ram_vn(&manager, 0, 0)),
            ExprTree::new(ram_vn(&manager, 4, 0)),
        );
        assert!(ct2.add_op_list(ExprTree::to_vector(lonely)));
        assert!(!propagate_size(&mut ct2).unwrap());
    }

    #[test]
    fn pcodecompile_fillin_zero_bool_and_shift_rules() {
        let manager = test_manager(false);
        let mut comp = TestCompile::new(&manager);
        // bool output: forced to size 1
        let eq = comp.create_op2(
            OpCode::CPUI_INT_EQUAL,
            ExprTree::new(ram_vn(&manager, 0, 4)),
            ExprTree::new(ram_vn(&manager, 4, 4)),
        );
        let mut ct = ConstructTpl::new();
        assert!(ct.add_op_list(ExprTree::to_vector(eq)));
        assert!(propagate_size(&mut ct).unwrap());
        assert_eq!(
            ct.get_opvec()[0].get_out().unwrap().get_size().get_real(),
            1
        );

        // shift: output inherits in(0)'s size, shift amount gets size 4
        let mut comp2 = TestCompile::new(&manager);
        let sh = comp2.create_op2(
            OpCode::CPUI_INT_LEFT,
            ExprTree::new(ram_vn(&manager, 0, 4)),
            ExprTree::new(ram_vn(&manager, 8, 0)),
        );
        let mut ct2 = ConstructTpl::new();
        assert!(ct2.add_op_list(ExprTree::to_vector(sh)));
        assert!(propagate_size(&mut ct2).unwrap());
        let op = &ct2.get_opvec()[0];
        assert_eq!(op.get_out().unwrap().get_size().get_real(), 4);
        assert_eq!(op.get_in(1).get_size().get_real(), 4);
    }

    #[test]
    fn pcodecompile_force_size_mismatch_is_error() {
        let manager = test_manager(false);
        let uniq = Rc::clone(manager.get_space_by_name("unique").unwrap());
        // two uses of the same local temp with conflicting real sizes
        let temp = |size: u64| {
            VarnodeTpl::new(
                ConstTpl::new_space(Rc::clone(&uniq)),
                ConstTpl::new_real(ConstType::Real, 0x100),
                ConstTpl::new_real(ConstType::Real, size),
            )
        };
        let mut op0 = OpTpl::new(OpCode::CPUI_COPY);
        op0.add_input(ram_vn(&manager, 0, 4));
        op0.set_output(temp(8));
        let mut ops = vec![op0];
        let mut vt = temp(0);
        let err =
            force_size(&mut vt, &ConstTpl::new_real(ConstType::Real, 4), &mut ops).unwrap_err();
        assert_eq!(err.explain(), "Localtemp size mismatch");
    }

    // -- Location -------------------------------------------------------------------

    #[test]
    fn pcodecompile_location_format() {
        let loc = Location::new(b"inject.pspec", 42);
        assert_eq!(loc.get_filename(), b"inject.pspec");
        assert_eq!(loc.get_lineno(), 42);
        assert_eq!(loc.format(), "inject.pspec:42");
    }

    // -- StarQuality smoke ------------------------------------------------------------

    #[test]
    fn pcodecompile_star_quality_holds_space_template() {
        let manager = test_manager(false);
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let qual = StarQuality {
            id: ConstTpl::new_space(Rc::clone(&ram)),
            size: 4,
        };
        assert_eq!(qual.id.get_type(), ConstType::Spaceid);
        assert!(Rc::ptr_eq(&qual.id.get_space(), &ram));
        let h = HandleTpl::new_from_varnode(&ram_vn(&manager, 0, 4));
        assert_eq!(h.get_ptr_space().get_real(), 0);
    }
}
