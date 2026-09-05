//! End-to-end ghidra-mode protocol tests: the command loop and the query
//! client driven over in-memory buffers by a MockJava that mirrors
//! `DecompileProcess.java`'s writer (`writeString` =
//! `{0,0,1,14}+bytes+{0,0,1,15}`, command framing
//! `{0,0,1,2}...{0,0,1,3}`, query responses `{0,0,1,8}...{0,0,1,9}`).
//!
//! The full-session test asserts the response byte stream EXACTLY against
//! hand-constructed bursts (the shapes of ghidra_process.cc:108-160,
//! 203-210, 253-260, 275-282, 313-334, 408-416, 447-455, 476-484).

use std::io::Cursor;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaError;
use kuna_base::marshal::{
    ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_CONTENT, ATTRIB_INDEX, ATTRIB_NAME,
};
use kuna_base::space::VarnodeStorage;

use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

use kuna_decomp::engine_translate::EngineTranslate;
use kuna_decomp::pcodeinject::{InjectContext, CALLOTHERFIXUP_TYPE};
use kuna_ghidra::client::GhidraClient;
use kuna_ghidra::ids::{ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETUSEROPNAME};
use kuna_ghidra::process::GhidraProcess;
use kuna_ghidra::protocol::{nibble_expand, string_data_size_header, WireError};
use kuna_ghidra::translate::{build_registry, GhidraTranslate, TspecModel};

// ---------------------------------------------------------------------------
// MockJava wire builder
// ---------------------------------------------------------------------------

/// Byte-stream builder mirroring DecompileProcess.java's writer.
#[derive(Clone, Default)]
struct Wire(Vec<u8>);

impl Wire {
    fn new() -> Wire {
        Wire(Vec::new())
    }
    /// A burst marker `{0,0,1,code}` (DecompileProcess.java:54-63).
    fn burst(mut self, code: u8) -> Wire {
        self.0.extend_from_slice(&[0, 0, 1, code]);
        self
    }
    fn raw(mut self, bytes: &[u8]) -> Wire {
        self.0.extend_from_slice(bytes);
        self
    }
    /// `writeString` (DecompileProcess.java:280-284).
    fn string(self, s: &[u8]) -> Wire {
        self.burst(14).raw(s).burst(15)
    }
    /// Command open + name (DecompileProcess.java sendCommand family).
    fn command(self, name: &str) -> Wire {
        self.burst(2).string(name.as_bytes())
    }
    /// Command close.
    fn end_command(self) -> Wire {
        self.burst(3)
    }
}

// ---------------------------------------------------------------------------
// The four registerProgram documents (tspec shaped exactly like
// SleighLanguage.encodeTranslator output — single line, XmlEncode(false);
// OTHER at unique index 1 as insertSpace requires)
// ---------------------------------------------------------------------------

const TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x10000000\">\
<spaces defaultspace=\"ram\">\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"8\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
<space name=\"register\" index=\"4\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
</spaces></sleigh>";
const PSPEC: &[u8] = b"<processor_spec><programcounter register=\"PC\"/></processor_spec>";
const CSPEC: &[u8] = b"<compiler_spec><default_proto/></compiler_spec>";
// Phase 3 decodes the wire corespec for real (build_core_types): mirror the
// standalone default core-type set so engine init still finds char/bool/void.
const CORETYPES: &[u8] = b"<coretypes>\
<type name=\"void\" size=\"1\" metatype=\"void\"/>\
<type name=\"bool\" size=\"1\" metatype=\"bool\"/>\
<type name=\"uint1\" size=\"1\" metatype=\"uint\"/>\
<type name=\"uint2\" size=\"2\" metatype=\"uint\"/>\
<type name=\"uint4\" size=\"4\" metatype=\"uint\"/>\
<type name=\"uint8\" size=\"8\" metatype=\"uint\"/>\
<type name=\"int1\" size=\"1\" metatype=\"int\"/>\
<type name=\"int2\" size=\"2\" metatype=\"int\"/>\
<type name=\"int4\" size=\"4\" metatype=\"int\"/>\
<type name=\"int8\" size=\"8\" metatype=\"int\"/>\
<type name=\"float4\" size=\"4\" metatype=\"float\"/>\
<type name=\"float8\" size=\"8\" metatype=\"float\"/>\
<type name=\"float10\" size=\"10\" metatype=\"float\"/>\
<type name=\"float16\" size=\"16\" metatype=\"float\"/>\
<type name=\"xunknown1\" size=\"1\" metatype=\"unknown\"/>\
<type name=\"xunknown2\" size=\"2\" metatype=\"unknown\"/>\
<type name=\"xunknown4\" size=\"4\" metatype=\"unknown\"/>\
<type name=\"xunknown8\" size=\"8\" metatype=\"unknown\"/>\
<type name=\"code\" size=\"1\" metatype=\"code\"/>\
<type name=\"char\" size=\"1\" metatype=\"int\" char=\"true\"/>\
<type name=\"wchar2\" size=\"2\" metatype=\"int\" utf=\"true\"/>\
<type name=\"wchar4\" size=\"4\" metatype=\"int\" utf=\"true\"/>\
</coretypes>";

/// The tspec-derived translate the mock shares with the process under test.
fn test_translate() -> TspecModel {
    let registry = build_registry();
    TspecModel::decode(TSPEC, &registry).expect("test tspec parses")
}

/// A packed `<optionslist>` with two single-param option children
/// (DecompileOptions.encode / appendOption shape).  Phase 3 DECODES this list
/// for real, so the ids must be the true upstream option element ids — an
/// unknown id is skipped with a "Warning:" line on the 16/17 frame, which
/// would break this test's byte-exact silent-success expectation.
fn packed_optionslist() -> Vec<u8> {
    let optionslist = ElementId::new("optionslist", 201);
    let readonly = kuna_decomp::options::ELEM_READONLY;
    let maxinstruction = kuna_decomp::options::ELEM_MAXINSTRUCTION;
    let mut packed = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut packed);
        enc.open_element(&optionslist);
        enc.open_element(&readonly);
        enc.write_string(&ATTRIB_CONTENT, b"on");
        enc.close_element(&readonly);
        enc.open_element(&maxinstruction);
        enc.write_string(&ATTRIB_CONTENT, b"100000");
        enc.close_element(&maxinstruction);
        enc.close_element(&optionslist);
    }
    packed
}

/// The packed `<command_getuseropname index=N>` document the client emits for
/// the init-time user-op probe (client.rs `get_user_op_name`).  Phase-2
/// registerProgram issues exactly one of these (index 0) during
/// `init_post_engine` → `userops.initialize`, and terminates the probe on the
/// host's empty answer.
fn get_user_op_name_doc(index: i32) -> Vec<u8> {
    let mut doc = Vec::new();
    {
        let mut e = PackedEncode::new(&mut doc);
        e.open_element(&ELEM_COMMAND_GETUSEROPNAME);
        e.write_signed_integer(&ATTRIB_INDEX, index as i64);
        e.close_element(&ELEM_COMMAND_GETUSEROPNAME);
    }
    doc
}

// ---------------------------------------------------------------------------
// (a) full session over the command loop
// ---------------------------------------------------------------------------

// NOTE: decompileAt is deliberately absent from this byte-exact test.  Since
// phase-2 step 6 wired `decompileAt` to the live engine, its response bytes are
// a real `<doc>` whose contents (the `Funcdata::encode` `<ast>` + the Clang
// markup) depend on the whole decompile pipeline's internal Varnode/op
// numbering — not hand-assertable byte-exact.  `decompileAt` is proven
// end-to-end in `test_decompile_at_emits_c` (the interactive MockJava e2e);
// this test keeps the byte-exact framing coverage of every OTHER command.
#[test]
fn test_full_session_byte_exact() {
    let options_packed = packed_optionslist();

    let input = Wire::new()
        // registerProgram: 4 XML string streams, no archid
        .command("registerProgram")
        .string(PSPEC)
        .string(CSPEC)
        .string(TSPEC)
        .string(CORETYPES)
        .end_command()
        // ...then the getUserOpName probe answer the engine's init reads back:
        // an empty name terminates the probe (client.rs `get_user_op_name`).
        // This query is nested inside the still-open registerProgram response.
        .burst(8)
        .string(b"")
        .burst(9)
        // setOptions: archid + packed <optionslist>
        .command("setOptions")
        .string(b"0")
        .string(&options_packed)
        .end_command()
        // setAction: archid + actionstring + printstring
        .command("setAction")
        .string(b"0")
        .string(b"decompile")
        .string(b"c")
        .end_command()
        // flushNative: archid only
        .command("flushNative")
        .string(b"0")
        .end_command()
        // an unregistered (signature) command: archid param is sent by Java
        // but never read by the Bad-command path — the next command's
        // burst-2 scan skips it (self-aligning)
        .command("getSignatureSettings")
        .string(b"0")
        .end_command()
        // deregisterProgram: archid; terminates the loop
        .command("deregisterProgram")
        .string(b"0")
        .end_command();

    let mut process = GhidraProcess::new(Cursor::new(input.0), Vec::new());
    let mut statuses = Vec::new();
    for _ in 0..6 {
        statuses.push(process.read_command().expect("command completes"));
    }
    assert_eq!(statuses, vec![0, 0, 0, 0, 0, 1]);
    let (_, out) = process.into_inner();

    let expected = Wire::new()
        // registerProgram: {6} then the nested getUserOpName probe query
        // {4}{14}<doc>{15}{5}, then the archid {14}"0"{15}, then the (empty)
        // warnings frame {16}{17}, then {7}.  The empty warnings prove the
        // live Architecture was built with no construction error.
        .burst(6)
        .burst(4)
        .string(&get_user_op_name_doc(0))
        .burst(5)
        .string(b"0")
        .burst(16)
        .burst(17)
        .burst(7)
        // setOptions: "t", empty warnings — accepted SILENTLY.  A non-empty
        // 16/17 message here fails DecompInterface init (openProgram stores it
        // and isErrorMessage flags any non-"warning" text as fatal), so the GUI
        // reports "Unable to initialize the DecompilerInterface" (see process.rs).
        .burst(6)
        .string(b"t")
        .burst(16)
        .burst(17)
        .burst(7)
        // setAction: "t", empty warnings
        .burst(6)
        .string(b"t")
        .burst(16)
        .burst(17)
        .burst(7)
        // flushNative: "0"
        .burst(6)
        .string(b"0")
        .burst(16)
        .burst(17)
        .burst(7)
        // Bad command: {6}{16}"Bad command: <name>"{17}{7}, no payload burst
        .burst(6)
        .burst(16)
        .raw(b"Bad command: getSignatureSettings")
        .burst(17)
        .burst(7)
        // deregisterProgram: "1", NO 16/17 frame (C++ nulls ghidra first)
        .burst(6)
        .string(b"1")
        .burst(7);

    assert_eq!(
        out, expected.0,
        "response stream mismatch\n got: {:02x?}\nwant: {:02x?}",
        out, expected.0
    );
}

/// A command against a deregistered (or never-registered) archid answers
/// with the exception frame and NO command-response close (the C++
/// passJavaException path, ghidra_process.cc:142-145).
#[test]
fn test_unregistered_archid_passes_java_exception() {
    let input = Wire::new()
        .command("flushNative")
        .string(b"0")
        .end_command();
    let mut process = GhidraProcess::new(Cursor::new(input.0), Vec::new());
    assert_eq!(process.read_command().unwrap(), 0);
    let (_, out) = process.into_inner();
    let expected = Wire::new()
        .burst(6)
        .burst(10)
        .string(b"decompiler")
        .string(b"No architecture registered with decompiler")
        .burst(11);
    assert_eq!(out, expected.0);
}

/// registerProgram with an unparseable tspec still succeeds at the protocol
/// level (returns an archid, does not JavaError), but engine construction fails
/// (the tspec drives `GhidraTranslate::new` / the `<sleigh>` decode, C++
/// `buildTranslator`).  The failure is recorded as a warning on the 16/17
/// frame — Java treats the non-empty nativeMessage as a registration failure —
/// and decompileAt then reports the undecoded address (no engine manager).
#[test]
fn test_bad_tspec_is_lenient() {
    let input = Wire::new()
        .command("registerProgram")
        .string(PSPEC)
        .string(CSPEC)
        .string(b"<not_a_sleigh_tag/>")
        .string(CORETYPES)
        .end_command()
        .command("decompileAt")
        .string(b"0")
        .string(b"\x50") // some nonzero payload standing in for the <addr>
        .end_command();
    let mut process = GhidraProcess::new(Cursor::new(input.0), Vec::new());
    assert_eq!(process.read_command().unwrap(), 0);
    assert_eq!(process.read_command().unwrap(), 0);
    let (_, out) = process.into_inner();
    let text = String::from_utf8_lossy(&out);
    assert!(
        text.contains("could not build the decompiler engine from the registerProgram specs"),
        "missing engine-construction-failure warning in: {text}"
    );
    assert!(
        text.contains("function at (address not decoded) not decompiled"),
        "missing decompileAt warning in: {text}"
    );
}

/// Phase-2: registerProgram over four tiny valid specs (plus the init-time
/// query answer) builds a live [`Architecture`].
///
/// Proof the engine was built: registerProgram issues exactly one nested query
/// (the getUserOpName probe, from `init_post_engine` → `userops.initialize`),
/// its warnings frame is empty (no construction error), and it returns archid
/// "0".  (The live manager's `<addr>` decode + the whole decompile drive are
/// proven end-to-end in `test_decompile_at_emits_c`.)
#[test]
fn test_register_program_builds_live_architecture() {
    let input = Wire::new()
        .command("registerProgram")
        .string(PSPEC)
        .string(CSPEC)
        .string(TSPEC)
        .string(CORETYPES)
        .end_command()
        // getUserOpName probe answer: empty name terminates the probe.
        .burst(8)
        .string(b"")
        .burst(9);

    let mut process = GhidraProcess::new(Cursor::new(input.0), Vec::new());
    assert_eq!(process.read_command().expect("registerProgram completes"), 0);
    let (_, out) = process.into_inner();

    // registerProgram issued exactly one init query (the getUserOpName probe) —
    // it ran `init_post_engine`, so the Architecture is present.
    let mut query_opens = 0usize;
    let mut i = 0usize;
    while i + 4 <= out.len() {
        if out[i..i + 4] == [0, 0, 1, 4] {
            query_opens += 1;
        }
        i += 1;
    }
    assert_eq!(
        query_opens, 1,
        "registerProgram should issue exactly one init query (getUserOpName probe)"
    );

    let text = String::from_utf8_lossy(&out);
    assert!(
        !text.contains("could not build the decompiler engine"),
        "engine construction unexpectedly failed: {text}"
    );

    // The response is archid "0" with an EMPTY 16/17 warnings frame (no
    // construction error): {6} {4}<query>{5} {14}"0"{15} {16}{17} {7}.
    let expected_tail = Wire::new().string(b"0").burst(16).burst(17).burst(7);
    assert!(
        out.ends_with(&expected_tail.0),
        "registerProgram must end with archid \"0\" + empty warnings frame; got {out:02x?}"
    );
}

// ---------------------------------------------------------------------------
// (c) the query client against MockJava responses
// ---------------------------------------------------------------------------

#[test]
fn test_get_register_round_trip() {
    let tr = test_translate();
    let reg_space = Rc::clone(tr.manager.get_space_by_name("register").unwrap());

    // MockJava answer: {8} writeString(<packed addr size=8>) {9}
    let mut packed = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut packed);
        Address::new(Rc::clone(&reg_space), 0)
            .encode_sized(&mut enc, 8)
            .unwrap();
    }
    let response = Wire::new().burst(8).string(&packed).burst(9);

    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    let mut decoder = PackedDecode::new(&tr.manager);
    assert!(client.get_register("RAX", &mut decoder).unwrap());
    let (addr, size) = Address::decode_sized(&mut decoder).unwrap();
    assert_eq!(addr.get_space().unwrap().get_name(), "register");
    assert_eq!(addr.get_offset(), 0);
    assert_eq!(size, 8);

    // and the query the client sent must be byte-exact:
    // {4}{14}<packed command_getregister name="RAX">{15}{5}
    let (_, sent) = client.into_inner();
    let mut expected_doc = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut expected_doc);
        enc.open_element(&ELEM_COMMAND_GETREGISTER);
        enc.write_string(&ATTRIB_NAME, b"RAX");
        enc.close_element(&ELEM_COMMAND_GETREGISTER);
    }
    let expected = Wire::new().burst(4).string(&expected_doc).burst(5);
    assert_eq!(sent, expected.0);
}

#[test]
fn test_query_java_exception_surfaces_as_kuna_java() {
    let tr = test_translate();
    let response = Wire::new()
        .burst(10)
        .string(b"java.lang.IllegalArgumentException")
        .string(b"no such register")
        .burst(11);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    let mut decoder = PackedDecode::new(&tr.manager);
    match client.get_register("XYZZY", &mut decoder) {
        Err(WireError::Kuna(KunaError::Java { type_name, explain })) => {
            assert_eq!(type_name, "java.lang.IllegalArgumentException");
            assert_eq!(explain, "no such register");
        }
        other => panic!("expected KunaError::Java, got {other:?}"),
    }
}

#[test]
fn test_get_bytes_nibble_stream_and_data_unavail() {
    let tr = test_translate();
    let ram = Rc::clone(tr.manager.get_space_by_name("ram").unwrap());

    // nibble-doubled payload
    let response = Wire::new()
        .burst(8)
        .burst(12)
        .raw(&nibble_expand(&[0xde, 0xad, 0xbe, 0xef]))
        .burst(13)
        .burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    let mut buf = [0u8; 4];
    client
        .get_bytes(&mut buf, &Address::new(Rc::clone(&ram), 0x401000))
        .unwrap();
    assert_eq!(buf, [0xde, 0xad, 0xbe, 0xef]);

    // empty response => DataUnavailError
    let response = Wire::new().burst(8).burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    let mut buf = [0u8; 4];
    match client.get_bytes(&mut buf, &Address::new(ram, 0x401000)) {
        Err(WireError::Kuna(KunaError::DataUnavail { explain })) => {
            assert!(
                explain.starts_with("GHIDRA has no data in the loadimage at "),
                "{explain}"
            );
        }
        other => panic!("expected DataUnavailError, got {other:?}"),
    }
}

#[test]
fn test_is_name_used_bool_stream() {
    let response = Wire::new().burst(8).string(b"t").burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    assert!(client.is_name_used("main", 0, 1).unwrap());
    let response = Wire::new().burst(8).string(b"f").burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    assert!(!client.is_name_used("main", 0, 1).unwrap());
}

#[test]
fn test_get_string_data_length_header() {
    let tr = test_translate();
    let ram = Rc::clone(tr.manager.get_space_by_name("ram").unwrap());
    // Java sends sz = len+1 pairs, the last pair the doubled NUL ("AA")
    let payload = b"hello\0";
    let header = string_data_size_header(payload.len() as u32);
    let response = Wire::new()
        .burst(8)
        .burst(12)
        .raw(&header)
        .raw(&[0]) // truncation flag: false (a literal 0x00 inside the byte burst)
        .raw(&nibble_expand(payload))
        .burst(13)
        .burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    let mut buffer = Vec::new();
    let trunc = client
        .get_string_data(
            &mut buffer,
            &Address::new(ram, 0x402000),
            "char",
            0xcafe,
            2048,
        )
        .unwrap();
    assert!(!trunc);
    assert_eq!(buffer, payload.to_vec());
}

#[test]
fn test_get_user_op_name_empty_terminates_probe() {
    let response = Wire::new().burst(8).string(b"").burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    assert_eq!(client.get_user_op_name(3).unwrap(), b"");
}

#[test]
fn test_get_register_name_query_shape() {
    let tr = test_translate();
    let reg_space = Rc::clone(tr.manager.get_space_by_name("register").unwrap());
    let vn = VarnodeStorage {
        space: Some(reg_space),
        offset: 0x10,
        size: 8,
    };
    let response = Wire::new().burst(8).string(b"RBP").burst(9);
    let mut client = GhidraClient::new(Cursor::new(response.0), Vec::new());
    assert_eq!(client.get_register_name(&vn).unwrap(), b"RBP");
}

// ---------------------------------------------------------------------------
// (d) getPcodeInject against bytes captured from a live Ghidra 12.1.2
//
// Every other ghidra-mode test has kuna answering kuna: the sim oracle compiles
// the cspec snippet with kuna's own SLEIGH, so a bug in that compiler would be
// invisible because both ends agree.  These two bytes-on-the-wire are the one
// input that came from Java — the first `getCallOtherFixup` exchange of
// `fmt_arm/main` under the stock C++ core, tapped at the pipe.
// ---------------------------------------------------------------------------

/// The `<sleigh>` document Ghidra sent for `fmt_arm` (ARM:LE:32:v8) — its space
/// INDICES are what make the packed `<addr>`s below decode: ram=3, register=4.
const ARM_CAPTURED_TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x1d8600\">\
<spaces defaultspace=\"ram\">\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"4\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
<space name=\"register\" index=\"4\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
</spaces></sleigh>";

/// `<command_getcallotherfixup name="setISAMode"><context><addr space=3
/// offset=66844/><addr/><input><addr space=4 offset=105 size=1/></input>
/// </context></command_getcallotherfixup>`
const ARM_CAPTURED_REQUEST: &str =
    "61f3ce718a7365744953414d6f646560de4bd45183d043848a9c8b4b8b424bd45184d041e9d321818b82a0dea1f3";

/// `<inst offset=4><addr space=3 offset=66844/><op code=1 size=1><addr space=4
/// offset=32 size=4/><addr space=4 offset=32 size=4/></op></inst>` — ARM.cspec's
/// `r0 = r0;` fixup body, already lifted for this one site.
const ARM_CAPTURED_RESPONSE: &str =
    "60e2d021844bd45183d043848a9c8b5be0ab2181d321814bd45184d041a0d321848b4bd45184d041a0d321848b9ba0e2";

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

/// A `Write` sink readable after the writer is buried inside a shared client.
#[derive(Clone, Default)]
struct SharedSink(Rc<std::cell::RefCell<Vec<u8>>>);
impl std::io::Write for SharedSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct RecordEmit {
    ops: Vec<(
        Address,
        OpCode,
        Option<VarnodeData>,
        Vec<VarnodeData>,
    )>,
}
impl PcodeEmit for RecordEmit {
    fn dump(
        &mut self,
        addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        self.ops
            .push((addr.clone(), opc, outvar.cloned(), vars.to_vec()));
    }
}

#[test]
fn test_inject_query_matches_captured_ghidra_bytes() {
    let registry = build_registry();
    let sink = SharedSink::default();
    let response = Wire::new()
        .burst(8)
        .string(&unhex(ARM_CAPTURED_RESPONSE))
        .burst(9);
    let client = Rc::new(std::cell::RefCell::new(GhidraClient::new(
        Cursor::new(response.0),
        sink.clone(),
    )));
    let tr = GhidraTranslate::new(ARM_CAPTURED_TSPEC, &registry, Rc::clone(&client))
        .expect("captured tspec parses");

    let ram = Rc::clone(tr.manager().get_space_by_name("ram").unwrap());
    let reg = Rc::clone(tr.manager().get_space_by_name("register").unwrap());
    // The context the engine builds for the CALLOTHER at fmt_arm 0x1051c:
    // slot 0 (the inject-id annotation) skipped, the declared operand kept, no
    // call address, no output.
    let context = InjectContext {
        baseaddr: Some(Address::new(Rc::clone(&ram), 0x1051c)),
        nextaddr: Some(Address::new(Rc::clone(&ram), 0x1051c)),
        calladdr: None,
        inputlist: vec![VarnodeData {
            space: Some(Rc::clone(&reg)),
            offset: 105,
            size: 1,
        }],
        output: Vec::new(),
    };

    let mut emit = RecordEmit::default();
    tr.fetch_inject_pcode(b"setISAMode", CALLOTHERFIXUP_TYPE, &context, &mut emit)
        .expect("the captured response decodes");

    // (i) the query kuna sends is the one Ghidra actually received.
    let expected = Wire::new()
        .burst(4)
        .string(&unhex(ARM_CAPTURED_REQUEST))
        .burst(5);
    assert_eq!(
        sink.0.borrow().as_slice(),
        expected.0.as_slice(),
        "the getCallOtherFixup request no longer matches the live-Ghidra capture"
    );

    // (ii) the answer decodes into the fixup body, stamped at the call site.
    assert_eq!(emit.ops.len(), 1, "expected one op, got {:?}", emit.ops);
    let (addr, opc, out, ins) = &emit.ops[0];
    assert_eq!(addr.get_offset(), 0x1051c);
    assert_eq!(addr.get_space().unwrap().get_name(), "ram");
    assert_eq!(*opc, OpCode::CPUI_COPY);
    for vn in std::iter::once(out.as_ref().expect("COPY has an output")).chain(ins) {
        assert_eq!(vn.space.as_ref().unwrap().get_name(), "register");
        assert_eq!(vn.offset, 0x20);
        assert_eq!(vn.size, 4);
    }
    assert_eq!(ins.len(), 1);
}
