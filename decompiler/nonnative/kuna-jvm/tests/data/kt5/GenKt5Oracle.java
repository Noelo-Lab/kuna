// Golden vectors for the Kotlin plugin's expression classes (KFunctionExprent,
// KVarExprent, KConstExprent, KFieldExprent, KAnnotationExprent, KSwitchHeadExprent,
// KNewExprent, KExitExprent, KInvocationExprent), KExprProcessor.jmpWrapper and the
// ResugarKotlinMethods / CollapseStringConcat passes, from Vineflower 1.12.0 and its
// bundled Kotlin plugin. Regenerate with `sh gen.sh`.
//
// The context is the one GenFunctionOracle (../b02) makes, with no plugin loaded:
// preferred-line-length 40, kt-collapse-string-concat 1, b02/B02Fields (args[0]) as
// the current class, and a fresh ImportCollector (so a fresh expression counter)
// before every column. args[1] is cases.txt; 600 random trees follow its rows.
// Trees are prefix forms over B02's leaves (C:TYPE:value, G0:int, G1:int, T:sig,
// FS:name:desc, FX:class:name:desc), method descriptors spelled with <> for (), plus
//   V:index:TYPE                      VarExprent(index, VarType.TYPE, null)
//   CL:sig                            ConstExprent(VARTYPE_CLASS, sig)
//   CN:sig                            ConstExprent(new VarType(sig), null)
//   U:KTYPE:I|J:value                 ConstExprent(KType.KTYPE, value)
//   (F:FUNC[!][@TYPE] ops)            FunctionExprent; ! = setNeedsCast(false), @ = setImplicitType
//   (KF:FUNC[!][@TYPE] ops)           new KFunctionExprent(FunctionType, ops, null)
//   (KK:KTYPE ops)                    new KFunctionExprent(KFunctionType, ops, null)
//   (KC f)                            new KFunctionExprent((FunctionExprent) f)
//   (K e)                             KUtils.replaceExprent(e)
//   (KV:DECL[:F] v)                   (KVarExprent) KUtils.replaceExprent(v), then
//                                     setDeclarationType(DECL, N = null); F = setEffectivelyFinal(true)
//                                     on v first
//   (I:class:name:desc:flags ops)     InvocationExprent: flags s = static (ops are the parameters),
//                                     otherwise the first op is the instance; i = <init>
//   (N:sig:flags[:desc] ops)          NewExprent(new VarType(sig), dims): with desc the ops are the
//                                     constructor parameters, with flag L the array elements, else
//                                     the dimensions; v/d = varArg/directArrayInit
//   (KN:sig:flags[:desc] ops)         the same wrapped in KNewExprent, then a = setInAnnotation(true),
//                                     E = setEnumConst(true)
//   (A:class:names ops)               AnnotationExprent; names comma-separated, - for none
//   (KA:TARGET:class:names ops)       the same wrapped in KAnnotationExprent; TARGET - for null
//   (KX:RETURN|THROW:retsig:lambda ops)  KExitExprent(new ExitExprent(..)): retsig - (null), U
//                                     (KType.UNIT) or a signature; lambda - for none
//   (KSH value)                       KSwitchHeadExprent(new SwitchHeadExprent(value))
//   (= left right)                    AssignmentExprent
//
// Rows (tab-separated, esc()'d, EXC:<class>:<message> for a throw):
//   T mw tree ids exprType bounds precedence toJava reformatted after copyIds copyDesc copyText
//     mw 1 = a MethodWrapper of B02Fields.<init> is CURRENT_METHOD_WRAPPER while writing;
//     ids = expression counter after the build; bounds as in GenFunctionOracle; after = desc(tree) after toJava;
//     desc = preorder "Class[:FUNC[/KTYPE]]#id";
//     copy columns from a second build: counter used by copy(), desc and toJava of the copy;
//     a tree whose build throws has the throw as its only column after the tree
//   P pass trees res descs texts   pass RESUGAR|COLLAPSE|COLLAPSE0 (option off) over a root whose
//     head is a sequence of one basic block per " | "-separated group of " ; "-separated trees
//   J case text                    KExprProcessor.jmpWrapper on a small statement graph
import org.jetbrains.java.decompiler.api.plugin.pass.PassContext;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.collectors.ImportCollector;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.CheckTypesResult;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.vineflower.kotlin.expr.*;
import org.vineflower.kotlin.expr.KFunctionExprent.KFunctionType;
import org.vineflower.kotlin.pass.CollapseStringConcatPass;
import org.vineflower.kotlin.pass.ResugarKotlinMethodsPass;
import org.vineflower.kotlin.stat.KSequenceStatement;
import org.vineflower.kotlin.struct.KType;
import org.vineflower.kotlin.util.KExprProcessor;
import org.vineflower.kotlin.util.KUtils;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;

public class GenKt5Oracle {
  static final StringBuilder OUT = new StringBuilder();
  static ClassesProcessor.ClassNode NODE;
  static StructClass CLASS;
  static StructMethod METHOD;
  static final Random RND = new Random(0x4B54_35L);

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

  static KType ktype(String name) {
    try {
      return (KType) KType.class.getField(name).get(null);
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

  static String hexString(String s) {
    StringBuilder sb = new StringBuilder("S:x");
    for (char c : s.toCharArray()) sb.append(String.format("%04x", (int) c));
    return sb.toString();
  }

  // ---- prefix-form parser (mirrored by kt5_exprents.rs) ----

  static String[] toks;
  static int pos;

  static Exprent build(String tree) {
    toks = tree.replace("(", " ( ").replace(")", " ) ").trim().split("\\s+");
    pos = 0;
    return parse();
  }

  static String descriptor(String spec) {
    return spec.replace('<', '(').replace('>', ')');
  }

  static List<Exprent> rest() {
    List<Exprent> ops = new ArrayList<>();
    while (!toks[pos].equals(")")) ops.add(parse());
    pos++;
    return ops;
  }

  static Exprent parse() {
    String t = toks[pos++];
    if (t.equals("(")) {
      String head = toks[pos++];
      String[] p = head.split(":", -1);
      switch (p[0]) {
        case "=": {
          List<Exprent> ops = rest();
          return new AssignmentExprent(ops.get(0), ops.get(1), null);
        }
        case "F":
        case "KF":
        case "KK": {
          String spec = head.substring(p[0].length() + 1);
          String implicit = null;
          int at = spec.indexOf('@');
          if (at >= 0) {
            implicit = spec.substring(at + 1);
            spec = spec.substring(0, at);
          }
          boolean noCast = spec.endsWith("!");
          if (noCast) spec = spec.substring(0, spec.length() - 1);
          List<Exprent> ops = rest();
          FunctionExprent f;
          if (p[0].equals("F")) f = new FunctionExprent(FunctionType.valueOf(spec), ops, null);
          else if (p[0].equals("KF")) f = new KFunctionExprent(FunctionType.valueOf(spec), ops, null);
          else f = new KFunctionExprent(KFunctionType.valueOf(spec), ops, null);
          if (noCast) f.setNeedsCast(false);
          if (implicit != null) f.setImplicitType(type(implicit));
          return f;
        }
        case "KC": {
          List<Exprent> ops = rest();
          return new KFunctionExprent((FunctionExprent) ops.get(0));
        }
        case "K": {
          List<Exprent> ops = rest();
          return KUtils.replaceExprent(ops.get(0));
        }
        case "KV": {
          List<Exprent> ops = rest();
          VarExprent v = (VarExprent) ops.get(0);
          if (p.length > 2 && p[2].equals("F")) v.setEffectivelyFinal(true);
          KVarExprent k = (KVarExprent) KUtils.replaceExprent(v);
          k.setDeclarationType(p[1].equals("N") ? null : KVarExprent.DeclarationType.valueOf(p[1]));
          return k;
        }
        case "I": {
          List<Exprent> ops = rest();
          InvocationExprent inv = new InvocationExprent();
          inv.setClassname(p[1]);
          inv.setName(p[2]);
          String desc = descriptor(p[3]);
          inv.setStringDescriptor(desc);
          inv.setDescriptor(MethodDescriptor.parseDescriptor(desc));
          boolean isStatic = p[4].contains("s");
          inv.setStatic(isStatic);
          inv.setFunctype(p[4].contains("i") ? InvocationExprent.Type.INIT : InvocationExprent.Type.GENERAL);
          if (!isStatic) inv.setInstance(ops.remove(0));
          inv.setLstParameters(ops);
          return inv;
        }
        case "N":
        case "KN": {
          List<Exprent> ops = rest();
          String flags = p[2];
          NewExprent n;
          if (p.length > 3) {
            n = new NewExprent(new VarType(p[1]), new ArrayList<>(), null);
            InvocationExprent ctor = new InvocationExprent();
            String cls = n.getNewType().value;
            ctor.setClassname(cls);
            ctor.setName("<init>");
            String desc = descriptor(p[3]);
            ctor.setStringDescriptor(desc);
            ctor.setDescriptor(MethodDescriptor.parseDescriptor(desc));
            ctor.setFunctype(InvocationExprent.Type.INIT);
            ctor.setLstParameters(ops);
            n.setConstructor(ctor);
          } else if (flags.contains("L")) {
            n = new NewExprent(new VarType(p[1]), new ArrayList<>(), null);
            n.setLstArrayElements(ops);
          } else {
            n = new NewExprent(new VarType(p[1]), ops, null);
          }
          n.setVarArgParam(flags.contains("v"));
          n.setDirectArrayInit(flags.contains("d"));
          if (p[0].equals("N")) return n;
          KNewExprent k = new KNewExprent(n);
          if (flags.contains("a")) k.setInAnnotation(true);
          if (flags.contains("E")) k.setEnumConst(true);
          return k;
        }
        case "A":
        case "KA": {
          List<Exprent> ops = rest();
          int c = p[0].equals("A") ? 1 : 2;
          List<String> names = p[c + 1].equals("-") ? new ArrayList<>() : new ArrayList<>(Arrays.asList(p[c + 1].split(",")));
          AnnotationExprent a = new AnnotationExprent(p[c], names, ops);
          if (p[0].equals("A")) return a;
          return new KAnnotationExprent(a, p[1].equals("-") ? null : KAnnotationExprent.UseSiteTarget.valueOf(p[1]));
        }
        case "KX": {
          List<Exprent> ops = rest();
          VarType ret = p[2].equals("-") ? null : p[2].equals("U") ? KType.UNIT : new VarType(p[2]);
          ExitExprent e = new ExitExprent(ExitExprent.Type.valueOf(p[1]), ops.isEmpty() ? null : ops.get(0), ret, null, null);
          KExitExprent k = new KExitExprent(e);
          if (!p[3].equals("-")) k.setLambdaName(p[3]);
          return k;
        }
        case "KSH": {
          List<Exprent> ops = rest();
          return new KSwitchHeadExprent(new SwitchHeadExprent(ops.get(0), null));
        }
        default:
          throw new IllegalStateException(head);
      }
    }
    if (t.startsWith("G")) {
      return new ConstExprent(Integer.parseInt(t.substring(3)), t.charAt(1) == '1', null);
    }
    if (t.startsWith("T:")) {
      return new ConstExprent(new VarType(t.substring(2)), null, null);
    }
    if (t.startsWith("CL:")) {
      return new ConstExprent(VarType.VARTYPE_CLASS, t.substring(3), null);
    }
    if (t.startsWith("CN:")) {
      return new ConstExprent(new VarType(t.substring(3)), null, null);
    }
    if (t.startsWith("U:")) {
      String[] p = t.split(":");
      return new ConstExprent(ktype(p[1]), value(p[2] + ":" + p[3]), null);
    }
    if (t.startsWith("V:")) {
      String[] p = t.split(":");
      return new VarExprent(Integer.parseInt(p[1]), type(p[2]), null);
    }
    if (t.startsWith("FS:") || t.startsWith("FX:")) {
      String[] p = t.split(":");
      String cls = t.startsWith("FS:") ? "b02/B02Fields" : p[1];
      return new FieldExprent(p[p.length - 2], cls, true, null, FieldDescriptor.parseDescriptor(p[p.length - 1]), null);
    }
    int colon = t.indexOf(':', 2);
    return new ConstExprent(type(t.substring(2, colon)), value(t.substring(colon + 1)), null);
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
      if (!(all.get(i) instanceof FunctionExprent)) continue;
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

  static String desc(Exprent root) {
    List<Exprent> all = new ArrayList<>();
    pre(root, all);
    StringBuilder sb = new StringBuilder();
    for (Exprent e : all) {
      if (sb.length() > 0) sb.append(' ');
      sb.append(e.getClass().getSimpleName());
      if (e instanceof FunctionExprent f) {
        sb.append(':').append(f.getFuncType());
        if (f instanceof KFunctionExprent k) sb.append('/').append(kType(k));
      }
      sb.append('#').append(e.id);
    }
    return sb.toString();
  }

  static String kType(KFunctionExprent k) {
    try {
      java.lang.reflect.Field f = KFunctionExprent.class.getDeclaredField("kType");
      f.setAccessible(true);
      return String.valueOf(f.get(k));
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static MethodWrapper methodWrapper() {
    VarProcessor varProc = new VarProcessor(METHOD, MethodDescriptor.parseDescriptor(METHOD.getDescriptor()));
    return new MethodWrapper(null, varProc, METHOD, CLASS, new CounterContainer());
  }

  static int counter() {
    return DecompilerContext.getCounterContainer().getCounter(CounterContainer.EXPRESSION_COUNTER);
  }

  static void fresh() {
    DecompilerContext.startClass(new ImportCollector(NODE));
    DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, null);
  }

  static void tree(boolean mw, String tree) {
    fresh();
    Exprent[] built = new Exprent[1];
    String failed = run(() -> {
      built[0] = build(tree);
      return null;
    });
    if (built[0] == null) {
      OUT.append(String.join("\t", "T", mw ? "1" : "0", tree, failed)).append('\n');
      return;
    }
    Exprent e = built[0];
    String ids = Integer.toString(counter());
    String exprType = run(() -> String.valueOf(e.getExprType()));
    String bnds = run(() -> bounds(e));
    fresh();
    String prec = run(() -> Integer.toString(e.getPrecedence()));
    fresh();
    if (mw) DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, methodWrapper());
    TextBuffer[] buf = new TextBuffer[1];
    String text = run(() -> {
      buf[0] = e.toJava(0);
      return buf[0].convertToStringAndAllowDataDiscard();
    });
    String reformatted = buf[0] == null ? text : run(() -> {
      buf[0].reformat();
      return buf[0].convertToStringAndAllowDataDiscard();
    });
    String after = run(() -> desc(e));

    fresh();
    Exprent e2 = build(tree);
    int before = counter();
    Exprent[] copy = new Exprent[1];
    String copyIds = run(() -> {
      copy[0] = e2.copy();
      return Integer.toString(counter() - before);
    });
    String copyDesc = copy[0] == null ? copyIds : run(() -> desc(copy[0]));
    if (mw) DecompilerContext.setProperty(DecompilerContext.CURRENT_METHOD_WRAPPER, methodWrapper());
    String copyText = copy[0] == null ? copyIds : run(() -> copy[0].toJava(0).convertToStringAndAllowDataDiscard());

    OUT.append(String.join("\t", "T", mw ? "1" : "0", tree, ids, exprType, bnds, prec, text, reformatted, after,
      copyIds, copyDesc, copyText)).append('\n');
  }

  // ---- passes ----

  static void pass(String which, String spec) {
    fresh();
    DecompilerContext.setProperty("kt-collapse-string-concat", which.equals("COLLAPSE0") ? "0" : "1");
    List<Statement> blocks = new ArrayList<>();
    for (String group : spec.split(" \\| ")) {
      BasicBlockStatement bb = BasicBlockStatement.create();
      for (String tree : group.split(" ; ")) bb.getExprents().add(build(tree));
      blocks.add(bb);
    }
    Statement head = blocks.size() == 1 ? blocks.get(0) : new SequenceStatement(blocks);
    RootStatement root = new RootStatement(head, new DummyExitStatement(), METHOD);
    PassContext ctx = new PassContext(root, null, METHOD, CLASS, null, null);
    String res = run(() -> Boolean.toString(which.equals("RESUGAR") ? new ResugarKotlinMethodsPass().run(ctx) : new CollapseStringConcatPass().run(ctx)));
    StringBuilder descs = new StringBuilder();
    StringBuilder texts = new StringBuilder();
    for (Statement b : blocks) {
      for (Exprent e : b.getExprents()) {
        if (descs.length() > 0) {
          descs.append(" ; ");
          texts.append(" ; ");
        }
        descs.append(run(() -> desc(e)));
        texts.append(run(() -> e.toJava(0).convertToStringAndAllowDataDiscard()));
      }
    }
    DecompilerContext.setProperty("kt-collapse-string-concat", "1");
    OUT.append(String.join("\t", "P", which, spec, res, esc(descs.toString()), esc(texts.toString()))).append('\n');
  }

  // ---- jmpWrapper ----

  static StatEdge edge(int type, Statement source, Statement destination, Statement closure) {
    StatEdge e = new StatEdge(type, source, destination, closure);
    source.addSuccessor(e);
    return e;
  }

  static void jmp(String name) {
    fresh();
    Statement stat = BasicBlockStatement.create();
    Statement target = BasicBlockStatement.create();
    Statement plain = BasicBlockStatement.create();
    SequenceStatement seq = new SequenceStatement(Arrays.asList(BasicBlockStatement.create(), BasicBlockStatement.create()));
    Statement run = new KSequenceStatement(seq);
    boolean isSwitch = name.contains("switch");
    Statement closure = name.contains("run") ? run : name.contains("noclosure") ? null : plain;
    int type = name.startsWith("continue") ? StatEdge.TYPE_CONTINUE : name.startsWith("regular") ? StatEdge.TYPE_REGULAR
      : name.startsWith("exception") ? StatEdge.TYPE_EXCEPTION : StatEdge.TYPE_BREAK;
    Statement dest = name.contains("exit") ? new DummyExitStatement() : target;
    StatEdge e = edge(type, stat, dest, closure);
    e.labeled = name.contains("labeled");
    e.explicit = !name.contains("implicit");
    if (name.contains("two")) edge(StatEdge.TYPE_BREAK, stat, target, plain);
    String text = run(() -> KExprProcessor.jmpWrapper(stat, 1, isSwitch).convertToStringAndAllowDataDiscard());
    OUT.append(String.join("\t", "J", name, text)).append('\n');
  }

  // ---- random trees ----

  static final String[] LEAVES;
  static {
    List<String> l = new ArrayList<>();
    for (int v : new int[]{0, 1, 2, 7, 65, 300, 65535, 100000, -1}) {
      l.add("C:VARTYPE_INT:I:" + v);
      l.add("G0:" + v);
    }
    l.add("G1:0");
    l.add("G1:1");
    l.add("C:VARTYPE_BOOLEAN:I:1");
    l.add("C:VARTYPE_CHAR:I:65");
    l.add("C:VARTYPE_CHAR:I:36");
    l.add("C:VARTYPE_LONG:J:5");
    l.add("C:VARTYPE_DOUBLE:D:" + Long.toHexString(Double.doubleToRawLongBits(2.5)));
    l.add("C:VARTYPE_STRING:" + hexString("s"));
    l.add("C:VARTYPE_STRING:" + hexString("a$b\n"));
    l.add("C:VARTYPE_STRING:" + hexString("é$"));
    l.add("C:VARTYPE_NULL:N");
    l.add("C:VARTYPE_INTEGER:I:4");
    l.add("V:1:VARTYPE_INT");
    l.add("V:2:VARTYPE_OBJECT");
    l.add("V:3:VARTYPE_STRING");
    l.add("V:4:VARTYPE_CLASS");
    l.add("V:5:VARTYPE_BOOLEAN");
    l.add("V:6:VARTYPE_LONG");
    l.add("CL:java/lang/String");
    l.add("CL:[I");
    l.add("FS:CONST:I");
    l.add("FX:java/lang/Integer:TYPE:Ljava/lang/Class;");
    LEAVES = l.toArray(new String[0]);
  }

  static final String[] TYPES = {"I", "J", "Z", "Ljava/lang/String;", "Ljava/lang/Object;", "[I", "Ljava/util/List;"};

  static final FunctionType[] FUNCS;
  static {
    List<FunctionType> f = new ArrayList<>(Arrays.asList(FunctionType.values()));
    f.remove(FunctionType.OTHER);
    for (int i = 0; i < 6; i++) {
      f.add(FunctionType.EQ);
      f.add(FunctionType.NE);
    }
    for (FunctionType t : new FunctionType[]{FunctionType.TERNARY, FunctionType.INSTANCEOF, FunctionType.BOOL_NOT,
      FunctionType.CAST, FunctionType.BIT_NOT, FunctionType.AND, FunctionType.OR, FunctionType.XOR, FunctionType.SHL,
      FunctionType.SHR, FunctionType.USHR, FunctionType.STR_CONCAT}) {
      f.add(t);
      f.add(t);
    }
    FUNCS = f.toArray(new FunctionType[0]);
  }

  static String leaf() {
    return LEAVES[RND.nextInt(LEAVES.length)];
  }

  static String typeLeaf() {
    return "T:" + TYPES[RND.nextInt(TYPES.length)];
  }

  static String func(String prefix, int depth) {
    FunctionType ft = FUNCS[RND.nextInt(FUNCS.length)];
    StringBuilder sb = new StringBuilder("(").append(prefix).append(':').append(ft.name());
    if ((ft == FunctionType.CAST || ft.castType != null) && RND.nextInt(4) == 0) sb.append('!');
    if ((ft.isPPMM() || ft == FunctionType.STR_CONCAT) && RND.nextInt(3) == 0) sb.append("@VARTYPE_STRING");
    List<String> ops = new ArrayList<>();
    if (ft == FunctionType.CAST || ft == FunctionType.INSTANCEOF) {
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

  static String gen(int depth) {
    if (depth <= 0 || RND.nextInt(4) == 0) return leaf();
    int pick = RND.nextInt(10);
    if (pick < 5) return func("KF", depth);
    if (pick < 6) return func("F", depth);
    if (pick < 7) return "(KC " + func("F", depth) + ")";
    KFunctionType kt = KFunctionType.values()[1 + RND.nextInt(4)];
    StringBuilder sb = new StringBuilder("(KK:").append(kt.name());
    int n = kt == KFunctionType.GET_KCLASS ? 1 : kt == KFunctionType.STR_TEMPLATE ? 1 + RND.nextInt(4) : 2;
    for (int i = 0; i < n; i++) sb.append(' ').append(gen(depth - 1));
    return sb.append(')').toString();
  }

  public static void main(String[] args) throws Exception {
    Map<String, Object> props = new HashMap<>();
    props.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "");
    props.put(IFernflowerPreferences.PREFERRED_LINE_LENGTH, "40");
    props.put("kt-collapse-string-concat", "1");
    org.jetbrains.java.decompiler.main.plugins.PluginSources.PLUGIN_SOURCES.clear();
    new Fernflower(null, props, IFernflowerLogger.NO_OP);

    byte[] bytes = Files.readAllBytes(Paths.get(args[0]));
    CLASS = StructClass.create(new DataInputFullStream(bytes), true);
    METHOD = CLASS.getMethod("<init>", "()V");
    NODE = new ClassesProcessor.ClassNode(ClassesProcessor.ClassNode.Type.ROOT, CLASS);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS_NODE, NODE);
    DecompilerContext.setProperty(DecompilerContext.CURRENT_CLASS, CLASS);

    for (String line : Files.readAllLines(Paths.get(args[1]))) {
      if (line.isBlank() || line.startsWith("#")) continue;
      String[] f = line.split("\t");
      switch (f[0]) {
        case "T" -> tree(false, f[1]);
        case "W" -> tree(true, f[1]);
        case "P" -> pass(f[1], f[2]);
        case "J" -> jmp(f[1]);
        default -> throw new IllegalStateException(line);
      }
    }

    for (int i = 0; i < 600; i++) tree(false, gen(1 + RND.nextInt(3)));

    System.out.print(OUT);
  }
}
