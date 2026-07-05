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
    ElementId, Encoder, PackedDecode, PackedEncode, ATTRIB_CONTENT, ATTRIB_NAME,
};
use kuna_base::space::VarnodeStorage;

use kuna_ghidra::client::GhidraClient;
use kuna_ghidra::ids::ELEM_COMMAND_GETREGISTER;
use kuna_ghidra::process::GhidraProcess;
use kuna_ghidra::protocol::{nibble_expand, string_data_size_header, WireError};
use kuna_ghidra::translate::{build_registry, GhidraTranslate};

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
const CORETYPES: &[u8] = b"<coretypes><type name=\"int\" size=\"4\" metatype=\"int\" id=\"-1\"/>\
</coretypes>";

/// The tspec-derived translate the mock shares with the process under test.
fn test_translate() -> GhidraTranslate {
    let registry = build_registry();
    GhidraTranslate::decode(TSPEC, &registry).expect("test tspec parses")
}

/// Encode an `<addr>` for ram:offset the way Java's AddressXML.encode does
/// (space + offset attributes in a packed document).
fn packed_ram_addr(tr: &GhidraTranslate, offset: u64) -> Vec<u8> {
    let ram = Rc::clone(tr.manager.get_space_by_name("ram").expect("ram"));
    let mut packed = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut packed);
        Address::new(ram, offset).encode(&mut enc).unwrap();
    }
    packed
}

/// A packed `<optionslist>` with two single-param option children
/// (DecompileOptions.encode / appendOption shape; the ids only need to be
/// self-consistent for the count).
fn packed_optionslist() -> Vec<u8> {
    let optionslist = ElementId::new("optionslist", 201);
    let readonly = ElementId::new("readonly", 217);
    let maxinstruction = ElementId::new("maxinstruction", 217 + 1);
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

// ---------------------------------------------------------------------------
// (a) full session over the command loop
// ---------------------------------------------------------------------------

#[test]
fn test_full_session_byte_exact() {
    let tr = test_translate();
    let addr_packed = packed_ram_addr(&tr, 0x401000);
    let options_packed = packed_optionslist();

    let input = Wire::new()
        // registerProgram: 4 XML string streams, no archid
        .command("registerProgram")
        .string(PSPEC)
        .string(CSPEC)
        .string(TSPEC)
        .string(CORETYPES)
        .end_command()
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
        // decompileAt: archid + packed <addr>
        .command("decompileAt")
        .string(b"0")
        .string(&addr_packed)
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
    for _ in 0..7 {
        statuses.push(process.read_command().expect("command completes"));
    }
    assert_eq!(statuses, vec![0, 0, 0, 0, 0, 0, 1]);
    let (_, out) = process.into_inner();

    // The rendered form of ram:0x401000 in the decompileAt warning
    let ram = tr.manager.get_space_by_name("ram").unwrap();
    let mut addr_text = String::new();
    addr_text.push(ram.get_shortcut());
    Address::new(Rc::clone(ram), 0x401000)
        .print_raw(&mut addr_text)
        .unwrap();

    let expected = Wire::new()
        // registerProgram: {6}{14}"0"{15}{16}{17}{7}
        .burst(6)
        .string(b"0")
        .burst(16)
        .burst(17)
        .burst(7)
        // setOptions: "t" + phase-1 warning
        .burst(6)
        .string(b"t")
        .burst(16)
        .raw(
            b"\nkuna ghidra-mode: setOptions: 2 option element(s) recorded \
              but not applied (phase 1)"
                .as_slice(),
        )
        .burst(17)
        .burst(7)
        // setAction: "t", empty warnings
        .burst(6)
        .string(b"t")
        .burst(16)
        .burst(17)
        .burst(7)
        // decompileAt: EMPTY payload (incomplete-function shape) + warning
        .burst(6)
        .burst(14)
        .burst(15)
        .burst(16)
        .raw(
            format!(
                "\nkuna ghidra-mode: engine bridge not yet implemented (phase 1); \
                 function at {addr_text} not decompiled"
            )
            .as_bytes(),
        )
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

/// The warning-string literals above must not drift: `\` line continuations
/// inside the source would silently change the bytes.  (This guards the
/// test itself, comparing against the same strings built without
/// continuation.)
#[test]
fn test_expected_warning_literals() {
    assert_eq!(
        b"\nkuna ghidra-mode: setOptions: 2 option element(s) recorded \
          but not applied (phase 1)"
            .as_slice()
            .len(),
        "\nkuna ghidra-mode: setOptions: 2 option element(s) recorded but not applied (phase 1)"
            .len()
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

/// registerProgram with an unparseable tspec still succeeds (lenient phase-1
/// parse), records a warning, and decompileAt then reports the undecoded
/// address.
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
        text.contains("could not parse tspec <sleigh> element"),
        "missing tspec warning in: {text}"
    );
    assert!(
        text.contains("function at (address not decoded) not decompiled"),
        "missing decompileAt warning in: {text}"
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
