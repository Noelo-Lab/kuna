mod http;
mod import_binary;
mod import_ghidra;
mod import_sign;
mod publication_worker;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::net::{SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kuna_bsim_server::Store;
use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use sha2::{Digest, Sha256};

const TOKEN_ENV: &str = "KUNA_BSIM_SERVER_TOKEN";
const WRITE_TOKEN_ENV: &str = "KUNA_BSIM_SERVER_WRITE_TOKEN";
const MAX_PEM_BYTES: u64 = 1024 * 1024;
const USAGE: &str = "Usage: kuna-bsim-server [serve] --store PATH --bind NUMERIC_ADDRESS:PORT \
                   [--tls-cert PEM --tls-key PEM]\n\
                   Non-loopback listeners require TLS and KUNA_BSIM_SERVER_TOKEN.\n\
                   If set, KUNA_BSIM_SERVER_TOKEN is required on read requests.\n\
                   Publication requires a separate KUNA_BSIM_SERVER_WRITE_TOKEN.\n\
                   Local imports: kuna-bsim-server import --help or import-ghidra --help";
type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    let args: Vec<OsString> = env::args_os().skip(1).collect();
    if args.first().is_some_and(|arg| arg == publication_worker::COMMAND) {
        std::process::exit(publication_worker::run(&args[1..]));
    }
    if args.first().is_some_and(|arg| arg == "import") {
        std::process::exit(import_binary::run(&args[1..]));
    }
    if args.first().is_some_and(|arg| arg == "import-ghidra") {
        std::process::exit(import_ghidra::run(&args[1..]));
    }
    if let Err(error) = serve(&args) {
        eprintln!("kuna-bsim-server: {error}");
        std::process::exit(1);
    }
}

fn serve(mut args: &[OsString]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "serve") {
        args = &args[1..];
    }
    if matches!(args, [help] if help == "--help" || help == "-h") {
        println!("{USAGE}");
        return Ok(());
    }
    let options = BindOptions::parse(args)?;
    let token = configured_token(TOKEN_ENV)?;
    let write_token = configured_token(WRITE_TOKEN_ENV)?;
    require_distinct_tokens(token, write_token)?;
    let tls = listener_security(options.bind, options.cert.as_deref(), options.key.as_deref(), token.is_some())?;
    let store = Store::open(options.store)?;
    let listener = TcpListener::bind(options.bind)?;
    let local_addr = listener.local_addr()?;
    http::run(listener, local_addr, store, tls, token, write_token)?;
    Ok(())
}

struct BindOptions {
    store: PathBuf,
    bind: SocketAddr,
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
}

impl BindOptions {
    fn parse(args: &[OsString]) -> Result<Self> {
        let mut store = None;
        let mut bind = None;
        let mut cert = None;
        let mut key = None;
        let mut args = args.iter();
        while let Some(flag) = args.next() {
            let (name, slot) = match flag.to_str() {
                Some("--store") => ("--store", &mut store),
                Some("--bind") => ("--bind", &mut bind),
                Some("--tls-cert") => ("--tls-cert", &mut cert),
                Some("--tls-key") => ("--tls-key", &mut key),
                _ => return Err(format!("unknown option.\n{USAGE}").into()),
            };
            let value = args.next().ok_or_else(|| format!("{name} requires a value"))?;
            if slot.replace(value).is_some() {
                return Err(format!("{name} may be supplied only once").into());
            }
        }
        let store = store.ok_or_else(|| format!("--store is required.\n{USAGE}"))?;
        let bind = bind
            .ok_or_else(|| format!("--bind is required.\n{USAGE}"))?
            .to_str()
            .and_then(|endpoint| endpoint.parse().ok())
            .ok_or("--bind must be a numeric IP address and port (bracket IPv6 addresses)")?;
        Ok(Self { store: store.into(), bind, cert: cert.map(PathBuf::from), key: key.map(PathBuf::from) })
    }
}

fn listener_security(
    bind: SocketAddr,
    cert: Option<&Path>,
    key: Option<&Path>,
    has_token: bool,
) -> Result<Option<Arc<ServerConfig>>> {
    if cert.is_some() != key.is_some() {
        return Err("--tls-cert and --tls-key must be supplied together".into());
    }
    if !bind.ip().is_loopback() && (cert.is_none() || !has_token) {
        return Err(format!("non-loopback bind requires --tls-cert, --tls-key and {TOKEN_ENV}").into());
    }
    let (Some(cert), Some(key)) = (cert, key) else { return Ok(None) };
    let certs = CertificateDer::pem_reader_iter(pem_file(cert)?.take(MAX_PEM_BYTES))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let key = PrivateKeyDer::from_pem_reader(pem_file(key)?.take(MAX_PEM_BYTES))?;
    let mut config = ServerConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    config.alpn_protocols.push(b"http/1.1".to_vec());
    Ok(Some(Arc::new(config)))
}

fn pem_file(path: &Path) -> Result<File> {
    let metadata = path.metadata()?;
    if !metadata.is_file() || metadata.len() > MAX_PEM_BYTES {
        return Err("TLS PEM must be a regular file of at most 1 MiB".into());
    }
    Ok(File::open(path)?)
}

fn configured_token(name: &str) -> Result<Option<[u8; 32]>> {
    let value = match env::var(name) {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => return Ok(None),
        Err(env::VarError::NotUnicode(_)) => return Err(format!("{name} is not a valid token").into()),
    };
    if value.is_empty() || value.len() > 4096 || !value.bytes().all(|b| b.is_ascii_graphic()) {
        return Err(format!("{name} must be a nonempty visible ASCII token of at most 4096 bytes").into());
    }
    Ok(Some(Sha256::digest(value.as_bytes()).into()))
}

fn require_distinct_tokens(read: Option<[u8; 32]>, write: Option<[u8; 32]>) -> Result<()> {
    if write.is_some() && read == write {
        return Err(format!("{WRITE_TOKEN_ENV} must differ from {TOKEN_ENV}").into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_binding_fails_closed() {
        let remote = "0.0.0.0:0".parse().unwrap();
        assert!(listener_security(remote, None, None, false).is_err());
        assert!(listener_security(remote, None, None, true).is_err());
        assert!(listener_security(remote, Some(Path::new("cert")), Some(Path::new("key")), false).is_err());
        let local = "[::1]:0".parse().unwrap();
        assert!(listener_security(local, None, None, false).unwrap().is_none());
        assert!(listener_security(local, Some(Path::new("cert")), None, true).is_err());
    }

    #[test]
    fn publication_credentials_cannot_reuse_the_read_token() {
        let read = Some([1; 32]);
        assert!(require_distinct_tokens(read, read).is_err());
        assert!(require_distinct_tokens(read, Some([2; 32])).is_ok());
        assert!(require_distinct_tokens(read, None).is_ok());
        assert!(require_distinct_tokens(None, read).is_ok());
        assert!(require_distinct_tokens(None, None).is_ok());
    }
}
