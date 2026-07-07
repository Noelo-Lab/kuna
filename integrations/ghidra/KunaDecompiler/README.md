# KunaDecompiler — the kuna core inside the stock Ghidra GUI

A standard Ghidra extension that makes the **stock Ghidra GUI** spawn **kuna** (the Rust
port of the Ghidra decompiler, this repository) as its native decompiler backend, in
place of the stock C++ `decompile` binary — the *unchanged* Ghidra GUI (Decompiler
window, analyzers, scripts) running on the kuna engine. It is a drop-in, backwards-
compatible backend: Ghidra's Java side is untouched; only the spawned process changes.

**Status: Phase 2 — kuna decompiles.** The `kuna_ghidra` binary speaks the complete
Ghidra⇄decompiler wire protocol *and* drives kuna's engine, so the Decompiler window
shows **real C produced by kuna**, with working click-to-address. Current scope: simple,
self-contained functions decompile cleanly; a function that references globals/types the
engine can't yet resolve shows placeholder names (`sub_…`/`DAT_…`) and default types —
correct names/types at scale is Phase 3 (the lazy symbol scope). See
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

## Run the kuna backend in a real Ghidra instance

The whole flow, against a **Ghidra release install** — build the backend, package the
extension, install it, enable the plugin, decompile:

```bash
# 1. Build the kuna_ghidra backend and package the extension in one step.
#    build.sh compiles the release binary, stages it into os/<platform>/, and
#    (because GHIDRA_INSTALL_DIR is set) builds the installable zip.
cd integrations/ghidra/KunaDecompiler
GHIDRA_INSTALL_DIR=/abs/path/to/ghidra_12.2 ./build.sh
#    -> dist/ghidra_12.2_..._KunaDecompiler.zip
```

2. **Install the extension:** in Ghidra's project window, **File → Install Extensions… →
   `+` → select `dist/…_KunaDecompiler.zip`**, then restart Ghidra when prompted.
3. **Enable the plugin:** open a program in the CodeBrowser, then **File → Configure →
   Miscellaneous** and check **KunaDecompilerPlugin**. On load it swaps the decompiler
   core and logs `Decompiler core is now KUNA: <path>`.
4. **Decompile:** open any function — the Decompiler window now shows kuna's C. (Try a
   small, self-contained function first; see the status note above for what Phase 2 does
   and doesn't yet resolve.)
5. **Toggle / revert** at runtime with **Tools → Kuna Decompiler → Use Kuna Core** (see
   *Revert* below).

Manual equivalents and the dev-checkout path are in the sections below.

### Just the binary (manual)

`build.sh` with no `GHIDRA_INSTALL_DIR` builds + stages the binary only; the equivalent
by hand, from the repo root:

```bash
cd decompiler
cargo build --release -p kuna-ghidra
cp target/release/kuna_ghidra ../integrations/ghidra/KunaDecompiler/os/linux_x86_64/
```

The binary is a gitignored build artifact; it must be present under
`os/<platform>/` **before** building the extension zip so it gets packaged (`build.sh`
picks the right `os/<platform>/` dir for your host; for other platforms see
`os/linux_x86_64/README.md`).

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
- Decompile a small, self-contained function: the Decompiler window shows kuna's C, and
  clicking a token navigates to its address — proof that the stock GUI is running on the
  kuna engine. If a function can't be decompiled yet (an un-supported engine path), the
  window shows a clean per-function message instead of hanging.
- Sanity check that it's really kuna and not the stock core: the two decompilers produce
  cosmetically different C, and `KunaCoreStatus.java` reports the `kuna_ghidra` path.

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

## Use with PyGhidra (headless / scripting)

Build the backend once (`cd decompiler && cargo build --release -p kuna-ghidra`), install
pyghidra (`pip install "$GHIDRA_INSTALL_DIR"/Ghidra/Features/PyGhidra/pypkg/dist/pyghidra-*.whl`,
or `pip install pyghidra`), and set `GHIDRA_INSTALL_DIR`. Then pick one of two ways to make
Ghidra use kuna.

### In-script toggle — recommended (no file changes)

The GUI plugin can't load headless, but you can do exactly what it does — set
`DecompileProcessFactory`'s cached `exepath` by reflection — straight from Python. This
flips the core on and off at runtime with **no file surgery**; it takes effect on the next
decompiler-process spawn (a freshly constructed `DecompInterface`):

```python
import os, pyghidra
pyghidra.start()
from ghidra.app.decompiler import DecompileProcessFactory, DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

def _exepath():
    f = DecompileProcessFactory.class_.getDeclaredField("exepath")
    f.setAccessible(True)
    return f
def enable_kuna(exe):  _exepath().set(None, exe)    # kuna ON  (abs path to kuna_ghidra)
def disable_kuna():    _exepath().set(None, None)   # back to the stock decompiler
def active_core():     return _exepath().get(None)  # what Ghidra will spawn next

with pyghidra.open_program("a.out") as flat:        # analyze=True by default
    program = flat.getCurrentProgram()
    enable_kuna(os.environ["KUNA_GHIDRA_EXE"])       # target/release/kuna_ghidra
    ifc = DecompInterface(); ifc.openProgram(program)
    for func in program.getFunctionManager().getFunctions(True):
        print(ifc.decompileFunction(func, 60, ConsoleTaskMonitor()).getDecompiledFunction().getC())
    ifc.dispose()
    disable_kuna()                                   # later DecompInterfaces use stock again
```

`enable_kuna`/`disable_kuna` are the headless equivalent of the extension's
**Tools → Kuna Decompiler → Use Kuna Core** checkbox (which already toggles the GUI at
runtime). An already-open `DecompInterface` keeps its current core until it respawns, so
toggle *before* constructing the one you want to run.

### Persistent swap — the file-drop seam

When you can't inject Python (e.g. `analyzeHeadless`) or want *every* Ghidra invocation to
use kuna, drop the binary named `decompile` into the module's `build/os/<platform>/` (Ghidra
searches it before `os/<platform>/`, so it shadows the stock binary without overwriting it —
delete the copy to revert):

```bash
PLAT=mac_arm_64   # or linux_x86_64 | mac_x86_64 | linux_arm_64 (see build.sh)
DROP="$GHIDRA_INSTALL_DIR/Ghidra/Features/Decompiler/build/os/$PLAT"
mkdir -p "$DROP" && cp decompiler/target/release/kuna_ghidra "$DROP/decompile"
```
