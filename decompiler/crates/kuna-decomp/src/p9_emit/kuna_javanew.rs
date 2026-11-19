//! (kuna `javanew`) `new T(args)`: an allocation and the constructor call that
//! initializes it, printed as one Java expression.
//!
//! Dalvik allocates an object with `new-instance` and initializes it with a
//! separate `invoke-direct` of `<init>`; Ghidra's Java output keeps the two
//! statements (`v = new(T); v.<init>(args);`). A Java reader expects
//! `v = new T(args);`. [`pairs`] finds each NEW whose object reaches nothing
//! before the `<init>` call that consumes it, in the same basic block, so the
//! printer can drop the NEW statement and print the call as the allocation.
//! Arguments keep their own statements and their order: they are evaluated
//! between the two operations, as Java evaluates them after the allocation.
//!
//! A constructor's call of another constructor on its own object is Java's
//! `this(args)` (same class) or `super(args)` (the superclass); [`ctor_calls`]
//! names them. A no-argument `super()` to `java.lang.Object` is the call Java
//! inserts implicitly, so it prints nothing.

use std::collections::HashMap;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::cpool::cpool_tag;
use crate::funcdata::Funcdata;

/// One allocation folded into its constructor call.
#[derive(Clone, Debug)]
pub struct NewInit {
    /// The NEW op, whose statement is dropped.
    pub new_op: OpId,
    /// The object the NEW creates.
    pub object: VarnodeId,
    /// The class the NEW names.
    pub class: String,
}

fn record_of(fd: &Funcdata, cpool: OpId) -> Option<std::rc::Rc<crate::cpool::CPoolRecord>> {
    let o = fd.obank().get(cpool)?;
    if o.code() != OpCode::CPUI_CPOOLREF {
        return None;
    }
    let refs: Vec<u64> = (1..o.num_input())
        .filter_map(|i| o.get_in(i).and_then(|v| fd.vbank().get(v)).map(|v| v.get_offset()))
        .collect();
    fd.get_arch().cpool_record(&refs)
}

fn def_of(fd: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    fd.vbank().get(vn).filter(|v| v.is_written()).and_then(|v| v.get_def())
}

/// Every `<init>` CALLIND that initializes an object allocated by a NEW earlier
/// in its block, keyed by the call.
pub fn pairs(fd: &Funcdata) -> HashMap<OpId, NewInit> {
    let mut out = HashMap::new();
    for i in 0..fd.num_calls() {
        let call = fd.get_call_specs(i).get_op();
        let Some(c) = fd.obank().get(call) else { continue };
        if c.code() != OpCode::CPUI_CALLIND || c.num_input() < 2 {
            continue;
        }
        let Some(target) = c.get_in(0).and_then(|vn| def_of(fd, vn)) else { continue };
        let Some(rec) = record_of(fd, target) else { continue };
        if rec.get_tag() != cpool_tag::POINTER_METHOD || rec.get_token() != b"<init>" {
            continue;
        }
        let Some(object) = c.get_in(1) else { continue };
        if fd.obank().get(target).and_then(|t| t.get_in(0)) != Some(object) {
            continue;
        }
        let Some(new_op) = def_of(fd, object) else { continue };
        let Some(n) = fd.obank().get(new_op) else { continue };
        if n.code() != OpCode::CPUI_NEW || n.num_input() != 1 || n.get_parent() != c.get_parent() {
            continue;
        }
        let Some(class) = n.get_in(0).and_then(|vn| def_of(fd, vn)).and_then(|d| record_of(fd, d)) else {
            continue;
        };
        if class.get_tag() != cpool_tag::CLASS_REFERENCE {
            continue;
        }
        let mut cur = fd.bb_op_next(new_op);
        let mut reached = false;
        while let Some(op) = cur {
            if op == call {
                reached = true;
                break;
            }
            let reads = fd
                .obank()
                .get(op)
                .is_some_and(|o| (0..o.num_input()).any(|k| o.get_in(k) == Some(object)));
            if reads && op != target {
                break;
            }
            cur = fd.bb_op_next(op);
        }
        if reached {
            let class = String::from_utf8_lossy(class.get_token()).into_owned();
            out.insert(call, NewInit { new_op, object, class });
        }
    }
    out
}

/// How a constructor's call of another constructor on its own object prints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtorCall {
    /// `this(args)`.
    This,
    /// `super(args)`.
    Super,
    /// The implicit `super()` to `java.lang.Object`: nothing.
    Implicit,
}

/// Every `<init>` CALLIND in a constructor that initializes the constructor's
/// own object (its `this` input), keyed by the call.
pub fn ctor_calls(fd: &Funcdata) -> HashMap<OpId, CtorCall> {
    let mut out = HashMap::new();
    let arch = fd.get_arch();
    let Some(model) = arch.dex.as_ref() else { return out };
    let Some((class, superclass)) = model.method_class(fd.get_address().get_offset()) else { return out };
    for i in 0..fd.num_calls() {
        let call = fd.get_call_specs(i).get_op();
        let Some(c) = fd.obank().get(call) else { continue };
        if c.code() != OpCode::CPUI_CALLIND || c.num_input() < 2 {
            continue;
        }
        let Some(target) = c.get_in(0).and_then(|vn| def_of(fd, vn)) else { continue };
        let Some(rec) = record_of(fd, target) else { continue };
        if rec.get_tag() != cpool_tag::POINTER_METHOD || rec.get_token() != b"<init>" {
            continue;
        }
        let Some(t) = fd.obank().get(target) else { continue };
        let Some(object) = c.get_in(1).filter(|&o| t.get_in(0) == Some(o)) else { continue };
        let mut source = object;
        while let Some(copy) = def_of(fd, source).and_then(|d| fd.obank().get(d)).filter(|d| d.code() == OpCode::CPUI_COPY) {
            let Some(from) = copy.get_in(0) else { break };
            source = from;
        }
        let is_this = fd.vbank().get(source).is_some_and(|v| {
            v.is_input() && v.get_offset() == crate::kuna_dex::INPUT_REGISTER_START
        });
        if !is_this {
            continue;
        }
        let Some(index) = t.get_in(1).and_then(|v| fd.vbank().get(v)).map(|v| v.get_offset()) else { continue };
        let Some(owner) = u32::try_from(index).ok().and_then(|i| model.method_owner(i)) else { continue };
        let kind = if owner == class {
            CtorCall::This
        } else if c.num_input() == 2
            && superclass.and_then(|s| model.type_descriptor(s)).as_deref() == Some("Ljava/lang/Object;")
        {
            CtorCall::Implicit
        } else {
            CtorCall::Super
        };
        out.insert(call, kind);
    }
    out
}
