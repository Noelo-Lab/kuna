// Generate the raw and packed Ghidra FID fixtures for fid_db_status_cli.rs.
// Ghidra 12.1.2: analyzeHeadless <scratch> fid-audit -deleteProject \
//   -import <fixtures>/fid_audit_x86_64 -scriptPath <fixtures> \
//   -postScript MakeFidAudit.java <output-directory>
import db.DBHandle;
import db.DBRecord;
import db.RecordIterator;
import db.Schema;
import db.Table;
import ghidra.app.script.GhidraScript;
import ghidra.feature.fid.db.FidDB;
import ghidra.feature.fid.db.FidFileManager;
import ghidra.feature.fid.db.LibraryRecord;
import ghidra.feature.fid.hash.FidHashQuad;
import ghidra.feature.fid.service.FidService;
import ghidra.framework.Application;
import ghidra.program.model.lang.CompilerSpecID;
import ghidra.program.model.lang.Language;
import ghidra.program.model.lang.LanguageID;
import ghidra.program.model.listing.Function;
import java.io.File;

public class MakeFidAudit extends GhidraScript {
    @Override
    public void run() throws Exception {
        File out = new File(getScriptArgs()[0]);
        File packed = new File(out, "fid_audit_ghidra.fidb");
        File raw = new File(out, "fid_audit_ghidra.fidbf");
        FidFileManager manager = FidFileManager.getInstance();
        File metadata = new File(out, "fid_audit_ghidra_metadata.fidbf");
        manager.createNewFidDatabase(packed);
        Language language = currentProgram.getLanguage();
        CompilerSpecID compiler = currentProgram.getCompilerSpec().getCompilerSpecID();
        FidService service = new FidService();
        try (FidDB db = manager.addUserFidFile(packed).getFidDB(true)) {
            LibraryRecord compatible = db.createNewLibrary("kuna fixture", "1", "audit",
                Application.getApplicationVersion(), new LanguageID("x86:LE:64:compat32"),
                language.getVersion(), language.getMinorVersion(), compiler);
            LibraryRecord incompatible = db.createNewLibrary("unrelated library", "1", "audit",
                Application.getApplicationVersion(), new LanguageID("x86:LE:32:default"),
                language.getVersion(), language.getMinorVersion(), new CompilerSpecID("default"));
            LibraryRecord overflow = db.createNewLibrary("overflow library", "1", "audit",
                Application.getApplicationVersion(), new LanguageID("AARCH64:LE:64:v8A"),
                language.getVersion(), language.getMinorVersion(), new CompilerSpecID("default"));
            Function mix = getFunctionAt(toAddr(0x401000));
            Function fold = getFunctionAt(toAddr(0x401020));
            FidHashQuad mixHash = service.hashFunction(mix);
            FidHashQuad foldHash = service.hashFunction(fold);
            db.createNewFunction(compatible, mixHash, "audit_mix", 0x401000,
                "fid_audit_x86_64", true);
            db.setAutoFailOnFunction(db.createNewFunction(compatible, mixHash, "rejected_alias",
                0x401000, "fid_audit_x86_64", true), true);
            db.setForceRelationOnFunction(db.createNewFunction(compatible, foldHash, "audit_fold",
                0x401020, "fid_audit_x86_64", true), true);
            db.createNewFunction(incompatible, mixHash, "foreign_alias", 0x401000,
                "fid_audit_x86_64", true);
            // For stock 16 KiB blocks the symbol fits in one overflow data node,
            // while the path needs an index node and multiple overflow data nodes.
            String longName = "kuna::audit::" + "nested_template::".repeat(300) + "probe";
            String longPath = "/fixture/" + "generated/".repeat(2400) + "input";
            db.createNewFunction(overflow, mixHash, longName, 0x401000, longPath, true);
            db.createNewFunction(overflow, foldHash, longName, 0x401020, longPath, true);
            db.saveDatabase("Kuna Ghidra FID interoperability fixture", monitor);
        }
        try (FidDB db = manager.addUserFidFile(packed).getFidDB(false)) {
            db.saveRawDatabaseFile(raw, monitor);
        }
        saveMetadataVariant(raw, metadata);
    }

    // 12.1.2's FID facade still emits the old v6 schema. Use the stock DB API
    // to write the new schema and its metadata values with the same record IDs.
    private void saveMetadataVariant(File raw, File output) throws Exception {
        DBHandle source = new DBHandle(raw);
        try {
            DBHandle target = new DBHandle(source.getBufferSize());
            try {
                long transaction = target.startTransaction();
                boolean success = false;
                try {
                    for (Table table : source.getTables()) {
                        boolean libraries = table.getName().equals("Libraries Table");
                        Schema schema = table.getSchema();
                        if (libraries) {
                            String[] columns = schema.getFieldNames().clone();
                            columns[7] = "Library Metadata";
                            schema = new Schema(schema.getVersion(), schema.getKeyName(),
                                schema.getFields(), columns);
                        }
                        Table copy = target.createTable(table.getName(), schema,
                            table.getIndexedColumns());
                        RecordIterator records = table.iterator();
                        while (records.hasNext()) {
                            DBRecord record = records.next();
                            if (libraries) {
                                switch (record.getString(4)) {
                                    case "x86:LE:64:compat32":
                                        record.setString(7, "gcc,windows:c,cpp");
                                        break;
                                    case "AARCH64:LE:64:v8A":
                                        record.setString(7, ":cpp");
                                        break;
                                    default:
                                        // Metadata providers also accept compiler-only values.
                                        break;
                                }
                            }
                            copy.putRecord(record);
                        }
                    }
                    success = true;
                }
                finally {
                    target.endTransaction(transaction, success);
                }
                target.saveAs(output, false, monitor);
            }
            finally {
                target.close();
            }
        }
        finally {
            source.close();
        }
    }
}
