`kuna functions --summary` is the first call on an unknown binary, but it orients a PyInstaller bundle, a .NET assembly or a UPX stub as if its wrapper were the program. This adds a `summary.runtime` array naming what wrapped or built the image and what to run instead, and prints the same hint as one `note:` line on stderr from the other binary subcommands.

```
$ python3 decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64.py
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64 --summary --json | grep -A6 runtime
    "runtime": [
      {
        "id": "pyinstaller",
        "version": "3.12",
        "hint": "PyInstaller bundle (Python 3.12): extract with pyinstxtractor-ng",
        "actionable": true
      }
$ kuna functions decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64 --json > /dev/null
note: decompiler/crates/kuna-analysis/tests/fixtures/pyinstaller_cookie_x86_64: PyInstaller bundle (Python 3.12): extract with pyinstxtractor-ng
```

Before this change the summary had no such field and nothing was printed.

## The fix

- `kuna-cli/src/runtime_hints.rs` recognizes: .NET (PE CLR data directory), PyInstaller (CArchive cookie, with the Python version it records), Nuitka, AutoIt (`AU3!EA06`), UPX (sections, `UPX!` in the first 4 KiB, or a PackHeader), NEOLite, twinBASIC and VB6 (`MSVBVM60.DLL`), plus the Go/Rust verdict the loader already makes (informational only).
- One regex pass over the file finds every string marker; the Windows-only markers need a PE, and a stray cookie magic is rejected by the version and library-name checks.
- The PE walk does not check `MZ`, so an image whose DOS magic is damaged is still named before its load fails.
- Stderr only, never stdout: `--json` documents are unchanged apart from the new summary field. `--summary`, `--help`, `unpack` and `--jobs` workers print no note.

## Tests

- `tests/cli/rthints.json` and `kuna-cli/tests/runtime_hints_cli.rs` on a new 5 KB fixture (an in-repo ELF with a cookie appended, generator alongside); both fail without the change.
- Unit tests cover each detector, a damaged `MZ`, and ELF files carrying Windows-only markers.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01VnVyLXPzZE2st3UdR1NQmM
