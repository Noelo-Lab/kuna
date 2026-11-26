use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use kuna_bsim_server::{
    ErrorResponse, PublishRequest, PublishResponse, QueryRequest, Store, StoreError,
    MAX_ARTIFACT_BYTES, MAX_PUBLISH_METADATA_BYTES, MAX_QUERY_BYTES, PUBLISH_CONTENT_TYPE,
};
use rustls::{ServerConfig, ServerConnection, StreamOwned};
use serde::Serialize;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::publication_worker::{Publisher, WorkerError};

mod query_json;

const ADMISSION_WORKERS: usize = 8;
const ADMISSION_QUEUE: usize = 8;
const MAX_SOURCE_ADMISSIONS: usize = 2;
const READ_WORKERS: usize = 8;
const READ_QUEUE: usize = 8;
const MAX_HEADER_BYTES: usize = 16 * 1024;
const MAX_HEADERS: usize = 48;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const UPLOAD_TIMEOUT: Duration = Duration::from_secs(120);
const METADATA_PREFIX_BYTES: usize = size_of::<u32>();
const QUERY_TIMEOUT: Duration = Duration::from_secs(30);
const RESPONSE_TIMEOUT: Duration = Duration::from_secs(120);

struct Service {
    store: Store,
    bind: SocketAddr,
    tls: Option<Arc<ServerConfig>>,
    token: Option<[u8; 32]>,
    write_token: Option<[u8; 32]>,
    publisher: Option<Publisher>,
    publish_slot: Arc<AtomicBool>,
}

pub(super) fn run(
    listener: TcpListener,
    bind: SocketAddr,
    store: Store,
    tls: Option<Arc<ServerConfig>>,
    token: Option<[u8; 32]>,
    write_token: Option<[u8; 32]>,
) -> io::Result<()> {
    let publisher = write_token.as_ref().map(|_| Publisher::new()).transpose()?;
    let tls_enabled = tls.is_some();
    let service = Arc::new(Service {
        store, bind, tls, token, write_token, publisher, publish_slot: Arc::default(),
    });
    let reads = spawn_queue("bsim-read", READ_WORKERS, READ_QUEUE, {
        let service = Arc::clone(&service);
        move |job: Execution| job.run(&service)
    })?;
    let writes = if service.publisher.is_some() {
        let service = Arc::clone(&service);
        Some(spawn_queue("bsim-publish", 1, 1, move |job: Execution| job.run(&service))?)
    } else {
        None
    };
    let admit = spawn_queue("bsim-admit", ADMISSION_WORKERS, ADMISSION_QUEUE, move |job| {
        service.admit_connection(job, &reads, writes.as_ref());
    })?;
    let scheme = if tls_enabled { "https" } else { "http" };
    eprintln!("kuna-bsim-server listening on {scheme}://{bind}");

    let sources = Arc::new(Mutex::new(BTreeMap::new()));
    loop {
        match listener.accept() {
            Ok((stream, source)) => {
                // A single source cannot occupy all handshake/header workers.
                // Count both active and queued jobs and never block accept.
                if let Some(slot) = SourceSlot::acquire(&sources, source.ip()) {
                    let _ = admit.try_send(Admission { stream, accepted_at: Instant::now(), slot });
                }
            }
            Err(error) if matches!(error.kind(), io::ErrorKind::Interrupted | io::ErrorKind::ConnectionAborted) => {}
            Err(error) => return Err(error),
        }
    }
}

fn spawn_queue<T: Send + 'static>(
    name: &str,
    workers: usize,
    capacity: usize,
    handle: impl Fn(T) + Send + Sync + 'static,
) -> io::Result<mpsc::SyncSender<T>> {
    let (send, receive) = mpsc::sync_channel(capacity);
    let receive = Arc::new(Mutex::new(receive));
    let handle = Arc::new(handle);
    for i in 0..workers {
        let receive = Arc::clone(&receive);
        let handle = Arc::clone(&handle);
        std::thread::Builder::new().name(format!("{name}-{i}")).spawn(move || loop {
            let job = receive.lock().expect("worker queue not poisoned").recv();
            let Ok(job) = job else { break };
            handle(job);
        })?;
    }
    Ok(send)
}

type Sources = Mutex<BTreeMap<IpAddr, usize>>;

struct SourceSlot {
    sources: Arc<Sources>,
    ip: IpAddr,
}

impl SourceSlot {
    fn acquire(sources: &Arc<Sources>, ip: IpAddr) -> Option<Self> {
        let ip = ip.to_canonical();
        let mut active = sources.lock().expect("source admission map not poisoned");
        let count = active.entry(ip).or_default();
        if *count == MAX_SOURCE_ADMISSIONS {
            return None;
        }
        *count += 1;
        Some(Self { sources: Arc::clone(sources), ip })
    }
}

impl Drop for SourceSlot {
    fn drop(&mut self) {
        let mut active = self.sources.lock().expect("source admission map not poisoned");
        let count = active.get_mut(&self.ip).expect("active source slot");
        *count -= 1;
        if *count == 0 {
            active.remove(&self.ip);
        }
    }
}

struct PublishSlot(Arc<AtomicBool>);

impl Drop for PublishSlot {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

struct Admission {
    stream: TcpStream,
    accepted_at: Instant,
    slot: SourceSlot,
}

struct Execution {
    request: Admitted,
    reader: BufReader<Connection>,
    publish_slot: Option<PublishSlot>,
}

impl Execution {
    fn run(self, service: &Service) {
        let Self { request, mut reader, publish_slot } = self;
        if reader.get_mut().socket_mut().remaining().is_err() {
            return;
        }
        if publish_slot.is_some() {
            reader.get_mut().socket_mut().until = Instant::now() + UPLOAD_TIMEOUT;
        }
        let result = service.execute(&mut reader, request);
        reader.get_mut().socket_mut().until = Instant::now() + RESPONSE_TIMEOUT;
        let _ = send_response(&mut reader, result);
        drop(publish_slot);
    }
}

fn send_response(reader: &mut BufReader<Connection>, result: Result<Response, Failure>) -> io::Result<()> {
    let response = match result {
        Ok(response) => response,
        Err(failure) => Response::failure(failure)?,
    };
    let connection = reader.get_mut();
    response.send(connection)?;
    if let Connection::Tls(tls) = connection {
        tls.conn.send_close_notify();
    }
    connection.flush()
}

impl Service {
    fn accept_head(&self, stream: TcpStream, accepted_at: Instant) -> io::Result<BufReader<Connection>> {
        let mut socket = DeadlineSocket { stream, until: accepted_at + REQUEST_TIMEOUT };
        socket.remaining()?;
        let connection = if let Some(config) = &self.tls {
            let mut tls = ServerConnection::new(Arc::clone(config)).map_err(io::Error::other)?;
            tls.complete_io(&mut socket)?;
            Connection::Tls(Box::new(StreamOwned::new(tls, socket)))
        } else {
            Connection::Plain(socket)
        };
        Ok(BufReader::new(connection))
    }

    fn admit_connection(
        &self,
        job: Admission,
        reads: &mpsc::SyncSender<Execution>,
        writes: Option<&mpsc::SyncSender<Execution>>,
    ) {
        let Admission { stream, accepted_at, slot: _source_slot } = job;
        // Admission may neither read a body nor dispatch an unauthenticated
        // request. Rejections keep the original handshake/header deadline.
        let Ok(mut reader) = self.accept_head(stream, accepted_at) else { return };
        let request = match self.admit(&mut reader) {
            Ok(request) => request,
            Err(failure) => {
                let _ = send_response(&mut reader, Err(failure));
                return;
            }
        };
        let (queue, publish_slot) = if matches!(request.endpoint, Endpoint::Publish) {
            let Some(writes) = writes else { return };
            match self.reserve_publication() {
                Ok(slot) => (writes, Some(slot)),
                Err(failure) => {
                    let _ = send_response(&mut reader, Err(failure));
                    return;
                }
            }
        } else {
            (reads, None)
        };
        match queue.try_send(Execution { request, reader, publish_slot }) {
            Ok(()) => {}
            Err(mpsc::TrySendError::Full(mut job) | mpsc::TrySendError::Disconnected(mut job)) => {
                let _ = send_response(&mut job.reader, Err(Failure::new("503 Service Unavailable", "request capacity is full")));
            }
        }
    }

    fn reserve_publication(&self) -> Result<PublishSlot, Failure> {
        self.publish_slot
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map(|_| PublishSlot(Arc::clone(&self.publish_slot)))
            .map_err(|_| Failure::new("503 Service Unavailable", "another publication is in progress"))
    }

    fn admit(&self, reader: &mut impl BufRead) -> Result<Admitted, Failure> {
        let head = read_head(reader)?;
        let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS];
        let mut parsed = httparse::Request::new(&mut headers);
        if parsed.parse(&head) != Ok(httparse::Status::Complete(head.len())) || parsed.version != Some(1) {
            return Err(Failure::bad("expected one valid HTTP/1.1 request"));
        }
        let host = one_header(parsed.headers, "Host")?.filter(|host| !host.is_empty())
            .ok_or_else(|| Failure::bad("missing Host header"))?;
        if self.tls.is_none() && !loopback_host(host, self.bind) {
            return Err(Failure::bad("Host does not match the loopback listener"));
        }
        let endpoint = parsed.path.ok_or_else(|| Failure::bad("missing request target"))
            .and_then(Endpoint::parse);
        let is_publish = matches!(endpoint, Ok(Endpoint::Publish));
        let token = if is_publish {
            Some(self.write_token.as_ref().ok_or_else(|| Failure::new("404 Not Found", "unknown API path"))?)
        } else {
            self.token.as_ref()
        };
        if let Some(token) = token {
            let provided = one_header(parsed.headers, "Authorization")?;
            if !provided.is_some_and(|header| bearer_matches(header, token)) {
                return Err(Failure {
                    status: "401 Unauthorized",
                    message: "a valid bearer token is required".into(),
                    extra: "WWW-Authenticate: Bearer\r\n",
                });
            }
        }
        let endpoint = endpoint?;
        let method = parsed.method.unwrap_or_default();
        if method != endpoint.method() {
            return Err(Failure {
                status: "405 Method Not Allowed",
                message: "this method is not supported on the requested path".into(),
                extra: if endpoint.method() == "GET" { "Allow: GET\r\n" } else { "Allow: POST\r\n" },
            });
        }
        if one_header(parsed.headers, "Transfer-Encoding")?.is_some() {
            return Err(Failure::bad("transfer encoding is not supported; use Content-Length"));
        }
        let len = one_header(parsed.headers, "Content-Length")?.map(parse_length).transpose()?;
        let limit = if is_publish {
            METADATA_PREFIX_BYTES as u64 + MAX_PUBLISH_METADATA_BYTES as u64 + MAX_ARTIFACT_BYTES
        } else {
            MAX_QUERY_BYTES as u64
        };
        if len.is_some_and(|len| len as u64 > limit) {
            return Err(Failure::new("413 Content Too Large", "request body exceeds the size limit"));
        }
        if one_header(parsed.headers, "Expect")?.is_some() {
            return Err(Failure::new("417 Expectation Failed", "Expect is not supported"));
        }
        if method == "GET" && len.unwrap_or_default() != 0 {
            return Err(Failure::bad("GET requests may not carry a body"));
        }
        let len = match endpoint {
            Endpoint::Publish => {
                let len = len.ok_or_else(|| Failure::new("411 Length Required", "publish requires Content-Length"))?;
                if !one_header(parsed.headers, "Content-Type")?
                    .is_some_and(|value| value.eq_ignore_ascii_case(PUBLISH_CONTENT_TYPE.as_bytes()))
                {
                    return Err(Failure::new("415 Unsupported Media Type", "publish requires application/vnd.kuna.bsim.publish"));
                }
                len
            }
            Endpoint::Query => {
                let len = len.ok_or_else(|| Failure::new("411 Length Required", "query requires Content-Length"))?;
                let is_json = one_header(parsed.headers, "Content-Type")?
                    .and_then(|v| std::str::from_utf8(v).ok())
                    .and_then(|v| v.split(';').next())
                    .is_some_and(|v| v.trim().eq_ignore_ascii_case("application/json"));
                if !is_json {
                    return Err(Failure::new("415 Unsupported Media Type", "query requires application/json"));
                }
                len
            }
            _ => 0,
        };
        Ok(Admitted { endpoint, len })
    }

    fn execute(&self, reader: &mut impl BufRead, request: Admitted) -> Result<Response, Failure> {
        let Admitted { endpoint, len } = request;
        let deadline = Some(Instant::now() + QUERY_TIMEOUT);
        match endpoint {
            Endpoint::Profiles => Response::json(&self.store.profiles_before(deadline)?),
            Endpoint::Publish => self.publish(reader, len),
            Endpoint::Query => {
                let mut body = vec![0; len];
                reader.read_exact(&mut body).map_err(Failure::request_io)?;
                let request = serde_json::from_slice::<QueryRequest>(&body)
                    .map_err(|_| Failure::bad("invalid query JSON"))?;
                let deadline = Some(Instant::now() + QUERY_TIMEOUT);
                let response = self.store.query_before(&request, deadline)?;
                Ok(Response::json_bytes(query_json::encode(&response).map_err(Failure::internal)?))
            }
            Endpoint::Function { profile, sha256, address } => {
                Response::json(&self.store.function_before(&profile, &sha256, address, deadline)?)
            }
            Endpoint::Binary(sha256) => {
                let (file, len) = self.store.open_binary(&sha256)?;
                Ok(Response {
                    status: "200 OK",
                    content_type: "application/octet-stream",
                    extra: "Content-Disposition: attachment\r\n",
                    body: Body::Binary(file, len),
                })
            }
        }
    }

    fn publish(&self, reader: &mut impl BufRead, len: usize) -> Result<Response, Failure> {
        let after_prefix = len.checked_sub(METADATA_PREFIX_BYTES)
            .ok_or_else(|| Failure::bad("missing publication metadata length"))?;
        let mut prefix = [0; METADATA_PREFIX_BYTES];
        reader.read_exact(&mut prefix).map_err(Failure::request_io)?;
        let metadata_len = u32::from_be_bytes(prefix) as usize;
        if metadata_len > MAX_PUBLISH_METADATA_BYTES {
            return Err(Failure::new("413 Content Too Large", "publication metadata exceeds the size limit"));
        }
        if metadata_len == 0 {
            return Err(Failure::bad("missing publication metadata"));
        }
        let artifact_len = after_prefix.checked_sub(metadata_len).filter(|len| *len != 0)
            .ok_or_else(|| Failure::bad("publication must contain metadata and a nonempty executable"))?;
        if artifact_len as u64 > MAX_ARTIFACT_BYTES {
            return Err(Failure::new("413 Content Too Large", "executable exceeds the artifact size limit"));
        }
        let mut metadata = vec![0; metadata_len];
        reader.read_exact(&mut metadata).map_err(Failure::request_io)?;
        let request: PublishRequest = serde_json::from_slice(&metadata)
            .map_err(|_| Failure::bad("invalid publication JSON"))?;
        if request.address.is_some() == request.symbol.is_some() {
            return Err(Failure::bad("publication requires exactly one address or symbol"));
        }

        let mut upload = Publisher::stage(&metadata).map_err(Failure::internal)?;
        let mut remaining = artifact_len;
        while remaining != 0 {
            let chunk = reader.fill_buf().map_err(Failure::request_io)?;
            if chunk.is_empty() {
                return Err(Failure::bad("incomplete publication executable"));
            }
            let count = remaining.min(chunk.len());
            upload.image.write_all(&chunk[..count]).map_err(Failure::internal)?;
            reader.consume(count);
            remaining -= count;
        }
        let publisher = self.publisher.as_ref().ok_or_else(|| Failure::internal("no publication worker configured"))?;
        let (profile, executable) = publisher.sign(&upload).map_err(|error| match error {
            WorkerError::Rejected => Failure::new("422 Unprocessable Content", "could not load executable or sign the requested function"),
            WorkerError::TimedOut => Failure::new("503 Service Unavailable", "publication signer exceeded its deadline"),
            WorkerError::Unavailable(error) => {
                eprintln!("kuna-bsim-server: {error}");
                Failure::new("503 Service Unavailable", "publication worker is unavailable")
            }
        })?;
        let [signature] = executable.functions.as_slice() else {
            return Err(Failure::internal("publication signer did not return exactly one function"));
        };
        let address = signature.address;
        let name = signature.name.clone();

        // Read the same staged file, not a path supplied by the client. The
        // store checks its bytes against the signer's source MD5 before writing.
        let result = self.store.publish_file(profile, executable, &mut upload.image).map_err(|error| match error {
            StoreError::Invalid(reason) => Failure::new("422 Unprocessable Content", reason),
            StoreError::Deadline => Failure::new("503 Service Unavailable", "store is busy writing another executable"),
            other => Failure::from(other),
        })?;
        Response::json(&PublishResponse {
            sha256: result.sha256,
            address,
            name,
            profile: profile.name().into(),
            outcome: result.outcome,
        })
    }
}

struct Admitted {
    endpoint: Endpoint,
    len: usize,
}

enum Endpoint {
    Profiles,
    Query,
    Publish,
    Function { profile: Box<str>, sha256: Box<str>, address: u64 },
    Binary(Box<str>),
}

impl Endpoint {
    fn parse(path: &str) -> Result<Self, Failure> {
        if !path.starts_with('/') || path.contains(['?', '#']) {
            return Err(Failure::bad("expected an API path without a query or fragment"));
        }
        let mut segments = path.split('/').skip(1);
        if segments.next() != Some("v1") {
            return Err(Failure::new("404 Not Found", "unknown API path"));
        }
        match (segments.next(), segments.next(), segments.next(), segments.next(), segments.next()) {
            (Some("profiles"), None, None, None, None) => Ok(Self::Profiles),
            (Some("query"), None, None, None, None) => Ok(Self::Query),
            (Some("publish"), None, None, None, None) => Ok(Self::Publish),
            (Some("binaries"), Some(sha256), None, None, None) => Ok(Self::Binary(sha256.into())),
            (Some("functions"), Some(profile), Some(sha256), Some(hex), None) => {
                if !(1..=16).contains(&hex.len())
                    || !hex.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                {
                    return Err(Failure::bad("function entry must be lowercase hexadecimal without a prefix"));
                }
                let address = u64::from_str_radix(hex, 16).map_err(|_| Failure::bad("invalid function entry"))?;
                Ok(Self::Function { profile: profile.into(), sha256: sha256.into(), address })
            }
            _ => Err(Failure::new("404 Not Found", "unknown API path")),
        }
    }

    fn method(&self) -> &'static str {
        if matches!(self, Self::Query | Self::Publish) { "POST" } else { "GET" }
    }
}

fn read_head(reader: &mut impl BufRead) -> Result<Vec<u8>, Failure> {
    let mut head = Vec::new();
    let mut taken = reader.take(MAX_HEADER_BYTES as u64 + 1);
    loop {
        let start = head.len();
        let count = taken.read_until(b'\n', &mut head).map_err(Failure::request_io)?;
        if head.len() > MAX_HEADER_BYTES {
            return Err(Failure::new("431 Request Header Fields Too Large", "request headers exceed the size limit"));
        }
        if count == 0 || !head.ends_with(b"\r\n") {
            return Err(Failure::bad("incomplete or malformed HTTP request headers"));
        }
        if &head[start..] == b"\r\n" {
            return Ok(head);
        }
    }
}

fn one_header<'a>(headers: &[httparse::Header<'a>], name: &str) -> Result<Option<&'a [u8]>, Failure> {
    let mut found = headers.iter().filter(|h| h.name.eq_ignore_ascii_case(name));
    let first = found.next().map(|h| h.value);
    if found.next().is_some() {
        return Err(Failure::bad("duplicate request header"));
    }
    Ok(first)
}

fn parse_length(value: &[u8]) -> Result<usize, Failure> {
    if value.is_empty() || !value.iter().all(u8::is_ascii_digit) {
        return Err(Failure::bad("invalid Content-Length header"));
    }
    std::str::from_utf8(value).expect("ASCII digits").parse().map_err(|_| {
        Failure::new("413 Content Too Large", "request body exceeds the size limit")
    })
}

fn bearer_matches(header: &[u8], expected: &[u8; 32]) -> bool {
    let Some((scheme, token)) = header.split_at_checked(7) else { return false };
    scheme.eq_ignore_ascii_case(b"Bearer ")
        && bool::from(Sha256::digest(token).as_slice().ct_eq(expected.as_slice()))
}

fn loopback_host(value: &[u8], bind: SocketAddr) -> bool {
    let Ok(host) = std::str::from_utf8(value) else { return false };
    let (hostname, port) = if let Some(ip) = host.strip_prefix('[') {
        let Some((ip, suffix)) = ip.split_once(']') else { return false };
        if !ip.contains(':') {
            return false;
        }
        let port = if suffix.is_empty() {
            None
        } else if let Some(port) = suffix.strip_prefix(':') {
            Some(port)
        } else {
            return false;
        };
        (ip, port)
    } else if let Some((host, port)) = host.split_once(':') {
        (host, Some(port))
    } else {
        (host, None)
    };
    let port = match port {
        None => Some(80),
        Some(value) if value.bytes().all(|b| b.is_ascii_digit()) => value.parse::<u16>().ok(),
        _ => None,
    };
    port == Some(bind.port())
        && (hostname.eq_ignore_ascii_case("localhost") || hostname.parse::<IpAddr>().ok() == Some(bind.ip()))
}

struct Failure {
    status: &'static str,
    message: Cow<'static, str>,
    extra: &'static str,
}

impl Failure {
    fn new(status: &'static str, message: impl Into<Cow<'static, str>>) -> Self {
        Self { status, message: message.into(), extra: "" }
    }

    fn bad(message: &'static str) -> Self {
        Self::new("400 Bad Request", message)
    }

    fn request_io(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => {
                Self::new("408 Request Timeout", "request deadline exceeded")
            }
            _ => Self::bad("could not read complete request"),
        }
    }

    fn internal(error: impl std::fmt::Display) -> Self {
        eprintln!("kuna-bsim-server: {error}");
        Self::new("500 Internal Server Error", "stored data is invalid or unavailable")
    }
}

impl From<StoreError> for Failure {
    fn from(error: StoreError) -> Self {
        match error {
            StoreError::Invalid(reason) => Self::new("400 Bad Request", reason),
            StoreError::NotFound(_) => Self::new("404 Not Found", error.to_string()),
            StoreError::Deadline => Self::new("503 Service Unavailable", "store query exceeded its deadline"),
            _ => Self::internal(error),
        }
    }
}

struct Response {
    status: &'static str,
    content_type: &'static str,
    extra: &'static str,
    body: Body,
}

enum Body {
    Json(Vec<u8>),
    Binary(File, u64),
}

impl Response {
    fn json(value: &impl Serialize) -> Result<Self, Failure> {
        Ok(Self::json_bytes(serde_json::to_vec(value).map_err(Failure::internal)?))
    }

    fn json_bytes(bytes: Vec<u8>) -> Self {
        Self {
            status: "200 OK",
            content_type: "application/json",
            extra: "",
            body: Body::Json(bytes),
        }
    }

    fn failure(failure: Failure) -> io::Result<Self> {
        Ok(Self {
            status: failure.status,
            content_type: "application/json",
            extra: failure.extra,
            body: Body::Json(serde_json::to_vec(&ErrorResponse { error: failure.message.into_owned() })?),
        })
    }

    fn send(self, conn: &mut impl Write) -> io::Result<()> {
        let len = match &self.body {
            Body::Json(bytes) => bytes.len() as u64,
            Body::Binary(_, len) => *len,
        };
        // Coalesce the small formatted headers into one write (and a single
        // TLS application-data record when TLS is configured).
        let header = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {len}\r\n\
             Connection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n{}\r\n",
            self.status, self.content_type, self.extra,
        );
        conn.write_all(header.as_bytes())?;
        match self.body {
            Body::Json(bytes) => conn.write_all(&bytes),
            Body::Binary(file, len) => {
                if io::copy(&mut file.take(len), conn)? != len {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "binary was truncated during download"));
                }
                Ok(())
            }
        }
    }
}

struct DeadlineSocket {
    stream: TcpStream,
    until: Instant,
}

impl DeadlineSocket {
    fn remaining(&self) -> io::Result<Duration> {
        self.until.checked_duration_since(Instant::now()).filter(|d| !d.is_zero())
            .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "socket deadline exceeded"))
    }
}

impl Read for DeadlineSocket {
    fn read(&mut self, dest: &mut [u8]) -> io::Result<usize> {
        self.stream.set_read_timeout(Some(self.remaining()?))?;
        self.stream.read(dest)
    }
}

impl Write for DeadlineSocket {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.stream.set_write_timeout(Some(self.remaining()?))?;
        self.stream.write(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

enum Connection {
    Plain(DeadlineSocket),
    Tls(Box<StreamOwned<ServerConnection, DeadlineSocket>>),
}

impl Connection {
    fn socket_mut(&mut self) -> &mut DeadlineSocket {
        match self {
            Self::Plain(socket) => socket,
            Self::Tls(tls) => &mut tls.sock,
        }
    }
}

impl Read for Connection {
    fn read(&mut self, dest: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Plain(socket) => socket.read(dest),
            Self::Tls(tls) => tls.read(dest),
        }
    }
}

impl Write for Connection {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match self {
            Self::Plain(socket) => socket.write(data),
            Self::Tls(tls) => tls.write(data),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Plain(socket) => socket.flush(),
            Self::Tls(tls) => tls.flush(),
        }
    }
}

#[cfg(test)]
mod tests;
