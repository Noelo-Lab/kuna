//! Client for Kuna's versioned BSim API. Match IDs never carry a URL.

use std::fmt;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::time::Duration;

use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_bsim_server::{
    ErrorResponse, ProfilesResponse, PublishRequest, PublishResponse, QueryRequest, QueryResponse,
    StoredFunction, MAX_ARTIFACT_BYTES, MAX_PUBLISH_METADATA_BYTES, MAX_RESPONSE_BYTES,
    PUBLISH_CONTENT_TYPE,
};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use ureq::tls::{Certificate, RootCerts, TlsConfig};
use url::{Host, Url};

// Includes the server's 10-second admission, 120-second upload, and
// 120-second signing deadlines, with time for the store acknowledgement.
const PUBLISH_TIMEOUT: Duration = Duration::from_secs(270);

#[derive(Clone, PartialEq, Eq)]
pub(super) struct MatchId<'a> {
    pub profile: LshProfile,
    pub sha256: &'a str,
    pub address: u64,
}

impl<'a> MatchId<'a> {
    pub fn parse(value: &'a str) -> Result<Self, String> {
        let invalid = || "expected a Kuna match ID PROFILE:SHA256:HEX_ADDRESS".to_owned();
        let (profile, rest) = value.split_once(':').ok_or_else(invalid)?;
        let (sha256, address) = rest.split_once(':').ok_or_else(invalid)?;
        let profile = LshProfile::from_name(profile).ok_or_else(invalid)?;
        if sha256.len() != 64 || !lower_hex(sha256) || address.is_empty() || !lower_hex(address) {
            return Err(invalid());
        }
        let address = u64::from_str_radix(address, 16).map_err(|_| invalid())?;
        Ok(Self { profile, sha256, address })
    }
}

impl fmt::Display for MatchId<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{:x}", self.profile.name(), self.sha256, self.address)
    }
}

fn lower_hex(value: &str) -> bool {
    value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub(super) struct Client {
    origin: Url,
    agent: ureq::Agent,
    authorization: Option<String>,
}

impl Client {
    pub fn new(origin: &str) -> Result<Self, String> {
        Self::connect(origin, authorization("KUNA_BSIM_SERVER_TOKEN")?)
    }

    pub fn writer(origin: &str) -> Result<Self, String> {
        let token = authorization("KUNA_BSIM_SERVER_WRITE_TOKEN")?
            .ok_or("set KUNA_BSIM_SERVER_WRITE_TOKEN to publish a function")?;
        Self::connect(origin, Some(token))
    }

    fn connect(origin: &str, authorization: Option<String>) -> Result<Self, String> {
        let origin = Url::parse(origin).map_err(|_| "invalid Kuna BSim server URL")?;
        let loopback = match origin.host() {
            Some(Host::Ipv4(ip)) => ip.is_loopback(),
            Some(Host::Ipv6(ip)) => ip.is_loopback(),
            _ => false,
        };
        if origin.host().is_none() || !origin.username().is_empty() || origin.password().is_some()
            || origin.path() != "/" || origin.query().is_some() || origin.fragment().is_some()
            || !(origin.scheme() == "https" || (origin.scheme() == "http" && loopback))
        {
            return Err("Kuna BSim requires a credential-free HTTPS origin or a literal loopback HTTP origin".into());
        }
        let mut config = ureq::Agent::config_builder()
            .proxy(None)
            .max_redirects(0)
            .timeout_global(Some(Duration::from_secs(120)));
        if origin.scheme() == "https" {
            let roots = rustls_native_certs::load_native_certs();
            if roots.certs.is_empty() {
                return Err("could not load TLS trust roots for Kuna BSim".into());
            }
            let tls = TlsConfig::builder().root_certs(RootCerts::from(
                roots.certs.iter().map(|cert| Certificate::from_der(cert.as_ref()).to_owned()),
            )).build();
            config = config.tls_config(tls);
        }
        Ok(Self { origin, agent: ureq::Agent::new_with_config(config.build()), authorization })
    }

    pub fn profiles(&self) -> Result<ProfilesResponse, String> {
        read_json(self.get("v1/profiles")?)
    }

    pub fn query(&self, query: &QueryRequest) -> Result<QueryResponse, String> {
        let url = self.url("v1/query");
        let mut request = self.agent.post(url.as_str());
        if let Some(auth) = &self.authorization {
            request = request.header("authorization", auth.as_str());
        }
        read_json(check_status(request.send_json(query))?)
    }

    pub fn publish(&self, binary: &Path, entry: &PublishRequest) -> Result<PublishResponse, String> {
        let io_error = |e| format!("read {}: {e}", binary.display());
        if !fs::metadata(binary).map_err(io_error)?.is_file() {
            return Err("publish requires a regular executable or object file".into());
        }
        let file = File::open(binary).map_err(io_error)?;
        let metadata = file.metadata().map_err(io_error)?;
        if !metadata.is_file() || !(1..=MAX_ARTIFACT_BYTES).contains(&metadata.len()) {
            return Err(format!("publish requires a regular nonempty file of at most {MAX_ARTIFACT_BYTES} bytes"));
        }
        let metadata_len = serde_json::to_vec(entry).map_err(|e| e.to_string())?;
        if metadata_len.len() > MAX_PUBLISH_METADATA_BYTES {
            return Err("publish metadata exceeds the server limit".into());
        }
        let header = (metadata_len.len() as u32).to_be_bytes();
        let len = 4 + metadata_len.len() as u64 + metadata.len();
        let mut input = HashingReader { file: file.take(metadata.len()), digest: Sha256::new() };
        let url = self.url("v1/publish");
        let auth = self.authorization.as_deref().ok_or("publish requires a writer token")?;
        let response = {
            let mut body = header.as_slice().chain(metadata_len.as_slice()).chain(&mut input);
            self.agent.post(url.as_str())
                .config().timeout_global(Some(PUBLISH_TIMEOUT)).http_status_as_error(false).build()
                .header("authorization", auth)
                .header("content-length", len.to_string())
                .content_type(PUBLISH_CONTENT_TYPE)
                .send(ureq::SendBody::from_reader(&mut body))
        };
        let sha256 = (input.file.limit() == 0).then(|| kuna_base::hex_lower(&input.digest.finalize()));
        let unknown = |detail: &str| match &sha256 {
            Some(digest) => format!("publication outcome unknown for SHA-256 {digest}: {detail}"),
            None => format!("publication outcome unknown: {detail}"),
        };
        let mut response = response.map_err(|e| unknown(&e.to_string()))?;
        if !response.status().is_success() {
            let status = response.status();
            let detail = response.body_mut().with_config().limit(MAX_PUBLISH_METADATA_BYTES as u64)
                .read_json::<ErrorResponse>().ok();
            let error = match detail {
                Some(response) => format!("Kuna BSim publish HTTP {status}: {:?}", response.error),
                None => format!("Kuna BSim publish HTTP {status}"),
            };
            return Err(if status.is_server_error() { unknown(&error) } else { error });
        }
        let published: PublishResponse = read_json(response).map_err(|error| unknown(&error))?;
        if sha256.as_deref() != Some(&published.sha256)
            || entry.address.is_some_and(|address| address != published.address)
            || entry.rename.as_ref().is_some_and(|name| name != &published.name)
            || entry.profile.as_ref().is_some_and(|profile| profile != &published.profile)
        {
            return Err(unknown("Kuna BSim server returned a different publication identity"));
        }
        Ok(published)
    }

    pub fn function(&self, id: &MatchId<'_>) -> Result<StoredFunction, String> {
        let function: StoredFunction = read_json(self.get(&format!(
            "v1/functions/{}/{}/{:x}", id.profile.name(), id.sha256, id.address,
        ))?)?;
        if function.sha256 != id.sha256 || function.address != id.address {
            return Err("Kuna BSim server returned a different match identity".into());
        }
        Ok(function)
    }

    pub fn download(&self, id: &MatchId<'_>) -> Result<Vec<u8>, String> {
        let mut response = self.get(&format!("v1/binaries/{}", id.sha256))?;
        let bytes = response.body_mut().with_config().limit(MAX_ARTIFACT_BYTES).read_to_vec()
            .map_err(|error| format!("read reference download: {error}"))?;
        if kuna_base::hex_lower(&Sha256::digest(&bytes)) != id.sha256 {
            return Err("Kuna BSim reference SHA-256 mismatch".into());
        }
        Ok(bytes)
    }

    fn get(&self, path: &str) -> Result<ureq::http::Response<ureq::Body>, String> {
        let url = self.url(path);
        let mut request = self.agent.get(url.as_str());
        if let Some(auth) = &self.authorization {
            request = request.header("authorization", auth.as_str());
        }
        check_status(request.call())
    }

    fn url(&self, path: &str) -> Url {
        let mut target = self.origin.clone();
        target.set_path(path);
        target
    }
}

fn authorization(var: &str) -> Result<Option<String>, String> {
    match std::env::var(var) {
        Ok(token) if !token.is_empty() && token.len() <= 4096
            && token.bytes().all(|c| c.is_ascii_graphic()) => Ok(Some(format!("Bearer {token}"))),
        Ok(_) | Err(std::env::VarError::NotUnicode(_)) => Err(format!("invalid {var}")),
        Err(std::env::VarError::NotPresent) => Ok(None),
    }
}

fn check_status(response: Result<ureq::http::Response<ureq::Body>, ureq::Error>)
    -> Result<ureq::http::Response<ureq::Body>, String>
{
    let response = response.map_err(|err| format!("Kuna BSim server: {err}"))?;
    if response.status() != ureq::http::StatusCode::OK {
        return Err(format!("unexpected Kuna BSim server HTTP status {}", response.status()));
    }
    Ok(response)
}

fn read_json<T: DeserializeOwned>(mut response: ureq::http::Response<ureq::Body>) -> Result<T, String> {
    response.body_mut().with_config().limit(MAX_RESPONSE_BYTES as u64).read_json()
        .map_err(|err| format!("invalid Kuna BSim server response: {err}"))
}

struct HashingReader {
    file: io::Take<File>,
    digest: Sha256,
}

impl Read for HashingReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.file.read(buf)?;
        self.digest.update(&buf[..len]);
        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::thread;

    use super::*;

    #[test]
    fn different_acknowledged_digest_does_not_report_a_successful_push() {
        let mut executable = tempfile::NamedTempFile::new().unwrap();
        let bytes = b"immutable publication bytes";
        executable.write_all(bytes).unwrap();
        let expected = kuna_base::hex_lower(&Sha256::digest(bytes));
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let client = Client::connect(&origin, Some("Bearer test".to_owned())).unwrap();
        let peer = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut request = BufReader::new(&mut stream);
            let mut line = String::new();
            let mut length = None;
            loop {
                line.clear();
                assert_ne!(request.read_line(&mut line).unwrap(), 0);
                if line == "\r\n" {
                    break;
                }
                if let Some((name, value)) = line.split_once(':') {
                    if name.eq_ignore_ascii_case("content-length") {
                        length = Some(value.trim().parse::<u64>().unwrap());
                    }
                }
            }
            let length = length.unwrap();
            assert_eq!(io::copy(&mut request.take(length), &mut io::sink()).unwrap(), length);
            let result = serde_json::json!({
                "sha256": "0".repeat(64), "address": 16, "name": "saved",
                "profile": "64", "outcome": "inserted",
            }).to_string();
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{result}",
                result.len()).unwrap();
        });
        let result = client.publish(executable.path(), &PublishRequest {
            executable: "app".to_owned(), address: Some(16), symbol: None,
            rename: Some("saved".to_owned()), profile: Some("64".to_owned()),
        });
        let error = result.unwrap_err();
        assert!(error.contains(&expected) && error.contains("different publication identity"), "{error}");
        peer.join().unwrap();
    }
}
