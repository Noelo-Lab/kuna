//! Read-only client for Ghidra's native PostgreSQL BSim layout.

use std::fmt::Write as _;
use std::time::Duration;

use kuna_analysis::analyzers::bsim::signature::settings_are_valid;
use serde::Serialize;
use tokio::runtime::{Builder, Runtime};
use tokio_postgres::config::SslMode;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Config, Error, IsolationLevel, NoTls, Row};
use tokio_postgres_rustls::MakeRustlsConnect;

const LAYOUT: i32 = 6;
const STARTUP_TIMEOUT: Duration = Duration::from_secs(10);

const MATCH_COLUMNS: &str = "d.id, d.name_func, d.addr, e.name_exec, e.md5, a.val, c.val, r.val, p.val";
const MATCH_JOINS: &str = "
    JOIN public.exetable e ON e.id = d.id_exe
    LEFT JOIN public.archtable a ON a.id = e.architecture
    LEFT JOIN public.comptable c ON c.id = e.name_compiler
    LEFT JOIN public.repotable r ON r.id = e.repository
    LEFT JOIN public.pathtable p ON p.id = e.path";

#[derive(Serialize)]
pub(super) struct RemoteMatch {
    pub(super) match_id: String,
    name: String,
    pub(super) address: String,
    executable: String,
    pub(super) executable_md5: String,
    architecture: Option<String>,
    compiler: Option<String>,
    repository_url: Option<String>,
}

#[derive(Serialize)]
pub(super) struct ScoredMatch {
    #[serde(flatten)]
    matched: RemoteMatch,
    similarity: f64,
    significance: f64,
}

pub(super) struct Client {
    inner: tokio_postgres::Client,
    runtime: Runtime,
}

pub(super) struct Transaction<'a> {
    inner: tokio_postgres::Transaction<'a>,
    runtime: &'a Runtime,
}

impl Transaction<'_> {
    fn query_one(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, Error> {
        self.runtime.block_on(self.inner.query_one(sql, params))
    }

    fn query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
        self.runtime.block_on(self.inner.query(sql, params))
    }

    fn query_opt(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Option<Row>, Error> {
        self.runtime.block_on(self.inner.query_opt(sql, params))
    }
}

pub(super) fn connect() -> Result<Client, String> {
    let spec = std::env::var("KUNA_BSIM_POSTGRES")
        .map_err(|_| "set KUNA_BSIM_POSTGRES to a PostgreSQL connection string".to_owned())?;
    let config = spec.parse::<Config>()
        .map_err(|_| "invalid KUNA_BSIM_POSTGRES connection string".to_owned())?;
    start_connection(config, STARTUP_TIMEOUT)
}

fn start_connection(mut config: Config, timeout: Duration) -> Result<Client, String> {
    if config.get_hosts().is_empty() {
        return Err("KUNA_BSIM_POSTGRES must specify a host or Unix socket directory".into());
    }
    config.connect_timeout(timeout);
    #[cfg(unix)]
    let sockets_only = config.get_hosts().iter()
        .all(|host| matches!(host, tokio_postgres::config::Host::Unix(_)));
    #[cfg(not(unix))]
    let sockets_only = false;
    let plain_socket =
        sockets_only && config.get_hostaddrs().is_empty() && config.get_ssl_mode() != SslMode::Require;
    if !plain_socket && config.get_ssl_mode() == SslMode::Disable {
        return Err("PostgreSQL TCP connections require verified TLS".into());
    }

    let runtime = Builder::new_current_thread().enable_all().build()
        .map_err(|_| "could not initialize PostgreSQL connection runtime".to_owned())?;
    let result = runtime.block_on(async {
        tokio::time::timeout(timeout, async {
            if plain_socket {
                let (client, connection) = config.ssl_mode(SslMode::Disable).connect(NoTls)
                    .await.map_err(connection_error)?;
                tokio::spawn(connection);
                Ok(client)
            } else {
                let tls = tokio::task::spawn_blocking(|| {
                    MakeRustlsConnect::with_native_certs().map(|(tls, _)| tls).ok()
                }).await.ok().flatten()
                    .ok_or_else(|| "could not load PostgreSQL TLS trust roots".to_owned())?;
                let (client, connection) = config.ssl_mode(SslMode::Require).connect(tls)
                    .await.map_err(connection_error)?;
                tokio::spawn(connection);
                Ok(client)
            }
        }).await
    });
    let message = match result {
        Ok(Ok(inner)) => return Ok(Client { inner, runtime }),
        Ok(Err(message)) => message,
        Err(_) => "PostgreSQL connection startup timed out".into(),
    };
    // The OS resolver and native root loader are blocking jobs. Closing
    // the runtime normally could wait for them past the deadline.
    runtime.shutdown_background();
    Err(message)
}

fn connection_error(error: Error) -> String {
    // Do not echo the connection string or server-supplied authentication messages.
    if let Some(code) = error.code() {
        return format!("PostgreSQL connection failed (SQLSTATE {})", code.code());
    }
    "PostgreSQL connection failed (check endpoint, credentials and TLS trust)".into()
}

fn failure(operation: &str, error: &Error) -> String {
    match error.code() {
        Some(code) => format!("PostgreSQL {operation} failed (SQLSTATE {})", code.code()),
        None => format!("PostgreSQL {operation} failed: {error}"),
    }
}

pub(super) fn read_only(client: &mut Client) -> Result<Transaction<'_>, String> {
    let runtime = &client.runtime;
    let inner = runtime.block_on(async {
        let tx = client.inner.build_transaction().read_only(true)
            .isolation_level(IsolationLevel::RepeatableRead).start().await
            .map_err(|e| failure("read-only transaction", &e))?;
        // lsh_load uses unqualified names internally. Put pg_temp last so
        // PostgreSQL cannot prefer it implicitly for relations.
        tx.batch_execute("SET LOCAL search_path TO pg_catalog, public, pg_temp;
                          SET LOCAL statement_timeout TO '30s'").await
            .map_err(|e| failure("session setup", &e))?;
        Ok::<_, String>(tx)
    })?;
    Ok(Transaction { inner, runtime })
}

pub(super) fn settings(tx: &mut Transaction<'_>) -> Result<u32, String> {
    let meta = tx.query_one(
        "SELECT (SELECT value FROM public.keyvaluetable WHERE key = 'layout')::int,
                (SELECT value FROM public.keyvaluetable WHERE key = 'major')::int,
                (SELECT value FROM public.keyvaluetable WHERE key = 'minor')::int,
                (SELECT value FROM public.keyvaluetable WHERE key = 'settings')::int", &[],
    ).map_err(|e| failure("BSim metadata read", &e))?;
    let layout: Option<i32> = meta.get(0);
    if layout != Some(LAYOUT) {
        return Err(format!("unsupported PostgreSQL BSim layout {layout:?}; expected {LAYOUT}"));
    }
    let major: Option<i32> = meta.get(1);
    let expected = kuna_decomp::capability::MAJOR_VERSION as i32;
    if major != Some(expected) {
        return Err(format!("BSim signature major version {major:?} does not match Kuna's {expected}"));
    }
    let minor: Option<i32> = meta.get(2);
    let expected = kuna_decomp::capability::MINOR_VERSION;
    if !minor.is_some_and(|v| v >= 0 && (v as u32).abs_diff(expected) <= 1) {
        return Err(format!("BSim signature minor version {minor:?} is incompatible with Kuna's {expected}"));
    }
    let settings = meta.get::<_, Option<i32>>(3).unwrap_or(0) as u32;
    if !settings_are_valid(settings) {
        return Err(format!("database has unsupported BSim signature settings {settings:#x}"));
    }
    Ok(settings)
}

fn encode_vector(mut features: &[u32]) -> String {
    // HashEntry.saveSQL stores hexadecimal term count and hash, in unsigned hash order.
    let mut out = String::with_capacity(features.len() * 11 + 2);
    out.push('(');
    while let Some(&hash) = features.first() {
        let count = features.partition_point(|&value| value == hash);
        if out.len() > 1 {
            out.push(',');
        }
        let _ = write!(out, "{:x}:{hash:x}", count.min(64));
        features = &features[count..];
    }
    out.push(')');
    out
}

fn validate_loader(tx: &Transaction<'_>) -> Result<(), String> {
    // The stock extension counts these names in all of pg_class, not just
    // along search_path. A duplicate in another schema also causes fallback.
    // SPI_getvalue reads the first two columns of the IDF and weight tables.
    let relations = tx.query(
        "SELECT c.relname::text, n.nspname = 'public' AND c.relkind IN ('r', 'p'),
                CASE c.relname
                    WHEN 'idflookup' THEN col.names = ARRAY['hash', 'lookup']
                        AND col.types[1] IN ('bigint'::regtype::oid, 'integer'::regtype::oid)
                        AND col.types[2] = 'integer'::regtype::oid
                    WHEN 'weighttable' THEN col.names = ARRAY['id', 'weight']
                        AND col.types[1] = 'integer'::regtype::oid
                        AND col.types[2] IN ('numeric'::regtype::oid, 'double precision'::regtype::oid)
                END
         FROM pg_catalog.pg_class c JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace
         LEFT JOIN LATERAL (
             SELECT array_agg(a.attname::text ORDER BY a.attnum) AS names,
                    array_agg(a.atttypid ORDER BY a.attnum) AS types
             FROM (SELECT attname, atttypid, attnum FROM pg_catalog.pg_attribute
                   WHERE attrelid = c.oid AND attnum > 0 AND NOT attisdropped
                   ORDER BY attnum LIMIT 2) a
         ) col ON c.relname <> 'keyvaluetable'
         WHERE c.relname IN ('idflookup', 'keyvaluetable', 'weighttable')
         ORDER BY c.relname::text COLLATE \"C\" LIMIT 4", &[],
    ).map_err(|e| failure("BSim loader namespace check", &e))?;
    let names = ["idflookup", "keyvaluetable", "weighttable"];
    if relations.len() != names.len() || relations.iter().zip(names).any(|(row, name)| {
        row.get::<_, &str>(0) != name || !row.get::<_, bool>(1)
    }) {
        return Err("invalid BSim loader namespace: expected one public table each for \
                    idflookup, keyvaluetable and weighttable, with no duplicate relation names".into());
    }

    if relations[0].get::<_, Option<bool>>(2) != Some(true) {
        return Err("invalid BSim IDF lookup table: expected hash and lookup as its first two columns".into());
    }
    if relations[2].get::<_, Option<bool>>(2) != Some(true) {
        return Err("invalid BSim weight table: expected id and weight as its first two columns".into());
    }

    // Match the extension's row-count and binning bounds. IDF indices must fit
    // its 512-entry weight array, and signed and unsigned hash aliases collide
    // in the extension's uint32 table.
    let config = tx.query_one(
        "SELECT (SELECT count(*) = 583 AND count(DISTINCT id) = 583
                        AND min(id) = 0 AND max(id) = 582 AND count(weight) = 583
                        AND bool_and(weight::double precision > '-Infinity'::double precision
                                     AND weight::double precision < 'Infinity'::double precision
                                     AND (id <> 581 OR weight > 0))
                 FROM public.weighttable),
                (SELECT count(*) BETWEEN 2 AND 1048576
                        AND count(DISTINCT (hash::bigint & 4294967295::bigint)) = count(*)
                        AND bool_and(coalesce(hash BETWEEN -2147483648 AND 4294967295
                                              AND lookup BETWEEN 0 AND 511, false))
                 FROM public.idflookup),
                (SELECT CASE WHEN count(*) = 1 THEN min(value::text) END
                 FROM public.keyvaluetable WHERE key = 'k'),
                (SELECT CASE WHEN count(*) = 1 THEN min(value::text) END
                 FROM public.keyvaluetable WHERE key = 'L')", &[],
    ).map_err(|e| failure("BSim loader configuration check", &e))?;
    if config.get::<_, Option<bool>>(0) != Some(true) {
        return Err("invalid BSim weight table: expected 583 indexed finite weights and a positive scale".into());
    }
    if config.get::<_, Option<bool>>(1) != Some(true) {
        return Err("invalid BSim IDF lookup table: expected 2–1048576 distinct 32-bit hashes with lookups in 0–511".into());
    }
    for (col, key, max) in [(2, "k", 31), (3, "L", 1024)] {
        let valid = config.get::<_, Option<&str>>(col).is_some_and(|value| {
            value.bytes().all(|b| b.is_ascii_digit())
                && value.parse::<u32>().is_ok_and(|n| (1..=max).contains(&n))
        });
        if !valid {
            return Err(format!("invalid BSim LSH configuration: expected one decimal {key} value in 1–{max}"));
        }
    }
    Ok(())
}

pub(super) fn search(
    tx: &mut Transaction<'_>,
    features: &[u32],
    similarity: f64,
    limit: usize,
) -> Result<(Vec<ScoredMatch>, bool), String> {
    if features.is_empty() {
        return Err("the query function has no BSim features".into());
    }
    validate_loader(tx)?;
    // Ghidra initializes the extension's connection-local weights and LSH bins
    // before constructing any vectors. The '%' operator uses the native GIN index.
    tx.query_one("SELECT public.lsh_load()", &[]).map_err(|e| failure("lshvector load", &e))?;
    let vector = encode_vector(features);
    let sql = format!("
        WITH input AS (SELECT public.lshvector_in($1::text::cstring) AS vec),
        scored AS MATERIALIZED (
            SELECT v.id, public.lshvector_compare(input.vec, v.vec) AS score
            FROM input CROSS JOIN public.vectable v WHERE v.vec OPERATOR(public.%) input.vec
        ), nearest AS (
            SELECT id, (score).sim AS sim, (score).sig AS sig FROM scored
            WHERE (score).sim > $2 AND (score).sig > 0
            ORDER BY sim DESC, sig DESC, id LIMIT $3
        )
        SELECT {MATCH_COLUMNS}, n.sim, n.sig
        FROM nearest n JOIN public.desctable d ON d.id_signature = n.id {MATCH_JOINS}
        ORDER BY n.sim DESC, n.sig DESC, n.id, d.id LIMIT $3");
    // A vector can name several functions. Probe one extra at both levels to
    // detect truncation without counting the database or downloading vectors.
    let rows = tx.query(&sql, &[&vector, &similarity, &(limit as i64 + 1)])
        .map_err(|e| failure("BSim query", &e))?;
    let truncated = rows.len() > limit;
    let matches = rows.iter().take(limit).map(|row| Ok(ScoredMatch {
        matched: remote_match(row)?,
        similarity: row.get(9),
        significance: row.get(10),
    })).collect::<Result<_, String>>()?;
    Ok((matches, truncated))
}

pub(super) fn matched_function(tx: &mut Transaction<'_>, id: i64) -> Result<RemoteMatch, String> {
    let sql = format!("SELECT {MATCH_COLUMNS} FROM public.desctable d {MATCH_JOINS}
        WHERE d.id = $1 AND EXISTS (SELECT 1 FROM public.vectable v WHERE v.id = d.id_signature)");
    let row = tx.query_opt(&sql, &[&id]).map_err(|e| failure("BSim match lookup", &e))?
        .ok_or_else(|| "match ID not found in this PostgreSQL BSim database".to_owned())?;
    remote_match(&row)
}

fn remote_match(row: &Row) -> Result<RemoteMatch, String> {
    let get_text = |index| row.try_get::<_, String>(index).map_err(|e| failure("match row decode", &e));
    let executable = get_text(3)?;
    let executable_md5 = get_text(4)?;
    let repository = row.get::<_, Option<&str>>(7);
    let path = row.get::<_, Option<&str>>(8);
    let repository_url = repository
        .and_then(|repository| repository_url(repository, path, &executable))
        .map(String::from);
    Ok(RemoteMatch {
        match_id: row.get::<_, i64>(0).to_string(),
        name: get_text(1)?,
        address: format!("0x{:x}", row.get::<_, i64>(2) as u64),
        executable,
        executable_md5,
        architecture: row.get(5),
        compiler: row.get(6),
        repository_url,
    })
}

fn repository_url(repository: &str, path: Option<&str>, executable: &str) -> Option<url::Url> {
    let mut url = url::Url::parse(repository).ok()?;
    if url.scheme() != "ghidra" || url.host().is_none() || !url.username().is_empty()
        || url.password().is_some() || url.query().is_some() || url.fragment().is_some()
        || url.path().trim_matches('/').is_empty()
    {
        return None;
    }
    let valid = |s: &str| !s.trim().is_empty() && s != "." && s != ".."
        && !s.contains(['/', '\\']) && !s.chars().any(char::is_control);
    {
        let mut segments = url.path_segments_mut().ok()?;
        segments.pop_if_empty();
        let folders = path.into_iter().map(|p| p.trim_matches('/')).filter(|p| !p.is_empty());
        for part in folders.flat_map(|p| p.split('/')).chain([executable]) {
            if !valid(part) {
                return None;
            }
            segments.push(part);
        }
    }
    Some(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_vector_preserves_unsigned_hashes_and_saturates_frequency() {
        let mut features = vec![7; 65];
        features.extend([0x8000_0000, u32::MAX]);
        assert_eq!(encode_vector(&features), "(40:7,1:80000000,1:ffffffff)");
    }

    #[cfg(unix)]
    #[test]
    fn postgres_startup_deadline_covers_authentication() {
        use std::io::{Read, Write};
        use std::os::unix::net::UnixListener;
        use std::thread;

        let dir = tempfile::tempdir().unwrap();
        let listener = UnixListener::bind(dir.path().join(".s.PGSQL.5432")).unwrap();
        let peer = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut header = [0; 4];
            stream.read_exact(&mut header).unwrap();
            let size = u32::from_be_bytes(header) as usize;
            let mut startup = [0; 1024];
            stream.read_exact(&mut startup[..size - 4]).unwrap();
            assert_eq!(startup[..4], [0, 3, 0, 0]); // PostgreSQL protocol 3.0.
            stream.write_all(b"R\0\0\0\x08\0\0\0\x03").unwrap(); // AuthenticationCleartextPassword.
            let mut password = [0; 12];
            stream.read_exact(&mut password).unwrap();
            assert_eq!(&password, b"p\0\0\0\x0bsecret\0");
            // Do not complete authentication. Expiry must also close the socket.
            assert_eq!(stream.read(&mut [0]).unwrap(), 0);
        });
        let mut config = Config::new();
        config.host_path(dir.path()).user("bsim_test").password("secret");
        let result = start_connection(config, Duration::from_secs(1));
        assert!(matches!(result, Err(message) if message == "PostgreSQL connection startup timed out"));
        peer.join().unwrap();
    }
}
