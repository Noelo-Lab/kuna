//! Catalog discovery without an installed console, with opt-in external queries.

use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;

use kuna_decomp::kuna_phases::{emit_catalog_json, emit_catalog_json_one, emit_catalog_markdown};

// A concurrent fork can inherit a just-written executable's writable descriptor
// until exec, so the copy or the fake console fails with ETXTBSY when run.
static PROCESS_SETUP: Mutex<()> = Mutex::new(());

trait Captured {
    fn captured(&mut self) -> Output;
}

impl Captured for Command {
    fn captured(&mut self) -> Output {
        let child = {
            let _setup = PROCESS_SETUP.lock().unwrap_or_else(|e| e.into_inner());
            self.stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
        };
        child.wait_with_output().unwrap()
    }
}

struct Catalog {
    dir: tempfile::TempDir,
    executable: PathBuf,
}

impl Catalog {
    fn isolated() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let executable = dir
            .path()
            .join(format!("kuna{}", std::env::consts::EXE_SUFFIX));
        {
            let _setup = PROCESS_SETUP.lock().unwrap_or_else(|e| e.into_inner());
            std::fs::copy(env!("CARGO_BIN_EXE_kuna"), &executable).unwrap();
        }
        Self { dir, executable }
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut cmd = Command::new(&self.executable);
        cmd.arg("catalog")
            .args(args)
            .current_dir(self.dir.path())
            .env_remove("KUNA_DECOMP_DBG")
            .env_remove("KUNA_ENGINE")
            .env("KUNA_ROOT", self.dir.path())
            .env("KUNA_SPECS", self.dir.path().join("missing-specs"))
            .env("SLEIGHHOME", self.dir.path().join("missing-specs"));
        cmd
    }

    fn run(&self, args: &[&str]) -> Output {
        self.command(args).captured()
    }

    #[cfg(unix)]
    fn console(&self, full: &str, one: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        std::fs::write(self.dir.path().join("catalog.json"), full).unwrap();
        std::fs::write(self.dir.path().join("one.json"), one).unwrap();
        let path = self.dir.path().join("decomp_dbg");
        let _setup = PROCESS_SETUP.lock().unwrap_or_else(|e| e.into_inner());
        std::fs::write(
            &path,
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > console-args\n\
             printf '%s\\n' \"$SLEIGHHOME\" > console-specs\n\
             printf '%s\\n' \"$KUNA_ENGINE\" > console-engine\n\
             cat > console-script\n\
             printf '[decomp]> phase catalog\\n'\n\
             case \"$(head -n 1 console-script)\" in\n\
             'phase catalog') cat catalog.json ;;\n\
             'phase catalog jumptablemax') cat one.json ;;\n\
             *) printf 'Unknown settable option\\n' ;;\n\
             esac\n",
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
        path
    }
}

fn succeeds(out: &Output) {
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        out.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn standalone_catalog_needs_neither_console_nor_specs() {
    let catalog = Catalog::isolated();
    for args in [
        vec![],
        vec!["--json"],
        vec!["--check"],
        vec!["--markdown"],
        vec!["--option", "jumptablemax"],
        vec!["--json", "--option", "jumptablemax"],
        vec!["--tier", "core"],
        vec!["--json", "--tier", "analysis"],
        vec![
            "--json",
            "--engine",
            "rust",
            "--sleighpath",
            "missing-specs",
        ],
    ] {
        succeeds(&catalog.run(&args));
    }
    let out = catalog.run(&["--json"]);
    let expected: serde_json::Value = serde_json::from_str(&emit_catalog_json(|_| None)).unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&out.stdout).unwrap(),
        expected
    );
    assert_eq!(
        catalog.run(&["--markdown"]).stdout,
        emit_catalog_markdown().as_bytes()
    );
    let out = catalog.run(&["--json", "--option", "no_such_catalog_option"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        out.stderr,
        b"error: unknown option: \"no_such_catalog_option\"\n"
    );
    succeeds(
        &catalog
            .command(&["--check"])
            .env("KUNA_DECOMP_DBG", "")
            .captured(),
    );
}

#[test]
fn explicit_missing_console_still_fails_instead_of_falling_back() {
    let catalog = Catalog::isolated();
    let missing = catalog.dir.path().join("missing-console");
    for args in [vec![], vec!["--json"], vec!["--check"]] {
        let out = catalog
            .command(&args)
            .env("KUNA_DECOMP_DBG", &missing)
            .captured();
        assert_eq!(out.status.code(), Some(1));
        assert!(out.stdout.is_empty());
        let stderr = String::from_utf8(out.stderr).unwrap();
        assert!(stderr.contains("decomp_dbg not found at"), "{stderr}");
        assert!(stderr.contains("KUNA_DECOMP_DBG"), "{stderr}");
    }
    let out = catalog.run(&["--json", "--decomp-dbg", missing.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    succeeds(
        &catalog
            .command(&["--markdown"])
            .env("KUNA_DECOMP_DBG", missing)
            .captured(),
    );
}

#[cfg(unix)]
#[test]
fn unselected_sibling_console_is_not_started_and_opt_in_is_byte_identical() {
    let catalog = Catalog::isolated();
    let console = catalog.console(
        &emit_catalog_json(|_| None),
        &emit_catalog_json_one("jumptablemax", None).unwrap(),
    );
    for args in [
        vec![],
        vec!["--json"],
        vec!["--check"],
        vec!["--option", "jumptablemax"],
        vec!["--json", "--option", "jumptablemax"],
        vec!["--option", "missing_option"],
        vec!["--json", "--option", "missing_option"],
        vec!["--json", "--option", "jumptablemax", "--tier", "core"],
        vec!["--tier", "analysis"],
        vec!["--json", "--tier", "transform"],
    ] {
        let marker = catalog.dir.path().join("console-script");
        let local = catalog.run(&args);
        assert!(
            !marker.exists(),
            "default catalog started the sibling console: {args:?}"
        );
        let external = catalog
            .command(&args)
            .env("KUNA_DECOMP_DBG", &console)
            .captured();
        assert!(
            marker.exists(),
            "explicit console was not started: {args:?}"
        );
        assert_eq!(local.status.code(), external.status.code(), "{args:?}");
        assert_eq!(local.stdout, external.stdout, "{args:?}");
        assert_eq!(local.stderr, external.stderr, "{args:?}");
        std::fs::remove_file(marker).unwrap();
    }
}

#[cfg(unix)]
#[test]
fn external_selection_preserves_precedence_legacy_flags_and_drift_check() {
    let catalog = Catalog::isolated();
    let console = catalog.console("[{\"option\":\"external_only\"}]", "{}");
    let out = catalog
        .command(&[
            "--json",
            "--decomp-dbg",
            console.to_str().unwrap(),
            "--engine",
            "rust",
            "--sleighpath",
            "custom-specs",
        ])
        .env("KUNA_DECOMP_DBG", catalog.dir.path().join("wrong-console"))
        .captured();
    succeeds(&out);
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(json[0]["option"], "external_only");
    for (file, expected) in [
        ("console-args", "-s\ncustom-specs\n"),
        ("console-specs", "custom-specs\n"),
        ("console-engine", "rust\n"),
        ("console-script", "phase catalog\nquit\n"),
    ] {
        assert_eq!(
            std::fs::read_to_string(catalog.dir.path().join(file)).unwrap(),
            expected
        );
    }
    let out = catalog
        .command(&["--check"])
        .env("KUNA_DECOMP_DBG", console)
        .captured();
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(stderr.contains("catalog DRIFT:"), "{stderr}");
    assert!(
        stderr.contains("catalog row \"external_only\" matches no registered kuna option"),
        "{stderr}"
    );
    assert!(stderr.contains("has no catalog row"), "{stderr}");
}
