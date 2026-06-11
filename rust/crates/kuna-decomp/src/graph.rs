//! Port of `decompiler/cpp/graph.{hh,cc}` — the Renoir-format graph dumpers.
//!
//! Three debug serializers emit a [`Funcdata`](crate::funcdata::Funcdata) or a
//! [`BlockGraph`](crate::block::BlockGraph) as a Renoir `*COLUMNAR_INPUT`
//! command stream:
//!   - [`dump_dataflow_graph`]   — the SSA data-flow graph (varnode + op
//!     vertices, def/use edges);
//!   - [`dump_controlflow_graph`] — the control-flow graph (basic-block
//!     vertices + edges);
//!   - [`dump_dom_graph`]        — the dominator tree.
//!
//! These are presentation-only (no IR mutation other than the temporary
//! `setMark`/`clearMark` bookkeeping the data-flow dump uses to print each
//! varnode vertex once).  The large fixed preamble/attribute blocks are
//! transcribed verbatim; the traversals (alive-op iteration order, the
//! `CPUI_LOAD/STORE/BRANCH/CALL`/`CPUI_INDIRECT` input-range special cases,
//! the block-index edges) follow the C++ exactly.
//!
//! The C++ writes to an `ostream`; the port appends to a `String` (the same
//! ASCII text).  C++ `dec`/`hex` stream-state is made explicit per field.
//!
//! SEAM(W4): `print_varnode_vertex` calls `Varnode::printRawNoMarkup`, whose
//! register-name branch needs the W4 `Translate` (reached through `glb`).  That
//! markup is supplied by the local [`GraphVarnodePrinter`] seam; the
//! shortcut+offset fallback (`else` branch of the C++) is otherwise fully
//! portable.  W4 supplies a printer over the real `Translate`.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::block::{block_get_start, block_get_stop, BlockGraph};
use crate::funcdata::Funcdata;
use crate::seams::BlockId;

/// Number of component blocks in the graph (C++ `BlockGraph::getSize`).
///
/// The C++ `BlockGraph` *is-a* `FlowBlock` whose `list` holds the components;
/// in the Rust port that list-bearing block is `graph.root`.  An unbuilt graph
/// (`root == None`) has size 0.
fn graph_size(graph: &BlockGraph) -> int4 {
    match graph.root {
        Some(root) => graph.block(root).get_size(),
        None => 0,
    }
}

/// The i-th component block of the graph (C++ `BlockGraph::getBlock`).
fn graph_block(graph: &BlockGraph, i: int4) -> BlockId {
    let root = graph
        .root
        .expect("graph_block: getBlock on an unbuilt BlockGraph");
    graph.block(root).get_block(i)
}

// ---------------------------------------------------------------------------
// SEAM(W4): the varnode register-markup printer.
// ---------------------------------------------------------------------------

/// SEAM(W4): supplies the `Varnode::printRawNoMarkup` register-name markup that
/// needs the W4 `Translate`.
///
/// The C++ `printRawNoMarkup` first tries `trans->getRegisterName(...)`; if the
/// varnode lands on a named register it prints `name[+off]`, else it prints the
/// space shortcut char followed by `Address::printRaw`.  The fallback branch is
/// available without W4 ([`default_varnode_markup`]); the register-name branch
/// is this seam.  W4 supplies an impl over the real `Translate`.
pub trait GraphVarnodePrinter {
    /// Markup for a varnode at `(addr, size)` — the C++
    /// `Varnode::printRawNoMarkup` text.  The default impl is the
    /// register-less fallback; W4 overrides it with the register-name form.
    fn print_raw_no_markup(&self, addr: &Address, size: int4) -> String {
        let _ = size;
        default_varnode_markup(addr)
    }
}

/// The register-less fallback of `Varnode::printRawNoMarkup`: the space
/// shortcut character followed by `Address::printRaw` (C++ `else` branch,
/// `varnode.cc:747-751`).
pub fn default_varnode_markup(addr: &Address) -> String {
    let mut s = String::new();
    s.push(addr.get_shortcut());
    // C++ `loc.printRaw(s)` — never errors for a normal (non-sentinel) address.
    addr.print_raw(&mut s)
        .expect("default_varnode_markup: Address::printRaw on a graph varnode");
    s
}

/// A no-op [`GraphVarnodePrinter`] using only the fallback markup (no W4
/// register names).  Useful for tests and pre-W4 callers.
pub struct FallbackVarnodePrinter;
impl GraphVarnodePrinter for FallbackVarnodePrinter {}

// ---------------------------------------------------------------------------
// Data-flow graph  (C++ `graph.cc:21-296`)
// ---------------------------------------------------------------------------

/// Append the hex form of `v` (C++ `s << hex << v`, no `0x`, lower case).
fn push_hex(s: &mut String, v: u64) {
    s.push_str(&format!("{:x}", v));
}

/// Append the decimal form of `v` (C++ `s << dec << v`).
fn push_dec_u32(s: &mut String, v: u32) {
    s.push_str(&format!("{}", v));
}

/// Append the decimal form of an `int4` (C++ `s << dec << v`).
fn push_dec_i32(s: &mut String, v: int4) {
    s.push_str(&format!("{}", v));
}

/// Print one varnode vertex if it has not been marked yet, then mark it
/// (C++ static `print_varnode_vertex`).
///
/// `vn` is the optional varnode id (C++ `Varnode *`, `None` == null).  The
/// varnode bank is borrowed mutably for the mark side effect.  Returns nothing;
/// the C++ early-returns on null / already-marked / fspec / iop varnodes.
fn print_varnode_vertex(
    data: &mut Funcdata,
    vn: Option<crate::seams::VarnodeId>,
    s: &mut String,
    printer: &dyn GraphVarnodePrinter,
) {
    let vn = match vn {
        None => return,
        Some(v) => v,
    };
    // Read-only inspection first (mirrors the C++ guard sequence).
    {
        let v = data
            .vbank()
            .get(vn)
            .expect("print_varnode_vertex: stale varnode id");
        if v.is_mark() {
            return;
        }
        let tp = v.get_space().get_type();
        if tp == spacetype::IPTR_FSPEC {
            return;
        }
        if tp == spacetype::IPTR_IOP {
            return;
        }
    }
    // Gather the fields we need to print without holding a borrow across the
    // markup call.
    let (create_index, space_name, addr, size, def, is_input) = {
        let v = data.vbank().get(vn).unwrap();
        (
            v.get_create_index(),
            v.get_space().get_name().to_string(),
            v.get_addr().clone(),
            v.get_size(),
            v.get_def(),
            v.is_input(),
        )
    };
    s.push('v');
    push_dec_u32(s, create_index);
    s.push(' ');
    s.push_str(&space_name);
    s.push_str(" var ");

    s.push_str(&printer.print_raw_no_markup(&addr, size));

    match def {
        Some(op) => {
            // op->getAddr().getOffset() in hex
            let off = data
                .obank()
                .get(op)
                .expect("print_varnode_vertex: stale def op")
                .get_addr()
                .get_offset();
            s.push(' ');
            push_hex(s, off);
        }
        None => {
            if is_input {
                s.push_str(" i");
            } else {
                s.push_str(" <na>");
            }
        }
    }
    s.push('\n');
    data.vbank_mut().get_mut(vn).unwrap().set_mark();
}

/// The (start, stop) input slot range for the varnode-vertex / edge passes,
/// reproducing the C++ `switch(op->code())` special cases (`graph.cc:91-103`,
/// `152-164`): `LOAD/STORE/BRANCH/CALL` skip input 0, `INDIRECT` keeps only
/// input 0.
fn input_range(code: OpCode, num_input: int4) -> (int4, int4) {
    let mut start = 0;
    let mut stop = num_input;
    match code {
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE | OpCode::CPUI_BRANCH | OpCode::CPUI_CALL => {
            start = 1;
        }
        OpCode::CPUI_INDIRECT => {
            stop = 1;
        }
        _ => {}
    }
    (start, stop)
}

/// Dump every varnode vertex (output + the relevant inputs of each alive op),
/// then clear the marks (C++ static `dump_varnode_vertex`).
fn dump_varnode_vertex(data: &mut Funcdata, s: &mut String, printer: &dyn GraphVarnodePrinter) {
    s.push_str("\n\n// Add Vertices\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddVertices,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=Internal, Location=1},\n");
    s.push_str("          {Name=SubClass, Location=2},\n");
    s.push_str("          {Name=Type, Location=3},\n");
    s.push_str("          {Name=Name, Location=4},\n");
    s.push_str("          {Name=Address, Location=5});\n\n");
    s.push_str("//START:varnodes\n");

    // C++ `for(oiter=beginOpAlive(); ...; ++oiter)`: snapshot the alive order
    // so the mutable mark side-effects don't disturb iteration.
    let alive: Vec<crate::seams::OpId> = data.obank().iter_alive().collect();
    for &op in alive.iter() {
        let (out, code, num_input) = {
            let o = data.obank().get(op).expect("dump_varnode_vertex: stale op");
            (o.get_out(), o.code(), o.num_input())
        };
        print_varnode_vertex(data, out, s, printer);
        let (start, stop) = input_range(code, num_input);
        for i in start..stop {
            let in_vn = data.obank().get(op).unwrap().get_in(i);
            print_varnode_vertex(data, in_vn, s, printer);
        }
    }
    s.push_str("*END_COLUMNS\n");
    // Second pass: clear all the marks (C++ clears out + every input).
    for &op in alive.iter() {
        let out = data.obank().get(op).unwrap().get_out();
        if let Some(vn) = out {
            data.vbank_mut().get_mut(vn).unwrap().clear_mark();
        }
        let num_input = data.obank().get(op).unwrap().num_input();
        for i in 0..num_input {
            let in_vn = data.obank().get(op).unwrap().get_in(i);
            if let Some(vn) = in_vn {
                data.vbank_mut().get_mut(vn).unwrap().clear_mark();
            }
        }
    }
}

/// Print one op vertex (C++ static `print_op_vertex`).
fn print_op_vertex(data: &Funcdata, op: crate::seams::OpId, s: &mut String) {
    let o = data.obank().get(op).expect("print_op_vertex: stale op");
    s.push('o');
    push_dec_u32(s, o.get_time());
    s.push(' ');
    if o.is_branch() {
        s.push_str("branch");
    } else if o.is_call() {
        s.push_str("call");
    } else if o.is_marker() {
        s.push_str("marker");
    } else {
        s.push_str("basic");
    }
    s.push_str(" op ");
    let name = o.get_op_name();
    if !name.is_empty() {
        s.push_str(name);
    } else {
        s.push_str("unkop");
    }
    s.push(' ');
    push_hex(s, o.get_addr().get_offset());
    s.push('\n');
}

/// Dump every op vertex (C++ static `dump_op_vertex`).
fn dump_op_vertex(data: &Funcdata, s: &mut String) {
    s.push_str("\n\n// Add Vertices\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddVertices,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=Internal, Location=1},\n");
    s.push_str("          {Name=SubClass, Location=2},\n");
    s.push_str("          {Name=Type, Location=3},\n");
    s.push_str("          {Name=Name, Location=4},\n");
    s.push_str("          {Name=Address, Location=5});\n\n");
    s.push_str("//START:opnodes\n");

    for op in data.obank().iter_alive() {
        print_op_vertex(data, op, s);
    }
    s.push_str("*END_COLUMNS\n");
}

/// Print the def/use edges for one op (C++ static `print_edges`).
fn print_edges(data: &Funcdata, op: crate::seams::OpId, s: &mut String) {
    let o = data.obank().get(op).expect("print_edges: stale op");
    let time = o.get_time();
    if let Some(vn) = o.get_out() {
        let create_index = data
            .vbank()
            .get(vn)
            .expect("print_edges: stale out varnode")
            .get_create_index();
        // 'o' << dec time << " v" << dec create_index << " output\n"
        s.push('o');
        push_dec_u32(s, time);
        s.push_str(" v");
        push_dec_u32(s, create_index);
        s.push_str(" output\n");
    }
    let (start, stop) = input_range(o.code(), o.num_input());
    for i in start..stop {
        let in_vn = o.get_in(i);
        let vn = match in_vn {
            None => continue, // C++ dereferences unconditionally; guard the seam
            Some(v) => v,
        };
        let v = data
            .vbank()
            .get(vn)
            .expect("print_edges: stale in varnode");
        let tp = v.get_space().get_type();
        if tp != spacetype::IPTR_FSPEC && tp != spacetype::IPTR_IOP {
            // 'v' << dec create_index << " o" << dec time << " input\n"
            s.push('v');
            push_dec_u32(s, v.get_create_index());
            s.push_str(" o");
            push_dec_u32(s, time);
            s.push_str(" input\n");
        }
    }
}

/// Dump all the def/use edges (C++ static `dump_edges`).
fn dump_edges(data: &Funcdata, s: &mut String) {
    s.push_str("\n\n// Add Edges\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddEdges,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=*FromKey, Location=1},\n");
    s.push_str("          {Name=*ToKey, Location=2},\n");
    s.push_str("          {Name=Name, Location=3});\n\n");
    s.push_str("//START:edges\n");

    for op in data.obank().iter_alive() {
        print_edges(data, op, s);
    }
    s.push_str("*END_COLUMNS\n");
}

/// Serialize the data-flow graph of a function in Renoir format
/// (C++ `dump_dataflow_graph`).
///
/// SEAM(W4): the varnode register markup comes from `printer`.
pub fn dump_dataflow_graph(data: &mut Funcdata, s: &mut String, printer: &dyn GraphVarnodePrinter) {
    s.push_str("*CMD=NewGraphWindow, WindowName=");
    s.push_str(data.get_name());
    s.push_str("-dataflow;\n");
    s.push_str("*CMD=*NEXUS,Name=");
    s.push_str(data.get_name());
    s.push_str("-dataflow;\n");

    s.push_str("\n// AutomaticArrangement\n");
    s.push_str(AUTOMATIC_ARRANGEMENT);

    s.push_str("\n// VertexColors\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexColors,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Mapping=({DisplayChoice=Magenta,AttributeValue=branch},\n");
    s.push_str("  {DisplayChoice=Blue,AttributeValue=register},\n");
    s.push_str("  {DisplayChoice=Black,AttributeValue=unique},\n");
    s.push_str("  {DisplayChoice=DarkGreen,AttributeValue=const},\n");
    s.push_str("  {DisplayChoice=DarkOrange,AttributeValue=ram},\n");
    s.push_str("  {DisplayChoice=Orange,AttributeValue=stack}),\n");
    s.push_str("  ChoiceForValueNotCovered=Red,\n");
    s.push_str("  Extraction=CompleteValue,\n");
    s.push_str("  ExtractionParams={},\n");
    s.push_str("  AttributeName=SubClass,\n");
    s.push_str("  ChoiceForMissingValue=Red,\n");
    s.push_str("  CanOverride=true,\n");
    s.push_str("  OverrideAttributeName=Color,\n");
    s.push_str("  UsingRange=false;\n");

    s.push_str("\n//     VertexIcons\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexIcons,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Mapping=({DisplayChoice=Circle,AttributeValue=var},\n");
    s.push_str("  {DisplayChoice=Square,AttributeValue=op}),\n");
    s.push_str("  ChoiceForValueNotCovered=Circle,\n");
    s.push_str("  Extraction=CompleteValue,\n");
    s.push_str("  ExtractionParams={},\n");
    s.push_str("  AttributeName=Type,\n");
    s.push_str("  ChoiceForMissingValue=Circle,\n");
    s.push_str("  CanOverride=true,\n");
    s.push_str("  OverrideAttributeName=Icon,\n");
    s.push_str("  UsingRange=false;\n");

    s.push_str("\n//     VertexLabels\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexLabels,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Center=({SpecialColor=Black,SpecialFontName=SansSerif,Format=StandardFormat,UseSpecialFontName=false,LabelAlignment=Center,TreatBackSlashNAsNewLine=false,MaxLines=4,FontSize=10,IncludeBackground=false,SqueezeLinesTogether=true,BackgroundColor=Black,UseSpecialColor=false,AttributeName=Name,MaxWidth=100}),\n");
    s.push_str("  East=(),\n");
    s.push_str("  SouthEast=(),\n");
    s.push_str("  North=(),\n");
    s.push_str("  West=(),\n");
    s.push_str("  SouthWest=(),\n");
    s.push_str("  NorthEast=(),\n");
    s.push_str("  South=(),\n");
    s.push_str("  NorthWest=();\n");

    s.push_str("\n// Attributes\n");
    for name in ["SubClass", "Type", "Internal", "Name", "Address"] {
        s.push_str("*CMD=DefineAttribute,\n");
        s.push_str("        Name=");
        s.push_str(name);
        s.push_str(",\n");
        s.push_str("        Type=String,\n");
        s.push_str("        Category=Vertices;\n\n");
    }

    s.push_str("*CMD=DefineAttribute,\n");
    s.push_str("        Name=Name,\n");
    s.push_str("        Type=String,\n");
    s.push_str("        Category=Edges;\n\n");

    s.push_str("*CMD=SetKeyAttribute,\n");
    s.push_str("        Category=Vertices,");
    s.push_str("        Name=Internal;\n\n");
    dump_varnode_vertex(data, s, printer);
    dump_op_vertex(data, s);
    dump_edges(data, s);
}

// ---------------------------------------------------------------------------
// Control-flow / dominator graphs  (C++ `graph.cc:298-500`)
// ---------------------------------------------------------------------------

/// Print one basic-block vertex (C++ static `print_block_vertex`).
fn print_block_vertex(graph: &BlockGraph, bl: crate::seams::BlockId, s: &mut String) {
    let blk = graph.block(bl);
    s.push(' ');
    push_dec_i32(s, blk.size_out());
    s.push(' ');
    push_dec_i32(s, blk.size_in());
    s.push(' ');
    push_dec_i32(s, blk.get_index());
    s.push(' ');
    push_hex(s, block_get_start(&graph.arena, bl).get_offset());
    s.push(' ');
    push_hex(s, block_get_stop(&graph.arena, bl).get_offset());
    s.push('\n');
}

/// Print the in-edges of one basic block as `inIndex blIndex` lines
/// (C++ static `print_block_edge`).
fn print_block_edge(graph: &BlockGraph, bl: crate::seams::BlockId, s: &mut String) {
    let blk = graph.block(bl);
    let blindex = blk.get_index();
    for i in 0..blk.size_in() {
        let in_id = blk.get_in(i);
        let in_index = graph.block(in_id).get_index();
        push_dec_i32(s, in_index);
        s.push(' ');
        push_dec_i32(s, blindex);
        s.push('\n');
    }
}

/// Dump the basic-block vertices (C++ static `dump_block_vertex`).
fn dump_block_vertex(graph: &BlockGraph, s: &mut String, falsenode: bool) {
    s.push_str("\n\n// Add Vertices\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddVertices,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=SizeOut, Location=1},\n");
    s.push_str("          {Name=SizeIn, Location=2},\n");
    s.push_str("          {Name=Internal, Location=3},\n");
    s.push_str("          {Name=Index, Location=4},\n");
    s.push_str("          {Name=Start, Location=5},\n");
    s.push_str("          {Name=Stop, Location=6});\n\n");

    if falsenode {
        s.push_str("-1 0 0 -1 0 0\n");
    }
    for i in 0..graph_size(graph) {
        print_block_vertex(graph, graph_block(graph, i), s);
    }
    s.push_str("*END_COLUMNS\n");
}

/// Dump the basic-block edges (C++ static `dump_block_edges`).
fn dump_block_edges(graph: &BlockGraph, s: &mut String) {
    s.push_str("\n\n// Add Edges\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddEdges,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=*FromKey, Location=1},\n");
    s.push_str("          {Name=*ToKey, Location=2});\n\n");

    for i in 0..graph_size(graph) {
        print_block_edge(graph, graph_block(graph, i), s);
    }
    s.push_str("*END_COLUMNS\n");
}

/// Print one dominator-tree edge (C++ static `print_dom_edge`).
fn print_dom_edge(graph: &BlockGraph, bl: crate::seams::BlockId, s: &mut String, falsenode: bool) {
    let blk = graph.block(bl);
    let blindex = blk.get_index();
    match blk.get_immed_dom() {
        Some(dom) => {
            let domindex = graph.block(dom).get_index();
            push_dec_i32(s, domindex);
            s.push(' ');
            push_dec_i32(s, blindex);
            s.push('\n');
        }
        None => {
            if falsenode {
                s.push_str("-1 ");
                push_dec_i32(s, blindex);
                s.push('\n');
            }
        }
    }
}

/// Dump the dominator-tree edges (C++ static `dump_dom_edges`).
fn dump_dom_edges(graph: &BlockGraph, s: &mut String, falsenode: bool) {
    s.push_str("\n\n// Add Edges\n");
    s.push_str("*CMD=*COLUMNAR_INPUT,\n");
    s.push_str("  Command=AddEdges,\n");
    s.push_str("  Parsing=WhiteSpace,\n");
    s.push_str("  Fields=({Name=*FromKey, Location=1},\n");
    s.push_str("          {Name=*ToKey, Location=2});\n\n");

    for i in 0..graph_size(graph) {
        print_dom_edge(graph, graph_block(graph, i), s, falsenode);
    }
    s.push_str("*END_COLUMNS\n");
}

/// Dump the block-graph attribute definitions (C++ static `dump_block_attributes`).
fn dump_block_attributes(s: &mut String) {
    s.push_str("\n// Attributes\n");
    for name in ["SizeOut", "SizeIn", "Internal", "Index", "Start", "Stop"] {
        s.push_str("*CMD=DefineAttribute,\n");
        s.push_str("        Name=");
        s.push_str(name);
        s.push_str(",\n");
        s.push_str("        Type=String,\n");
        s.push_str("        Category=Vertices;\n\n");
    }

    s.push_str("*CMD=SetKeyAttribute,\n");
    s.push_str("        Category=Vertices,");
    s.push_str("        Name=Index;\n\n");
}

/// Dump the block-graph arrangement/color/icon/label preferences
/// (C++ static `dump_block_properties`).
fn dump_block_properties(s: &mut String) {
    s.push_str("\n// AutomaticArrangement\n");
    s.push_str(AUTOMATIC_ARRANGEMENT);

    s.push_str("\n// VertexColors\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexColors,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Mapping=({DisplayChoice=Red,AttributeValue=0},\n");
    s.push_str("  {DisplayChoice=Blue,AttributeValue=1},\n");
    s.push_str("  {DisplayChoice=Yellow,AttributeValue=2}),\n");
    s.push_str("  ChoiceForValueNotCovered=Purple,\n");
    s.push_str("  Extraction=CompleteValue,\n");
    s.push_str("  ExtractionParams={},\n");
    s.push_str("  AttributeName=SizeOut,\n");
    s.push_str("  ChoiceForMissingValue=Purple,\n");
    s.push_str("  CanOverride=true,\n");
    s.push_str("  OverrideAttributeName=Color,\n");
    s.push_str("  UsingRange=false;\n");

    s.push_str("\n//     VertexIcons\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexIcons,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Mapping=({DisplayChoice=Square,AttributeValue=0}),\n");
    s.push_str("  ChoiceForValueNotCovered=Circle,\n");
    s.push_str("  Extraction=CompleteValue,\n");
    s.push_str("  ExtractionParams={},\n");
    s.push_str("  AttributeName=SizeIn,\n");
    s.push_str("  ChoiceForMissingValue=Circle,\n");
    s.push_str("  CanOverride=true,\n");
    s.push_str("  OverrideAttributeName=Icon,\n");
    s.push_str("  UsingRange=false;\n");

    s.push_str("\n//     VertexLabels\n");
    s.push_str("  *CMD = AlterLocalPreferences, Name = VertexLabels,\n");
    s.push_str("  ~ReplaceAllParams = TRUE,\n");
    s.push_str("  Center=({MaxLines=4,SqueezeLinesTogether=true,TreatBackSlashNAsNewLine=false,FontSize=10,Format=StandardFormat,IncludeBackground=false,BackgroundColor=Black,AttributeName=Start,UseSpecialFontName=false,SpecialColor=Black,SpecialFontName=SansSerif,UseSpecialColor=false,LabelAlignment=Center,MaxWidth=100}),\n");
    s.push_str("  East=(),\n");
    s.push_str("  SouthEast=(),\n");
    s.push_str("  North=(),\n");
    s.push_str("  West=(),\n");
    s.push_str("  SouthWest=(),\n");
    s.push_str("  NorthEast=(),\n");
    s.push_str("  South=(),\n");
    s.push_str("  NorthWest=();\n");
}

/// Serialize the control-flow graph in Renoir format
/// (C++ `dump_controlflow_graph`).
pub fn dump_controlflow_graph(name: &str, graph: &BlockGraph, s: &mut String) {
    s.push_str("*CMD=NewGraphWindow, WindowName=");
    s.push_str(name);
    s.push_str("-controlflow;\n");
    s.push_str("*CMD=*NEXUS,Name=");
    s.push_str(name);
    s.push_str("-controlflow;\n");
    dump_block_properties(s);
    dump_block_attributes(s);
    dump_block_vertex(graph, s, false);
    dump_block_edges(graph, s);
}

/// Serialize the dominator tree in Renoir format (C++ `dump_dom_graph`).
pub fn dump_dom_graph(name: &str, graph: &BlockGraph, s: &mut String) {
    // count = number of blocks with no immediate dominator
    let mut count: int4 = 0;
    for i in 0..graph_size(graph) {
        if graph.block(graph_block(graph, i)).get_immed_dom().is_none() {
            count += 1;
        }
    }
    let falsenode = count > 1;
    s.push_str("*CMD=NewGraphWindow, WindowName=");
    s.push_str(name);
    s.push_str("-dom;\n");
    s.push_str("*CMD=*NEXUS,Name=");
    s.push_str(name);
    s.push_str("-dom;\n");
    dump_block_properties(s);
    dump_block_attributes(s);
    dump_block_vertex(graph, s, falsenode);
    dump_dom_edges(graph, s, falsenode);
}

/// The shared `AutomaticArrangement` preference block, verbatim from the C++
/// (identical in `dump_dataflow_graph` and `dump_block_properties`).
const AUTOMATIC_ARRANGEMENT: &str = concat!(
    "  *CMD = AlterLocalPreferences, Name = AutomaticArrangement,\n",
    "  ~ReplaceAllParams = TRUE,\n",
    "  EnableAutomaticArrangement=true,\n",
    "  OnlyActOnVerticesWithoutCoordsIfOff=false,\n",
    "  DontUpdateMediumWithUserArrangement=false,\n",
    "  UserAddedArrangmentParams=({ServiceName=SimpleHierarchyFromSources,ServiceParams={~SkipPromptForParams=true}}),\n",
    "  SmallSize=50,\n",
    "  DontUpdateLargeWithUserArrangement=true,\n",
    "  NewVertexActionIfOff=ArrangeByMDS,\n",
    "  MediumSizeArrangement=SimpleHierarchyFromSources,\n",
    "  SmallSizeArrangement=SimpleHierarchyFromSources,\n",
    "  MediumSize=800,\n",
    "  LargeSizeArrangement=ArrangeInCircle,\n",
    "  DontUpdateSmallWithUserArrangement=false,\n",
    "  ActionSizeGainIfOff=1.0;\n",
);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_range_special_cases() {
        // Normal op: full input range.
        assert_eq!(input_range(OpCode::CPUI_INT_ADD, 2), (0, 2));
        // LOAD/STORE/BRANCH/CALL skip input 0.
        assert_eq!(input_range(OpCode::CPUI_LOAD, 2), (1, 2));
        assert_eq!(input_range(OpCode::CPUI_STORE, 3), (1, 3));
        assert_eq!(input_range(OpCode::CPUI_BRANCH, 1), (1, 1));
        assert_eq!(input_range(OpCode::CPUI_CALL, 1), (1, 1));
        // INDIRECT keeps only input 0.
        assert_eq!(input_range(OpCode::CPUI_INDIRECT, 2), (0, 1));
    }

    #[test]
    fn controlflow_graph_preamble_shape() {
        // An empty block graph still emits the full Renoir preamble; this pins
        // the verbatim header text and the empty-graph edge/vertex framing.
        let graph = BlockGraph::new();
        let mut s = String::new();
        dump_controlflow_graph("myfunc", &graph, &mut s);
        assert!(s.starts_with("*CMD=NewGraphWindow, WindowName=myfunc-controlflow;\n"));
        assert!(s.contains("*CMD=*NEXUS,Name=myfunc-controlflow;\n"));
        assert!(s.contains("EnableAutomaticArrangement=true,\n"));
        assert!(s.contains("Command=AddVertices,\n"));
        assert!(s.contains("Command=AddEdges,\n"));
        // No false node for an empty graph.
        assert!(!s.contains("-1 0 0 -1 0 0\n"));
        // Two *END_COLUMNS (vertex pass + edge pass).
        assert_eq!(s.matches("*END_COLUMNS\n").count(), 2);
    }

    #[test]
    fn dom_graph_no_falsenode_when_empty() {
        // count of dom-less blocks is 0 for an empty graph, so falsenode stays
        // false and the dom preamble omits the synthetic -1 root vertex.
        let graph = BlockGraph::new();
        let mut s = String::new();
        dump_dom_graph("f", &graph, &mut s);
        assert!(s.starts_with("*CMD=NewGraphWindow, WindowName=f-dom;\n"));
        assert!(!s.contains("-1 0 0 -1 0 0\n"));
    }

    #[test]
    fn default_varnode_markup_uses_shortcut() {
        use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
        use std::rc::Rc;
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        let a = Address::new(ram, 0x1234);
        let markup = default_varnode_markup(&a);
        // Shortcut char of a processor space, then the raw offset rendering.
        assert!(!markup.is_empty());
        assert!(markup.contains("1234"));
    }
}
