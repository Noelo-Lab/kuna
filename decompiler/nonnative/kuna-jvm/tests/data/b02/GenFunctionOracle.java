// Golden vectors for FunctionExprent and AssignmentExprent over constant and static
// field leaves, from Vineflower 1.12.0 itself. Regenerate with `sh gen.sh <jar>`.
//
// The context is the one GenConstOracle makes, with preferred-line-length 40 so that
// reformat() exercises the newline groups, and with b02/B02Fields (args[0]) as the
// current class. Each tree is written in prefix form and every column is computed on a
// tree freshly built from that text:
//   tree   (FUNC[!][@TYPE] operand...) | (=[FUNC] left right) | C:TYPE:value | G0:int |
//          G1:int | T:signature | FS:name:descriptor | FX:class:name:descriptor
//          ! = setNeedsCast(false), @TYPE = setImplicitType(VarType.TYPE),
//          = = AssignmentExprent(left, right, FUNC or null, null),
//          FS / FX = a static FieldExprent of B02Fields / of another class,
//          C = ConstExprent(VarType.TYPE, value, null), G = ConstExprent(int, boolPermitted, null),
//          T = ConstExprent(new VarType(signature), null, null); values as in GenConstOracle,
//          strings as S:x followed by hex UTF-16 units
//   F tree exprType inferred inferredText bounds precedence toJava reformatted
//     inferred/inferredText: getInferredExprType(null), then toJava of that same tree
//     bounds: per Function/AssignmentExprent in preorder, "i:" + L<j>=type / U<j>=type pairs (or the
//             throw), joined by " | "; i and j are preorder indices
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.AssignmentExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FieldExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.vars.CheckTypesResult;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.TextBuffer;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;

public class GenFunctionOracle {
  static final StringBuilder OUT = new StringBuilder();
  static ClassesProcessor.ClassNode NODE;
  static final Random RND = new Random(0xF0B02L);

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
      case 'S': {
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i < v.length(); i += 4) sb.append((char) Integer.parseInt(v.substring(i, i + 4), 16));
        return sb.toString();
      }
      default: throw new IllegalStateException(spec);
    }
  }

  // ---- prefix-form parser (mirrored by b02_function_exprent.rs) ----

  static String[] toks;
  static int pos;

  static Exprent build(String tree) {
    toks = tree.replace("(", " ( ").replace(")", " ) ").trim().split("\\s+");
    pos = 0;
    return parse();
  }

  static Exprent parse() {
    String t = toks[pos++];
    if (t.equals("(")) {
      String head = toks[pos++];
      if (head.startsWith("=")) {
        Exprent left = parse();
        Exprent right = parse();
        pos++;
        FunctionType cond = head.length() > 1 ? FunctionType.valueOf(head.substring(1)) : null;
        return new AssignmentExprent(left, right, cond, null);
      }
      String implicit = null;
      int at = head.indexOf('@');
      if (at >= 0) {
        implicit = head.substring(at + 1);
        head = head.substring(0, at);
      }
      boolean noCast = head.endsWith("!");
      if (noCast) head = head.substring(0, head.length() - 1);
      List<Exprent> ops = new ArrayList<>();
      while (!toks[pos].equals(")")) ops.add(parse());
      pos++;
      FunctionExprent f = new FunctionExprent(FunctionType.valueOf(head), ops, null);
      if (noCast) f.setNeedsCast(false);
      if (implicit != null) f.setImplicitType(type(implicit));
      return f;
    }
    if (t.startsWith("G")) {
      return new ConstExprent(Integer.parseInt(t.substring(3)), t.charAt(1) == '1', null);
    }
    if (t.startsWith("T:")) {
      return new ConstExprent(new VarType(t.substring(2)), null, null);
    }
    if (t.startsWith("FS:") || t.startsWith("FX:")) {
      String[] p = t.split(":");
      String cls = t.startsWith("FS:") ? "b02/B02Fields" : p[1];
      String name = p[p.length - 2];
      String desc = p[p.length - 1];
      return new FieldExprent(name, cls, true, null, FieldDescriptor.parseDescriptor(desc), null);
    }
    int colon = t.indexOf(':', 2);
    return new ConstExprent(type(t.substring(2, colon)), value(t.substring(colon + 1)), null);
  }

  // ---- random trees ----

  static String hexString(String s) {
    StringBuilder sb = new StringBuilder("S:x");
    for (char c : s.toCharArray()) sb.append(String.format("%04x", (int) c));
    return sb.toString();
  }

  static final String[] LEAVES;
  static {
    List<String> l = new ArrayList<>();
    for (int v : new int[]{0, 1, 2, 3, 7, 10, 65, 97, 127, 255, 0xff00, 0xffff, 65535, 100000, -1, -5}) {
      l.add("C:VARTYPE_INT:I:" + v);
      l.add("G0:" + v);
    }
    l.add("G1:0");
    l.add("G1:1");
    l.add("C:VARTYPE_BOOLEAN:I:1");
    l.add("C:VARTYPE_BOOLEAN:I:0");
    l.add("C:VARTYPE_CHAR:I:65");
    l.add("C:VARTYPE_CHAR:I:10");
    l.add("C:VARTYPE_BYTECHAR:I:66");
    l.add("C:VARTYPE_SHORT:I:-300");
    l.add("C:VARTYPE_LONG:J:5");
    l.add("C:VARTYPE_LONG:J:-1");
    l.add("C:VARTYPE_FLOAT:F:" + Integer.toHexString(Float.floatToRawIntBits(1.5f)));
    l.add("C:VARTYPE_FLOAT:F:" + Integer.toHexString(Float.floatToRawIntBits(0.1f)));
    l.add("C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(2.5)));
    l.add("C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(-0.75)));
    l.add("C:VARTYPE_STRING:" + hexString("s"));
    l.add("C:VARTYPE_STRING:" + hexString("a b\n"));
    l.add("C:VARTYPE_NULL:N");
    l.add("C:VARTYPE_INTEGER:I:4");
    LEAVES = l.toArray(new String[0]);
  }

  static final String[] RARE_LEAVES = {
    "C:VARTYPE_INT:I:2147483647",
    "C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(Math.PI / 2)),
    "C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(Math.PI * 3)),
    "C:VARTYPE_FLOAT:F:" + Integer.toHexString(Float.floatToRawIntBits((float) Math.PI)),
    "C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(Double.NaN)),
  };

  static final String[] TYPES = {"I", "J", "D", "Z", "C", "Ljava/lang/String;", "Ljava/lang/Object;", "[I",
    "Ljava/lang/Runnable;", "Ljava/util/List;"};

  static final String[] IMPLICIT = {"VARTYPE_INT", "VARTYPE_LONG", "VARTYPE_STRING", "VARTYPE_CHAR",
    "VARTYPE_DOUBLE"};

  static final FunctionType[] FUNCS;
  static {
    List<FunctionType> f = new ArrayList<>(Arrays.asList(FunctionType.values()));
    f.remove(FunctionType.OTHER);
    FUNCS = f.toArray(new FunctionType[0]);
  }

  static String leaf() {
    if (RND.nextInt(25) == 0) return RARE_LEAVES[RND.nextInt(RARE_LEAVES.length)];
    return LEAVES[RND.nextInt(LEAVES.length)];
  }

  static String typeLeaf() {
    return "T:" + TYPES[RND.nextInt(TYPES.length)];
  }

  static String gen(int depth) {
    if (depth <= 0 || RND.nextInt(5) == 0) return leaf();
    FunctionType ft = FUNCS[RND.nextInt(FUNCS.length)];
    StringBuilder sb = new StringBuilder("(").append(ft.name());
    if ((ft == FunctionType.CAST || ft.castType != null) && RND.nextInt(4) == 0) sb.append('!');
    if ((ft.isPPMM() || ft == FunctionType.STR_CONCAT) && RND.nextInt(5) != 0) {
      sb.append('@').append(IMPLICIT[RND.nextInt(IMPLICIT.length)]);
    }
    List<String> ops = new ArrayList<>();
    if (ft == FunctionType.CAST) {
      ops.add(gen(depth - 1));
      ops.add(typeLeaf());
      if (RND.nextInt(6) == 0) ops.add(typeLeaf());
    } else if (ft == FunctionType.INSTANCEOF) {
      ops.add(gen(depth - 1));
      ops.add(typeLeaf());
    } else if (ft == FunctionType.TERNARY) {
      for (int i = 0; i < 3; i++) ops.add(gen(depth - 1));
    } else {
      for (int i = 0; i < ft.arity; i++) ops.add(gen(depth - 1));
    }
    for (String o : ops) sb.append(' ').append(o);
    return sb.append(')').toString();
  }

  // ---- columns ----

  static void pre(Exprent e, List<Exprent> out) {
    out.add(e);
    for (Exprent c : e.getAllExprents()) pre(c, out);
  }

  static int indexOf(List<Exprent> all, Exprent e) {
    for (int i = 0; i < all.size(); i++) if (all.get(i) == e) return i;
    return -1;
  }

  static String bounds(Exprent root) {
    List<Exprent> all = new ArrayList<>();
    pre(root, all);
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < all.size(); i++) {
      if (!(all.get(i) instanceof FunctionExprent) && !(all.get(i) instanceof AssignmentExprent)) continue;
      Exprent e = all.get(i);
      String one = run(() -> {
        CheckTypesResult r = e.checkExprTypeBounds();
        StringBuilder b = new StringBuilder();
        for (CheckTypesResult.ExprentTypePair p : r.getLowerBounds()) b.append("L").append(indexOf(all, p.exprent)).append('=').append(p.type).append(',');
        for (CheckTypesResult.ExprentTypePair p : r.getUpperBounds()) b.append("U").append(indexOf(all, p.exprent)).append('=').append(p.type).append(',');
        return b.toString();
      });
      if (sb.length() > 0) sb.append(" | ");
      sb.append(i).append(':').append(one);
    }
    return sb.toString();
  }

  static void fresh() {
    DecompilerContext.startClass(new ImportCollector(NODE));
  }

  static void tree(String tree) {
    fresh();
    Exprent e = build(tree);
    String exprType = run(() -> String.valueOf(e.getExprType()));
    String bnds = run(() -> bounds(e));
    fresh();
    String prec = run(() -> Integer.toString(e.getPrecedence()));
    fresh();
    TextBuffer[] buf = new TextBuffer[1];
    String text = run(() -> {
      buf[0] = e.toJava(0);
      return buf[0].convertToStringAndAllowDataDiscard();
    });
    String reformatted = buf[0] == null ? text : run(() -> {
      buf[0].reformat();
      return buf[0].convertToStringAndAllowDataDiscard();
    });

    fresh();
    Exprent e2 = build(tree);
    String inferred = run(() -> String.valueOf(e2.getInferredExprType(null)));
    fresh();
    String inferredText = run(() -> e2.toJava(0).convertToStringAndAllowDataDiscard());

    OUT.append(String.join("\t", "F", tree, exprType, inferred, inferredText, bnds, prec, text, reformatted)).append('\n');
  }

  public static void main(String[] args) throws Exception {
    Map<String, Object> props = new HashMap<>();
    props.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    props.put(IFernflowerPreferences.PREFERRED_LINE_LENGTH, "40");
    new Fernflower(null, props, IFernflowerLogger.NO_OP);

    byte[] bytes = Files.readAllBytes(Paths.get(args[0]));
    StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
    NODE = new ClassesProcessor.ClassNode(ClassesProcessor.ClassNode.Type.ROOT, cl);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, NODE);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, cl);

    String i1 = "C:VARTYPE_INT:I:1", i2 = "C:VARTYPE_INT:I:2", i3 = "C:VARTYPE_INT:I:3";
    String f1 = "C:VARTYPE_FLOAT:F:3fc00000", d1 = "C:VARTYPE_DOUBLE:D:4004000000000000";
    String s1 = "C:VARTYPE_STRING:" + hexString("x"), b1 = "G1:1", b0 = "G1:0";
    String[] fixed = {
      "(ADD " + i1 + " (ADD " + i2 + " " + i3 + "))",
      "(ADD (ADD " + i1 + " " + i2 + ") " + i3 + ")",
      "(SUB " + i1 + " (SUB " + i2 + " " + i3 + "))",
      "(SUB (SUB " + i1 + " " + i2 + ") " + i3 + ")",
      "(MUL " + i1 + " (ADD " + i2 + " " + i3 + "))",
      "(ADD " + f1 + " (ADD " + f1 + " " + f1 + "))",
      "(MUL " + d1 + " (MUL " + d1 + " " + d1 + "))",
      "(DIV " + i1 + " (MUL " + i2 + " " + i3 + "))",
      "(AND C:VARTYPE_INT:I:255 G0:65535)",
      "(OR C:VARTYPE_INT:I:65280 C:VARTYPE_INT:I:16711680)",
      "(AND C:VARTYPE_CHAR:I:65 G0:65535)",
      "(XOR " + b1 + " " + b0 + ")",
      "(EQ C:VARTYPE_CHAR:I:65 G0:66)",
      "(LT G0:97 C:VARTYPE_CHAR:I:98)",
      "(NE (IPP " + i1 + ") G0:3)",
      "(EQ (IPP@VARTYPE_CHAR " + i1 + ") G0:66)",
      "(BOOLEAN_AND (BOOLEAN_OR " + b1 + " " + b0 + ") (BOOLEAN_AND " + b0 + " " + b1 + "))",
      "(BOOLEAN_OR " + b1 + " (BOOLEAN_OR " + b0 + " (BOOLEAN_OR " + b1 + " " + b0 + ")))",
      "(STR_CONCAT " + s1 + " (STR_CONCAT " + i1 + " " + s1 + "))",
      "(STR_CONCAT@VARTYPE_STRING (ADD " + i1 + " " + i2 + ") " + s1 + ")",
      "(TERNARY " + b1 + " " + i1 + " " + i2 + ")",
      "(TERNARY " + b1 + " (TERNARY " + b0 + " " + i1 + " " + i2 + ") " + i3 + ")",
      "(TERNARY (TERNARY " + b0 + " " + b1 + " " + b0 + ") " + s1 + " C:VARTYPE_NULL:N)",
      "(TERNARY " + b1 + " " + s1 + " " + i1 + ")",
      "(NEG (NEG " + i1 + "))",
      "(BIT_NOT (ADD " + i1 + " " + i2 + "))",
      "(BOOL_NOT (EQ " + i1 + " " + i2 + "))",
      "(PPI@VARTYPE_INT " + i1 + ")",
      "(IMM@VARTYPE_INT " + i1 + ")",
      "(IPP " + i1 + ")",
      "(MMI " + i1 + ")",
      "(CAST " + i1 + " T:J)",
      "(CAST! " + i1 + " T:J)",
      "(CAST (ADD " + i1 + " " + i2 + ") T:Ljava/lang/Object;)",
      "(CAST " + s1 + " T:Ljava/lang/Runnable; T:Ljava/io/Serializable;)",
      "(CAST C:VARTYPE_NULL:N T:Ljava/lang/String;)",
      "(ADD (CAST! (ADD " + i1 + " " + i2 + ") T:I) " + i3 + ")",
      "(I2L " + i1 + ")",
      "(I2L! (ADD " + i1 + " " + i2 + "))",
      "(D2I (MUL " + d1 + " " + d1 + "))",
      "(I2C G0:65)",
      "(INSTANCEOF " + s1 + " T:Ljava/lang/String;)",
      "(INSTANCEOF C:VARTYPE_NULL:N T:[I)",
      "(ARRAY_LENGTH C:VARTYPE_NULL:N)",
      "(ARRAY_LENGTH (CAST C:VARTYPE_NULL:N T:[I))",
      "(LCMP C:VARTYPE_LONG:J:1 C:VARTYPE_LONG:J:2)",
      "(DCMPG " + d1 + " (ADD " + d1 + " " + d1 + "))",
      "(SHL " + i1 + " (SHR " + i2 + " " + i3 + "))",
      "(USHR (USHR " + i1 + " " + i2 + ") " + i3 + ")",
      "(ADD C:VARTYPE_DOUBLE:D:3ff921fb54442d18 " + d1 + ")",
      "(MUL C:VARTYPE_DOUBLE:D:3ff921fb54442d18 " + d1 + ")",
      "(MUL " + d1 + " C:VARTYPE_DOUBLE:D:3ff921fb54442d18)",
      "(REM C:VARTYPE_LONG:J:9 " + i2 + ")",
      "(ADD " + i1 + " (ADD " + i2 + " (ADD " + i3 + " (ADD " + i1 + " (ADD " + i2 + " (ADD " + i3 + " (ADD " + i1 + " " + i2 + ")))))))",
      "(BOOLEAN_AND (EQ (ADD " + i1 + " " + i2 + ") (MUL " + i3 + " " + i1 + ")) (NE (SUB " + i2 + " " + i3 + ") (DIV " + i1 + " " + i2 + ")))",
      "(TERNARY (LT (ADD " + i1 + " " + i2 + ") " + i3 + ") (STR_CONCAT " + s1 + " (ADD " + i1 + " " + i2 + ")) (STR_CONCAT " + s1 + " " + s1 + "))",
    };
    for (String t : fixed) tree(t);
    for (int k = 0; k < 1500; k++) tree(gen(1 + RND.nextInt(5)));

    String[] fields = {"FS:CONST:I", "FS:LETTER:C", "FS:counter:I", "FS:BIG:J", "FS:NAME:Ljava/lang/String;",
      "FS:missing:I", "FX:java/lang/System:out:Ljava/io/PrintStream;", "FX:java/lang/Integer:MAX_VALUE:I"};
    String[] conds = {"", "ADD", "SUB", "MUL", "DIV", "AND", "OR", "XOR", "REM", "SHL", "SHR", "USHR"};
    String[] assigns = {
      "(= FS:CONST:I C:VARTYPE_INT:I:5)",
      "(= FS:counter:I G0:65)",
      "(= FS:LETTER:C G0:98)",
      "(= FS:LETTER:C G0:10)",
      "(= FS:LETTER:C G0:300)",
      "(=ADD FS:counter:I C:VARTYPE_INT:I:1)",
      "(=SHL FS:counter:I G0:2)",
      "(=USHR FS:BIG:J G0:3)",
      "(= FS:CONST:I C:VARTYPE_LONG:J:5)",
      "(= FS:BIG:J G0:3)",
      "(= FS:NAME:Ljava/lang/String; C:VARTYPE_NULL:N)",
      "(= FX:java/lang/System:out:Ljava/io/PrintStream; C:VARTYPE_NULL:N)",
      "(= FS:counter:I (TERNARY G1:1 G0:1 G0:2))",
      "(= FS:LETTER:C (TERNARY G1:1 G0:1 G0:2))",
      "(= FS:counter:I (CAST C:VARTYPE_LONG:J:7 T:I))",
      "(= FS:counter:I (CAST! C:VARTYPE_LONG:J:7 T:I))",
      "(= FS:counter:I (= FS:CONST:I G0:3))",
      "(= C:VARTYPE_INT:I:1 C:VARTYPE_INT:I:2)",
      "(= FS:NAME:Ljava/lang/String; (STR_CONCAT " + s1 + " " + i1 + "))",
      "(=ADD FS:NAME:Ljava/lang/String; " + s1 + ")",
      "(= FS:missing:I C:VARTYPE_DOUBLE:D:4004000000000000)",
    };
    for (String t : assigns) tree(t);
    for (int k = 0; k < 300; k++) {
      String cond = conds[RND.nextInt(conds.length)];
      tree("(=" + cond + " " + fields[RND.nextInt(fields.length)] + " " + gen(RND.nextInt(4)) + ")");
    }

    System.out.print(OUT);
  }
}
