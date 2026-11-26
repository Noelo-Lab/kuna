// Generated with Ghidra 12.1.2 (c0f584bf229fffba61b36431f3ce30c0c3e4e682), ProgramDB v32.
// Input arm-relocations.o SHA-256: e407ae635de6fabfc773b746d929551764b4a14d75932c7702cf8015b5b3d93a.
// Regenerate the archives from the repo root using the stock analyzeHeadless and JDK 21:
//   root="$PWD"; tmp="$(mktemp -d)"; mkdir "$tmp/home"; fixtures="$root/tests/fixtures/bsim"
//   HOME="$tmp/home" JAVA_TOOL_OPTIONS="-Duser.name=fixture -Duser.home=$tmp/home -Dapplication.cachedir=$tmp/home -Dapplication.tempdir=$tmp/home" \
//     analyzeHeadless "$tmp" bsim-arm -noanalysis -scriptPath "$fixtures" \
//     -preScript ExportBsimArmRelocations.java "$fixtures/arm-relocations.o" \
//     "$tmp/arm-relocations.gzf" "$tmp/arm-relocations-patched.gzf"
//   cp "$tmp"/arm-relocations*.gzf "$fixtures"
// The exporter checks the database contents; stock packFile output can vary across runs.
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HexFormat;
import java.util.Map;

import ghidra.app.script.GhidraScript;
import ghidra.app.util.bin.ByteArrayProvider;
import ghidra.app.util.importer.ProgramLoader;
import ghidra.app.util.opinion.ElfLoader;
import ghidra.program.database.ProgramDB;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.reloc.Relocation;
import ghidra.program.model.symbol.SourceType;

public class ExportBsimArmRelocations extends GhidraScript {
    private static final String INPUT_NAME = "arm-relocations.o";
    private static final Map<Integer, Integer> RELOCATION_COUNTS =
        Map.of(2, 5, 10, 1, 11, 1, 30, 1, 47, 4, 48, 4, 102, 1);

    @Override public void run() throws Exception {
        var args = getScriptArgs();
        if (args.length != 3) {
            throw new IllegalArgumentException("expected ET_REL input and two GZF outputs");
        }
        // Loading via a FileByteProvider can retain the absolute input path in deleted DB records.
        try (var source = new ByteArrayProvider(INPUT_NAME, Files.readAllBytes(Path.of(args[0])));
                var loaded = ProgramLoader.builder()
                    .source(source)
                    .name("annotated-arm-relocations")
                    .loaders(ElfLoader.class)
                    .project(state.getProject())
                    .monitor(monitor)
                    .load()) {
            currentProgram = loaded.getPrimaryDomainObject();
            try {
                var memory = currentProgram.getMemory();
                var entry = memory.getBlock(".text").getStart();
                var end = entry.add(0x21); // 34-byte fixture_arm_relocations, before the wide jump.
                var call = entry.add(2);
                var patch = call.add(2);
                try (var transaction = currentProgram.openTransaction("Name fixture function")) {
                    currentProgram.setExecutablePath(INPUT_NAME);
                    disassemble(entry);
                    var function = currentProgram.getFunctionManager().getFunctionAt(entry);
                    var listing = currentProgram.getListing();
                    if (function == null || entry.getOffset() != 0x10000 ||
                            listing.getInstructionAt(call) == null || listing.getInstructionContaining(end) == null) {
                        throw new IllegalStateException("expected the imported Thumb function at 00010000");
                    }
                    function.setBody(new AddressSet(entry, end));
                    function.setName("project_arm_relocations", SourceType.USER_DEFINED);
                    println(function.getName() + " at " + entry + " (" + function.getBody() + ")");
                }
                var file = loaded.getPrimary().save(monitor);
                var before = inspectRelocations("loader-only");
                file.packFile(new File(args[1]), monitor);

                try (var transaction = currentProgram.openTransaction("Patch external THM_CALL displacement")) {
                    clearListing(call, call.add(3));
                    memory.setByte(patch, (byte) (memory.getByte(patch) ^ 2));
                    if (!disassemble(call)) {
                        throw new IllegalStateException("cannot disassemble patched external THM_CALL");
                    }
                }
                file.save(monitor);
                var after = inspectRelocations("user-patched");
                var expected = before.get(call.getOffset()).clone();
                expected[2] ^= 2;
                for (var site : before.keySet()) {
                    if (!Arrays.equals(after.get(site), site == call.getOffset() ? expected : before.get(site))) {
                        throw new IllegalStateException("unexpected user patch at " + Long.toHexString(site));
                    }
                }
                file.packFile(new File(args[2]), monitor);
            }
            finally {
                currentProgram = null;
            }
        }
    }

    private Map<Long, byte[]> inspectRelocations(String variant) throws Exception {
        int version = ((ProgramDB) currentProgram).getStoredVersion();
        if (version != 32) {
            throw new IllegalStateException("expected a v32 ProgramDB, got " + version);
        }
        var counts = new HashMap<Integer, Integer>();
        var relocated = new HashMap<Long, byte[]>();
        var memory = currentProgram.getMemory();
        var hex = HexFormat.of();
        var entries = currentProgram.getRelocationTable().getRelocations();
        while (entries.hasNext()) {
            var relocation = entries.next();
            var site = relocation.getAddress();
            int type = relocation.getType();
            int width = type == 11 || type == 102 ? 2 : 4;
            if (relocation.getStatus() != Relocation.Status.APPLIED || relocation.getLength() != width) {
                throw new IllegalStateException("expected an applied type " + type + " width " + width + " at " + site);
            }
            var source = memory.getBlock(site).getSourceInfos().getFirst();
            var fileBytes = source.getFileBytes().orElseThrow();
            long offset = source.getFileBytesOffset(site);
            byte[] original = new byte[width];
            byte[] modified = new byte[width];
            byte[] bytes = new byte[width];
            fileBytes.getOriginalBytes(offset, original);
            fileBytes.getModifiedBytes(offset, modified);
            memory.getBytes(site, bytes);
            if (!Arrays.equals(original, relocation.getBytes()) || !Arrays.equals(modified, bytes) ||
                    Arrays.equals(original, modified)) {
                throw new IllegalStateException("expected changed file-backed bytes at " + site);
            }
            counts.merge(type, 1, Integer::sum);
            relocated.put(site.getOffset(), bytes);
            println("%s v%d %s type=%d width=%d file+%#x original=%s loaded=%s values=%s symbol=%s"
                .formatted(variant, version, site, type, width, offset, hex.formatHex(original),
                    hex.formatHex(bytes), Arrays.toString(relocation.getValues()), relocation.getSymbolName()));
        }
        if (!counts.equals(RELOCATION_COUNTS) || relocated.size() != 17) {
            throw new IllegalStateException("unexpected ARM relocation coverage: " + counts);
        }
        for (var block : memory.getBlocks()) {
            println("%s %s %s-%s initialized=%s".formatted(
                variant, block.getName(), block.getStart(), block.getEnd(), block.isInitialized()));
        }
        return relocated;
    }
}
