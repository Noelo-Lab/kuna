// Generated with stock Ghidra 12.1.2 PUBLIC (20260605, c0f584bf229fffba61b36431f3ce30c0c3e4e682).
// Release ZIP SHA-256: b62e81a0390618466c019c60d8c2f796ced2509c4c1aea4a37644a77272cf99d.
// Input prog SHA-256: 1275196357da9c6677c06dfd85b71c1fffba93a35deae4d44817b72bcfb98bea.
// Regenerate from the repo root with stock analyzeHeadless in PATH. No Ghidra is needed in CI.
//   root="$PWD"; tmp="$(mktemp -d)"; mkdir "$tmp/home"
//   env -i HOME="$tmp/home" PATH="$PATH" \
//     JAVA_TOOL_OPTIONS="-Duser.name=fixture -Duser.home=$tmp/home -Dapplication.cachedir=$tmp/home -Dapplication.tempdir=$tmp/home" \
//     analyzeHeadless "$tmp" bsim -noanalysis -scriptPath "$root/tests/fixtures/bsim" \
//     -preScript ExportBsimFixture.java "$root/tests/fixtures/fid/callees/prog" "$tmp/reference.gzf"
//   (cd "$tmp" && rm -f bsim.rep/{idata,versioned}/~*.bak &&
//     python3 -m zipfile -c "$root/tests/fixtures/bsim/stock-project.zip" bsim.gpr bsim.rep)
//   cp "$tmp/reference.gzf" tests/fixtures/bsim/reference.gzf
// To regenerate unnamed.gzf, repeat in a new scratch project with output
// "$tmp/unnamed.gzf" and append --placeholders-only. Copy just that archive.
// The ZIP compresses the stock project marker and saved pages; it is not a Ghidra export.
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;

import ghidra.app.script.GhidraScript;
import ghidra.app.util.bin.ByteArrayProvider;
import ghidra.app.util.importer.ProgramLoader;
import ghidra.app.util.opinion.ElfLoader;
import ghidra.program.model.symbol.SourceType;

public class ExportBsimFixture extends GhidraScript {
    @Override public void run() throws Exception {
        var args = getScriptArgs();
        boolean placeholdersOnly = args.length == 3 && args[2].equals("--placeholders-only");
        if (args.length != 2 && !placeholdersOnly) {
            throw new IllegalArgumentException("expected input, GZF output and optional --placeholders-only");
        }
        // Feed bytes, not a file-backed provider: even deleted DB records can retain an FSRL.
        try (var source = new ByteArrayProvider("prog", Files.readAllBytes(Path.of(args[0])));
                var loaded = ProgramLoader.builder()
                .source(source)
                .name("annotated-prog")
                .loaders(ElfLoader.class)
                .project(state.getProject())
                .monitor(monitor)
                .load()) {
            currentProgram = loaded.getPrimaryDomainObject();
            try {
                try (var transaction = currentProgram.openTransaction("Name fixture functions")) {
                    currentProgram.setExecutablePath("prog");
                    addFunction(0x401000, "project_scale_a");
                    addFunction(0x401020, "project_scale_b");
                    if (placeholdersOnly) {
                        currentProgram.getFunctionManager().getFunctionAt(toAddr(0x401000))
                            .setName(null, SourceType.DEFAULT);
                        currentProgram.getFunctionManager().getFunctionAt(toAddr(0x401020))
                            .setName("sub_401020", SourceType.USER_DEFINED);
                        currentProgram.getFunctionManager().getFunctionAt(toAddr(0x401090))
                            .setName(null, SourceType.DEFAULT);
                    }
                }
                loaded.getPrimary().save(monitor).packFile(new File(args[1]), monitor);
            }
            finally {
                currentProgram = null;
            }
        }
    }

    private void addFunction(long address, String name) throws Exception {
        var entry = toAddr(address);
        if (!disassemble(entry) || createFunction(entry, name) == null) {
            throw new IllegalStateException("cannot create fixture entry " + name);
        }
    }
}
