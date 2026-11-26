// Generated with Ghidra 12.1.2 (Nix build of c0f584bf229fffba61b36431f3ce30c0c3e4e682), ProgramDB v32.
// Input relocatable.o SHA-256: 17164fabbc7486bc86bd4cb7f141db9b142df47275f31490a30a1dc794e01d13.
// Input arm_thumb_le32.o SHA-256: 6a21eea7835188543cf8baa56f0505dfd2661c03e1f26cc07f9317b4f9d20a17.
// Regenerate from the repo root with analyzeHeadless in PATH; no Ghidra is needed in CI.
//   root="$PWD"; tmp="$(mktemp -d)"; mkdir "$tmp/home"; fixtures="$root/tests/fixtures/bsim"
//   export_fixture() {
//     env -i HOME="$tmp/home" PATH="$PATH" \
//       JAVA_TOOL_OPTIONS="-Duser.name=fixture -Duser.home=$tmp/home -Dapplication.cachedir=$tmp/home -Dapplication.tempdir=$tmp/home" \
//       analyzeHeadless "$tmp" "$1" -noanalysis -scriptPath "$fixtures" \
//       -preScript ExportBsimRelocatableFixture.java "${@:2}"
//   }
//   export_fixture bsim-x86 "$fixtures/relocatable.o" "$tmp/relocatable.gzf" "$tmp/relocatable-patched.gzf"
//   export_fixture bsim-arm --arm \
//     "$root/decompiler/crates/kuna-analysis/tests/fixtures/arm_thumb_le32.o" "$tmp/relocatable-arm.gzf"
//   cp "$tmp"/relocatable*.gzf "$fixtures"
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HexFormat;

import ghidra.app.script.GhidraScript;
import ghidra.app.util.bin.ByteArrayProvider;
import ghidra.app.util.importer.ProgramLoader;
import ghidra.app.util.opinion.ElfLoader;
import ghidra.program.database.ProgramDB;
import ghidra.program.model.address.Address;
import ghidra.program.model.reloc.Relocation;
import ghidra.program.model.symbol.SourceType;

public class ExportBsimRelocatableFixture extends GhidraScript {
    @Override public void run() throws Exception {
        var args = getScriptArgs();
        if (args.length != 3) {
            throw new IllegalArgumentException("expected ET_REL input and two GZF outputs, or --arm INPUT GZF");
        }
        var arm = args[0].equals("--arm");
        var input = args[arm ? 1 : 0];
        var filename = arm ? "arm_thumb_le32.o" : "relocatable.o";
        // A file-backed provider can retain an absolute path in deleted DB records.
        try (var source = new ByteArrayProvider(filename, Files.readAllBytes(Path.of(input)));
                var loaded = ProgramLoader.builder()
                    .source(source)
                    .name("annotated-relocatable")
                    .loaders(ElfLoader.class)
                    .project(state.getProject())
                    .monitor(monitor)
                    .load()) {
            currentProgram = loaded.getPrimaryDomainObject();
            try {
                try (var transaction = currentProgram.openTransaction("Name fixture functions")) {
                    currentProgram.setExecutablePath(filename);
                    if (arm) {
                        nameFunction(".text", "project_thumb_add");
                    }
                    else {
                        nameFunction(".text.rel_b", "project_rel_b");
                        nameFunction(".text.rel_a", "project_rel_a");
                    }
                }
                var site = arm
                    ? currentProgram.getMemory().getBlock(".text").getStart().add(0x1e)
                    : currentProgram.getMemory().getBlock(".text.rel_a").getStart().add(5);
                var relocation = findRelocation(site);
                var file = loaded.getPrimary().save(monitor);
                inspectRelocation("loader-only", relocation, arm ? 10 : 4);
                file.packFile(new File(args[arm ? 2 : 1]), monitor);
                if (arm) {
                    return;
                }

                try (var transaction = currentProgram.openTransaction("Patch call displacement")) {
                    var call = site.subtract(1);
                    var lastDisplacementByte = site.add(3);
                    clearListing(call, lastDisplacementByte);
                    var memory = currentProgram.getMemory();
                    memory.setByte(lastDisplacementByte,
                        (byte) (memory.getByte(lastDisplacementByte) ^ 1));
                    if (!disassemble(call)) {
                        throw new IllegalStateException("cannot disassemble patched call");
                    }
                }
                file.save(monitor);
                inspectRelocation("user-patched", findRelocation(site), 4);
                file.packFile(new File(args[2]), monitor);
            }
            finally {
                currentProgram = null;
            }
        }
    }

    private void nameFunction(String section, String name) throws Exception {
        var entry = currentProgram.getMemory().getBlock(section).getStart();
        disassemble(entry);
        if (currentProgram.getListing().getInstructionAt(entry) == null) {
            throw new IllegalStateException("cannot disassemble " + section);
        }
        var function = currentProgram.getFunctionManager().getFunctionAt(entry);
        if (function == null) {
            function = createFunction(entry, name);
            if (function == null) {
                throw new IllegalStateException("cannot create fixture function " + name);
            }
        }
        else {
            function.setName(name, SourceType.USER_DEFINED);
        }
        println(name + " " + section + "+0 = " + entry);
    }

    private Relocation findRelocation(Address site) {
        var relocations = currentProgram.getRelocationTable().getRelocations(site);
        if (relocations.size() != 1) {
            throw new IllegalStateException("expected exactly one relocation at " + site);
        }
        return relocations.getFirst();
    }

    private void inspectRelocation(String variant, Relocation relocation, int expectedType) throws Exception {
        int version = ((ProgramDB) currentProgram).getStoredVersion();
        var site = relocation.getAddress();
        if (version != 32 || relocation.getStatus() != Relocation.Status.APPLIED ||
                relocation.getType() != expectedType || relocation.getLength() != 4) {
            throw new IllegalStateException("expected an applied relocation type " + expectedType + " in a v32 DB at " + site);
        }
        var memory = currentProgram.getMemory();
        var source = memory.getBlock(site).getSourceInfos().getFirst();
        var fileBytes = source.getFileBytes().orElseThrow();
        long offset = source.getFileBytesOffset(site);
        byte[] original = new byte[4];
        byte[] modified = new byte[4];
        byte[] loaded = new byte[4];
        fileBytes.getOriginalBytes(offset, original);
        fileBytes.getModifiedBytes(offset, modified);
        memory.getBytes(site, loaded);
        var hex = HexFormat.of();
        println("%s v%d %s file+%#x original=%s modified=%s memory=%s relocation=%s"
            .formatted(variant, version, site, offset, hex.formatHex(original),
                hex.formatHex(modified), hex.formatHex(loaded), hex.formatHex(relocation.getBytes())));
    }
}
