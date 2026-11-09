// Golden vectors for tests/kp1_plugins.rs: the bundled VariableRenaming plugin's name providers
// (JADNameProvider, TinyNameProvider) driven call by call over the fixture's methods, and
// Character.isUpperCase/toLowerCase (TinyNameProvider) over every char. Each `op` line is one
// call and its result; the test replays them in order. Run through gen.sh.
import java.io.File;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.IVariableNameProvider;
import org.jetbrains.java.decompiler.main.plugins.PluginSources;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.Pair;
import org.vineflower.variablerenaming.JADNameProvider;
import org.vineflower.variablerenaming.TinyNameProvider;

public class GenPluginsOracle {
  static class JadProbe extends JADNameProvider {
    JadProbe(boolean renameParameters, StructMethod method) {
      super(renameParameters, method);
    }

    String next(String type) {
      return getNewName(type);
    }
  }

  static StructClass naming;
  static final Map<String, IVariableNameProvider> PROVIDERS = new HashMap<>();
  static String engine;
  static String caseName;

  static String esc(String s) {
    if (s == null) return "\\N";
    StringBuilder b = new StringBuilder();
    for (char c : s.toCharArray()) {
      switch (c) {
        case '\\' -> b.append("\\\\");
        case '\t' -> b.append("\\t");
        case '\n' -> b.append("\\n");
        default -> {
          if (c > 0x7e) b.append(String.format("\\u%04x", (int) c));
          else b.append(c);
        }
      }
    }
    return b.toString();
  }

  static void emit(String... fields) {
    StringBuilder b = new StringBuilder("op\t").append(engine).append('\t').append(caseName);
    for (String f : fields) b.append('\t').append(f);
    System.out.println(b);
  }

  // "N" = null; "@CODETYPE:dim:value" = new VarType(CodeType, dim[, value]); otherwise a descriptor
  static VarType type(String enc) {
    if (enc.equals("N")) return null;
    if (enc.startsWith("@")) {
      String[] p = enc.substring(1).split(":", -1);
      CodeType t = CodeType.valueOf(p[0]);
      int dim = Integer.parseInt(p[1]);
      return p[2].isEmpty() ? new VarType(t, dim) : new VarType(t, dim, p[2]);
    }
    return new VarType(enc);
  }

  static StructMethod method(String key) {
    String[] p = key.split(" ");
    StructMethod mt = naming.getMethod(p[0], p[1]);
    if (mt == null) throw new IllegalStateException(key);
    return mt;
  }

  static String fail(Throwable t) {
    while (t instanceof InvocationTargetException && t.getCause() != null) t = t.getCause();
    return "!" + t.getClass().getSimpleName();
  }

  static void start(String eng, String name) {
    engine = eng;
    caseName = name;
    PROVIDERS.clear();
  }

  static void create(String id, String methodKey, boolean renameParameters) {
    StructMethod mt = method(methodKey);
    IVariableNameProvider p = engine.equals("jad") ? new JadProbe(renameParameters, mt) : new TinyNameProvider(renameParameters, mt);
    PROVIDERS.put(id, p);
    emit("new", id, methodKey, renameParameters ? "1" : "0");
  }

  static void next(String id, String typeEnc, String cast) {
    IVariableNameProvider p = PROVIDERS.get(id);
    String result;
    try {
      if (p instanceof JadProbe jad) {
        result = esc(jad.next(cast));
      } else {
        Method m = TinyNameProvider.class.getDeclaredMethod("getNewName", Pair.class);
        m.setAccessible(true);
        result = esc((String) m.invoke(p, Pair.of(type(typeEnc), cast)));
      }
    } catch (Throwable t) {
      result = fail(t);
    }
    emit("next", id, typeEnc, esc(cast), result);
  }

  // entries: "var.version~type~cast" joined by '|'
  static void rename(String id, String entries) {
    Map<VarVersionPair, Pair<VarType, String>> map = new LinkedHashMap<>();
    for (String e : entries.split("\\|")) {
      String[] f = e.split("~", -1);
      String[] vv = f[0].split("\\.");
      map.put(new VarVersionPair(Integer.parseInt(vv[0]), Integer.parseInt(vv[1])), Pair.of(type(f[1]), f[2]));
    }
    String result;
    try {
      Map<VarVersionPair, String> r = PROVIDERS.get(id).rename(map);
      List<String> parts = new ArrayList<>();
      for (Map.Entry<VarVersionPair, String> en : r.entrySet()) {
        parts.add(en.getKey().var + "." + en.getKey().version + "=" + esc(en.getValue()));
      }
      result = String.join("|", parts);
    } catch (Throwable t) {
      result = fail(t);
    }
    emit("rename", id, esc(entries), result);
  }

  static void param(String id, int flags, String typeEnc, String name, int index) {
    String result;
    try {
      result = esc(PROVIDERS.get(id).renameParameter(flags, type(typeEnc), name, index));
    } catch (Throwable t) {
      result = fail(t);
    }
    emit("param", id, Integer.toString(flags), typeEnc, esc(name), Integer.toString(index), result);
  }

  static void parent(String child, String parent) {
    String result = "ok";
    try {
      PROVIDERS.get(child).addParentContext(PROVIDERS.get(parent));
    } catch (Throwable t) {
      result = fail(t);
    }
    emit("parent", child, parent, result);
  }

  static final String ST = "st (JDILjava/lang/String;Ljava/io/File;Ljava/lang/Class;)I";
  static final String INST = "inst ([I[[Ljava/lang/String;[Ljava/lang/Object;)V";
  static final String LAMBDAS = "lambdas (Ljava/lang/String;I)V";
  static final String KEYWORDS = "keywords (Ljava/lang/Class;Ljava/lang/Package;)V";
  static final String ABS = "abs (Ljava/lang/String;I)V";

  static void sequences(String eng) {
    start(eng, "ints");
    create("a", INST, false);
    for (int i = 0; i < 30; i++) next("a", "I", "int");
    for (int i = 0; i < 3; i++) next("a", "J", "long");
    for (int i = 0; i < 3; i++) next("a", "@BYTECHAR:0:", "int");
    next("a", "@SHORTCHAR:0:", "int");
    next("a", "[I", "int[]");
    next("a", "[I", "int[]");
    next("a", "[[I", "int[][]");
    next("a", "[[[I", "int[][][]");

    start(eng, "primitives");
    create("a", INST, false);
    for (String[] t : new String[][] {{"B", "byte"}, {"C", "char"}, {"S", "short"}, {"Z", "boolean"}, {"D", "double"},
                                       {"F", "float"}, {"J", "long"}}) {
      for (int i = 0; i < 3; i++) next("a", t[0], t[1]);
    }
    next("a", "@NULL:0:", "null");
    next("a", "@VOID:0:", "void");
    next("a", "@UNKNOWN:0:", "<unknown>");

    start(eng, "objects");
    create("a", INST, false);
    String[][] objects = {
      {"Ljava/lang/String;", "String"}, {"Ljava/lang/String;", "String"}, {"Ljava/lang/String;", "String"},
      {"Ljava/io/File;", "File"}, {"Ljava/io/File;", "File"}, {"Ljava/io/File;", "File"},
      {"Ljava/lang/Class;", "Class"}, {"Ljava/lang/Class;", "Class"}, {"Ljava/lang/Long;", "Long"},
      {"Ljava/lang/Byte;", "Byte"}, {"Ljava/lang/Short;", "Short"}, {"Ljava/lang/Boolean;", "Boolean"},
      {"Ljava/lang/Boolean;", "Boolean"}, {"Ljava/lang/Package;", "Package"}, {"Ljava/lang/Enum;", "Enum"},
      {"Ljava/lang/Enum;", "Enum"}, {"Ljava/lang/Object;", "Object"}, {"Ljava/lang/Object;", "Object"},
      {"Ljava/lang/Object;", "Object"}, {"[Ljava/lang/Object;", "Object[]"}, {"[Ljava/lang/Object;", "Object[]"},
      {"[Ljava/lang/Object;", "Object..."}, {"[Ljava/lang/Object;", "Object..."}, {"[[Ljava/lang/String;", "String[][]"},
      {"[[Ljava/lang/String;", "String[][]"}, {"[Ljava/lang/String;", "Strings"}, {"[Ljava/lang/String;", "Strings"},
      {"Ljava/util/Map$Entry;", "Map.Entry"}, {"Ljava/util/Map$Entry;", "Map.Entry"},
      {"Ljava/util/Map$Entry;", "Outer$Inner"}, {"Ljava/lang/String;", "string"}, {"Ljava/lang/String;", "string"},
      {"Lp/foo;", "foo"}, {"Lp/foo;", "foo"}, {"Lp/X;", ""}, {"Lp/Int;", "Int"}, {"Lp/Int;", "Int"},
      {"Lp/New;", "New"}, {"Lp/This;", "This"}, {"Lp/Var;", "Var"}, {"Lp/Record;", "Record"},
      {"Ljava/lang/Long;", "List<String>"}, {"Ljava/util/List;", "List"}, {"Ljava/util/List;", "List"},
      {"[Ljava/util/List;", "List[]"}, {"[[Ljava/util/List;", "List[][]"}, {"@GENVAR:0:T", "T"},
      {"@GENVAR:0:T", "T"}, {"@GENVAR:1:T", "T[]"}, {"@GENVAR:1:T", "Ts"},
    };
    for (String[] o : objects) next("a", o[0], o[1]);

    start(eng, "unicode");
    create("a", INST, false);
    String[] names = {"Ärger", "Ärger", "ÄRGER[]", "Ωmega", "ΣΑΣ", "İstanbul",
                      "İx", "ⅠV", "Ⓐb", "ß", "ǅab", "ẞ", "AÄ", "Z"};
    for (String n : names) next("a", "Lp/U;", n);

    start(eng, "static-this");
    create("a", ST, false);
    next("a", "Lp/This;", "This");
    next("a", "Lp/This;", "This");
    next("a", "Ljava/lang/Enum;", "Enum");
    create("b", INST, false);
    next("b", "Lp/This;", "This");
    next("b", "Ljava/lang/Enum;", "Enum");
    next("b", "N", "Null");
  }

  static void renames(String eng) {
    String stEntries = "0.0~J~long|2.0~D~double|4.0~I~int|5.0~Ljava/lang/String;~String|6.0~Ljava/io/File;~java.io.File"
                       + "|7.0~Ljava/lang/Class;~Class<?>|8.0~I~int|9.0~J~long|11.0~Z~boolean|12.0~Ljava/lang/String;~String"
                       + "|13.0~Ljava/io/File;~File|8.1~I~int|14.0~[I~int[]";
    String stParams = "0.0~N~long|2.0~N~double|4.0~N~int|5.0~N~String|6.0~N~File|7.0~N~Class<?>"
                      + "|8.0~I~int|9.0~J~long";
    String instEntries = "0.0~N~Naming|1.0~[I~int[]|2.0~[[Ljava/lang/String;~String[][]|3.0~[Ljava/lang/Object;~Object[]"
                         + "|4.0~I~int|5.0~I~int|6.0~I~int|7.0~I~int|8.0~I~int|9.0~Ljava/util/List;~List<String>"
                         + "|10.0~Ljava/util/Map$Entry;~Map.Entry<String, Integer>|11.0~Ljava/util/Map;~java.util.Map<String, List<Integer>>"
                         + "|12.0~C~char|13.0~B~byte|14.0~S~short|15.0~S~short|16.0~F~float|17.0~D~double|19.0~Ljava/lang/Character;~Character"
                         + "|20.0~Ljava/lang/Long;~Long|21.0~Ljava/lang/Byte;~Byte|22.0~Ljava/lang/Short;~Short"
                         + "|23.0~Ljava/lang/Boolean;~Boolean|24.0~Ljava/lang/Package;~Package|25.0~Ljava/lang/Enum;~Enum<?>"
                         + "|26.0~Ljava/lang/Class;~Class<?>|27.0~[Ljava/lang/Integer;~Integer[]|28.0~[[I~int[][]"
                         + "|29.0~Ljava/lang/StringBuilder;~StringBuilder|30.0~Lp/Naming$Inner;~Naming.Inner|31.0~Lp/Naming$Inner;~Naming$Inner";
    String unsorted = "9.2~I~int|9.0~I~int|3.1~Ljava/lang/String;~String|3.0~Ljava/lang/String;~String|1.0~I~int|7.0~Z~boolean";

    for (boolean rp : new boolean[] {false, true}) {
      start(eng, rp ? "st-params" : "st");
      create("a", ST, rp);
      rename("a", stEntries);
      rename("a", stEntries);
      start(eng, rp ? "st-lvt-params" : "st-lvt");
      create("a", ST, rp);
      rename("a", stParams);
      start(eng, rp ? "inst-params" : "inst");
      create("a", INST, rp);
      rename("a", instEntries);
      rename("a", unsorted);
      start(eng, rp ? "this-params" : "this");
      create("a", INST, rp);
      rename("a", "0.0~N~this|1.0~[I~int[]|4.0~I~int");
    }

    start(eng, "parent");
    create("p", LAMBDAS, false);
    rename("p", "3.0~I~int|4.0~Ljava/lang/String;~String|5.0~Ljava/lang/Runnable;~Runnable|6.0~Ljava/lang/Object;~Object");
    create("c", ST, false);
    parent("c", "p");
    rename("c", "8.0~I~int|9.0~Ljava/lang/String;~String|10.0~Ljava/lang/Object;~Object|11.0~Ljava/lang/Runnable;~Runnable");
    rename("p", "7.0~I~int|8.0~Ljava/lang/String;~String");
    create("d", KEYWORDS, false);
    parent("d", "c");
    parent("d", "d");
    rename("d", "2.0~I~int|3.0~Ljava/lang/Object;~Object|4.0~Ljava/lang/Class;~Class<?>");

    start(eng, "parameters");
    ImportCollector ic = new ImportCollector(new ClassNode(ClassNode.Type.ROOT, naming));
    DecompilerContext.startClass(ic);
    for (boolean rp : new boolean[] {false, true}) {
      String id = rp ? "on" : "off";
      create(id, ST, rp);
      param(id, 0x0009, "J", "a", 0);
      param(id, 0x0009, "D", "b", 2);
      param(id, 0x0009, "I", "c", 4);
      param(id, 0x0009, "Ljava/lang/String;", "d", 5);
      param(id, 0x0009, "Ljava/io/File;", "f", 6);
      param(id, 0x0009, "Ljava/lang/Class;", "k", 7);
      param(id, 0x0009, "I", "again", 4);
      param(id, 0x0009, "Ljava/lang/String;", null, 8);
      rename(id, "0.0~J~long|8.0~I~int|9.0~J~long|11.0~Z~boolean|12.0~Ljava/lang/String;~String");
      param(id, 0x0409, "I", "c", 4);
      param(id, 0x0409, "[Ljava/util/Map$Entry;", "es", 9);
      param(id, 0x0101, "Ljava/lang/Object;", "o", 10);
      param(id, 0x0101, "[[I", null, 11);
      param(id, 0x0001, "Lp/Naming$Inner;", "in", 12);
      param(id, 0x0001, "@GENVAR:0:T", "t", 13);
    }
    create("abs", ABS, false);
    param("abs", 0x0401, "Ljava/lang/String;", "name", 1);
    param("abs", 0x0401, "I", null, 2);
  }

  static void convert() {
    try {
      Method m = TinyNameProvider.class.getDeclaredMethod("convertToName", int.class);
      m.setAccessible(true);
      int[] idx = {-30, -27, -26, -25, -1, 0, 1, 7, 8, 11, 25, 26, 27, 51, 52, 675, 676, 677, 17575, 17576, 456975, 456976,
                   Integer.MAX_VALUE, Integer.MIN_VALUE};
      for (int i : idx) System.out.println("convert\t" + i + "\t" + esc((String) m.invoke(null, i)));
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static void chars() {
    for (int c = 0; c <= 0xFFFF; c++) {
      boolean upper = Character.isUpperCase((char) c);
      char lower = Character.toLowerCase((char) c);
      if (upper || lower != c) {
        System.out.println(String.format("char\t%04x\t%s\t%04x", c, upper ? "1" : "0", (int) lower));
      }
    }
  }

  public static void main(String[] args) throws Exception {
    Map<String, Object> options = new HashMap<>(IFernflowerPreferences.DEFAULTS);
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) {}
      @Override public void writeMessage(String message, Severity severity, Throwable t) {}
    };
    PluginSources.PLUGIN_SOURCES.clear();
    Fernflower fernflower = new Fernflower(null, options, logger);
    fernflower.addSource(new File(args[0], "classes"));
    naming = DecompilerContext.getStructContext().getClass("p/Naming");

    sequences("jad");
    sequences("tiny");
    renames("jad");
    renames("tiny");
    convert();
    chars();
  }
}
