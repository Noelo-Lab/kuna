use super::*;
use kuna_analysis::analyzers::bsim::corpus::CorpusExecutable;
use kuna_analysis::analyzers::bsim::lshweights::LshProfile;
use kuna_analysis::analyzers::bsim::signature::BsimSignature;
use md5::Md5;

fn service() -> (tempfile::TempDir, Service) {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let service = Service {
        store,
        bind: "127.0.0.1:4567".parse().unwrap(),
        tls: None,
        token: Some(Sha256::digest(b"secret").into()),
        write_token: None,
        publisher: None,
        publish_slot: Arc::default(),
    };
    (dir, service)
}

fn submit(service: &Service, message: impl AsRef<[u8]>) -> Response {
    let mut reader = io::Cursor::new(message.as_ref());
    let response = service.admit(&mut reader).and_then(|request| {
        let _slot = if matches!(request.endpoint, Endpoint::Publish) {
            Some(service.reserve_publication()?)
        } else {
            None
        };
        service.execute(&mut reader, request)
    });
    match response {
        Ok(response) => response,
        Err(failure) => Response::failure(failure).unwrap(),
    }
}

#[test]
fn validates_host_auth_and_request_bounds() {
    let (_dir, service) = service();
    let missing = submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: 127.0.0.1:4567\r\n\r\n");
    assert_eq!(missing.status, "401 Unauthorized");
    assert_eq!(missing.extra, "WWW-Authenticate: Bearer\r\n");
    let invalid = submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: external.example:4567\r\nAuthorization: Bearer secret\r\n\r\n");
    assert_eq!(invalid.status, "400 Bad Request");
    let correct = submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: LOCALHOST:4567\r\nAuthorization: bearer secret\r\n\r\n");
    assert_eq!(correct.status, "200 OK");
    let duplicate = submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: localhost:4567\r\nHost: 127.0.0.1:4567\r\nAuthorization: Bearer secret\r\n\r\n");
    assert_eq!(duplicate.status, "400 Bad Request");
    let huge = submit(&service, "POST /v1/query HTTP/1.1\r\nHost: localhost:4567\r\nAuthorization: Bearer secret\r\nContent-Type: application/json\r\nContent-Length: 3000000\r\n\r\n");
    assert_eq!(huge.status, "413 Content Too Large");
    let wrong_method = submit(&service, "POST /v1/profiles HTTP/1.1\r\nHost: localhost:4567\r\nAuthorization: Bearer secret\r\n\r\n");
    assert_eq!(wrong_method.status, "405 Method Not Allowed");
    assert_eq!(wrong_method.extra, "Allow: GET\r\n");
    let v6 = "[::1]:4567".parse().unwrap();
    assert!(loopback_host(b"[0:0:0:0:0:0:0:1]:4567", v6));
    assert!(!loopback_host(b"localhost:4568", v6));
    assert!(!loopback_host(b"127.0.0.1:4567", v6));
}

#[test]
fn wire_match_entry_and_download_are_bound_to_the_same_digest() {
    let (_dir, service) = service();
    let bytes = b"immutable reference";
    let address = 0xfedcba9876543210;
    let result = service.store.import(LshProfile::Size64, CorpusExecutable {
        name: "a.bin".into(),
        architecture: "x86:LE:64:default".into(),
        compiler: "gcc".into(),
        md5: kuna_base::hex_lower(&Md5::digest(bytes)),
        complete: true,
        functions: vec![BsimSignature {
            name: "fn".into(),
            address,
            settings: 0x49,
            features: vec![1, 2, 2, 3],
            has_unimplemented: false,
            has_bad_data: false,
        }],
    }, bytes).unwrap();
    let sha256 = result.sha256;
    let query = r#"{"profile":"64","settings":73,"features":[1,2,2,3],"min_similarity":0.8,"max_matches":1}"#;
    let req = format!("POST /v1/query HTTP/1.1\r\nHost: localhost:4567\r\nAuthorization: Bearer secret\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{query}", query.len());
    let resp = submit(&service, &req);
    assert_eq!(resp.status, "200 OK");
    let Body::Json(body) = resp.body else { panic!("expected JSON response") };
    let actual: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let hit = &actual["matches"][0];
    assert_eq!(hit["sha256"], sha256);
    assert_eq!(hit["address"].as_u64(), Some(address));
    assert!(hit.get("url").is_none());

    let req = format!("GET /v1/functions/64/{sha256}/{address:x} HTTP/1.1\r\nHost: localhost:4567\r\nAuthorization: Bearer secret\r\n\r\n");
    let resp = submit(&service, &req);
    assert_eq!(resp.status, "200 OK");
    let Body::Json(body) = resp.body else { panic!("expected JSON response") };
    let entry: kuna_bsim_server::StoredFunction = serde_json::from_slice(&body).unwrap();
    assert_eq!(entry.sha256, sha256);
    assert_eq!(entry.address, address);
    let req = format!("GET /v1/binaries/{sha256} HTTP/1.1\r\nHost: localhost:4567\r\nAuthorization: Bearer secret\r\n\r\n");
    let resp = submit(&service, &req);
    assert_eq!(resp.status, "200 OK");
    let mut wire = Vec::new();
    resp.send(&mut wire).unwrap();
    assert!(wire.ends_with(bytes));
}

fn publish_headers(token: &str) -> String {
    format!("POST /v1/publish HTTP/1.1\r\nHost: localhost:4567\r\n\
             Authorization: Bearer {token}\r\nContent-Type: {PUBLISH_CONTENT_TYPE}\r\n")
}

fn publish_frame(metadata: &[u8], executable: &[u8]) -> Vec<u8> {
    [(metadata.len() as u32).to_be_bytes().as_slice(), metadata, executable].concat()
}

fn publish_request(token: &str, len: u64, frame: &[u8]) -> Vec<u8> {
    let mut request = format!("{}Content-Length: {len}\r\n\r\n", publish_headers(token)).into_bytes();
    request.extend_from_slice(frame);
    request
}

#[test]
fn publication_is_opt_in_and_separately_authorized() {
    let (_dir, mut service) = service();
    let write = publish_request("writer", 0, &[]);
    assert_eq!(submit(&service, &write).status, "404 Not Found");

    service.write_token = Some(Sha256::digest(b"writer").into());
    let read = publish_request("secret", 0, &[]);
    let denied = submit(&service, read);
    assert_eq!(denied.status, "401 Unauthorized");
    assert_eq!(denied.extra, "WWW-Authenticate: Bearer\r\n");
    assert_eq!(submit(&service, "POST /v1/publish HTTP/1.1\r\nHost: localhost:4567\r\n\r\n").status,
               "401 Unauthorized");
    assert_eq!(submit(&service, "POST /v1/publish HTTP/1.1\r\nHost: external.example:4567\r\n\
        Authorization: Bearer writer\r\n\r\n").status, "400 Bad Request");
    assert_eq!(submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: localhost:4567\r\n\
        Authorization: Bearer writer\r\n\r\n").status, "401 Unauthorized");
    let wrong_method = submit(&service, "GET /v1/publish HTTP/1.1\r\nHost: localhost:4567\r\n\
        Authorization: Bearer writer\r\n\r\n");
    assert_eq!(wrong_method.status, "405 Method Not Allowed");
    assert_eq!(wrong_method.extra, "Allow: POST\r\n");
    assert_eq!(submit(&service, &write).status, "400 Bad Request");

    let _busy = service.reserve_publication().ok().unwrap();
    assert_eq!(submit(&service, &write).status, "503 Service Unavailable");
    assert_eq!(submit(&service, "GET /v1/profiles HTTP/1.1\r\nHost: localhost:4567\r\n\
        Authorization: Bearer secret\r\n\r\n").status, "200 OK");
}

#[cfg(any(target_os = "linux", target_os = "macos", windows))]
#[test]
fn publication_requires_bounded_fixed_length_framing() {
    let (_dir, mut service) = service();
    service.write_token = Some(Sha256::digest(b"writer").into());
    let headers = publish_headers("writer");
    assert_eq!(submit(&service, format!("{headers}\r\n")).status, "411 Length Required");
    assert_eq!(submit(&service, format!("{headers}Transfer-Encoding: chunked\r\n\r\n")).status, "400 Bad Request");
    assert_eq!(submit(&service, format!("{headers}Content-Length: 8\r\nContent-Length: 8\r\n\r\n")).status,
               "400 Bad Request");
    assert_eq!(submit(&service, format!("{headers}Expect: 100-continue\r\nContent-Length: 8\r\n\r\n")).status,
               "417 Expectation Failed");
    assert_eq!(submit(&service, "POST /v1/publish HTTP/1.1\r\nHost: localhost:4567\r\n\
        Authorization: Bearer writer\r\nContent-Type: application/json\r\nContent-Length: 8\r\n\r\n").status,
               "415 Unsupported Media Type");

    let oversized_request = METADATA_PREFIX_BYTES as u64 + MAX_PUBLISH_METADATA_BYTES as u64 + MAX_ARTIFACT_BYTES + 1;
    assert_eq!(submit(&service, publish_request("writer", oversized_request, &[])).status, "413 Content Too Large");
    let oversized_metadata = (MAX_PUBLISH_METADATA_BYTES as u32 + 1).to_be_bytes();
    assert_eq!(submit(&service, publish_request("writer", u64::from(MAX_PUBLISH_METADATA_BYTES as u32) + 6,
               &oversized_metadata)).status, "413 Content Too Large");
    let metadata = br#"{"executable":"fixture","symbol":"zext_arg"}"#;
    let frame = publish_frame(metadata, b"executable");
    let oversized_artifact = METADATA_PREFIX_BYTES as u64 + metadata.len() as u64 + MAX_ARTIFACT_BYTES + 1;
    assert_eq!(submit(&service, publish_request("writer", oversized_artifact, &frame)).status, "413 Content Too Large");

    for incomplete in [
        publish_request("writer", 4, &[0; 3]),
        publish_request("writer", 5, &[0; 4]),
        publish_request("writer", 5, &2u32.to_be_bytes()),
        publish_request("writer", frame.len() as u64, &frame[..5]),
        publish_request("writer", frame.len() as u64, &frame[..frame.len() - 1]),
        publish_request("writer", (METADATA_PREFIX_BYTES + metadata.len()) as u64, &frame),
    ] {
        assert_eq!(submit(&service, incomplete).status, "400 Bad Request");
    }
    for invalid in [
        br#"{"executable":"fixture","symbol":"zext_arg","features":[1,2,3]}"#.as_slice(),
        br#"{"executable":"fixture","symbol":"zext_arg","address":0}"#,
        br#"{"executable":"fixture"}"#,
    ] {
        let body = publish_frame(invalid, b"executable");
        assert_eq!(submit(&service, publish_request("writer", body.len() as u64, &body)).status, "400 Bad Request");
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", windows))]
#[test]
fn publication_upload_timeout_releases_the_writer() {
    struct TimedOut;
    impl Read for TimedOut {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::TimedOut.into())
        }
    }

    let (_dir, mut service) = service();
    service.write_token = Some(Sha256::digest(b"writer").into());
    let frame = publish_frame(br#"{"executable":"fixture","symbol":"zext_arg"}"#, b"");
    let head_and_metadata = publish_request("writer", (frame.len() + 1) as u64, &frame);
    let mut reader = io::Cursor::new(head_and_metadata).chain(BufReader::new(TimedOut));
    let result = service.admit(&mut reader).and_then(|request| {
        let _slot = service.reserve_publication()?;
        service.execute(&mut reader, request)
    });
    assert!(matches!(result, Err(Failure { status: "408 Request Timeout", .. })));
    assert!(service.reserve_publication().is_ok());
}

#[test]
fn per_source_admission_counts_queued_and_open_connections() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let bind = listener.local_addr().unwrap();
    let sources = Arc::new(Mutex::new(BTreeMap::new()));
    let mut pending = Vec::new();
    for _ in 0..MAX_SOURCE_ADMISSIONS {
        let client = TcpStream::connect(bind).unwrap();
        let (server, addr) = listener.accept().unwrap();
        let slot = SourceSlot::acquire(&sources, addr.ip()).unwrap();
        pending.push((client, Admission { stream: server, accepted_at: Instant::now(), slot }));
    }
    let _client = TcpStream::connect(bind).unwrap();
    let (_server, addr) = listener.accept().unwrap();
    assert!(SourceSlot::acquire(&sources, addr.ip()).is_none());
    assert!(SourceSlot::acquire(&sources, "::ffff:127.0.0.1".parse().unwrap()).is_none());
    // A different source is not charged for the stalled connections.
    let other = SourceSlot::acquire(&sources, "127.0.0.2".parse().unwrap());
    assert!(other.is_some());
    drop(other);

    drop(pending.pop());
    assert!(SourceSlot::acquire(&sources, addr.ip()).is_some());
    drop(pending);
    assert!(sources.lock().unwrap().is_empty());
}

#[test]
fn header_admission_authenticates_before_dispatch_and_reserves_read_capacity() {
    let (_dir, mut service) = service();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    service.bind = listener.local_addr().unwrap();
    service.write_token = Some(Sha256::digest(b"writer").into());
    let (reads, pending_reads) = mpsc::sync_channel(1);
    let (writes, pending_writes) = mpsc::sync_channel(1);
    let sources = Arc::new(Mutex::new(BTreeMap::new()));

    let admit = |request: &[u8]| {
        let mut client = TcpStream::connect(service.bind).unwrap();
        client.set_read_timeout(Some(REQUEST_TIMEOUT)).unwrap();
        client.write_all(request).unwrap();
        let (stream, addr) = listener.accept().unwrap();
        let slot = SourceSlot::acquire(&sources, addr.ip()).unwrap();
        service.admit_connection(Admission { stream, accepted_at: Instant::now(), slot }, &reads, Some(&writes));
        client
    };
    let token = |token| format!(
        "GET /v1/profiles HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer {token}\r\n\r\n",
        service.bind,
    );
    let mut denied = admit(token("wrong").as_bytes());
    let mut response = String::new();
    denied.read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.1 401 Unauthorized\r\n"));
    assert!(pending_reads.try_recv().is_err());
    assert!(pending_writes.try_recv().is_err());

    let _read_client = admit(token("secret").as_bytes());
    let publish = format!(
        "POST /v1/publish HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer writer\r\n\
         Content-Type: {PUBLISH_CONTENT_TYPE}\r\nContent-Length: 12\r\n\r\n",
        service.bind,
    );
    let _write_client = admit(publish.as_bytes()); // no body; only execution may wait for it
    let write = pending_writes.try_recv().ok().expect("write admitted despite full read queue");
    assert!(matches!(write.request.endpoint, Endpoint::Publish));
    assert!(service.reserve_publication().is_err());
    // Neither the authorized writer nor its upload occupies read capacity.
    let read = pending_reads.try_recv().ok().unwrap();
    assert!(matches!(read.request.endpoint, Endpoint::Profiles));
    let _second_read = admit(token("secret").as_bytes());
    assert!(pending_reads.try_recv().is_ok());
    drop(write);
    assert!(service.reserve_publication().is_ok());
}
