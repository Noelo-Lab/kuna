# KunaDecompiler — the kuna core inside the stock Ghidra GUI

A standard Ghidra extension that makes Ghidra spawn **kuna** (the Rust port of the
Ghidra decompiler, this repository) as its native decompiler process, in place of the
stock C++ `decompile` binary — the full Ghidra GUI (Decompiler window, analyzers,
scripts) on top of the kuna engine.

**Status: Phase 1 — protocol skeleton.** The `kuna_ghidra` binary speaks the complete
Ghidra⇄decompiler wire protocol (burst framing, packed documents, query upcalls), but
the engine bridge is not yet connected: Ghidra will show a clean per-function error
message in the Decompiler window instead of decompiled C until Phase 2 lands. See
[`docs/ghidra-integration.md`](../../../docs/ghidra-integration.md) for the design and
phase plan. Target Ghidra version: **12.2** (the swap relies on the exact shape of
`DecompileProcessFactory`; see *How it works* below).

## How it works

Every decompiler consumer in Ghidra funnels process creation through one static
factory, `ghidra.app.decompiler.DecompileProcessFactory`, which resolves the executable
once and caches it in a `private static String exepath` field. There is no property,
env var, or ExtensionPoint to substitute it — so at plugin load,
`KunaDecompilerPlugin` resolves the `kuna_ghidra` binary shipped in this extension's
`os/<platform>/` directory (or the `-Dkuna.decompiler.exe` override) and writes that
field by reflection, before the lazily-spawned first decompiler process exists. A
Tools-menu checkbox toggles between the kuna core and the stock one at runtime.

## Build the kuna binary

From the kuna repo root:

```bash
cd decompiler
cargo build --release -p kuna-ghidra
cp target/release/kuna_ghidra ../integrations/ghidra/KunaDecompiler/os/linux_x86_64/
```

The binary is a gitignored build artifact; it must be present under
`os/linux_x86_64/` **before** building the extension zip so it gets packaged (other
platforms: see `os/linux_x86_64/README.md`).

## Build & install the extension

### Against a Ghidra release install

Use the Gradle version named by `application.gradle.min` in
`<GHIDRA_INSTALL_DIR>/Ghidra/application.properties`:

```bash
cd integrations/ghidra/KunaDecompiler
gradle -PGHIDRA_INSTALL_DIR=<Absolute path to Ghidra> buildExtension
# -> dist/ghidra_<version>_<release>_<date>_KunaDecompiler.zip
```

Then in Ghidra's project window: **File → Install Extensions… → + → select the zip**,
and restart Ghidra when prompted.

### Against a Ghidra source (dev) checkout

No zip needed — inject this directory as an external module when launching Ghidra
(this is the seam GhidraDev/VSCode dev launches use,
`GhidraApplicationLayout.java:174-181`):

```
-Dghidra.external.modules=/abs/path/to/kuna/integrations/ghidra/KunaDecompiler
```

added to the launcher's VM args (e.g. the Eclipse/VSCode launch configuration created
by `gradle prepDev` / the VSCode integration), with the extension's Java compiled into
the project. During development you can skip copying the binary and instead pass
`-Dkuna.decompiler.exe=/abs/path/to/kuna/decompiler/target/release/kuna_ghidra`.

## Enable the plugin

Open a program in the CodeBrowser, then **File → Configure → Miscellaneous** (the
gear/"Configure All Plugins" view) and check **KunaDecompilerPlugin**. On load it
performs the swap and logs `Decompiler core is now KUNA: <path>` to the application
log; if the binary is missing it shows an error dialog with the build/copy commands and
leaves the stock core active.

## Verify

- Run the bundled script `KunaCoreStatus.java` (Script Manager, category *Kuna*): it
  prints the executable path currently installed in `DecompileProcessFactory` and
  whether the next decompiler process is the kuna or stock core.
- Decompile any function: in Phase 1 the Decompiler window shows the kuna phase-1
  per-function message instead of C — that message coming from the Decompiler window
  *is* the proof that Ghidra spawned kuna.

## Revert

- **Runtime:** uncheck **Tools → Kuna Decompiler → Use Kuna Core** — future decompiler
  spawns use the stock binary again. Programs with an already-running decompiler
  process keep it until that process restarts (reopen the program, or the process is
  reset after an error/timeout); reopening the program is the reliable nudge.
- **Plugin off:** un-check the plugin in File → Configure (its dispose restores the
  stock core), or **File → Install Extensions…** and uncheck KunaDecompiler, then
  restart Ghidra.

## Alternative: the no-code file-drop seam

You can run kuna under Ghidra without this extension at all: copy the kuna binary
**named `decompile`** to

```
<GHIDRA_INSTALL_DIR>/Ghidra/Features/Decompiler/build/os/<platform>/decompile
```

`Application.getModuleOSFile` searches a module's `build/os/<platform>/` *before*
`os/<platform>/`, so this shadows the stock binary without overwriting it (delete the
`build/os` copy to revert). Tradeoffs versus the extension:

- applies **unconditionally** to that install — every project, every tool, no toggle,
  no logging of which core is active;
- invisible: nothing in the GUI indicates the core was swapped (use
  `KunaCoreStatus.java`'s output path, or the phase-1 message, to tell);
- survives extension uninstalls but may be wiped by Ghidra upgrades/rebuilds;
- per-install file surgery rather than a distributable artifact.

The extension is the recommended path; the file drop is handy for quick experiments on
a release install.
