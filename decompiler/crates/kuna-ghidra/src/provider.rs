//! Ghidra-backed engine providers — the Phase-2 scaffolding.
//!
//! Upstream, `ArchitectureGhidra::init` installs a family of component
//! subclasses that back the engine's loader / translator / symbol table /
//! type factory / … with queries to the Java host instead of a local file
//! (`decompiler/cpp/loadimage_ghidra.cc`, `ghidra_translate.cc`,
//! `database_ghidra.cc`, `typegrp_ghidra.cc`, `comment_ghidra.cc`,
//! `string_ghidra.cc`, `ghidra_context.cc`, `cpool_ghidra.cc`,
//! `inject_ghidra.cc`).  This module ports them into kuna as query-backed
//! implementations of kuna's existing engine traits.
//!
//! **Phase-2 status.** [`GhidraLoadImage`] is implemented and tested here as
//! the demonstrator for the pattern the whole phase turns on: the engine,
//! mid-decompile, re-entrantly calls a provider (here `load_fill`), which
//! issues a callback query on the *same* streams the command response is
//! being written to (`SharedClient`, below).  Proving that hook plugs into a
//! real kuna trait (`kuna_sleigh::loadimage::LoadImage`) with no change to
//! the standalone engine is what de-risks the rest.  The remaining providers
//! (`ContextGhidra`, the `ScopeGhidra` lazy symbol cache, `TypeFactoryGhidra`,
//! …) and the wiring into an `Architecture` are the gated work — see
//! `docs/rust-port/ghidra-phase2-plan.md`.

use std::cell::RefCell;
use std::io::{Read, Write};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::Decoder;
use kuna_sleigh::loadimage::LoadImage;

use kuna_decomp::dtype::RemoteTypeFetch;
use kuna_decomp::remote_provider::RemoteProviderFetch;

use crate::client::GhidraClient;
use crate::protocol::WireError;

/// A [`GhidraClient`] shared between the command loop and the engine's
/// providers.
///
/// This is the kuna analog of the C++ `glb` back-pointer every ghidra-mode
/// component holds to its `ArchitectureGhidra` (e.g. `LoadImageGhidra::glb`,
/// `loadimage_ghidra.hh`).  During a `decompileAt` the engine re-entrantly
/// calls a provider, which issues a query on the same `sin`/`sout` the
/// command response is mid-flight on — so the client must be *shared*, not
/// owned, by both the process loop and every provider.  `Rc<RefCell<…>>` is
/// the single-threaded shared-mutable analog of the raw C++ pointer (the
/// protocol is strictly synchronous — at most one borrow is ever live,
/// because a query fully completes before the engine regains control).
pub type SharedClient<R, W> = Rc<RefCell<GhidraClient<R, W>>>;

/// C++ `LoadImageGhidra` (`loadimage_ghidra.cc`): every engine read of
/// program bytes — constant-pointer dereference, read-only propagation,
/// string probing — becomes a `getBytes` query to the host.  There is no
/// local cache (the Java side caches; `loadimage_ghidra.cc` comment).
pub struct GhidraLoadImage<R: Read, W: Write> {
    client: SharedClient<R, W>,
}

impl<R: Read, W: Write> GhidraLoadImage<R, W> {
    /// Construct over the shared query client (C++ `new LoadImageGhidra(this)`,
    /// `ghidra_arch.cc:283-287`).
    pub fn new(client: SharedClient<R, W>) -> Self {
        GhidraLoadImage { client }
    }
}

impl<R: Read, W: Write> LoadImage for GhidraLoadImage<R, W> {
    /// C++ constructs it with the pseudo-filename `"ghidra_progam"` [sic]
    /// (`loadimage_ghidra.cc:20-25`).
    fn get_file_name(&self) -> &str {
        "ghidra_progam"
    }

    /// C++ `LoadImageGhidra::loadFill` → `glb->getBytes`
    /// (`loadimage_ghidra.cc:37-41`).  An unmapped range comes back as the
    /// empty-response `DataUnavailError` from `getBytes`.
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        self.client
            .borrow_mut()
            .get_bytes(ptr, addr)
            .map_err(wire_to_kuna)
    }

    /// C++ `getArchType` → `"ghidra"` (`loadimage_ghidra.cc:43-47`).
    fn get_arch_type(&self) -> Vec<u8> {
        b"ghidra".to_vec()
    }

    /// C++ `adjustVma` throws `LowlevelError` (`loadimage_ghidra.cc:49-53`) —
    /// it is a raw-binary concern never reached in ghidra mode.  kuna's trait
    /// method cannot return an error, so this is a no-op.
    /// TODO(phase-2): if a caller ever reaches it, surface a fatal error
    /// through the process loop rather than silently doing nothing.
    fn adjust_vma(&mut self, _adjust: i64) {}
}

/// The Phase-3 wire-fetch adapter: `kuna-decomp`'s [`RemoteProviderFetch`] +
/// [`RemoteTypeFetch`] seams implemented over the [`SharedClient`] — the same
/// re-entrant pattern as [`GhidraLoadImage`].  The `RemoteScope` /
/// `TypeFactoryImpl` hold `Rc`s of one instance; every miss becomes a
/// getMappedSymbols / getNamespacePath / getComments / getDataType query
/// nested inside the still-open command response.
pub struct GhidraRemoteFetch<R: Read, W: Write> {
    client: SharedClient<R, W>,
}

impl<R: Read, W: Write> GhidraRemoteFetch<R, W> {
    /// Construct over the shared query client.
    pub fn new(client: SharedClient<R, W>) -> Self {
        GhidraRemoteFetch { client }
    }
}

impl<R: Read, W: Write> RemoteProviderFetch for GhidraRemoteFetch<R, W> {
    fn fetch_mapped_symbols(
        &self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_mapped_symbols_xml(addr, decoder)
            .map_err(wire_to_kuna)
    }

    fn fetch_namespace_path(&self, id: u64, decoder: &mut dyn Decoder) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_namespace_path(id, decoder)
            .map_err(wire_to_kuna)
    }

    fn fetch_external_ref(&self, addr: &Address, decoder: &mut dyn Decoder) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_external_ref(addr, decoder)
            .map_err(wire_to_kuna)
    }

    fn fetch_tracked_registers(
        &self,
        addr: &Address,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_tracked_registers(addr, decoder)
            .map_err(wire_to_kuna)
    }

    fn fetch_comments(
        &self,
        fad: &Address,
        flags: u32,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_comments(fad, flags, decoder)
            .map_err(wire_to_kuna)
    }
}

impl<R: Read, W: Write> RemoteTypeFetch for GhidraRemoteFetch<R, W> {
    fn fetch_data_type(
        &self,
        name: &str,
        id: u64,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<bool> {
        self.client
            .borrow_mut()
            .get_data_type(name, id, decoder)
            .map_err(wire_to_kuna)
    }
}

/// Map a query [`WireError`] onto the [`KunaError`] a [`LoadImage`] method
/// must return.
///
/// A `DataUnavail` passes through unchanged (the `getBytes` empty-response
/// case — an unmapped address).  A pipe/IO failure means the host connection
/// died mid-query; upstream `exit(1)`s from inside `readToAnyBurst`
/// (`ghidra_arch.cc:95-96`), but the trait method can only return a
/// `KunaError`, so it is mapped to a `LowlevelError` here.
///
/// TODO(phase-2): the process loop needs a way for a provider to signal
/// "host pipe dead → terminate" so a mid-decompile pipe death still exits
/// the process with status 1 (the `SharedClient` should carry a fatal flag
/// the loop checks after each command).
pub(crate) fn wire_to_kuna(err: WireError) -> KunaError {
    match err {
        WireError::Kuna(e) => e,
        WireError::PipeClosed => KunaError::lowlevel("ghidra host pipe closed during query"),
        WireError::Io(e) => KunaError::lowlevel(format!("i/o error during query: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use kuna_base::marshal::{Encoder, PackedEncode};

    use crate::translate::{build_registry, TspecModel};

    const TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x10000000\">\
<spaces defaultspace=\"ram\">\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"8\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
</spaces></sleigh>";

    fn test_translate() -> TspecModel {
        TspecModel::decode(TSPEC, &build_registry()).expect("test tspec parses")
    }

    fn burst(v: &mut Vec<u8>, code: u8) {
        v.extend_from_slice(&[0, 0, 1, code]);
    }

    /// Nibble-double each byte as `'A'+hi, 'A'+lo` — DecompileProcess.java's
    /// getBytes byte-stream encoding (:867-885).
    fn nibble_expand(bytes: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(bytes.len() * 2);
        for &b in bytes {
            v.push(b'A' + (b >> 4));
            v.push(b'A' + (b & 0x0f));
        }
        v
    }

    /// A getBytes query response carrying `data` (burst 8, byte-stream
    /// 12/13, burst 9).
    fn get_bytes_response(data: &[u8]) -> Vec<u8> {
        let mut r = Vec::new();
        burst(&mut r, 8);
        burst(&mut r, 12);
        r.extend_from_slice(&nibble_expand(data));
        burst(&mut r, 13);
        burst(&mut r, 9);
        r
    }

    /// The unmapped-address case: an empty query response (burst 8 then a
    /// close burst) that getBytes turns into DataUnavailError.
    fn empty_response() -> Vec<u8> {
        let mut r = Vec::new();
        burst(&mut r, 8);
        burst(&mut r, 9);
        r
    }

    fn shared(reader: Vec<u8>) -> SharedClient<Cursor<Vec<u8>>, Vec<u8>> {
        Rc::new(RefCell::new(GhidraClient::new(
            Cursor::new(reader),
            Vec::new(),
        )))
    }

    #[test]
    fn load_fill_pulls_bytes_from_host() {
        let tr = test_translate();
        let ram = Rc::clone(tr.manager.get_space_by_name("ram").expect("ram"));
        let want = [0xde, 0xad, 0xbe, 0xef];

        let client = shared(get_bytes_response(&want));
        // Exercise it through the trait object, proving the hook plugs in.
        let mut loader: Box<dyn LoadImage> = Box::new(GhidraLoadImage::new(Rc::clone(&client)));

        assert_eq!(loader.get_arch_type(), b"ghidra");
        assert_eq!(loader.get_file_name(), "ghidra_progam");

        let mut buf = [0u8; 4];
        loader
            .load_fill(&mut buf, &Address::new(ram, 0x401000))
            .expect("load_fill");
        assert_eq!(buf, want);

        // The query it issued is byte-exact getBytes framing. Drop the loader
        // first so `client` is the sole owner of the shared handle.
        drop(loader);
        let (_r, w) = Rc::try_unwrap(client).unwrap().into_inner().into_inner();

        // Encode the expected <command_getbytes> document once, then wrap it
        // in the query/string bursts.
        let mut cmd = Vec::new();
        {
            let ram2 = Rc::clone(test_translate().manager.get_space_by_name("ram").unwrap());
            let mut e = PackedEncode::new(&mut cmd);
            e.open_element(&crate::ids::ELEM_COMMAND_GETBYTES);
            Address::new(ram2, 0x401000)
                .encode_sized(&mut e, 4)
                .unwrap();
            e.close_element(&crate::ids::ELEM_COMMAND_GETBYTES);
        }
        let mut expected = Vec::new();
        burst(&mut expected, 4); // query open
        burst(&mut expected, 14); // string open
        expected.extend_from_slice(&cmd);
        burst(&mut expected, 15); // string close
        burst(&mut expected, 5); // query close
        assert_eq!(w, expected, "getBytes request framing");
    }

    #[test]
    fn load_fill_unmapped_is_data_unavail() {
        let tr = test_translate();
        let ram = Rc::clone(tr.manager.get_space_by_name("ram").expect("ram"));
        let client = shared(empty_response());
        let mut loader = GhidraLoadImage::new(client);

        let mut buf = [0u8; 4];
        let err = loader
            .load_fill(&mut buf, &Address::new(ram, 0x0))
            .expect_err("unmapped address must fail");
        assert!(
            matches!(err, KunaError::DataUnavail { .. }),
            "expected DataUnavail, got {err:?}"
        );
    }
}
