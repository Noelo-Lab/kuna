import java.io.*;
import java.lang.reflect.Field;
import java.nio.charset.StandardCharsets;
import java.util.*;
import java.util.zip.*;

import org.vineflower.kotlin.metadata.BitEncoding;
import org.vineflower.kotlin.metadata.MetadataNameResolver;
import org.vineflower.kt.metadata.ProtoBuf;
import org.vineflower.kt.metadata.deserialization.Flags;
import org.vineflower.kt.metadata.jvm.JvmProtoBuf;
import org.vineflower.kt.protobuf.ExtensionRegistryLite;

/**
 * Oracle for the kuna-jvm KT1 slice: reads `kotlin/Metadata` the way
 * KotlinChooser.parseMetadataFor does and decodes it with the Kotlin plugin's
 * own BitEncoding, shaded protobuf classes and MetadataNameResolver.
 *
 *   jar <label> <jar> [<entry-list>]   every class (or the listed ones) with kotlin/Metadata
 *   cases <file>                       hand-written annotation values (raw syntax)
 *   flags                              the Flags field layout and sample values
 */
public class KtMetadataDump {
  static final ExtensionRegistryLite EXT = ExtensionRegistryLite.newInstance();
  static { JvmProtoBuf.registerAllExtensions(EXT); }

  static PrintStream out;
  static MetadataNameResolver resolver;

  public static void main(String[] args) throws Exception {
    out = new PrintStream(new FileOutputStream(FileDescriptor.out), false, "UTF-8");
    switch (args[0]) {
      case "jar" -> dumpJar(args[1], args[2], args.length > 3 ? args[3] : null);
      case "cases" -> dumpCases(args[1]);
      case "flags" -> dumpFlags();
      default -> throw new IllegalArgumentException(args[0]);
    }
    out.flush();
  }

  // ---- text

  static String esc(String s) {
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c <= 0x20 || c >= 0x7f || "\\:[]{}#".indexOf(c) >= 0) {
        b.append(String.format("\\u%04x", (int) c));
      } else {
        b.append(c);
      }
    }
    return b.toString();
  }

  static String unesc(String s) {
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c == '\\') {
        b.append((char) Integer.parseInt(s.substring(i + 2, i + 6), 16));
        i += 5;
      } else {
        b.append(c);
      }
    }
    return b.toString();
  }

  static String exc(Throwable e) {
    String n = e.getClass().getSimpleName();
    if (e instanceof NullPointerException || e instanceof ClassCastException || e.getMessage() == null) {
      return "!" + n;
    }
    return "!" + n + ": " + esc(e.getMessage());
  }

  static void p(String s) { out.println(s); }

  // ---- annotation values: the exprents the chooser casts

  static final class Elem {
    char tag;          // B C D F I J S Z s c e [ @
    Object value;      // Integer/Long/Float/Double/String; e: String[2]; [: List<Elem>; @: Anno
  }

  static final class Anno {
    String type;
    final List<String> names = new ArrayList<>();
    final List<Elem> values = new ArrayList<>();
  }

  static String raw(Elem e) {
    switch (e.tag) {
      case 'B', 'C', 'I', 'S', 'Z', 'J': return e.tag + ":" + constText(e.value);
      case 'F', 'D': return e.tag + ":" + constText(e.value);
      case 's': return "s:" + (e.value instanceof String s ? esc(s) : "#" + constText(e.value));
      case 'c': return "c:" + esc((String) e.value);
      case 'e': { String[] v = (String[]) e.value; return "e:" + esc(v[0]) + ":" + esc(v[1]); }
      case '[': {
        StringBuilder b = new StringBuilder("[");
        for (Elem x : (List<Elem>) e.value) b.append(' ').append(raw(x));
        return b.append(" ]").toString();
      }
      case '@': {
        Anno a = (Anno) e.value;
        StringBuilder b = new StringBuilder("@:" + esc(a.type) + " {");
        for (int i = 0; i < a.names.size(); i++) b.append(" n:").append(esc(a.names.get(i))).append(' ').append(raw(a.values.get(i)));
        return b.append(" }").toString();
      }
    }
    throw new IllegalStateException();
  }

  /** Pool values as typed text: Integer `I`, Long `J`, Float `F` (raw bits), Double `D` (raw bits), String `s`. */
  static String constText(Object v) {
    if (v instanceof Integer i) return "I" + i;
    if (v instanceof Long l) return "J" + l;
    if (v instanceof Float f) return "F" + Float.floatToRawIntBits(f);
    if (v instanceof Double d) return "D" + Double.doubleToRawLongBits(d);
    if (v instanceof String s) return "s" + esc(s);
    throw new IllegalStateException(String.valueOf(v));
  }

  static Object parseConst(String t) {
    return switch (t.charAt(0)) {
      case 'I' -> Integer.parseInt(t.substring(1));
      case 'J' -> Long.parseLong(t.substring(1));
      case 'F' -> Float.intBitsToFloat(Integer.parseInt(t.substring(1)));
      case 'D' -> Double.longBitsToDouble(Long.parseLong(t.substring(1)));
      case 's' -> unesc(t.substring(1));
      default -> throw new IllegalArgumentException(t);
    };
  }

  static Elem parseRaw(Iterator<String> it) {
    String t = it.next();
    Elem e = new Elem();
    e.tag = t.charAt(0);
    switch (e.tag) {
      case 'B', 'C', 'I', 'S', 'Z', 'J', 'F', 'D' -> e.value = parseConst(t.substring(2));
      case 's' -> e.value = t.startsWith("s:#") ? parseConst(t.substring(3)) : unesc(t.substring(2));
      case 'c' -> e.value = unesc(t.substring(2));
      case 'e' -> { String[] v = t.substring(2).split(":", -1); e.value = new String[]{unesc(v[0]), unesc(v[1])}; }
      case '[' -> {
        List<Elem> l = new ArrayList<>();
        while (true) {
          String n = peek(it);
          if (n.equals("]")) { it.next(); break; }
          l.add(parseRaw(it));
        }
        e.value = l;
      }
      case '@' -> {
        Anno a = new Anno();
        a.type = unesc(t.substring(2));
        it.next();
        while (true) {
          String n = it.next();
          if (n.equals("}")) break;
          a.names.add(unesc(n.substring(2)));
          a.values.add(parseRaw(it));
        }
        e.value = a;
      }
      default -> throw new IllegalArgumentException(t);
    }
    return e;
  }

  static String pending;
  static String peek(Iterator<String> it) { return ((PeekIt) it).peek(); }

  static final class PeekIt implements Iterator<String> {
    final List<String> toks; int i;
    PeekIt(List<String> t) { toks = t; }
    public boolean hasNext() { return i < toks.size(); }
    public String next() { return toks.get(i++); }
    String peek() { return toks.get(i); }
  }

  // ---- class files

  static Object[] readPool(DataInputStream in) throws IOException {
    int n = in.readUnsignedShort();
    Object[] cp = new Object[n];
    int[] refs = new int[n];
    for (int i = 1; i < n; i++) {
      int tag = in.readUnsignedByte();
      switch (tag) {
        case 1 -> cp[i] = in.readUTF();
        case 3 -> cp[i] = in.readInt();
        case 4 -> cp[i] = in.readFloat();
        case 5 -> { cp[i] = in.readLong(); i++; }
        case 6 -> { cp[i] = in.readDouble(); i++; }
        case 7, 8, 16, 19, 20 -> { refs[i] = in.readUnsignedShort(); cp[i] = new int[]{tag}; }
        case 9, 10, 11, 12, 17, 18 -> { in.readInt(); }
        case 15 -> { in.readUnsignedByte(); in.readUnsignedShort(); }
        default -> throw new IOException("bad pool tag " + tag);
      }
    }
    for (int i = 1; i < n; i++) {
      if (cp[i] instanceof int[]) cp[i] = cp[refs[i]];
    }
    return cp;
  }

  static void skipAttributes(DataInputStream in) throws IOException {
    int n = in.readUnsignedShort();
    for (int i = 0; i < n; i++) { in.readUnsignedShort(); in.skipNBytes(in.readInt() & 0xffffffffL); }
  }

  static String classValue(String descriptor) {
    String d = descriptor;
    while (d.startsWith("[")) d = d.substring(1);
    if (d.startsWith("L") && d.endsWith(";")) return d.substring(1, d.length() - 1);
    return switch (d) {
      case "B" -> "byte"; case "C" -> "char"; case "D" -> "double"; case "F" -> "float";
      case "I" -> "int"; case "J" -> "long"; case "S" -> "short"; case "Z" -> "boolean"; case "V" -> "void";
      default -> d;
    };
  }

  static String typeValue(String descriptor) {
    return descriptor.startsWith("L") && descriptor.endsWith(";") ? descriptor.substring(1, descriptor.length() - 1) : descriptor;
  }

  static Anno readAnno(DataInputStream in, Object[] cp) throws IOException {
    Anno a = new Anno();
    a.type = typeValue((String) cp[in.readUnsignedShort()]);
    int n = in.readUnsignedShort();
    for (int i = 0; i < n; i++) {
      a.names.add((String) cp[in.readUnsignedShort()]);
      a.values.add(readElem(in, cp));
    }
    return a;
  }

  static Elem readElem(DataInputStream in, Object[] cp) throws IOException {
    Elem e = new Elem();
    e.tag = (char) in.readUnsignedByte();
    switch (e.tag) {
      case 'e' -> e.value = new String[]{(String) cp[in.readUnsignedShort()], (String) cp[in.readUnsignedShort()]};
      case 'c' -> e.value = classValue((String) cp[in.readUnsignedShort()]);
      case '[' -> {
        int n = in.readUnsignedShort();
        List<Elem> l = new ArrayList<>();
        for (int i = 0; i < n; i++) l.add(readElem(in, cp));
        e.value = l;
      }
      case '@' -> e.value = readAnno(in, cp);
      default -> e.value = cp[in.readUnsignedShort()];
    }
    return e;
  }

  /** The first kotlin/Metadata in RuntimeVisibleAnnotations, then RuntimeInvisibleAnnotations. */
  static Anno findMetadata(byte[] bytes) throws IOException {
    DataInputStream in = new DataInputStream(new ByteArrayInputStream(bytes));
    in.readInt(); in.readUnsignedShort(); in.readUnsignedShort();
    Object[] cp = readPool(in);
    in.readUnsignedShort(); in.readUnsignedShort(); in.readUnsignedShort();
    in.skipNBytes(2L * in.readUnsignedShort());
    for (int k = 0; k < 2; k++) {
      int n = in.readUnsignedShort();
      for (int i = 0; i < n; i++) { in.readUnsignedShort(); in.readUnsignedShort(); in.readUnsignedShort(); skipAttributes(in); }
    }
    Map<String, List<Anno>> attrs = new HashMap<>();
    int n = in.readUnsignedShort();
    for (int i = 0; i < n; i++) {
      String name = (String) cp[in.readUnsignedShort()];
      int len = in.readInt();
      byte[] body = in.readNBytes(len);
      if (name.equals("RuntimeVisibleAnnotations") || name.equals("RuntimeInvisibleAnnotations")) {
        DataInputStream a = new DataInputStream(new ByteArrayInputStream(body));
        int count = a.readUnsignedShort();
        List<Anno> list = new ArrayList<>();
        for (int j = 0; j < count; j++) list.add(readAnno(a, cp));
        attrs.put(name, list);
      }
    }
    for (String key : new String[]{"RuntimeVisibleAnnotations", "RuntimeInvisibleAnnotations"}) {
      List<Anno> list = attrs.get(key);
      if (list != null) for (Anno a : list) if (a.type.equals("kotlin/Metadata")) return a;
    }
    return null;
  }

  // ---- the chooser's casts

  static boolean isConstExprent(Elem e) { return "BCDFIJSZsc".indexOf(e.tag) >= 0; }

  static int intValue(Elem e) {
    if (!isConstExprent(e)) throw new ClassCastException("not ConstExprent");
    return (int) (Integer) e.value;
  }

  static String stringValue(Elem e) {
    if (!isConstExprent(e)) throw new ClassCastException("not ConstExprent");
    return (String) e.value;
  }

  static String[] dataFromExpr(Elem e) {
    if (e.tag != '[') throw new ClassCastException("not NewExprent");
    List<Elem> l = (List<Elem>) e.value;
    String[] r = new String[l.size()];
    for (int i = 0; i < r.length; i++) r[i] = stringValue(l.get(i));
    return r;
  }

  // ---- records

  static void dumpJar(String label, String jar, String list) throws Exception {
    Set<String> only = null;
    if (list != null) {
      only = new HashSet<>();
      for (String l : java.nio.file.Files.readAllLines(java.nio.file.Path.of(list))) {
        if (l.startsWith(label + "!")) only.add(l.substring(label.length() + 1));
      }
    }
    try (ZipFile z = new ZipFile(jar)) {
      List<String> names = new ArrayList<>();
      for (Enumeration<? extends ZipEntry> en = z.entries(); en.hasMoreElements(); ) {
        String n = en.nextElement().getName();
        if (n.endsWith(".class") && !n.startsWith("META-INF/") && (only == null || only.contains(n))) names.add(n);
      }
      Collections.sort(names);
      for (String n : names) {
        byte[] bytes = z.getInputStream(z.getEntry(n)).readAllBytes();
        Anno a = findMetadata(bytes);
        if (a == null) continue;
        record(label + "!" + n, a);
      }
    }
  }

  static void dumpCases(String file) throws Exception {
    String name = null;
    Anno a = null;
    for (String line : java.nio.file.Files.readAllLines(java.nio.file.Path.of(file))) {
      line = line.strip();
      if (line.isEmpty() || line.startsWith("#")) continue;
      if (line.startsWith("case ")) { name = line.substring(5); a = new Anno(); a.type = "kotlin/Metadata"; continue; }
      if (line.equals("end")) { record("case!" + name, a); continue; }
      int sp = line.indexOf(' ');
      a.names.add(unesc(line.substring(0, sp)));
      PeekIt it = new PeekIt(Arrays.asList(line.substring(sp + 1).split(" ")));
      a.values.add(parseRaw(it));
    }
  }

  static void record(String label, Anno a) {
    p("=== " + label);
    for (int i = 0; i < a.names.size(); i++) p("raw " + esc(a.names.get(i)) + " " + raw(a.values.get(i)));
    resolver = null;
    decode(a);
    if (resolver != null) {
      int size = recordsSize(resolver);
      p("names " + size);
      for (int i = -1; i <= size; i++) p(" " + i + nm(i));
    }
    p("end");
  }

  static int recordsSize(MetadataNameResolver r) {
    try {
      Field f = MetadataNameResolver.class.getDeclaredField("records");
      f.setAccessible(true);
      return ((List<?>) f.get(r)).size();
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static void decode(Anno a) {
    int kIndex = a.names.indexOf("k");
    int d1Index = a.names.indexOf("d1");
    int d2Index = a.names.indexOf("d2");
    if (kIndex == -1) { p("nok"); return; }
    try {
      int k = intValue(a.values.get(kIndex));
      p("k " + k);
      if (d1Index == -1) { p("nod1"); return; }
      if (d2Index == -1) p("nod2");
      String[] data1 = dataFromExpr(a.values.get(d1Index));
      byte[] buf = BitEncoding.decodeBytes(data1);
      CRC32 crc = new CRC32();
      crc.update(buf);
      p("bytes " + buf.length + " " + String.format("%08x", crc.getValue()));
      ByteArrayInputStream input = new ByteArrayInputStream(buf);
      if (d2Index != -1) {
        String[] data2 = dataFromExpr(a.values.get(d2Index));
        JvmProtoBuf.StringTableTypes types = JvmProtoBuf.StringTableTypes.parseDelimitedFrom(input, EXT);
        p("types " + (types == null ? "null" : String.valueOf(types.getRecordList().size())) + " left " + input.available());
        MetadataNameResolver r = new MetadataNameResolver(types, data2);
        resolver = r;
        p("resolver " + recordsSize(r));
        if (types != null) for (JvmProtoBuf.StringTableTypes.Record rec : types.getRecordList()) dumpRecord(rec, " ");
      } else {
        p("resolver none");
      }
      if (k == 1) {
        ProtoBuf.Class c = ProtoBuf.Class.parseFrom(input, EXT);
        p("msg Class");
        dumpClass(c, " ");
      } else if (k == 2) {
        ProtoBuf.Package pk = ProtoBuf.Package.parseFrom(input, EXT);
        p("msg File");
        dumpPackage(pk, " ");
      } else if (k == 3) {
        ProtoBuf.Function f = ProtoBuf.Function.parseFrom(input, EXT);
        p("msg SyntheticClass");
        dumpFunction(f, " ");
      } else if (k == 4) {
        p("msg MultifileFacade " + data1.length);
        for (String s : data1) p(" " + esc(s));
      } else if (k == 5) {
        ProtoBuf.Package pk = ProtoBuf.Package.parseFrom(input, EXT);
        p("msg MultifilePart");
        dumpPackage(pk, " ");
        int xsIndex = a.names.indexOf("xs");
        p(xsIndex == -1 ? "xs none" : "xs " + esc(stringValue(a.values.get(xsIndex))));
      } else {
        p("msg none");
      }
    } catch (Exception e) {
      p(exc(e));
    } catch (Error e) {
      p("!" + exc(e));
    }
  }

  // ---- messages: every field the plugin reads

  static String nm(int v) {
    if (resolver == null) return "";
    try {
      return " = " + esc(resolver.resolve(v));
    } catch (Exception e) {
      return " = " + exc(e);
    }
  }

  static void i(String ind, String name, boolean has, int v, boolean isName) {
    p(ind + name + (has ? " + " : " - ") + v + (has && isName ? nm(v) : ""));
  }

  static void b(String ind, String name, boolean has, boolean v) {
    p(ind + name + (has ? " + " : " - ") + v);
  }

  static void e(String ind, String name, boolean has, Enum<?> v) {
    p(ind + name + (has ? " + " : " - ") + v.name());
  }

  static void list(String ind, String name, List<Integer> l) {
    StringBuilder s = new StringBuilder(ind + name + " " + l.size());
    for (Integer x : l) s.append(' ').append(x);
    p(s.toString());
  }

  static void flags(String ind, String name, boolean has, int v, Object[][] fields) {
    StringBuilder s = new StringBuilder(ind + name + (has ? " + " : " - ") + v + " {");
    for (Object[] f : fields) s.append(' ').append(f[0]).append('=').append(((Flags.FlagField<?>) f[1]).get(v));
    p(s.append(" }").toString());
  }

  static final Object[][] CLASS_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"VISIBILITY", Flags.VISIBILITY}, {"MODALITY", Flags.MODALITY},
    {"CLASS_KIND", Flags.CLASS_KIND}, {"IS_INNER", Flags.IS_INNER}, {"IS_DATA", Flags.IS_DATA},
    {"IS_EXTERNAL_CLASS", Flags.IS_EXTERNAL_CLASS}, {"IS_EXPECT_CLASS", Flags.IS_EXPECT_CLASS},
    {"IS_VALUE_CLASS", Flags.IS_VALUE_CLASS}, {"IS_FUN_INTERFACE", Flags.IS_FUN_INTERFACE},
    {"HAS_ENUM_ENTRIES", Flags.HAS_ENUM_ENTRIES}};
  static final Object[][] CONSTRUCTOR_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"VISIBILITY", Flags.VISIBILITY}, {"IS_SECONDARY", Flags.IS_SECONDARY},
    {"IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES", Flags.IS_CONSTRUCTOR_WITH_NON_STABLE_PARAMETER_NAMES},
    {"RETURN_VALUE_STATUS_CTOR", Flags.RETURN_VALUE_STATUS_CTOR}};
  static final Object[][] FUNCTION_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"VISIBILITY", Flags.VISIBILITY}, {"MODALITY", Flags.MODALITY},
    {"MEMBER_KIND", Flags.MEMBER_KIND}, {"IS_OPERATOR", Flags.IS_OPERATOR}, {"IS_INFIX", Flags.IS_INFIX},
    {"IS_INLINE", Flags.IS_INLINE}, {"IS_TAILREC", Flags.IS_TAILREC}, {"IS_EXTERNAL_FUNCTION", Flags.IS_EXTERNAL_FUNCTION},
    {"IS_SUSPEND", Flags.IS_SUSPEND}, {"IS_EXPECT_FUNCTION", Flags.IS_EXPECT_FUNCTION},
    {"IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES", Flags.IS_FUNCTION_WITH_NON_STABLE_PARAMETER_NAMES},
    {"RETURN_VALUE_STATUS_FUNCTION", Flags.RETURN_VALUE_STATUS_FUNCTION}};
  static final Object[][] PROPERTY_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"VISIBILITY", Flags.VISIBILITY}, {"MODALITY", Flags.MODALITY},
    {"MEMBER_KIND", Flags.MEMBER_KIND}, {"IS_VAR", Flags.IS_VAR}, {"HAS_GETTER", Flags.HAS_GETTER},
    {"HAS_SETTER", Flags.HAS_SETTER}, {"IS_CONST", Flags.IS_CONST}, {"IS_LATEINIT", Flags.IS_LATEINIT},
    {"HAS_CONSTANT", Flags.HAS_CONSTANT}, {"IS_EXTERNAL_PROPERTY", Flags.IS_EXTERNAL_PROPERTY},
    {"IS_DELEGATED", Flags.IS_DELEGATED}, {"IS_EXPECT_PROPERTY", Flags.IS_EXPECT_PROPERTY},
    {"RETURN_VALUE_STATUS_PROPERTY", Flags.RETURN_VALUE_STATUS_PROPERTY}};
  static final Object[][] ACCESSOR_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"VISIBILITY", Flags.VISIBILITY}, {"MODALITY", Flags.MODALITY},
    {"IS_NOT_DEFAULT", Flags.IS_NOT_DEFAULT}, {"IS_EXTERNAL_ACCESSOR", Flags.IS_EXTERNAL_ACCESSOR},
    {"IS_INLINE_ACCESSOR", Flags.IS_INLINE_ACCESSOR}};
  static final Object[][] PARAMETER_FLAGS = {
    {"HAS_ANNOTATIONS", Flags.HAS_ANNOTATIONS}, {"DECLARES_DEFAULT_VALUE", Flags.DECLARES_DEFAULT_VALUE},
    {"IS_CROSSINLINE", Flags.IS_CROSSINLINE}, {"IS_NOINLINE", Flags.IS_NOINLINE}};
  static final Object[][] EXPRESSION_FLAGS = {
    {"IS_NEGATED", Flags.IS_NEGATED}, {"IS_NULL_CHECK_PREDICATE", Flags.IS_NULL_CHECK_PREDICATE}};

  static void dumpRecord(JvmProtoBuf.StringTableTypes.Record r, String ind) {
    p(ind + "record");
    String n = ind + " ";
    i(n, "range", r.hasRange(), r.getRange(), false);
    i(n, "predefined_index", r.hasPredefinedIndex(), r.getPredefinedIndex(), false);
    String s;
    try {
      s = esc(r.getString());
    } catch (Exception ex) {
      s = exc(ex);
    }
    p(n + "string" + (r.hasString() ? " + " : " - ") + s);
    e(n, "operation", r.hasOperation(), r.getOperation());
    list(n, "substring_index", r.getSubstringIndexList());
    if (r.getSubstringIndexCount() != r.getSubstringIndexList().size()) throw new IllegalStateException();
    list(n, "replace_char", r.getReplaceCharList());
    if (r.getReplaceCharCount() != r.getReplaceCharList().size()) throw new IllegalStateException();
  }

  static void dumpClass(ProtoBuf.Class c, String ind) {
    flags(ind, "flags", c.hasFlags(), c.getFlags(), CLASS_FLAGS);
    i(ind, "fq_name", c.hasFqName(), c.getFqName(), true);
    i(ind, "companion_object_name", c.hasCompanionObjectName(), c.getCompanionObjectName(), true);
    String n = ind + " ";
    p(ind + "supertype " + c.getSupertypeCount());
    for (ProtoBuf.Type t : c.getSupertypeList()) dumpType(t, n);
    p(ind + "constructor " + c.getConstructorCount());
    for (ProtoBuf.Constructor x : c.getConstructorList()) dumpConstructor(x, n);
    p(ind + "function " + c.getFunctionCount());
    for (ProtoBuf.Function x : c.getFunctionList()) dumpFunction(x, n);
    p(ind + "property " + c.getPropertyCount());
    for (ProtoBuf.Property x : c.getPropertyList()) dumpProperty(x, n);
    p(ind + "type_alias " + c.getTypeAliasCount());
    for (ProtoBuf.TypeAlias x : c.getTypeAliasList()) dumpTypeAlias(x, n);
    p(ind + "type_table" + (c.hasTypeTable() ? " +" : " -"));
    if (c.hasTypeTable()) dumpTypeTable(c.getTypeTable(), n);
    i(ind, "ext.class_module_name", c.hasExtension(JvmProtoBuf.classModuleName), c.getExtension(JvmProtoBuf.classModuleName), true);
    p(ind + "ext.class_local_variable " + c.getExtension(JvmProtoBuf.classLocalVariable).size());
    for (ProtoBuf.Property x : c.getExtension(JvmProtoBuf.classLocalVariable)) dumpProperty(x, n);
    i(ind, "ext.anonymous_object_origin_name", c.hasExtension(JvmProtoBuf.anonymousObjectOriginName), c.getExtension(JvmProtoBuf.anonymousObjectOriginName), true);
    i(ind, "ext.jvm_class_flags", c.hasExtension(JvmProtoBuf.jvmClassFlags), c.getExtension(JvmProtoBuf.jvmClassFlags), false);
  }

  static void dumpPackage(ProtoBuf.Package c, String ind) {
    String n = ind + " ";
    p(ind + "function " + c.getFunctionCount());
    for (ProtoBuf.Function x : c.getFunctionList()) dumpFunction(x, n);
    p(ind + "property " + c.getPropertyCount());
    for (ProtoBuf.Property x : c.getPropertyList()) dumpProperty(x, n);
    p(ind + "type_alias " + c.getTypeAliasCount());
    for (ProtoBuf.TypeAlias x : c.getTypeAliasList()) dumpTypeAlias(x, n);
    p(ind + "type_table" + (c.hasTypeTable() ? " +" : " -"));
    if (c.hasTypeTable()) dumpTypeTable(c.getTypeTable(), n);
    i(ind, "ext.package_module_name", c.hasExtension(JvmProtoBuf.packageModuleName), c.getExtension(JvmProtoBuf.packageModuleName), true);
    p(ind + "ext.package_local_variable " + c.getExtension(JvmProtoBuf.packageLocalVariable).size());
    for (ProtoBuf.Property x : c.getExtension(JvmProtoBuf.packageLocalVariable)) dumpProperty(x, n);
  }

  static void dumpFunction(ProtoBuf.Function f, String ind) {
    p(ind + "Function");
    ind = ind + " ";
    String n = ind + " ";
    flags(ind, "flags", f.hasFlags(), f.getFlags(), FUNCTION_FLAGS);
    i(ind, "name", f.hasName(), f.getName(), true);
    p(ind + "return_type" + (f.hasReturnType() ? " +" : " -"));
    if (f.hasReturnType()) dumpType(f.getReturnType(), n);
    p(ind + "type_parameter " + f.getTypeParameterCount());
    for (ProtoBuf.TypeParameter x : f.getTypeParameterList()) dumpTypeParameter(x, n);
    p(ind + "receiver_type" + (f.hasReceiverType() ? " +" : " -"));
    if (f.hasReceiverType()) dumpType(f.getReceiverType(), n);
    p(ind + "value_parameter " + f.getValueParameterCount());
    for (ProtoBuf.ValueParameter x : f.getValueParameterList()) dumpValueParameter(x, n);
    p(ind + "context_receiver_type " + f.getContextReceiverTypeCount());
    for (ProtoBuf.Type x : f.getContextReceiverTypeList()) dumpType(x, n);
    p(ind + "type_table" + (f.hasTypeTable() ? " +" : " -"));
    if (f.hasTypeTable()) dumpTypeTable(f.getTypeTable(), n);
    p(ind + "contract" + (f.hasContract() ? " +" : " -"));
    if (f.hasContract()) dumpContract(f.getContract(), n);
    p(ind + "ext.method_signature" + (f.hasExtension(JvmProtoBuf.methodSignature) ? " +" : " -"));
    dumpMethodSignature(f.getExtension(JvmProtoBuf.methodSignature), n);
    i(ind, "ext.lambda_class_origin_name", f.hasExtension(JvmProtoBuf.lambdaClassOriginName), f.getExtension(JvmProtoBuf.lambdaClassOriginName), true);
  }

  static void dumpProperty(ProtoBuf.Property f, String ind) {
    p(ind + "Property");
    ind = ind + " ";
    String n = ind + " ";
    flags(ind, "flags", f.hasFlags(), f.getFlags(), PROPERTY_FLAGS);
    i(ind, "name", f.hasName(), f.getName(), true);
    p(ind + "return_type" + (f.hasReturnType() ? " +" : " -"));
    if (f.hasReturnType()) dumpType(f.getReturnType(), n);
    p(ind + "setter_value_parameter" + (f.hasSetterValueParameter() ? " +" : " -"));
    if (f.hasSetterValueParameter()) dumpValueParameter(f.getSetterValueParameter(), n);
    flags(ind, "getter_flags", f.hasGetterFlags(), f.getGetterFlags(), ACCESSOR_FLAGS);
    flags(ind, "setter_flags", f.hasSetterFlags(), f.getSetterFlags(), ACCESSOR_FLAGS);
    p(ind + "ext.property_signature" + (f.hasExtension(JvmProtoBuf.propertySignature) ? " +" : " -"));
    JvmProtoBuf.JvmPropertySignature s = f.getExtension(JvmProtoBuf.propertySignature);
    p(n + "field" + (s.hasField() ? " +" : " -"));
    dumpFieldSignature(s.getField(), n + " ");
    p(n + "synthetic_method" + (s.hasSyntheticMethod() ? " +" : " -"));
    if (s.hasSyntheticMethod()) dumpMethodSignature(s.getSyntheticMethod(), n + " ");
    p(n + "getter" + (s.hasGetter() ? " +" : " -"));
    if (s.hasGetter()) dumpMethodSignature(s.getGetter(), n + " ");
    p(n + "setter" + (s.hasSetter() ? " +" : " -"));
    if (s.hasSetter()) dumpMethodSignature(s.getSetter(), n + " ");
    i(ind, "ext.flags", f.hasExtension(JvmProtoBuf.flags), f.getExtension(JvmProtoBuf.flags), false);
  }

  static void dumpConstructor(ProtoBuf.Constructor c, String ind) {
    p(ind + "Constructor");
    ind = ind + " ";
    String n = ind + " ";
    flags(ind, "flags", c.hasFlags(), c.getFlags(), CONSTRUCTOR_FLAGS);
    p(ind + "value_parameter " + c.getValueParameterCount());
    for (ProtoBuf.ValueParameter x : c.getValueParameterList()) dumpValueParameter(x, n);
    p(ind + "ext.constructor_signature" + (c.hasExtension(JvmProtoBuf.constructorSignature) ? " +" : " -"));
    dumpMethodSignature(c.getExtension(JvmProtoBuf.constructorSignature), n);
  }

  static void dumpValueParameter(ProtoBuf.ValueParameter v, String ind) {
    p(ind + "ValueParameter");
    ind = ind + " ";
    String n = ind + " ";
    flags(ind, "flags", v.hasFlags(), v.getFlags(), PARAMETER_FLAGS);
    i(ind, "name", v.hasName(), v.getName(), true);
    p(ind + "type" + (v.hasType() ? " +" : " -"));
    if (v.hasType()) dumpType(v.getType(), n);
    p(ind + "vararg_element_type" + (v.hasVarargElementType() ? " +" : " -"));
    if (v.hasVarargElementType()) dumpType(v.getVarargElementType(), n);
    i(ind, "type_id", v.hasTypeId(), v.getTypeId(), false);
  }

  static void dumpType(ProtoBuf.Type t, String ind) {
    p(ind + "Type");
    ind = ind + " ";
    String n = ind + " ";
    p(ind + "argument " + t.getArgumentCount());
    for (ProtoBuf.Type.Argument a : t.getArgumentList()) {
      p(n + "Argument");
      e(n + " ", "projection", a.hasProjection(), a.getProjection());
      p(n + " type" + (a.hasType() ? " +" : " -"));
      if (a.hasType()) dumpType(a.getType(), n + "  ");
      i(n + " ", "type_id", a.hasTypeId(), a.getTypeId(), false);
    }
    b(ind, "nullable", t.hasNullable(), t.getNullable());
    i(ind, "class_name", t.hasClassName(), t.getClassName(), true);
    i(ind, "type_parameter_name", t.hasTypeParameterName(), t.getTypeParameterName(), true);
    i(ind, "type_alias_name", t.hasTypeAliasName(), t.getTypeAliasName(), true);
    b(ind, "ext.is_raw", t.hasExtension(JvmProtoBuf.isRaw), t.getExtension(JvmProtoBuf.isRaw));
  }

  static void dumpTypeParameter(ProtoBuf.TypeParameter t, String ind) {
    p(ind + "TypeParameter");
    ind = ind + " ";
    i(ind, "id", t.hasId(), t.getId(), false);
    i(ind, "name", t.hasName(), t.getName(), true);
    b(ind, "reified", t.hasReified(), t.getReified());
    e(ind, "variance", t.hasVariance(), t.getVariance());
    p(ind + "upper_bound " + t.getUpperBoundCount());
    for (ProtoBuf.Type x : t.getUpperBoundList()) dumpType(x, ind + " ");
  }

  static void dumpTypeTable(ProtoBuf.TypeTable t, String ind) {
    p(ind + "type " + t.getTypeCount());
    for (ProtoBuf.Type x : t.getTypeList()) dumpType(x, ind + " ");
    i(ind, "first_nullable", t.hasFirstNullable(), t.getFirstNullable(), false);
  }

  static void dumpTypeAlias(ProtoBuf.TypeAlias t, String ind) {
    p(ind + "TypeAlias");
    ind = ind + " ";
    i(ind, "name", t.hasName(), t.getName(), true);
    p(ind + "underlying_type" + (t.hasUnderlyingType() ? " +" : " -"));
    if (t.hasUnderlyingType()) dumpType(t.getUnderlyingType(), ind + " ");
  }

  static void dumpContract(ProtoBuf.Contract c, String ind) {
    p(ind + "effect " + c.getEffectCount());
    for (ProtoBuf.Effect x : c.getEffectList()) {
      String n = ind + "  ";
      p(ind + " Effect");
      e(n, "effect_type", x.hasEffectType(), x.getEffectType());
      p(n + "effect_constructor_argument " + x.getEffectConstructorArgumentCount());
      for (ProtoBuf.Expression y : x.getEffectConstructorArgumentList()) dumpExpression(y, n + " ");
      p(n + "conclusion_of_conditional_effect" + (x.hasConclusionOfConditionalEffect() ? " +" : " -"));
      if (x.hasConclusionOfConditionalEffect()) dumpExpression(x.getConclusionOfConditionalEffect(), n + " ");
      e(n, "kind", x.hasKind(), x.getKind());
    }
  }

  static void dumpExpression(ProtoBuf.Expression x, String ind) {
    p(ind + "Expression");
    ind = ind + " ";
    String n = ind + " ";
    flags(ind, "flags", x.hasFlags(), x.getFlags(), EXPRESSION_FLAGS);
    i(ind, "value_parameter_reference", x.hasValueParameterReference(), x.getValueParameterReference(), false);
    e(ind, "constant_value", x.hasConstantValue(), x.getConstantValue());
    p(ind + "is_instance_type" + (x.hasIsInstanceType() ? " +" : " -"));
    if (x.hasIsInstanceType()) dumpType(x.getIsInstanceType(), n);
    i(ind, "is_instance_type_id", x.hasIsInstanceTypeId(), x.getIsInstanceTypeId(), false);
    p(ind + "and_argument " + x.getAndArgumentCount());
    for (ProtoBuf.Expression y : x.getAndArgumentList()) dumpExpression(y, n);
    p(ind + "or_argument " + x.getOrArgumentCount());
    for (ProtoBuf.Expression y : x.getOrArgumentList()) dumpExpression(y, n);
  }

  static void dumpMethodSignature(JvmProtoBuf.JvmMethodSignature s, String ind) {
    i(ind, "name", s.hasName(), s.getName(), true);
    i(ind, "desc", s.hasDesc(), s.getDesc(), true);
  }

  static void dumpFieldSignature(JvmProtoBuf.JvmFieldSignature s, String ind) {
    i(ind, "name", s.hasName(), s.getName(), true);
    i(ind, "desc", s.hasDesc(), s.getDesc(), true);
  }

  // ---- flags layout

  static void dumpFlags() throws Exception {
    for (Field f : Flags.class.getFields()) {
      Flags.FlagField<?> ff = (Flags.FlagField<?>) f.get(null);
      StringBuilder s = new StringBuilder(f.getName() + " " + ff.offset + " " + ff.bitWidth);
      int mask = ((1 << ff.bitWidth) - 1) << ff.offset;
      for (int v = 0; v < (1 << ff.bitWidth); v++) {
        s.append(' ').append(ff.get(v << ff.offset)).append('/').append(ff.get((v << ff.offset) | ~mask));
      }
      p(s.toString());
    }
  }
}
