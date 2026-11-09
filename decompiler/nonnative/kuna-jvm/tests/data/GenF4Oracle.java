// Golden vectors for the F4 slice: options, ConverterHelper, PrintStreamLogger,
// DecompilerContext.getIntOption, the StructContext class universe and
// IdentifierConverter over f4_classes/. Run from this directory:
//   javac --release 8 -d f4_classes f4_classes/src/o/*.java f4_classes/src/p/*.java f4_classes/src/q/*.java
//   java -cp vineflower-1.12.0.jar GenF4Oracle.java .
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.io.PrintStream;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IContextSource;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IResultSaver;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IIdentifierRenamer;
import org.jetbrains.java.decompiler.modules.renamer.ConverterHelper;
import org.jetbrains.java.decompiler.modules.renamer.IdentifierConverter;
import org.jetbrains.java.decompiler.modules.renamer.PoolInterceptor;
import org.jetbrains.java.decompiler.struct.ContextUnit;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructContext;

public class GenF4Oracle {
  static String esc(String s) {
    if (s == null) return "\\N";
    StringBuilder b = new StringBuilder();
    for (char c : s.toCharArray()) {
      switch (c) {
        case '\\' -> b.append("\\\\");
        case '\t' -> b.append("\\t");
        case '\n' -> b.append("\\n");
        case '\r' -> b.append("\\r");
        default -> b.append(c);
      }
    }
    return b.toString();
  }

  static String hex(String s) {
    if (s == null) return "N";
    StringBuilder b = new StringBuilder("x");
    for (char c : s.toCharArray()) b.append(String.format("%04x", (int) c));
    return b.toString();
  }

  static String b(boolean v) {
    return v ? "1" : "0";
  }

  static final class MemorySource implements IContextSource {
    final String name;
    final boolean lazy;
    final List<IContextSource.Entry> classes = new ArrayList<>();
    final List<IContextSource.Entry> others = new ArrayList<>();
    final Map<String, byte[]> data = new HashMap<>();

    MemorySource(String name, boolean lazy) {
      this.name = name;
      this.lazy = lazy;
    }

    MemorySource cls(String entry, int multirelease, Path file) throws Exception {
      classes.add(new IContextSource.Entry(entry, multirelease));
      if (multirelease == IContextSource.Entry.BASE_VERSION) data.put(entry + ".class", Files.readAllBytes(file));
      return this;
    }

    MemorySource other(String path, String content) {
      others.add(IContextSource.Entry.atBase(path));
      data.put(path, content.getBytes(StandardCharsets.UTF_8));
      return this;
    }

    @Override public String getName() { return name; }
    @Override public Entries getEntries() { return new Entries(classes, List.of(), others, List.of()); }
    @Override public boolean isLazy() { return lazy; }
    @Override public InputStream getInputStream(String resource) {
      byte[] b = data.get(resource);
      return b == null ? null : new ByteArrayInputStream(b);
    }
  }

  static final class RecordingLogger extends IFernflowerLogger {
    final List<String> lines;
    RecordingLogger(List<String> lines) { this.lines = lines; }
    @Override public void writeMessage(String message, Severity severity) {
      if (accepts(severity)) lines.add(String.join("\t", "L", severity.name(), esc(message)));
    }
    @Override public void writeMessage(String message, Severity severity, Throwable t) {
      if (accepts(severity)) lines.add(String.join("\t", "L", severity.name(), esc(message)));
    }
  }

  static final String ABSTRACT_NAMES = "p/A foo (IJLjava/lang/String;)V x y z\np/A bar ()V\n\np/A baz (D[J)V d arr\nshort\n";
  static final String[] UNIVERSE = {"p/I", "p/J", "p/A", "p/B", "p/C", "p/D", "p/E", "p/F", "p/G", "q/L", "q/K", "q/M",
    "java/lang/Object", "java/lang/Runnable", "x/Unknown", "wrong/Name"};

  static String nameOf(StructClass cl) {
    return cl == null ? "\\N" : cl.qualifiedName + "/" + (cl.isOwn() ? "own" : "lib");
  }

  static void structContext(Path out, Path classes) throws Exception {
    List<String> lines = new ArrayList<>();
    RecordingLogger logger = new RecordingLogger(lines);
    logger.setSeverity(IFernflowerLogger.Severity.TRACE);
    IResultSaver saver = new IResultSaver() {
      public void saveFolder(String path) {}
      public void copyFile(String source, String path, String entryName) {}
      public void saveClassFile(String path, String qualifiedName, String entryName, String content, int[] mapping) {}
      public void createArchive(String path, String archiveName, java.util.jar.Manifest manifest) {}
      public void saveDirEntry(String path, String archiveName, String entryName) {}
      public void copyEntry(String source, String path, String archiveName, String entry) {}
      public void saveClassEntry(String path, String archiveName, String qualifiedName, String entryName, String content) {}
      public void closeArchive(String path, String archiveName) {}
    };
    StructContext sc = new StructContext(saver, null);
    DecompilerContext ctx = new DecompilerContext(new HashMap<>(IFernflowerPreferences.DEFAULTS), logger, sc, new ClassesProcessor(sc), null);
    DecompilerContext.setCurrentContext(ctx);

    int base = IContextSource.Entry.BASE_VERSION;
    MemorySource lib1 = new MemorySource("lib1", false).cls("p/D", base, classes.resolve("p/D.class"))
      .cls("q/L", base, classes.resolve("q/L.class")).cls("q/K", base, classes.resolve("q/K.class"));
    MemorySource own = new MemorySource("own", false);
    for (String n : new String[]{"p/I", "p/J", "p/A", "p/B", "p/C", "p/D", "p/E", "p/F"}) own.cls(n, base, classes.resolve(n + ".class"));
    own.cls("wrong/Name", base, classes.resolve("p/G.class")).cls("p/MR", 11, classes.resolve("p/D.class"))
      .other("fernflower_abstract_parameter_names.txt", ABSTRACT_NAMES).other("META-INF/MANIFEST.MF", "Manifest-Version: 1.0\n");
    MemorySource lib2 = new MemorySource("lib2", false).cls("p/B", base, classes.resolve("p/B.class"));
    MemorySource lazy = new MemorySource("lazy", true).cls("q/M", base, classes.resolve("q/M.class"));

    sc.addSpace(lib1, false);
    sc.addSpace(own, true);
    sc.addSpace(lib2, false);
    sc.addSpace(lazy, false);
    queries(sc, lines);
    lines.add("== reload");
    sc.reloadContext();
    queries(sc, lines);
    lines.add("== clear");
    sc.clear();
    lines.add("O\t" + String.join(" ", sc.getOwnClasses().stream().map(GenF4Oracle::nameOf).toList()));
    lines.add("U\t" + sc.getUnits().size());
    Files.write(out.resolve("f4_struct_context.txt"), lines, StandardCharsets.UTF_8);
  }

  static void queries(StructContext sc, List<String> lines) {
    lines.add("O\t" + String.join(" ", sc.getOwnClasses().stream().map(GenF4Oracle::nameOf).toList()));
    for (ContextUnit unit : sc.getUnits()) {
      lines.add(String.join("\t", "U", unit.getName(), b(unit.isOwn()), b(unit.isRoot()), b(unit.isLazy()),
        String.join(" ", unit.getClassNames()), String.join(" ", unit.getOtherEntries().stream().map(IContextSource.Entry::path).toList())));
    }
    for (String n : UNIVERSE) lines.add(String.join("\t", "G", n, nameOf(sc.getClass(n))));
    for (String n : UNIVERSE) lines.add(String.join("\t", "H", n, b(sc.hasClass(n))));
    for (String a : UNIVERSE) {
      StringBuilder inst = new StringBuilder();
      List<String> anc = new ArrayList<>();
      for (String c : UNIVERSE) {
        inst.append(sc.instanceOf(a, c) ? '1' : '0');
        StructClass common = sc.findCommonAncestor(a, c);
        anc.add(common == null ? "-" : common.qualifiedName);
      }
      lines.add(String.join("\t", "I", a, inst.toString()));
      lines.add(String.join("\t", "A", a, String.join(" ", anc)));
    }
    for (String[] q : new String[][]{{"p/A", "foo", "(IJLjava/lang/String;)V"}, {"p/A", "baz", "(D[J)V"}, {"p/A", "bar", "()V"}, {"p/B", "foo", "(IJLjava/lang/String;)V"}}) {
      List<String> names = new ArrayList<>();
      for (int i = -0; i < 5; i++) names.add(sc.renameAbstractParameter(q[0], q[1], q[2], i, "def" + i));
      lines.add(String.join("\t", "P", q[0], q[1], q[2], String.join(" ", names)));
    }
  }

  public static void main(String[] args) throws Exception {
    Path out = Path.of(args.length > 0 ? args[0] : ".");
    String threads = String.valueOf(Runtime.getRuntime().availableProcessors());

    List<String> prefs = new ArrayList<>();
    for (Field f : IFernflowerPreferences.class.getDeclaredFields()) {
      if (f.getType() != String.class) continue;
      IFernflowerPreferences.Name n = f.getAnnotation(IFernflowerPreferences.Name.class);
      IFernflowerPreferences.Description d = f.getAnnotation(IFernflowerPreferences.Description.class);
      IFernflowerPreferences.ShortName sn = f.getAnnotation(IFernflowerPreferences.ShortName.class);
      IFernflowerPreferences.Type t = f.getAnnotation(IFernflowerPreferences.Type.class);
      IFernflowerPreferences.DynamicDefaultValue dd = f.getAnnotation(IFernflowerPreferences.DynamicDefaultValue.class);
      prefs.add(String.join("\t", "F", f.getName(), esc((String) f.get(null)), esc(sn == null ? null : sn.value()),
        esc(n == null ? null : n.value()), esc(d == null ? null : d.value()),
        esc(t == null ? null : t.value().name()), esc(dd == null ? null : dd.value())));
    }
    for (Map.Entry<String, Object> e : IFernflowerPreferences.getDefaults().entrySet()) {
      String v = e.getKey().equals(IFernflowerPreferences.THREADS) && threads.equals(e.getValue()) ? "@THREADS@" : e.getValue().toString();
      prefs.add(String.join("\t", "D", esc(e.getKey()), esc(v)));
    }
    for (Map.Entry<String, Object> e : new HashMap<>(IFernflowerPreferences.DEFAULTS).entrySet()) {
      String v = e.getKey().equals(IFernflowerPreferences.THREADS) && threads.equals(e.getValue()) ? "@THREADS@" : e.getValue().toString();
      prefs.add(String.join("\t", "C", esc(e.getKey()), esc(v)));
    }
    Files.write(out.resolve("f4_preferences.txt"), prefs, StandardCharsets.UTF_8);

    List<String> ids = new ArrayList<>(List.of(
      "", "a", "ab", "abc", "abcd", "abcde", "a$", "$", "$$", "$$$$", "a$b", "ab$cd", "abc$def$ghi", "abcd$ef",
      "ab$cdef", "a$b$c$d", "_", "__", "___", "____", "Foo", "Bar", "Main", "Ab1", "1a", "12", "123abc", "9Inner",
      "1", "1$", "<init>", "<clinit>", "<inita>", "class", "Class", "int", "goto", "const", "enum", "assert", "true",
      "null", "var", "record", "yield", "con", "CON", "Con", "con1", "com1", "COM9", "lpt5", "LPT1", "aux", "Aux",
      "nul", "prn", "PRN.txt", "com0", "lpt10", "a b", "a-b", "a.b", "a/b", "\u0000", "a\u0000b", "a\u0001",
      "ab\u007f", "a​b", "a­b", "é", "café", "ñame", "日本", "αβγ",
      "İ", "K", "İcon", "a\n", "a\nb", "ab\r", " ", "a\u0085", "x y", "𝐀",
      "a𝐀", "a\uD800", "\uDC00a", "٣abc", "１２Ａ", "Ⅻ", "a·b", "€uro",
      "_1", "a_b_c", "0Foo", "00Foo", "11111x", "Inner", "2Inner", "3Inner", "$Foo", "$1", "Foo$1", "Foo$Bar",
      "abcdefgh", "ABCD", "ABC", "AB$", "a$$", "ab$$", "abcd$", "ééé", "éééé"));
    ids.add("a".repeat(249));
    ids.add("a".repeat(250));
    ids.add("x".repeat(300));
    ids.add("a$".repeat(5));
    ids.add("ab$".repeat(10));
    ids.add("abcd".repeat(3));
    Random rnd = new Random(42);
    char[] alphabet = {'a', 'b', 'Z', '1', '7', '$', '_', 'é', '\u0000', '.', ' ', '٣'};
    for (int i = 0; i < 300; i++) {
      int len = 1 + rnd.nextInt(7);
      StringBuilder s = new StringBuilder();
      for (int j = 0; j < len; j++) s.append(alphabet[rnd.nextInt(alphabet.length)]);
      ids.add(s.toString());
    }

    List<String> conv = new ArrayList<>();
    ConverterHelper probe = new ConverterHelper();
    for (String id : ids) {
      conv.add(String.join("\t", "R", hex(id),
        b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_CLASS, id, null, null)),
        b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_FIELD, "pkg/Cls", id, "I")),
        b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_METHOD, "pkg/Cls", id, "()V"))));
    }
    conv.add(String.join("\t", "R", "N", b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_CLASS, null, null, null)),
      b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_FIELD, "pkg/Cls", null, "I")),
      b(probe.toBeRenamed(IIdentifierRenamer.Type.ELEMENT_METHOD, "pkg/Cls", null, "()V"))));
    ConverterHelper naming = new ConverterHelper();
    conv.add(String.join("\t", "N", "N", hex(naming.getNextClassName("pkg/X", null))));
    for (String id : ids) {
      conv.add(String.join("\t", "N", hex(id), hex(naming.getNextClassName("pkg/" + id, id))));
    }
    conv.add(String.join("\t", "N", "N", hex(naming.getNextClassName("pkg/X", null))));
    for (int i = 0; i < 3; i++) {
      conv.add(String.join("\t", "M", hex(naming.getNextFieldName("pkg/X", "f", "I")), hex(naming.getNextMethodName("pkg/X", "m", "()V"))));
    }
    for (String full : List.of("pkg/Foo", "Foo", "a/b/c/D", "a/b/", "", "/x", "é/èç", "a$b/c$d")) {
      conv.add(String.join("\t", "S", hex(full), hex(ConverterHelper.getSimpleClassName(full)),
        hex(ConverterHelper.replaceSimpleClassName(full, "New"))));
    }
    Files.write(out.resolve("f4_converter_helper.txt"), conv, StandardCharsets.UTF_8);

    Map<String, Object> props = new HashMap<>(IFernflowerPreferences.DEFAULTS);
    StructContext sc = new StructContext(null, null);
    DecompilerContext ctx = new DecompilerContext(props, IFernflowerLogger.NO_OP, sc, new ClassesProcessor(sc), null);
    DecompilerContext.setCurrentContext(ctx);

    List<String> log = new ArrayList<>();
    for (IFernflowerLogger.Severity sev : IFernflowerLogger.Severity.values()) {
      ByteArrayOutputStream bytes = new ByteArrayOutputStream();
      PrintStream ps = new PrintStream(bytes, true, StandardCharsets.UTF_8);
      PrintStreamLogger logger = new PrintStreamLogger(ps);
      logger.setSeverity(sev);
      logger.writeMessage("t", IFernflowerLogger.Severity.TRACE);
      logger.writeMessage("i", IFernflowerLogger.Severity.INFO);
      logger.writeMessage("w", IFernflowerLogger.Severity.WARN);
      logger.writeMessage("e", IFernflowerLogger.Severity.ERROR);
      logger.startProcessingClass("A");
      logger.writeMessage("in A", IFernflowerLogger.Severity.WARN);
      logger.startReadingClass("B");
      logger.startClass("C");
      logger.startMethod("m ()V");
      logger.writeMessage("deep", IFernflowerLogger.Severity.ERROR);
      logger.writeMessage("deep trace", IFernflowerLogger.Severity.TRACE);
      logger.endMethod();
      logger.startWriteClass("W");
      logger.writeMessage("w2", IFernflowerLogger.Severity.INFO);
      logger.endWriteClass();
      logger.endClass();
      logger.endReadingClass();
      logger.endProcessingClass();
      logger.writeMessage("out", IFernflowerLogger.Severity.INFO);
      ps.flush();
      log.add("== " + sev.name());
      for (String line : bytes.toString(StandardCharsets.UTF_8).split("\n", -1)) {
        log.add(esc(line));
      }
    }
    Files.write(out.resolve("f4_logger.txt"), log, StandardCharsets.UTF_8);

    List<String> ints = new ArrayList<>();
    for (String v : new String[]{"0", "1", "160", "-5", "+5", "-0", "007", " 12", "12 ", "", "+", "-", "2147483647",
      "2147483648", "-2147483648", "-2147483649", "1e3", "0x10", "abc", "٣", "１２", "99999999999"}) {
      DecompilerContext.setProperty("int-probe", v);
      ints.add(String.join("\t", "I", hex(v), String.valueOf(DecompilerContext.getIntOption("int-probe")), b(DecompilerContext.getOption("int-probe"))));
    }
    ints.add(String.join("\t", "I", "N", String.valueOf(DecompilerContext.getIntOption("absent-probe")), b(DecompilerContext.getOption("absent-probe"))));
    Files.write(out.resolve("f4_context.txt"), ints, StandardCharsets.UTF_8);

    structContext(out, out.resolve("f4_classes"));
    identifierConverter(out, out.resolve("f4_classes"));
  }

  static final String[] OBFUSCATED = {"o/a", "o/b", "o/c", "o/d", "o/e", "o/Good", "o/f"};

  @SuppressWarnings("unchecked")
  static void identifierConverter(Path out, Path classes) throws Exception {
    StructContext sc = new StructContext(null, null);
    PoolInterceptor interceptor = new PoolInterceptor();
    DecompilerContext ctx = new DecompilerContext(new HashMap<>(IFernflowerPreferences.DEFAULTS), IFernflowerLogger.NO_OP, sc, new ClassesProcessor(sc), interceptor);
    DecompilerContext.setCurrentContext(ctx);
    MemorySource own = new MemorySource("own", false);
    for (String n : OBFUSCATED) own.cls(n, IContextSource.Entry.BASE_VERSION, classes.resolve(n + ".class"));
    sc.addSpace(own, true);
    new IdentifierConverter(sc, new ConverterHelper(), interceptor).rename();
    List<String> lines = new ArrayList<>();
    for (String field : new String[]{"mapOldToNewNames", "mapNewToOldNames"}) {
      Field f = PoolInterceptor.class.getDeclaredField(field);
      f.setAccessible(true);
      for (Map.Entry<String, String> e : ((Map<String, String>) f.get(interceptor)).entrySet()) {
        lines.add(String.join("\t", field.equals("mapOldToNewNames") ? "O" : "N", esc(e.getKey()), esc(e.getValue())));
      }
    }
    Files.write(out.resolve("f4_identifier_converter.txt"), lines, StandardCharsets.UTF_8);
  }
}
