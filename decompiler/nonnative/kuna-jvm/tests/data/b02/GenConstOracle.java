// Golden vectors for ConstExprent (toJava, getPrecedence, toString and the static
// uninlining tables) from Vineflower 1.12.0 itself. Regenerate with
// `sh gen.sh <vineflower-1.12.0.jar>`.
//
// The context is made the way Fernflower's constructor makes one (no runtime); the
// current class node is pkg.Hello (../f6/tree/pkg/Hello.class) and every row runs on a
// fresh ImportCollector. Rows are tab-separated, strings escaped as in esc() (null = \N),
// a throw is "EXC:<simple class name>:<message>".
//   KD i bits / KF i bits    UNINLINED_DOUBLES / UNINLINED_FLOATS keys in iteration order
//   NP kind:bits             NO_PAREN_VALUES members, sorted
//   R ctor type value opts toJava precedence
//     ctor  T = ConstExprent(VarType, Object, null) / ConstExprent(.., null, wasCondy)
//           G0 / G1 = ConstExprent(int, boolPermitted, null)
//     type  a VarType.VARTYPE_* field name
//     value N | I:int | J:long | F:floatBitsHex | D:doubleBitsHex | S:escapedString
//     opts  four flags: keep-literals, ascii-strings, decompiler-comments, wasCondy; only
//           the flags that can matter for the value's type are varied
//     toJava is the text plus TextTokenDumpVisitor's listing, the listing dropped when empty
//   TS ctor type value opts toString
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.extern.TextTokenVisitor;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.token.TextTokenDumpVisitor;

import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;

public class GenConstOracle {
  static final StringBuilder OUT = new StringBuilder();
  static ClassesProcessor.ClassNode NODE;

  static String esc(String s) {
    if (s == null) return "\\N";
    StringBuilder b = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
      char c = s.charAt(i);
      if (c == '\\') b.append("\\\\");
      else if (c == '\t') b.append("\\t");
      else if (c == '\n') b.append("\\n");
      else if (c == '\r') b.append("\\r");
      else if (c < 0x20 || c > 0x7e) b.append(String.format("\\u%04x", (int) c));
      else b.append(c);
    }
    return b.toString();
  }

  static String run(Supplier<String> op) {
    try {
      return esc(op.get());
    } catch (Throwable t) {
      return "EXC:" + t.getClass().getSimpleName() + ":" + esc(t.getMessage());
    }
  }

  static final String NO_TOKENS = "\n/*\nTokens:\n*/\n";

  static String dump(TextBuffer buf) {
    TextBuffer[] holder = {buf};
    buf.visitTokens(TextTokenVisitor.createVisitor(next -> new TextTokenDumpVisitor(next, holder[0])));
    String text = buf.convertToStringAndAllowDataDiscard();
    return text.endsWith(NO_TOKENS) ? text.substring(0, text.length() - NO_TOKENS.length()) : text;
  }

  static void row(String... fields) {
    OUT.append(String.join("\t", fields)).append('\n');
  }

  static Object field(String name) throws Exception {
    Field f = ConstExprent.class.getDeclaredField(name);
    f.setAccessible(true);
    return f.get(null);
  }

  static VarType type(String name) {
    try {
      return (VarType) VarType.class.getField(name).get(null);
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static Object value(String spec) {
    if (spec.equals("N")) return null;
    String v = spec.substring(2);
    switch (spec.charAt(0)) {
      case 'I': return Integer.parseInt(v);
      case 'J': return Long.parseLong(v);
      case 'F': return Float.intBitsToFloat(Integer.parseUnsignedInt(v, 16));
      case 'D': return Double.longBitsToDouble(Long.parseUnsignedLong(v, 16));
      case 'S': return v;
      default: throw new IllegalStateException(spec);
    }
  }

  static String fspec(float f) {
    return "F:" + Integer.toHexString(Float.floatToRawIntBits(f));
  }

  static String dspec(double d) {
    return "D:" + Long.toHexString(Double.doubleToRawLongBits(d));
  }

  static final String[][] LIT = {{"0", "0"}, {"1", "0"}};
  static final String[][] ASC = {{"0", "0"}, {"0", "1"}};
  static final String[][] ALL = {{"0", "0"}, {"1", "0"}, {"0", "1"}, {"1", "1"}};
  static final String[][] ONE = {{"0", "0"}};

  static String[][] optsFor(String ctor, String typeName) {
    if (ctor.startsWith("G")) return LIT;
    switch (typeName) {
      case "VARTYPE_CHAR": case "VARTYPE_CHARACTER": case "VARTYPE_STRING": return ASC;
      case "VARTYPE_BOOLEAN": case "VARTYPE_BOOLEAN_OBJ": case "VARTYPE_NULL": case "VARTYPE_CLASS":
      case "VARTYPE_METHODTYPE": return ONE;
      case "VARTYPE_OBJECT": case "VARTYPE_UNKNOWN": return ALL;
      default: return LIT;
    }
  }

  static boolean toStringRow = false;

  static void render(String ctor, String typeName, String valueSpec, String com, String condy) {
    for (String[] o : optsFor(ctor, typeName)) {
      DecompilerContext.setProperty(IFernflowerPreferences.LITERALS_AS_IS, o[0]);
      DecompilerContext.setProperty(IFernflowerPreferences.ASCII_STRING_CHARACTERS, o[1]);
      DecompilerContext.setProperty(IFernflowerPreferences.DECOMPILER_COMMENTS, com);
      DecompilerContext.startClass(new ImportCollector(NODE));
      ConstExprent[] holder = new ConstExprent[1];
      String made = run(() -> {
        Object val = value(valueSpec);
        if (ctor.equals("T")) {
          holder[0] = condy.equals("1")
            ? new ConstExprent(type(typeName), val, null, true)
            : new ConstExprent(type(typeName), val, null);
        } else {
          holder[0] = new ConstExprent((Integer) val, ctor.equals("G1"), null);
        }
        return "";
      });
      String opts = o[0] + o[1] + com + condy;
      if (!made.isEmpty()) {
        row("R", ctor, typeName, esc(valueSpec), opts, made, made);
        continue;
      }
      ConstExprent c = holder[0];
      String text = run(() -> dump(c.toJava(0)));
      DecompilerContext.startClass(new ImportCollector(NODE));
      String prec = run(() -> Integer.toString(c.getPrecedence()));
      row("R", ctor, typeName, esc(valueSpec), opts, text, prec);
      if (toStringRow) {
        DecompilerContext.startClass(new ImportCollector(NODE));
        row("TS", ctor, typeName, esc(valueSpec), opts, run(c::toString));
      }
    }
  }

  static void render(String ctor, String typeName, String valueSpec) {
    render(ctor, typeName, valueSpec, "1", "0");
  }

  @SuppressWarnings("unchecked")
  public static void main(String[] args) throws Exception {
    Map<String, Object> props = new HashMap<>();
    props.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    new Fernflower(null, props, IFernflowerLogger.NO_OP);

    byte[] bytes = Files.readAllBytes(Paths.get(args[0]));
    StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
    NODE = new ClassesProcessor.ClassNode(ClassesProcessor.ClassNode.Type.ROOT, cl);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, NODE);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);

    Map<Double, ?> doubles = (Map<Double, ?>) field("UNINLINED_DOUBLES");
    Map<Float, ?> floats = (Map<Float, ?>) field("UNINLINED_FLOATS");
    Set<Object> noParen = (Set<Object>) field("NO_PAREN_VALUES");
    int i = 0;
    for (Double d : doubles.keySet()) row("KD", Integer.toString(i++), dspec(d).substring(2));
    i = 0;
    for (Float f : floats.keySet()) row("KF", Integer.toString(i++), fspec(f).substring(2));
    TreeSet<String> np = new TreeSet<>();
    for (Object o : noParen) np.add(o instanceof Double ? dspec((Double) o) : fspec((Float) o));
    for (String s : np) row("NP", s);

    // int family, including every escape, printable class and surrogate edge
    TreeSet<Integer> ints = new TreeSet<>(Arrays.asList(
      0, 1, -1, 2, 5, 7, 8, 9, 10, 11, 12, 13, 14, 31, 32, 33, 34, 39, 48, 65, 92, 97, 126, 127, 128, 129, 159,
      160, 173, 255, 256, 1000, 0x2028, 0x2029, 0x200b, 0x0378, 0x4e2d, 0x7fff, 0x8000, 0xd7ff, 0xd800, 0xdbff,
      0xdc00, 0xdfff, 0xe000, 0xf8ff, 0xfeff, 0xfffd, 0xfffe, 0xffff, 0x10000, 0x10008, 0x1000a, 0x10027,
      0xff, 0xff00, 0xff0000, 0xff000000, 0x7f, -128, -129, 32767, -32768, 65535, 65536,
      Integer.MAX_VALUE, Integer.MIN_VALUE, Integer.MAX_VALUE - 1, Integer.MIN_VALUE + 1, 123456789, -987654321));
    String[] intTypes = {"VARTYPE_INT", "VARTYPE_BYTE", "VARTYPE_SHORT", "VARTYPE_CHAR", "VARTYPE_BYTECHAR",
      "VARTYPE_SHORTCHAR", "VARTYPE_BOOLEAN", "VARTYPE_INTEGER", "VARTYPE_CHARACTER", "VARTYPE_BYTE_OBJ",
      "VARTYPE_SHORT_OBJ", "VARTYPE_BOOLEAN_OBJ"};
    for (int v : ints) {
      for (String t : intTypes) render("T", t, "I:" + v);
      render("G0", "VARTYPE_INT", "I:" + v);
      render("G1", "VARTYPE_INT", "I:" + v);
    }
    TreeSet<Integer> chars = new TreeSet<>();
    for (int c = 0; c < 0x180; c++) chars.add(c);
    for (int block = 0; block < 0x100; block++) {
      for (int lo : new int[]{0x00, 0x80, 0xff}) chars.add(block * 0x100 + lo);
    }
    chars.removeAll(ints);
    for (int c : chars) render("T", "VARTYPE_CHAR", "I:" + c);

    long[] longs = {0, 1, -1, 2, 10, 255, 0xffffffffL, 1L << 40, -(1L << 40), Long.MAX_VALUE, Long.MIN_VALUE,
      Long.MAX_VALUE - 1, Long.MIN_VALUE + 1, 1234567890123456789L, -42};
    for (long v : longs) {
      render("T", "VARTYPE_LONG", "J:" + v);
      render("T", "VARTYPE_LONG_OBJ", "J:" + v);
    }

    Random rnd = new Random(0xB02L);
    LinkedHashSet<Float> fl = new LinkedHashSet<>(Arrays.asList(
      0.0f, -0.0f, 1.0f, -1.0f, 0.1f, -0.1f, 0.5f, 0.25f, 0.3f, 1.1f, 2.5f, 3.0f, 10.0f, 100.0f, 123.456f, 1e-5f,
      1.0E-10f, 1e7f, 1.0E10f, 9999999.0f, 1.2345678E7f, 3.4028235E38f, Float.MIN_VALUE, Float.MIN_NORMAL,
      Float.MAX_VALUE, -Float.MAX_VALUE, Float.NaN, Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY,
      (float) Math.PI, (float) Math.E, 0.33333334f, 0.6666667f, 1.4E-45f, 0.017453292f, 57.29578f));
    fl.addAll(floats.keySet());
    for (Double d : doubles.keySet()) fl.add(d.floatValue());
    for (Float f : new ArrayList<>(fl)) {
      if (fl.size() > 500) break;
      if (!Float.isNaN(f) && !Float.isInfinite(f)) {
        fl.add(Math.nextUp(f));
        fl.add(Math.nextDown(f));
      }
    }
    for (int k = 0; k < 100; k++) {
      fl.add(Float.intBitsToFloat(rnd.nextInt()));
      fl.add((float) (rnd.nextInt(2000000) - 1000000) / (float) Math.pow(10, rnd.nextInt(8)));
    }
    int n = 0;
    for (float f : fl) {
      render("T", "VARTYPE_FLOAT", fspec(f));
      if (n++ % 25 == 0) render("T", "VARTYPE_FLOAT_OBJ", fspec(f));
    }

    LinkedHashSet<Double> dl = new LinkedHashSet<>(Arrays.asList(
      0.0, -0.0, 1.0, -1.0, 0.1, -0.1, 0.5, 0.25, 0.3, 1.1, 2.5, 3.0, 10.0, 100.0, 123.456, 1e-5, 1.0E-10, 1e7,
      1.0E10, 9999999.0, 1.2345678E7, 1.0E100, 1.7976931348623157E308, Double.MIN_VALUE, Double.MIN_NORMAL,
      Double.MAX_VALUE, -Double.MAX_VALUE, Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY,
      Math.PI, Math.E, 1.0 / 3, 2.0 / 3, 0.017453292519943295, 57.29577951308232, 4.9E-324, 3.4028234663852886E38,
      0.1f + 0.0, 0.3f + 0.0, 1.1f + 0.0, 123.456f + 0.0));
    dl.addAll(doubles.keySet());
    for (Float f : floats.keySet()) dl.add(f.doubleValue());
    for (Double d : new ArrayList<>(dl)) {
      if (dl.size() > 900) break;
      if (!Double.isNaN(d) && !Double.isInfinite(d)) {
        dl.add(Math.nextUp(d));
        dl.add(Math.nextDown(d));
      }
    }
    for (int k = 0; k < 100; k++) {
      dl.add(Double.longBitsToDouble(rnd.nextLong()));
      dl.add((double) Float.intBitsToFloat(rnd.nextInt()));
      dl.add((double) (rnd.nextInt(2000000) - 1000000) / Math.pow(10, rnd.nextInt(12)));
    }
    n = 0;
    for (double d : dl) {
      render("T", "VARTYPE_DOUBLE", dspec(d));
      if (n++ % 25 == 0) render("T", "VARTYPE_DOUBLE_OBJ", dspec(d));
    }

    String[] strings = {"", "a", "hello world", "tab\there", "nl\nx", "cr\rx", "ff\fx", "bs\bx", "q\"q", "ap'ap",
      "back\\slash", "\u0000", "\u0001\u001f", "\u007f", "\u0080\u009f", " ­", "café",
      "中文", "  ", "​", "﻿", "�", "￿", "😀", "\ud800", "x\udc00y",
      "􏿿", "", "͸", "mixé\t\"\\ \ud800", "$var$", "\\u0041"};
    for (String s : strings) render("T", "VARTYPE_STRING", "S:" + s);

    toStringRow = true;
    render("T", "VARTYPE_INT", "I:2147483647");
    render("T", "VARTYPE_CHAR", "I:10");
    render("T", "VARTYPE_DOUBLE", dspec(Math.PI / 2));
    render("T", "VARTYPE_NULL", "N");
    render("T", "VARTYPE_NULL", "I:0");
    for (String s : new String[]{"java/lang/String", "[I", "[[Ljava/lang/Object;", "pkg/Hello", "java/util/Map$Entry", "I"}) {
      render("T", "VARTYPE_CLASS", "S:" + s);
    }
    for (String t : new String[]{"VARTYPE_STRING", "VARTYPE_CLASS", "VARTYPE_INT", "VARTYPE_OBJECT", "VARTYPE_INTEGER"}) {
      render("T", t, "N");
    }
    for (String com : new String[]{"0", "1"}) {
      render("T", "VARTYPE_METHODTYPE", "S:(ILjava/lang/String;)V", com, "0");
      render("T", "VARTYPE_METHODTYPE", "S:()Ljava/lang/Object;", com, "1");
    }
    render("T", "VARTYPE_INT", "I:42", "1", "1");
    render("T", "VARTYPE_STRING", "S:condy", "1", "1");
    render("T", "VARTYPE_DOUBLE", dspec(Math.PI), "1", "1");
    render("T", "VARTYPE_INT", "S:notanint");
    render("T", "VARTYPE_FLOAT", "I:3");
    render("T", "VARTYPE_UNKNOWN", "I:5");
    render("T", "VARTYPE_OBJECT", "I:5");
    render("T", "VARTYPE_BOOLEAN", "J:1");

    System.out.print(OUT);
  }
}
