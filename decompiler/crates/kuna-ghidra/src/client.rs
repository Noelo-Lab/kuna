//! The decompiler -> Ghidra query client — a port of the query methods on
//! `ArchitectureGhidra` (`decompiler/cpp/ghidra_arch.cc:445-896`).
//!
//! Every query has identical outer framing (e.g. getRegister,
//! ghidra_arch.cc:445-459):
//!
//! ```text
//!   {0,0,1,4}                  query open
//!   {0,0,1,14}                 string stream open
//!   <packed command_* element>
//!   {0,0,1,15}                 string stream close
//!   {0,0,1,5}                  query close
//!   flush
//! ```
//!
//! then the response is read per query: a packed element via
//! [`read_all`](crate::protocol::read_all), a plain string stream
//! (getRegisterName / getUserOpName / getCodeLabel), a bool stream
//! (isNameUsed), or a byte stream (getBytes / getStringData).  A Java
//! exception frame (burst 10) surfaces as [`KunaError::Java`] from any of
//! the response readers.
//!
//! Phase-1 scope: the framing and request encodings are complete and
//! byte-exact; responses are handed back as raw decoded payloads (the
//! caller's [`Decoder`] holds the packed document / the byte buffers hold
//! the raw bytes).  The typed decoding into engine structures
//! (VarnodeData caches, p-code emit, symbol scopes, ...) is the phase-2
//! engine bridge — see `docs/ghidra-integration.md`.
//!
//! IMPORTANT protocol constraint (DecompileProcess.java:472-473,571-572 vs
//! :512-513 etc.): the Java client services queries only while a
//! registerProgram / decompileAt / structureGraph / generateSignatures /
//! debugSignatures command is in flight; issuing a query at any other time
//! desyncs the protocol.  Enforcing that is the phase-2 session's job.

use std::io::{Read, Write};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::address::{ATTRIB_FIRST, ATTRIB_LAST};
use kuna_base::error::KunaError;
use kuna_base::marshal::{
    Decoder, Encoder, PackedEncode, ATTRIB_CONTENT, ATTRIB_ID, ATTRIB_INDEX, ATTRIB_NAME,
    ATTRIB_SIZE, ATTRIB_TYPE, ELEM_VALUE,
};
use kuna_base::space::VarnodeStorage;

use crate::ids::{
    ATTRIB_MAXSIZE, ELEM_COMMAND_GETBYTES, ELEM_COMMAND_GETCALLFIXUP, ELEM_COMMAND_GETCALLMECH,
    ELEM_COMMAND_GETCALLOTHERFIXUP, ELEM_COMMAND_GETCODELABEL, ELEM_COMMAND_GETCOMMENTS,
    ELEM_COMMAND_GETCPOOLREF, ELEM_COMMAND_GETDATATYPE, ELEM_COMMAND_GETEXTERNALREF,
    ELEM_COMMAND_GETMAPPEDSYMBOLS, ELEM_COMMAND_GETNAMESPACEPATH, ELEM_COMMAND_GETPCODE,
    ELEM_COMMAND_GETPCODEEXECUTABLE, ELEM_COMMAND_GETREGISTER, ELEM_COMMAND_GETREGISTERNAME,
    ELEM_COMMAND_GETSTRINGDATA, ELEM_COMMAND_GETTRACKEDREGISTERS, ELEM_COMMAND_GETUSEROPNAME,
    ELEM_COMMAND_ISNAMEUSED,
};
use crate::protocol::{
    java_alignment, nibble_condense, read_all, read_bool_stream, read_response_end,
    read_string_stream, read_to_any_burst, read_to_response, string_data_size, write_burst,
    WireError, WireResult, BURST_BYTES_CLOSE, BURST_BYTES_OPEN, BURST_QUERY_CLOSE,
    BURST_QUERY_OPEN, BURST_STRING_CLOSE, BURST_STRING_OPEN,
};

/// The injection variety of a getPcodeInject query — selects the command
/// element (C++ `InjectPayload::{CALLFIXUP_TYPE, CALLOTHERFIXUP_TYPE,
/// CALLMECHANISM_TYPE, EXECUTABLEPCODE_TYPE}`, inject.hh; the numeric
/// values never travel on the wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InjectType {
    /// C++ `CALLFIXUP_TYPE` -> \<command_getcallfixup>.
    CallFixup,
    /// C++ `CALLOTHERFIXUP_TYPE` -> \<command_getcallotherfixup>.
    CallOtherFixup,
    /// C++ `CALLMECHANISM_TYPE` -> \<command_getcallmech>.
    CallMechanism,
    /// C++ `EXECUTABLEPCODE_TYPE` (and the default) ->
    /// \<command_getpcodeexecutable>.
    ExecutablePcode,
}

/// The client half of ghidra mode: owns the two protocol streams and
/// exposes the typed query methods of `ArchitectureGhidra`
/// (ghidra_arch.cc:445-896).
///
/// Response documents are decoded into a caller-supplied [`Decoder`]
/// (matching the C++ `Decoder&` out-parameters): the caller constructs a
/// `PackedDecode` over its `AddrSpaceManager`, whose space indices must
/// match the tspec the Java side sent (see `crate::translate`).
#[derive(Debug)]
pub struct GhidraClient<R: Read, W: Write> {
    /// The input stream from the client (C++ `sin`).
    sin: R,
    /// The output stream to the client (C++ `sout`).
    sout: W,
}

impl<R: Read, W: Write> GhidraClient<R, W> {
    /// Construct over the two protocol streams.
    pub fn new(sin: R, sout: W) -> Self {
        GhidraClient { sin, sout }
    }

    /// Tear down into the underlying streams (test access).
    pub fn into_inner(self) -> (R, W) {
        (self.sin, self.sout)
    }

    /// Mutably borrow the input stream (C++ `sin`) for the command loop's own
    /// framing reads.
    ///
    /// The loop and the engine's providers share one `GhidraClient` (a
    /// [`SharedClient`](crate::provider::SharedClient)): the loop frames command
    /// responses through these accessors, and mid-decompile the engine's
    /// providers issue callback queries through the typed query methods — on the
    /// *same* streams.  The borrow handed out here must be short-lived (framing
    /// one read/write, then dropped) so it never overlaps a provider's query
    /// borrow; the protocol is synchronous, so those borrows never actually
    /// overlap.  See [`crate::process`] for the no-borrow-across-`raw_action`
    /// invariant that makes this sound.
    pub fn sin_mut(&mut self) -> &mut R {
        &mut self.sin
    }

    /// Mutably borrow the output stream (C++ `sout`) for the command loop's own
    /// framing writes.  See [`sin_mut`](GhidraClient::sin_mut).
    pub fn sout_mut(&mut self) -> &mut W {
        &mut self.sout
    }

    /// The shared request framing: query open, string stream holding the
    /// packed command document, string close, query close, flush.
    fn send_query_document(&mut self, packed: &[u8]) -> WireResult<()> {
        write_burst(&mut self.sout, BURST_QUERY_OPEN)?;
        write_burst(&mut self.sout, BURST_STRING_OPEN)?;
        self.sout.write_all(packed).map_err(WireError::Io)?;
        write_burst(&mut self.sout, BURST_STRING_CLOSE)?;
        write_burst(&mut self.sout, BURST_QUERY_CLOSE)?;
        self.sout.flush().map_err(WireError::Io)?;
        Ok(())
    }

    /// A plain-string response: response header, string stream, response
    /// end (the tail of getRegisterName / getUserOpName / getCodeLabel).
    fn read_string_response(&mut self) -> WireResult<Vec<u8>> {
        read_to_response(&mut self.sin)?;
        let res = read_string_stream(&mut self.sin)?;
        read_response_end(&mut self.sin)?;
        Ok(res)
    }

    /// Query for the storage location of a named register (C++
    /// `getRegister`, ghidra_arch.cc:445-459).  The response (an \<addr>
    /// element) lands in `decoder`; `false` means an empty response.
    pub fn get_register(&mut self, regname: &str, decoder: &mut dyn Decoder) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETREGISTER);
            encoder.write_string(&ATTRIB_NAME, regname.as_bytes());
            encoder.close_element(&ELEM_COMMAND_GETREGISTER);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query for the name of a register overlapping a storage range (C++
    /// `getRegisterName`, ghidra_arch.cc:466-485).  Returns the name bytes
    /// or empty.
    pub fn get_register_name(&mut self, vndata: &VarnodeStorage) -> WireResult<Vec<u8>> {
        let space = vndata
            .space
            .as_ref()
            .ok_or_else(|| java_alignment("getRegisterName without an address space"))?;
        let addr = Address::new(Rc::clone(space), vndata.offset);
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETREGISTERNAME);
            addr.encode_sized(&mut encoder, vndata.size as i32)
                .map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETREGISTERNAME);
        }
        self.send_query_document(&buf)?;
        self.read_string_response()
    }

    /// Query for registers with known values at an address (C++
    /// `getTrackedRegisters`, ghidra_arch.cc:494-508).  The response
    /// (\<tracked_pointset>) lands in `decoder`.
    pub fn get_tracked_registers(
        &mut self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETTRACKEDREGISTERS);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETTRACKEDREGISTERS);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query for the name of a user-defined (CALLOTHER) op (C++
    /// `getUserOpName`, ghidra_arch.cc:514-532).  An empty result marks the
    /// end of the init-time index probe loop (ghidra_translate.cc:107-117).
    pub fn get_user_op_name(&mut self, index: i32) -> WireResult<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETUSEROPNAME);
            encoder.write_signed_integer(&ATTRIB_INDEX, index as i64);
            encoder.close_element(&ELEM_COMMAND_GETUSEROPNAME);
        }
        self.send_query_document(&buf)?;
        self.read_string_response()
    }

    /// Query for the p-code of one instruction (C++ `getPcode`,
    /// ghidra_arch.cc:538-552).  The response (\<inst> with \<op> children,
    /// or \<unimpl>) lands in `decoder`; `false` (empty response) is the
    /// caller's BadDataError.
    pub fn get_pcode(&mut self, addr: &Address, decoder: &mut dyn Decoder) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETPCODE);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETPCODE);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query for the symbols mapped at an address (C++
    /// `getMappedSymbolsXML`, ghidra_arch.cc:561-575).  The response
    /// (\<doc> with \<mapsym>, or \<hole>) lands in `decoder`.
    pub fn get_mapped_symbols_xml(
        &mut self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETMAPPEDSYMBOLS);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETMAPPEDSYMBOLS);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Resolve an external reference (C++ `getExternalRef`,
    /// ghidra_arch.cc:586-600).  The response (\<function> or \<hole>)
    /// lands in `decoder`.
    pub fn get_external_ref(
        &mut self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETEXTERNALREF);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETEXTERNALREF);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query the namespace path for a scope id (C++ `getNamespacePath`,
    /// ghidra_arch.cc:608-622).  The response (\<parent> with \<val>
    /// children) lands in `decoder`.
    pub fn get_namespace_path(&mut self, id: u64, decoder: &mut dyn Decoder) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETNAMESPACEPATH);
            encoder.write_unsigned_integer(&ATTRIB_ID, id);
            encoder.close_element(&ELEM_COMMAND_GETNAMESPACEPATH);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Ask whether a symbol name is already used in a namespace range (C++
    /// `isNameUsed`, ghidra_arch.cc:624-643).  The response is a bool
    /// stream.
    pub fn is_name_used(&mut self, nm: &str, start_id: u64, stop_id: u64) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_ISNAMEUSED);
            encoder.write_string(&ATTRIB_NAME, nm.as_bytes());
            encoder.write_unsigned_integer(&ATTRIB_FIRST, start_id);
            encoder.write_unsigned_integer(&ATTRIB_LAST, stop_id);
            encoder.close_element(&ELEM_COMMAND_ISNAMEUSED);
        }
        self.send_query_document(&buf)?;
        read_to_response(&mut self.sin)?;
        let res = read_bool_stream(&mut self.sin)?;
        read_response_end(&mut self.sin)?;
        Ok(res)
    }

    /// Query the primary symbol name at an address (C++ `getCodeLabel`,
    /// ghidra_arch.cc:649-667).  Returns the label bytes or empty.
    pub fn get_code_label(&mut self, addr: &Address) -> WireResult<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETCODELABEL);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETCODELABEL);
        }
        self.send_query_document(&buf)?;
        self.read_string_response()
    }

    /// Query for a data-type by name and id (C++ `getDataType`,
    /// ghidra_arch.cc:675-690).  The response (\<type>) lands in `decoder`.
    /// (The id travels as a *signed* integer, matching the C++ encode.)
    pub fn get_data_type(
        &mut self,
        name: &str,
        id: u64,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETDATATYPE);
            encoder.write_string(&ATTRIB_NAME, name.as_bytes());
            // C++ writeSignedInteger(ATTRIB_ID, id) with a uint8 id: the
            // value is reinterpreted as signed
            encoder.write_signed_integer(&ATTRIB_ID, id as i64);
            encoder.close_element(&ELEM_COMMAND_GETDATATYPE);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query all comments for a function matching the property `flags` (C++
    /// `getComments`, ghidra_arch.cc:700-715).  The response (\<commentdb>)
    /// lands in `decoder` — always written by Java, possibly empty.
    pub fn get_comments(
        &mut self,
        fad: &Address,
        flags: u32,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETCOMMENTS);
            encoder.write_unsigned_integer(&ATTRIB_TYPE, flags as u64);
            fad.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETCOMMENTS);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Query a range of load-image bytes (C++ `getBytes`,
    /// ghidra_arch.cc:723-760).  `buf.len()` bytes at `inaddr` are fetched
    /// through the nibble-doubled byte stream; an empty response raises
    /// `DataUnavailError`.
    pub fn get_bytes(&mut self, buf: &mut [u8], inaddr: &Address) -> WireResult<()> {
        let size = buf.len();
        let mut req: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut req);
            encoder.open_element(&ELEM_COMMAND_GETBYTES);
            inaddr
                .encode_sized(&mut encoder, size as i32)
                .map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETBYTES);
        }
        self.send_query_document(&req)?;

        read_to_response(&mut self.sin)?;
        let t = read_to_any_burst(&mut self.sin)?;
        if t == BURST_BYTES_OPEN {
            let mut dblbuf = vec![0u8; size * 2];
            self.sin.read_exact(&mut dblbuf).map_err(|_| {
                // C++ checks gcount and throws the alignment JavaError
                java_alignment("Could not read expected number of bytes")
            })?;
            buf.copy_from_slice(&nibble_condense(&dblbuf));
        } else if (t & 1) == 1 {
            let mut errmsg = String::from("GHIDRA has no data in the loadimage at ");
            if let Some(spc) = inaddr.get_space() {
                errmsg.push(spc.get_shortcut());
            }
            let _ = inaddr.print_raw(&mut errmsg);
            return Err(WireError::Kuna(KunaError::data_unavail(errmsg)));
        } else {
            return Err(java_alignment("Expecting bytes or end of query response"));
        }
        let t = read_to_any_burst(&mut self.sin)?;
        if t != BURST_BYTES_CLOSE {
            return Err(java_alignment("Expecting byte alignment end"));
        }
        read_response_end(&mut self.sin)
    }

    /// Query string data at an address, pre-translated to UTF-8 by the
    /// client (C++ `getStringData`, ghidra_arch.cc:774-816).  The C++
    /// passes a `Datatype*`; phase 1 takes the (name, unsized-id) pair the
    /// encode actually needs.  On success `buffer` holds the returned bytes
    /// (including the trailing NUL, as upstream) and the returned bool is
    /// the truncation flag; an empty response leaves `buffer` empty.
    pub fn get_string_data(
        &mut self,
        buffer: &mut Vec<u8>,
        addr: &Address,
        type_name: &str,
        type_id: u64,
        max_bytes: i32,
    ) -> WireResult<bool> {
        let mut req: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut req);
            encoder.open_element(&ELEM_COMMAND_GETSTRINGDATA);
            encoder.write_signed_integer(&ATTRIB_MAXSIZE, max_bytes as i64);
            encoder.write_string(&ATTRIB_TYPE, type_name.as_bytes());
            encoder.write_unsigned_integer(&ATTRIB_ID, type_id);
            addr.encode(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(&ELEM_COMMAND_GETSTRINGDATA);
        }
        self.send_query_document(&req)?;

        read_to_response(&mut self.sin)?;
        let mut is_trunc = false;
        let mut t = read_to_any_burst(&mut self.sin)?;
        if t == BURST_BYTES_OPEN {
            // two raw size-header bytes, one raw truncation-flag byte
            // (which MAY be a literal 0x00), then size*2 nibble-doubled
            // bytes whose final pair is the doubled NUL terminator
            let mut header = [0u8; 3];
            self.sin
                .read_exact(&mut header)
                .map_err(|_| WireError::PipeClosed)?;
            let size = string_data_size(header[0], header[1]) as usize;
            is_trunc = header[2] != 0;
            let mut dblbuf = vec![0u8; size * 2];
            self.sin
                .read_exact(&mut dblbuf)
                .map_err(|_| WireError::PipeClosed)?;
            *buffer = nibble_condense(&dblbuf);
            t = read_to_any_burst(&mut self.sin)?;
            if t != BURST_BYTES_CLOSE {
                return Err(java_alignment("Expecting byte alignment end"));
            }
            t = read_to_any_burst(&mut self.sin)?;
        }
        if (t & 1) == 1 {
            // response end consumed (or empty response: leave buffer empty)
            Ok(is_trunc)
        } else {
            Err(java_alignment("Expecting end of query response"))
        }
    }

    /// Retrieve p-code to inject for a named payload (C++ `getPcodeInject`,
    /// ghidra_arch.cc:833-869) — the four command-element variants selected
    /// by `inject_type`.  The C++ encodes an `InjectContext` (`<context>`)
    /// after the name; the engine-side context type is phase 2, so the
    /// caller supplies `encode_context` to write it (TODO(phase-2): take
    /// the engine's InjectContext directly).  The response (\<inst> with
    /// \<op> children) lands in `decoder`.
    pub fn get_pcode_inject(
        &mut self,
        name: &str,
        inject_type: InjectType,
        encode_context: impl FnOnce(&mut dyn Encoder) -> Result<(), KunaError>,
        decoder: &mut dyn Decoder,
    ) -> WireResult<bool> {
        let elem = match inject_type {
            InjectType::CallFixup => &ELEM_COMMAND_GETCALLFIXUP,
            InjectType::CallOtherFixup => &ELEM_COMMAND_GETCALLOTHERFIXUP,
            InjectType::CallMechanism => &ELEM_COMMAND_GETCALLMECH,
            InjectType::ExecutablePcode => &ELEM_COMMAND_GETPCODEEXECUTABLE,
        };
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(elem);
            encoder.write_string(&ATTRIB_NAME, name.as_bytes());
            encode_context(&mut encoder).map_err(WireError::Kuna)?;
            encoder.close_element(elem);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }

    /// Resolve a constant-pool reference (C++ `getCPoolRef`,
    /// ghidra_arch.cc:877-896).  The response (a cpool record) lands in
    /// `decoder`.
    pub fn get_cpool_ref(&mut self, refs: &[u64], decoder: &mut dyn Decoder) -> WireResult<bool> {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut buf);
            encoder.open_element(&ELEM_COMMAND_GETCPOOLREF);
            encoder.write_signed_integer(&ATTRIB_SIZE, refs.len() as i64);
            for &r in refs {
                encoder.open_element(&ELEM_VALUE);
                encoder.write_unsigned_integer(&ATTRIB_CONTENT, r);
                encoder.close_element(&ELEM_VALUE);
            }
            encoder.close_element(&ELEM_COMMAND_GETCPOOLREF);
        }
        self.send_query_document(&buf)?;
        read_all(&mut self.sin, decoder)
    }
}
