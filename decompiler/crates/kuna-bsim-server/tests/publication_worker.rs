#![cfg(all(feature = "http", any(target_os = "linux", target_os = "macos", windows)))]

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use kuna_bsim_server::{ErrorResponse, ImportOutcome, PUBLISH_CONTENT_TYPE, ProfilesResponse, PublishResponse, StoredFunction};
use sha2::{Digest, Sha256};

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn exchange(addr: SocketAddr, headers: &str, frames: &[&[u8]]) -> Vec<u8> {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
    stream.set_write_timeout(Some(Duration::from_secs(30))).unwrap();
    stream.write_all(headers.as_bytes()).unwrap();
    for frame in frames {
        stream.write_all(frame).unwrap();
    }
    let mut wire = Vec::new();
    stream.read_to_end(&mut wire).unwrap();
    wire
}

fn body<'a>(wire: &'a [u8], status: &str) -> &'a [u8] {
    let header_end = wire.windows(4).position(|bytes| bytes == b"\r\n\r\n").expect("HTTP response headers");
    let header = std::str::from_utf8(&wire[..header_end]).unwrap();
    assert_eq!(header.lines().next().unwrap(), format!("HTTP/1.1 {status}"), "{}", String::from_utf8_lossy(&wire[header_end + 4..]));
    &wire[header_end + 4..]
}

fn publish(addr: SocketAddr, metadata: &[u8], image: &[u8]) -> Vec<u8> {
    let len = size_of::<u32>() + metadata.len() + image.len();
    let head = format!("POST /v1/publish HTTP/1.1\r\nHost: {addr}\r\nAuthorization: Bearer writer\r\n\
                        Content-Type: {PUBLISH_CONTENT_TYPE}\r\nContent-Length: {len}\r\n\r\n");
    exchange(addr, &head, &[&(metadata.len() as u32).to_be_bytes(), metadata, image])
}

fn get(addr: SocketAddr, path: &str) -> Vec<u8> {
    exchange(addr, &format!("GET /v1/{path} HTTP/1.1\r\nHost: {addr}\r\nAuthorization: Bearer reader\r\n\r\n"), &[])
}

#[test]
fn worker_signs_only_the_staged_image_and_preserves_the_download() {
    let dir = tempfile::tempdir().unwrap();
    let mut server = Server(Command::new(env!("CARGO_BIN_EXE_kuna-bsim-server"))
        .args(["serve", "--bind", "127.0.0.1:0", "--store"])
        .arg(dir.path())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env_remove("SLEIGHHOME")
        .env("KUNA_SPECS", "../../../specs")
        .env("KUNA_BSIM_SERVER_TOKEN", "reader")
        .env("KUNA_BSIM_SERVER_WRITE_TOKEN", "writer")
        // This global interactive option would prevent loading the x86 file.
        .env("KUNA_ARM_ISA", "thumb")
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .spawn().unwrap());
    let mut stderr = BufReader::new(server.0.stderr.take().unwrap());
    let mut startup = String::new();
    stderr.read_line(&mut startup).unwrap();
    let addr = startup.trim().strip_prefix("kuna-bsim-server listening on http://")
        .unwrap_or_else(|| panic!("server startup: {startup}")).parse().unwrap();

    let metadata = br#"{"executable":"fixture","symbol":"zext_arg","rename":"published_zext"}"#;
    let rejected = publish(addr, metadata, b"not an executable");
    let error: ErrorResponse = serde_json::from_slice(body(&rejected, "422 Unprocessable Content")).unwrap();
    assert_eq!(error.error, "could not load executable or sign the requested function");
    let universal = include_bytes!("../../kuna-analysis/tests/fixtures/macho_fat");
    let rejected = publish(addr, br#"{"executable":"fat","symbol":"main"}"#, universal);
    body(&rejected, "422 Unprocessable Content");

    let image = include_bytes!("../../kuna-analysis/tests/fixtures/zextarg_x86_64");
    let result = publish(addr, metadata, image);
    let published: PublishResponse = serde_json::from_slice(body(&result, "200 OK")).unwrap();
    assert_eq!(published.name, "published_zext");
    assert_eq!(published.profile, "64");
    assert_eq!(published.sha256, kuna_base::hex_lower(&Sha256::digest(image)));
    assert_eq!(published.outcome, ImportOutcome::Inserted);
    assert_ne!(published.address, 0);

    let result = get(addr, &format!("functions/64/{}/{:x}", published.sha256, published.address));
    let found: StoredFunction = serde_json::from_slice(body(&result, "200 OK")).unwrap();
    assert_eq!(found.name, published.name);
    assert_eq!(found.executable, "fixture");
    assert!(!found.complete);
    let profiles: ProfilesResponse = serde_json::from_slice(body(&get(addr, "profiles"), "200 OK")).unwrap();
    assert_eq!(profiles.profiles.iter().find(|info| info.profile == "64").unwrap().functions, 1);
    assert_eq!(body(&get(addr, &format!("binaries/{}", published.sha256)), "200 OK"), image);
}

#[test]
fn worker_rejects_missing_start_signal() {
    let dir = tempfile::tempdir().unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_kuna-bsim-server"))
        .arg("--publication-worker")
        .current_dir(dir.path())
        .stdin(Stdio::null())
        .output().unwrap();
    assert_eq!(result.status.code(), Some(69));
    assert!(result.stdout.is_empty());
    assert!(result.stderr.is_empty());
}
