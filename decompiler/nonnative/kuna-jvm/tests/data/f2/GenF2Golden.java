import java.io.*;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.net.URI;
import java.nio.file.*;
import java.util.*;
import java.util.function.Supplier;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.modules.renamer.PoolInterceptor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.struct.consts.*;
import org.jetbrains.java.decompiler.struct.gen.*;
import org.jetbrains.java.decompiler.struct.gen.generics.*;
import org.jetbrains.java.decompiler.struct.match.*;

/**
 * Golden vectors for the kuna-jvm F2 slice, computed by Vineflower 1.12.0 itself.
 *
 * JDK 21:
 *   javac -cp vineflower-1.12.0.jar -d OUT GenF2Golden.java PoolSample.java
 *   java -cp vineflower-1.12.0.jar:OUT GenF2Golden . OUT
 *
 * Inputs are harvested from the running JDK's java.base classes (the committed vectors:
 * OpenJDK 21.0.12) plus hand-written edge cases. A context-dependent result (one that consulted the StructContext, which is
 * empty here) is suffixed with '!'.
 */
public class GenF2Golden {
  static final class Log extends IFernflowerLogger {
    final List<String> msgs = new ArrayList<>();

    @Override
    public void writeMessage(String m, Severity s) {
      msgs.add(s + ": " + m);
    }

    @Override
    public void writeMessage(String m, Severity s, Throwable t) {
      msgs.add(s + ": " + m);
    }
  }

  static final class Ctx extends StructContext {
    boolean touched;

    Ctx() {
      super(null, null);
    }

    @Override
    public StructClass getClass(String name) {
      touched = true;
      return super.getClass(name);
    }

    @Override
    public boolean instanceOf(String a, String b) {
      touched = true;
      return super.instanceOf(a, b);
    }

    @Override
    public StructClass findCommonAncestor(String a, String b) {
      touched = true;
      return super.findCommonAncestor(a, b);
    }
  }

  static Log LOG;
  static Ctx CTX;

  static void setContext(PoolInterceptor interceptor) {
    LOG = new Log();
    CTX = new Ctx();
    DecompilerContext.setCurrentContext(
      new DecompilerContext(new HashMap<>(), LOG, CTX, new ClassesProcessor(CTX), interceptor));
  }

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

  static String exc(Throwable t) {
    return "EXC:" + t.getClass().getSimpleName() + ":" + esc(t.getMessage());
  }

  static String dump(VarType t) {
    if (t == null) return "\\N";
    StringBuilder sb = new StringBuilder();
    if (t instanceof GenericType g) {
      sb.append("G[").append(t.type).append(',').append(t.arrayDim).append(',').append(esc(t.value))
        .append(',').append(t.typeFamily).append(',').append(t.stackSize)
        .append(",w").append(g.getWildcard()).append(",g").append(g.isGeneric() ? 1 : 0)
        .append(",p=").append(dump(g.getParent())).append(",a=(");
      List<VarType> args = g.getArguments();
      for (int i = 0; i < args.size(); i++) {
        if (i > 0) sb.append(';');
        sb.append(dump(args.get(i)));
      }
      sb.append(")]");
    } else {
      sb.append("V[").append(t.type).append(',').append(t.arrayDim).append(',').append(esc(t.value))
        .append(',').append(t.typeFamily).append(',').append(t.stackSize).append(']');
    }
    return sb.toString();
  }

  static String dumps(List<? extends VarType> l) {
    return "(" + l.stream().map(GenF2Golden::dump).collect(Collectors.joining(";")) + ")";
  }

  static String dumpMap(Map<VarType, VarType> m) {
    return "{" + m.entrySet().stream().map(e -> dump(e.getKey()) + "=>" + dump(e.getValue()))
      .collect(Collectors.joining("|")) + "}";
  }

  /** Runs an op; marks a result that consulted the struct context with '!'. */
  static String run(Supplier<String> op) {
    CTX.touched = false;
    String r;
    try {
      r = op.get();
    } catch (Throwable t) {
      r = exc(t);
    }
    return CTX.touched ? r + "!" : r;
  }

  // ---------------------------------------------------------------- harvest

  static final TreeSet<String> fieldDescs = new TreeSet<>();
  static final TreeSet<String> methodDescs = new TreeSet<>();
  static final TreeSet<String> classNames = new TreeSet<>();
  static final TreeSet<String> fieldSigs = new TreeSet<>();
  static final TreeSet<String> methodSigs = new TreeSet<>();
  static final TreeMap<String, String> classSigs = new TreeMap<>();
  static final TreeSet<String> methodPairs = new TreeSet<>();
  static final TreeSet<String> fieldPairs = new TreeSet<>();
  static final TreeMap<String, String[]> classInfo = new TreeMap<>();

  static void harvest() throws IOException {
    FileSystem fs = FileSystems.getFileSystem(URI.create("jrt:/"));
    List<Path> paths;
    try (Stream<Path> s = Files.walk(fs.getPath("/modules/java.base"))) {
      paths = s.filter(p -> p.toString().endsWith(".class")).sorted().collect(Collectors.toList());
    }
    for (Path p : paths) {
      parseClass(Files.readAllBytes(p));
    }
  }

  static String cls(Object[] cp, int idx) {
    return (String) cp[((int[]) cp[idx])[0]];
  }

  static String attrs(DataInputStream in, Object[] cp) throws IOException {
    int n = in.readUnsignedShort();
    String sig = null;
    for (int i = 0; i < n; i++) {
      String an = (String) cp[in.readUnsignedShort()];
      int len = in.readInt();
      if (an.equals("Signature")) {
        sig = (String) cp[in.readUnsignedShort()];
      } else {
        in.skipNBytes(len);
      }
    }
    return sig;
  }

  static void parseClass(byte[] b) throws IOException {
    DataInputStream in = new DataInputStream(new ByteArrayInputStream(b));
    in.readInt();
    in.readUnsignedShort();
    in.readUnsignedShort();
    int n = in.readUnsignedShort();
    Object[] cp = new Object[n];
    for (int i = 1; i < n; i++) {
      int tag = in.readUnsignedByte();
      switch (tag) {
        case 1 -> cp[i] = in.readUTF();
        case 3, 4 -> in.readInt();
        case 5, 6 -> {
          in.readLong();
          i++;
        }
        case 7 -> cp[i] = new int[] {in.readUnsignedShort()};
        case 8, 16, 19, 20 -> in.readUnsignedShort();
        case 9, 10, 11, 12, 17, 18 -> in.readInt();
        case 15 -> {
          in.readUnsignedByte();
          in.readUnsignedShort();
        }
        default -> throw new IOException("tag " + tag);
      }
    }
    in.readUnsignedShort();
    String name = cls(cp, in.readUnsignedShort());
    int sup = in.readUnsignedShort();
    if (sup != 0) classNames.add(cls(cp, sup));
    classNames.add(name);
    int ni = in.readUnsignedShort();
    List<String> ifaces = new ArrayList<>();
    for (int i = 0; i < ni; i++) {
      String iface = cls(cp, in.readUnsignedShort());
      classNames.add(iface);
      ifaces.add(iface);
    }
    for (int kind = 0; kind < 2; kind++) {
      int cnt = in.readUnsignedShort();
      for (int j = 0; j < cnt; j++) {
        in.readUnsignedShort();
        in.readUnsignedShort();
        String desc = (String) cp[in.readUnsignedShort()];
        (kind == 0 ? fieldDescs : methodDescs).add(desc);
        String sig = attrs(in, cp);
        if (sig != null) {
          (kind == 0 ? fieldSigs : methodSigs).add(sig);
          (kind == 0 ? fieldPairs : methodPairs).add(name + "\u0001" + desc + "\u0001" + sig);
        }
      }
    }
    String sig = attrs(in, cp);
    if (sig != null) {
      classSigs.put(name, sig);
      classInfo.put(name, new String[] {sig, sup == 0 ? null : cls(cp, sup), String.join(" ", ifaces)});
    }
  }

  static <T> List<T> sample(Collection<T> all, int n) {
    List<T> l = new ArrayList<>(all);
    if (l.size() <= n) return l;
    List<T> out = new ArrayList<>();
    for (int i = 0; i < n; i++) out.add(l.get((int) ((long) i * l.size() / n)));
    return out;
  }

  // ---------------------------------------------------------------- VarType

  static final String[] EDGE_VT = {
    "", "[", "[[", "I", "[I", "[[J", "V", "Z", "B", "C", "D", "F", "S", "X", "Y", "U", "G", "N", "A", "Q",
    "II", "Ljava/lang/String;", "[Ljava/lang/Object;", "[[Ljava/util/Map$Entry;", "L;", "Lfoo", "L", "LL;",
    "java/lang/String", "[Ljava/lang/String", "é", "éé", "[é", "😀",
    "L😀;", "\ud800", "a", "[a", "T", "TT;", "ab;",
  };

  static void vartype(PrintWriter w) {
    List<String[]> inputs = new ArrayList<>();
    for (String d : sample(fieldDescs, 300)) inputs.add(new String[] {d, "0"});
    for (String c : sample(classNames, 150)) {
      inputs.add(new String[] {c, "1"});
      inputs.add(new String[] {c, "0"});
    }
    for (String e : EDGE_VT) {
      inputs.add(new String[] {e, "0"});
      inputs.add(new String[] {e, "1"});
    }
    for (String[] in : inputs) {
      String r;
      try {
        VarType t = new VarType(in[0], in[1].equals("1"));
        String dec = run(() -> {
          VarType d = t.decreaseArrayDim();
          return d == t ? "same" : dump(d);
        });
        String resize = run(() -> dump(t.resizeArrayDim(0)));
        String resize3 = run(() -> dump(t.resizeArrayDim(3)));
        r = dump(t) + "\t" + esc(t.toString()) + "\t" + t.hashCode() + "\t" + dec + "\t" + resize + "\t" + resize3
          + "\t" + VarType.isPrimitive(t) + "\t" + t.isGeneric();
      } catch (Throwable e) {
        r = exc(e);
      }
      w.println("VT\t" + esc(in[0]) + "\t" + in[1] + "\t" + r);
    }
    for (CodeType ct : CodeType.values()) {
      String chr = run(() -> esc(VarType.getChar(ct)));
      String ctor = run(() -> dump(new VarType(ct)));
      String ctor2 = run(() -> dump(new VarType(ct, 2)));
      w.println("CT\t" + ct + "\t" + chr + "\t" + ctor + "\t" + ctor2);
    }
    for (TypeFamily f : TypeFamily.values()) {
      w.println("FB\t" + f + "\t" + constName(VarType.findFamilyBottom(f)) + "\t" + f.isNumeric() + "\t" + f.intOrBool());
    }
  }

  // ---------------------------------------------------------------- descriptors

  static final NewClassNameBuilder RENAMER = cn ->
    cn.startsWith("java/util/") ? "ju/" + cn.substring(10) : cn.equals("java/lang/String") ? "S" : null;

  static final String[] EDGE_MD = {
    "()V", "(IJ)V", "([[Ljava/lang/String;I)[J", "(Lfoo;[I)Ljava/util/List;", "(", "()", "x", "(I", "(IL)V",
    "(I)", "()Ljava/lang/String;", "([I[J[[D)[[Ljava/util/Map;", "(é)V", "(Ljava/lang/été;)V",
    "(ZBCSIJFD)V", ")(", "(I)V)V", "(XY)U",
  };

  static void descriptors(PrintWriter w) {
    List<String> mds = new ArrayList<>(sample(methodDescs, 400));
    mds.addAll(Arrays.asList(EDGE_MD));
    String prev = null;
    for (String d : mds) {
      String r;
      try {
        MethodDescriptor md = MethodDescriptor.parseDescriptor(d);
        r = dumps(Arrays.asList(md.params)) + "\t" + dump(md.ret) + "\t" + esc(md.toString()) + "\t" + md.hashCode()
          + "\t" + run(() -> esc(md.buildNewDescriptor(RENAMER)));
        if (prev != null) {
          String p = prev;
          r += "\t" + run(() -> "" + MethodDescriptor.parseDescriptor(p).equals(md));
        } else {
          r += "\t-";
        }
        prev = d;
      } catch (Throwable e) {
        r = exc(e);
      }
      w.println("MD\t" + esc(d) + "\t" + r);
    }
    List<String> fds = new ArrayList<>(sample(fieldDescs, 200));
    fds.addAll(Arrays.asList("I", "[[Ljava/util/List;", "Ljava/lang/String;", "", "L;"));
    prev = null;
    for (String d : fds) {
      String r;
      try {
        FieldDescriptor fd = FieldDescriptor.parseDescriptor(d);
        r = dump(fd.type) + "\t" + esc(fd.descriptorString) + "\t" + fd.hashCode() + "\t"
          + run(() -> esc(fd.buildNewDescriptor(RENAMER)));
        if (prev != null) {
          String p = prev;
          r += "\t" + run(() -> "" + FieldDescriptor.parseDescriptor(p).equals(fd));
        } else {
          r += "\t-";
        }
        prev = d;
      } catch (Throwable e) {
        r = exc(e);
      }
      w.println("FD\t" + esc(d) + "\t" + r);
    }
  }

  // ---------------------------------------------------------------- signatures

  static final String[][] EDGE_CS = {
    {"a/B", "<T:Ljava/lang/Object;>Ljava/lang/Object;"},
    {"a/B", "<K:Ljava/lang/Object;V:Ljava/lang/Object;>Ljava/util/AbstractMap<TK;TV;>;Ljava/util/Map<TK;TV;>;Ljava/lang/Cloneable;"},
    {"a/B", "<E::Ljava/lang/Comparable<-TE;>;>Ljava/lang/Object;"},
    {"a/B", "<T:Ljava/lang/Number;:Ljava/lang/Comparable<TT;>;:Ljava/io/Serializable;>Ljava/lang/Object;"},
    {"a/B", "Ljava/lang/Object;"},
    {"a/B", "Ljava/lang/Object;Ljava/util/List<Ljava/lang/String;>;"},
    {"a/B", ""},
    {"a/B", "<T:Ljava/lang/Object;"},
    {"a/B", "<>Ljava/lang/Object;"},
    {"a/B", "<T>Ljava/lang/Object;"},
    {"a/B$C", "<U:Ljava/lang/Object;>La/B<TU;>.D<TU;>;"},
    {"a.b.C", "<T:Ljava/lang/Object;>Ljava/lang/Object;"},
    {"a/été", "<é:Ljava/lang/Object;>Ljava/lang/Object;"},
    {"a/B", "<T:Ljava/lang/Object;>Ljava/lang/Enum<La/B<TT;>;>;"},
    {"a/B", "<T:[Ljava/lang/Object;>Ljava/lang/Object;"},
  };

  static final String[] EDGE_MS = {
    "<T:Ljava/lang/Object;>(TT;[TT;)TT;", "(Ljava/util/List<+Ljava/lang/Number;>;)V^Ljava/io/IOException;",
    "<X:Ljava/lang/Throwable;>(Ljava/util/function/Supplier<+TX;>;)TT;^TX;", "(I)V", "()Ljava/util/Map$Entry<TK;TV;>;",
    "(Lpkg/Outer<TT;>.Inner<TU;>;)V", "(Lpkg/Outer<TT;>.Inner;)V", "(Lpkg/Outer.Inner<TU;>;)V", "(Lpkg/A.B.C;)V",
    "(", "(Ljava/util/List<*>;)V", "(Ljava/util/List<[I>;)V", "()V^", "()V^^Ljava/lang/Exception;",
    "()V^Ljava/lang/Exception;^", "<T:Ljava/lang/Object;>()V", "()", ")V", "(TT)V", "(Ljava/util/Map<TK;*>;)TV;",
    "<A:Ljava/lang/Object;B:TA;>(TB;)TA;", "(Lpkg/O<TT;>.I<TU;>.J<TV;>;)V", "(é)V", "(Té;)Té;",
    "(Ljava/util/List<-[Ljava/lang/String;>;)V",
  };

  static final String[] EDGE_FS = {
    "TT;", "[TT;", "Ljava/util/List<Ljava/lang/String;>;", "Ljava/util/Map<TK;Ljava/util/List<-TV;>;>;",
    "Ljava/util/List<*>;", "Lpkg/Outer<Ljava/lang/String;>.Inner<Ljava/lang/Integer;>.Deep;", "I", "[[I", "",
    "Lx", "T", "Ljava/util/List<", "Lpkg/Outer.Inner;", "Lpkg/Outer<TT;>.Inner;", "[[Ljava/util/List<+TT;>;",
    "Ljava/util/List<+Ljava/util/List<*>;>;", "Lpkg/é<Té;>;", "L;", "[", "Ljava/util/List<;>;",
    "Lpkg/Outer<TT;>.;", "Lpkg/A<TT;>.B<TT;>.C<TT;>;", "Ljava/lang/String;", "TT", "[Lpkg/O<TT;>.I<[TU;>;",
  };

  static String dumpClass(GenericClassDescriptor d) {
    StringBuilder sb = new StringBuilder();
    sb.append("sc=").append(dump(d.superclass)).append(";si=").append(dumps(d.superinterfaces))
      .append(";fp=(").append(d.fparameters.stream().map(GenF2Golden::esc).collect(Collectors.joining(","))).append(")")
      .append(";fb=(");
    for (int i = 0; i < d.fbounds.size(); i++) {
      if (i > 0) sb.append(',');
      sb.append(dumps(d.fbounds.get(i)));
    }
    sb.append(");gt=").append(dump(d.genericType));
    return sb.toString();
  }

  static String dumpMethod(GenericMethodDescriptor d) {
    StringBuilder sb = new StringBuilder();
    sb.append("tp=(").append(d.typeParameters.stream().map(GenF2Golden::esc).collect(Collectors.joining(","))).append(")")
      .append(";tb=(");
    for (int i = 0; i < d.typeParameterBounds.size(); i++) {
      if (i > 0) sb.append(',');
      sb.append(dumps(d.typeParameterBounds.get(i)));
    }
    sb.append(");pt=").append(dumps(d.parameterTypes)).append(";rt=").append(dump(d.returnType))
      .append(";et=").append(dumps(d.exceptionTypes));
    return sb.toString();
  }

  static String logged(String r) {
    return LOG.msgs.isEmpty() ? r : r + "\t" + esc(String.join("|", LOG.msgs));
  }

  static void signatures(PrintWriter w) {
    List<Map.Entry<String, String>> cs = new ArrayList<>(sample(classSigs.entrySet(), 300));
    for (String[] e : EDGE_CS) cs.add(Map.entry(e[0], e[1]));
    for (Map.Entry<String, String> e : cs) {
      setContext(null);
      String r = run(() -> {
        GenericClassDescriptor d = GenericMain.parseClassSignature(e.getKey(), e.getValue());
        return d == null ? "\\N" : dumpClass(d);
      });
      w.println("CS\t" + esc(e.getKey()) + "\t" + esc(e.getValue()) + "\t" + logged(r));
    }
    List<String> ms = new ArrayList<>(sample(methodSigs, 400));
    ms.addAll(Arrays.asList(EDGE_MS));
    for (String s : ms) {
      setContext(null);
      String r = run(() -> {
        GenericMethodDescriptor d = GenericMain.parseMethodSignature(s);
        return d == null ? "\\N" : dumpMethod(d);
      });
      w.println("MS\t" + esc(s) + "\t" + logged(r));
    }
    List<String> fs = new ArrayList<>(sample(fieldSigs, 300));
    fs.addAll(Arrays.asList(EDGE_FS));
    for (String s : fs) {
      setContext(null);
      String r = run(() -> {
        GenericFieldDescriptor d = GenericMain.parseFieldSignature(s);
        return d == null ? "\\N" : dump(d.type);
      });
      w.println("FS\t" + esc(s) + "\t" + logged(r));
    }
    List<String> nt = new ArrayList<>(Arrays.asList(EDGE_FS));
    nt.addAll(Arrays.asList("*", "+TT;", "-Ljava/lang/Object;", "[[I", "IJ", "*TT;", "TT;TU;", "Ljava/util/List<TT;>;I", "", "+",
      "[+I", "Ljava/util/Map<TK;TV;>;Ljava/lang/Object;", "Lpkg/O<TT;>.I<TU;>;Z", "Q", "é"));
    for (String s : nt) {
      w.println("NT\t" + esc(s) + "\t" + run(() -> esc(GenericType.getNextType(s))));
    }
  }

  // ---------------------------------------------------------------- generic algebra

  static void genericOps(PrintWriter w) {
    setContext(null);
    List<String[]> uses = new ArrayList<>();
    for (String s : sample(fieldSigs, 300)) uses.add(new String[] {"F", s, "0"});
    for (String s : sample(methodSigs, 300)) {
      GenericMethodDescriptor d = GenericMain.parseMethodSignature(s);
      if (d == null) continue;
      for (int i = 0; i < d.parameterTypes.size(); i++) uses.add(new String[] {"MP", s, "" + i});
      uses.add(new String[] {"MR", s, "0"});
    }
    for (Map.Entry<String, String> e : sample(classSigs.entrySet(), 300)) {
      GenericClassDescriptor d = GenericMain.parseClassSignature(e.getKey(), e.getValue());
      if (d == null) continue;
      uses.add(new String[] {"CS", e.getValue(), "0"});
      for (int i = 0; i < d.superinterfaces.size(); i++) uses.add(new String[] {"CI", e.getValue(), "" + i});
    }
    List<String[]> candidates = new ArrayList<>();
    for (String[] u : uses) {
      VarType use = resolveUse(u);
      if (use instanceof GenericType && classSigs.containsKey(use.value)) candidates.add(u);
    }
    for (String[] u : sample(candidates, 320)) {
      VarType use = resolveUse(u);
      GenericType gu = (GenericType) use;
      String declSig = classSigs.get(use.value);
      GenericClassDescriptor dd = GenericMain.parseClassSignature(use.value, declSig);
      if (dd == null) continue;
      GenericType decl = dd.genericType;
      Set<VarType> named = new HashSet<>();
      for (String p : dd.fparameters) named.add(GenericType.parse("T" + p + ";"));

      StringBuilder r = new StringBuilder();
      r.append(run(() -> "" + decl.equals(use)));
      r.append('\t').append(run(() -> "" + decl.equalsExact(use)));
      r.append('\t').append(run(() -> "" + decl.argumentsEqual(gu)));
      Map<VarType, VarType> map = new HashMap<>();
      r.append('\t').append(run(() -> {
        decl.mapGenVarsTo(gu, map);
        return dumpMap(map);
      }));
      r.append('\t').append(run(() -> {
        VarType x = decl.remap(map);
        return x == decl ? "same" : dump(x);
      }));
      r.append('\t').append(run(() -> {
        VarType x = use.remap(map);
        return x == use ? "same" : dump(x);
      }));
      r.append('\t').append(run(() -> {
        Map<VarType, VarType> m2 = new HashMap<>(map);
        GenericType.cleanLoweredGenericTypes(m2, decl, gu, null);
        return dumpMap(m2);
      }));
      r.append('\t').append(run(() -> dumps(decl.getAllGenericVars())));
      r.append('\t').append(run(() -> dumps(gu.getAllGenericVars())));
      r.append('\t').append(run(() -> "" + gu.isTypeUnfinished()));
      r.append('\t').append(run(() -> "" + gu.hasUnknownGenericType(named)));
      r.append('\t').append(run(() -> dump(GenericType.withWildcard(use, GenericType.WILDCARD_EXTENDS))));
      r.append('\t').append(run(() -> {
        Map<VarType, VarType> m3 = new HashMap<>();
        for (GenericType gv : gu.getAllGenericVars()) m3.put(gv, VarType.VARTYPE_OBJECT);
        VarType x = use.remap(m3);
        return x == use ? "same" : dump(x);
      }));
      r.append('\t').append(run(() -> dump(use.resizeArrayDim(2))));
      r.append('\t').append(run(() -> dump(use.resizeArrayDim(2).decreaseArrayDim())));
      w.println("GO\t" + esc(use.value) + "\t" + esc(declSig) + "\t" + u[0] + "\t" + esc(u[1]) + "\t" + u[2] + "\t" + r);
    }
  }

  static VarType resolveUse(String[] u) {
    switch (u[0]) {
      case "F": {
        GenericFieldDescriptor d = GenericMain.parseFieldSignature(u[1]);
        return d == null ? null : d.type;
      }
      case "MP":
        return GenericMain.parseMethodSignature(u[1]).parameterTypes.get(Integer.parseInt(u[2]));
      case "MR":
        return GenericMain.parseMethodSignature(u[1]).returnType;
      case "CS":
        return GenericMain.parseClassSignature("x", u[1]).superclass;
      default:
        return GenericMain.parseClassSignature("x", u[1]).superinterfaces.get(Integer.parseInt(u[2]));
    }
  }

  // ---------------------------------------------------------------- lattice

  static final String[] CONST_NAMES = {
    "VARTYPE_UNKNOWN", "VARTYPE_INT", "VARTYPE_FLOAT", "VARTYPE_LONG", "VARTYPE_DOUBLE", "VARTYPE_BYTE",
    "VARTYPE_CHAR", "VARTYPE_SHORT", "VARTYPE_BOOLEAN", "VARTYPE_BYTECHAR", "VARTYPE_SHORTCHAR", "VARTYPE_NULL",
    "VARTYPE_STRING", "VARTYPE_CLASS", "VARTYPE_OBJECT", "VARTYPE_INTEGER", "VARTYPE_CHARACTER", "VARTYPE_BYTE_OBJ",
    "VARTYPE_SHORT_OBJ", "VARTYPE_BOOLEAN_OBJ", "VARTYPE_FLOAT_OBJ", "VARTYPE_DOUBLE_OBJ", "VARTYPE_LONG_OBJ",
    "VARTYPE_VOID", "VARTYPE_METHODTYPE",
  };

  static VarType constant(String name) {
    try {
      return (VarType) VarType.class.getField(name).get(null);
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static String constName(VarType t) {
    for (String n : CONST_NAMES) {
      if (constant(n) == t) return "C:" + n;
    }
    return dump(t);
  }

  /** The lattice operands: "C:<constant>", "S:<VarType(String)>", "G:<GenericType.parse>". */
  static final String[] LATTICE = {
    "C:VARTYPE_UNKNOWN", "C:VARTYPE_INT", "C:VARTYPE_FLOAT", "C:VARTYPE_LONG", "C:VARTYPE_DOUBLE", "C:VARTYPE_BYTE",
    "C:VARTYPE_CHAR", "C:VARTYPE_SHORT", "C:VARTYPE_BOOLEAN", "C:VARTYPE_BYTECHAR", "C:VARTYPE_SHORTCHAR",
    "C:VARTYPE_NULL", "C:VARTYPE_STRING", "C:VARTYPE_OBJECT", "C:VARTYPE_INTEGER", "C:VARTYPE_CHARACTER",
    "C:VARTYPE_VOID", "S:[I", "S:[[I", "S:[Ljava/lang/Object;", "S:[Ljava/lang/String;", "S:[C", "S:A", "S:G",
    "G:TT;", "G:[TT;", "G:Ljava/util/List<Ljava/lang/String;>;", "G:Ljava/util/List<+Ljava/lang/String;>;",
    "G:Ljava/util/List<*>;", "G:Ljava/util/ArrayList<Ljava/lang/String;>;", "G:Ljava/util/List<+Ljava/lang/Object;>;",
    "G:Ljava/util/List<Ljava/util/List<Ljava/lang/String;>;>;",
  };

  static VarType latticeType(String s) {
    if (s.startsWith("C:")) return constant(s.substring(2));
    if (s.startsWith("S:")) return new VarType(s.substring(2));
    return GenericType.parse(s.substring(2));
  }

  static String ident(VarType r, VarType a, VarType b) {
    if (r == null) return "\\N";
    if (r == a) return "=1";
    if (r == b) return "=2";
    return constName(r);
  }

  static final Map<VarType, List<VarType>> NAMED = new HashMap<>();

  static void lattice(PrintWriter w) {
    setContext(null);
    NAMED.put(GenericType.parse("TT;"),
      List.of(VarType.VARTYPE_STRING, GenericType.parse("Ljava/util/List<Ljava/lang/String;>;")));
    for (String sa : LATTICE) {
      for (String sb : LATTICE) {
        VarType a = latticeType(sa);
        VarType b = latticeType(sb);
        w.println("LA\t" + sa + "\t" + sb
          + "\t" + run(() -> "" + a.higherInLatticeThan(b))
          + "\t" + run(() -> "" + a.higherEqualInLatticeThan(b))
          + "\t" + run(() -> "" + a.higherCrossFamilyThan(b, true))
          + "\t" + run(() -> "" + a.higherCrossFamilyThan(b, false))
          + "\t" + run(() -> ident(VarType.meet(a, b), a, b))
          + "\t" + run(() -> ident(VarType.join(a, b), a, b))
          + "\t" + run(() -> "" + a.equals(b))
          + "\t" + run(() -> "" + new GenericsChecker().isProperlyBounded(a, b))
          + "\t" + run(() -> "" + GenericType.isAssignable(a, b, NAMED))
          + "\t" + run(() -> "" + GenericType.areArgumentsAssignable(a, b, NAMED)));
      }
    }
  }

  // ---------------------------------------------------------------- constant pools

  static final class PB {
    final ByteArrayOutputStream bo = new ByteArrayOutputStream();
    final DataOutputStream o = new DataOutputStream(bo);
    int count = 1;

    PB utf(String s) throws IOException {
      o.writeByte(1);
      o.writeUTF(s);
      count++;
      return this;
    }

    PB integer(int v) throws IOException {
      o.writeByte(3);
      o.writeInt(v);
      count++;
      return this;
    }

    PB flt(int bits) throws IOException {
      o.writeByte(4);
      o.writeInt(bits);
      count++;
      return this;
    }

    PB lng(long v) throws IOException {
      o.writeByte(5);
      o.writeLong(v);
      count += 2;
      return this;
    }

    PB dbl(long bits) throws IOException {
      o.writeByte(6);
      o.writeLong(bits);
      count += 2;
      return this;
    }

    PB ref1(int tag, int idx) throws IOException {
      o.writeByte(tag);
      o.writeShort(idx);
      count++;
      return this;
    }

    PB ref2(int tag, int a, int b) throws IOException {
      o.writeByte(tag);
      o.writeShort(a);
      o.writeShort(b);
      count++;
      return this;
    }

    PB mh(int kind, int ref) throws IOException {
      o.writeByte(15);
      o.writeByte(kind);
      o.writeShort(ref);
      count++;
      return this;
    }

    PB raw(int... bytes) {
      for (int b : bytes) bo.write(b);
      return this;
    }

    byte[] bytes(int sizeOverride) throws IOException {
      ByteArrayOutputStream all = new ByteArrayOutputStream();
      DataOutputStream d = new DataOutputStream(all);
      d.writeInt(0xCAFEBABE);
      d.writeShort(0);
      d.writeShort(65);
      d.writeShort(sizeOverride > 0 ? sizeOverride : count);
      d.write(bo.toByteArray());
      return all.toByteArray();
    }
  }

  static PB kitchenSink() throws IOException {
    return new PB()
      .ref1(7, 30).ref2(10, 1, 3).ref2(12, 31, 32).ref2(9, 1, 5).ref2(12, 33, 34).ref2(11, 7, 3).ref1(7, 35)
      .ref1(8, 36).ref1(16, 32).mh(1, 4).mh(6, 2).mh(9, 6).mh(4, 4).ref2(18, 0, 3).ref2(17, 1, 5).ref1(19, 37)
      .ref1(20, 38).integer(Integer.MIN_VALUE).integer(-1).flt(0x7fc00001).flt(0x80000000).lng(Long.MAX_VALUE)
      .dbl(Double.doubleToRawLongBits(1e300)).dbl(0x7ff8000000000123L).ref1(7, 39).ref1(7, 40)
      .utf("java/lang/String").utf("valueOf").utf("(I)Ljava/lang/String;").utf("CASE_INSENSITIVE_ORDER")
      .utf("Ljava/util/Comparator;").utf("java/util/List").utf("été 😀 \ud800 \u0000 \udc00x")
      .utf("java.base").utf("java/lang").utf("[Ljava/lang/String;").utf("").ref1(7, 42).utf("[I")
      .lng(Long.MIN_VALUE).utf("\u0000\u007f\u0080߿ࠀ￿");
  }

  static String dumpConst(PooledConstant c) {
    if (c == null) return "\\N";
    if (c instanceof PrimitiveConstant p) {
      String v;
      if (p.value == null) v = "\\N";
      else if (p.value instanceof Integer i) v = "I" + i;
      else if (p.value instanceof Long l) v = "J" + l;
      else if (p.value instanceof Float f) v = "F" + Integer.toHexString(Float.floatToRawIntBits(f));
      else if (p.value instanceof Double d) v = "D" + Long.toHexString(Double.doubleToRawLongBits(d));
      else v = "S" + esc((String) p.value);
      return "P" + p.type + "," + p.index + "," + (p.isArray ? 1 : 0) + "," + v;
    }
    LinkConstant l = (LinkConstant) c;
    return "L" + l.type + "," + l.index1 + "," + l.index2 + "," + esc(l.classname) + "," + esc(l.elementname) + ","
      + esc(l.descriptor);
  }

  static String hex(byte[] b) {
    StringBuilder sb = new StringBuilder();
    for (byte x : b) sb.append(String.format("%02x", x & 0xff));
    return sb.toString();
  }

  static String parsePool(byte[] cls) {
    try {
      DataInputStream in = new DataInputStream(new ByteArrayInputStream(cls, 8, cls.length - 8));
      ConstantPool pool = new ConstantPool(in);
      return pool.getPool().stream().map(GenF2Golden::dumpConst).collect(Collectors.joining("|"));
    } catch (Throwable t) {
      return exc(t);
    }
  }

  static void pools(PrintWriter w, Path classesDir) throws IOException {
    setContext(null);
    Map<String, byte[]> pools = new LinkedHashMap<>();
    pools.put("kitchen-sink", kitchenSink().bytes(0));
    pools.put("bad-tag-2", new PB().utf("a").raw(2).bytes(3));
    pools.put("bad-tag-200", new PB().utf("a").raw(200).bytes(3));
    pools.put("truncated", new PB().utf("a").bytes(3));
    pools.put("class-to-integer", new PB().ref1(7, 2).integer(5).bytes(0));
    pools.put("class-to-zero", new PB().ref1(7, 0).bytes(0));
    pools.put("class-to-long-gap", new PB().lng(1).ref1(7, 2).bytes(0));
    pools.put("fieldref-to-utf8", new PB().ref1(7, 3).ref2(9, 1, 3).utf("a/B").bytes(0));
    pools.put("mh-virtual-to-field",
      new PB().ref1(7, 4).ref2(9, 1, 3).ref2(12, 5, 6).utf("a/B").utf("f").utf("I").mh(5, 2).bytes(0));
    pools.put("methodref-bad-desc", new PB().ref1(7, 4).ref2(10, 1, 3).ref2(12, 5, 6).utf("a/B").utf("m").utf("I").bytes(0));
    pools.put("empty", new PB().bytes(0));
    pools.put("size-zero", new PB().bytes(0));
    pools.put("long-at-end", new PB().utf("x").lng(7).bytes(3));
    List<Path> classes;
    try (Stream<Path> s = Files.list(classesDir)) {
      classes = s.filter(p -> p.getFileName().toString().startsWith("PoolSample")).sorted().collect(Collectors.toList());
    }
    for (Path p : classes) pools.put(p.getFileName().toString(), Files.readAllBytes(p));

    byte[] sizeZero = pools.get("size-zero");
    sizeZero[9] = 0;
    for (Map.Entry<String, byte[]> e : pools.entrySet()) {
      w.println("PO\t" + e.getKey() + "\t" + hex(e.getValue()) + "\t" + parsePool(e.getValue()));
    }

    PoolInterceptor interceptor = new PoolInterceptor();
    interceptor.addName("java/lang/String", "a/S");
    interceptor.addName("java/util/List", "b/L");
    interceptor.addName("java/lang/String valueOf (I)Ljava/lang/String;", "a/S vo (I)La/S;");
    interceptor.addName("java/lang/String CASE_INSENSITIVE_ORDER Ljava/util/Comparator;", "a/S cio Ljava/util/Comparator;");
    interceptor.addName("java/lang/Object", "o/O");
    for (String name : List.of("kitchen-sink", "PoolSample.class")) {
      setContext(interceptor);
      byte[] cls = pools.get(name);
      ConstantPool pool;
      try {
        pool = new ConstantPool(new DataInputStream(new ByteArrayInputStream(cls, 8, cls.length - 8)));
      } catch (IOException ex) {
        throw new UncheckedIOException(ex);
      }
      List<PooledConstant> all = pool.getPool();
      for (int i = 0; i < all.size(); i++) {
        PooledConstant c = all.get(i);
        if (c == null) continue;
        int idx = i;
        String r;
        if (c instanceof PrimitiveConstant) {
          r = run(() -> dumpConst(pool.getPrimitiveConstant(idx)));
        } else {
          r = run(() -> dumpConst(pool.getLinkConstant(idx)));
          if (c.type == CodeConstants.CONSTANT_NameAndType) {
            LinkConstant nt = (LinkConstant) c;
            for (String cn : List.of("a/S", "java/lang/String", "x/Y")) {
              r += "\t" + run(() -> esc(String.join(" ", pool.getClassElement(ConstantPool.METHOD, cn, nt.index1, nt.index2))));
              r += "\t" + run(() -> esc(String.join(" ", pool.getClassElement(ConstantPool.FIELD, cn, nt.index1, nt.index2))));
            }
          }
        }
        w.println("PI\t" + name + "\t" + i + "\t" + r);
      }
    }
  }

  // ---------------------------------------------------------------- match engine

  static final String[] MATCH = {
    String.join("\n",
      "statement type:if iftype:if exprsize:-1",
      " exprent position:head type:if",
      "  exprent type:function functype:eq",
      "   exprent type:field name:$fieldname$",
      "   exprent type:constant consttype:null",
      " statement type:basicblock",
      "  exprent position:-1 type:assignment ret:$assignfield$",
      "   exprent type:var index:$var$",
      "   exprent type:field name:$fieldname$",
      " statement type:sequence statsize:2",
      "  statement type:trycatch",
      "   statement type:basicblock exprsize:1",
      "    exprent type:assignment",
      "     exprent type:var index:$var$",
      "     exprent type:invocation invclass:java/lang/Class signature:forName(Ljava/lang/String;)Ljava/lang/Class;",
      "      exprent position:0 type:constant consttype:string constvalue:$classname$",
      "   statement type:basicblock exprsize:1",
      "    exprent type:exit exittype:throw",
      "  statement type:basicblock exprsize:1",
      "   exprent type:assignment",
      "    exprent type:field name:$fieldname$ ret:$field$",
      "    exprent type:var index:$var$"),
    "statement type:if iftype:ifelse",
    "exprent type:invocation invclass:java/lang/StringBuilder signature:append(Ljava/lang/String;)Ljava/lang/StringBuilder; parameter:0:$p0$ parameter:1:x",
    "exprent type:function functype:ternary\n exprent type:constant consttype:string constvalue:$c$\n exprent type:exit exittype:return",
    "statement type:do\n statement type:switch\n  statement type:sequence statsize:3 exprsize:2\n statement type:basicblock",
    "exprent type:var index:3 name:x",
    "statement unknownprop:1",
    "exprent type:nosuchtype functype:add exittype:jump consttype:int",
    "statement position:-1 ret:$r$",
    "statement\n exprent\n  exprent\n   exprent\n exprent\nstatement",
    "  exprent type:var",
    "exprent type:var\n\texprent type:const",
    "statement type:if\n   exprent type:if\n exprent type:var",
    "statement statsize:x",
    "statement type",
    "exprent parameter:a:b",
    "statement type:if:sequence",
    "statement\n\n exprent",
    "",
    "exprent type:switchhead\n exprent type:yield\n  exprent type:new\n   exprent type:monitor\n    exprent type:array\n"
      + "     exprent type:assert\n      exprent type:annotation\n       exprent type:switch",
  };

  static String dumpNode(MatchNode n) {
    StringBuilder sb = new StringBuilder();
    sb.append("N(").append(n.getType()).append(';');
    List<String> rules = new ArrayList<>();
    for (IMatchable.MatchProperties p : IMatchable.MatchProperties.values()) {
      List<String> vals = new ArrayList<>();
      n.iterateRules((k, v) -> {
        if (k == p) vals.add(v.parameter + ":" + (v.value == null ? "\\N" : esc(v.toString())) + ":"
          + (v.value == null ? "-" : v.isVariable()));
        return true;
      });
      if (!vals.isEmpty()) rules.add(p + "=" + String.join(",", vals));
    }
    sb.append(String.join("|", rules)).append(';');
    for (MatchNode c : n.getChildren()) sb.append(dumpNode(c));
    sb.append(')');
    return sb.toString();
  }

  static void match(PrintWriter w) throws Exception {
    Field root = MatchEngine.class.getDeclaredField("rootNode");
    root.setAccessible(true);
    for (String d : MATCH) {
      String r;
      try {
        MatchEngine e = new MatchEngine(d);
        r = dumpNode((MatchNode) root.get(e));
      } catch (Throwable t) {
        r = exc(t);
      }
      w.println("ME\t" + esc(d) + "\t" + r);
    }
  }

  // ---------------------------------------------------------------- verify*

  static void verify(PrintWriter w) {
    for (String pair : sample(methodPairs, 300)) {
      String[] p = pair.split("\u0001");
      setContext(null);
      String r = run(() -> {
        GenericMethodDescriptor d = GenericMain.parseMethodSignature(p[2]);
        if (d == null) return "\\N";
        MethodDescriptor md = MethodDescriptor.parseDescriptor(p[1]);
        GenericsChecker checker = null;
        String[] info = classInfo.get(p[0]);
        if (info != null) {
          GenericClassDescriptor cd = GenericMain.parseClassSignature(p[0], info[0]);
          if (cd != null) checker = cd.getChecker();
        }
        d.verifyTypes(checker, Arrays.asList(md.params), md.ret, List.of());
        return dumpMethod(d);
      });
      String[] owner = classInfo.get(p[0]);
      w.println("VM\t" + esc(p[0]) + "\t" + esc(owner == null ? null : owner[0]) + "\t" + esc(p[1]) + "\t" + esc(p[2])
        + "\t" + logged(r));
    }
    for (String pair : sample(fieldPairs, 300)) {
      String[] p = pair.split("\u0001");
      setContext(null);
      String loose = run(() -> {
        GenericFieldDescriptor d = GenericMain.parseFieldSignature(p[2]);
        if (d == null) return "\\N";
        d.verifyLoosely(new VarType(p[1]));
        return dump(d.type);
      });
      String loosely = logged(loose);
      setContext(null);
      String strict = run(() -> {
        GenericFieldDescriptor d = GenericMain.parseFieldSignature(p[2]);
        if (d == null) return "\\N";
        String[] info = classInfo.get(p[0]);
        GenericClassDescriptor cd = info == null ? null : GenericMain.parseClassSignature(p[0], info[0]);
        d.verifyType(cd, new VarType(p[1]));
        return dump(d.type);
      });
      String[] owner = classInfo.get(p[0]);
      w.println("VF\t" + esc(p[0]) + "\t" + esc(owner == null ? null : owner[0]) + "\t" + esc(p[1]) + "\t" + esc(p[2])
        + "\t" + loosely.replace('\t', ' ') + "\t" + logged(strict).replace('\t', ' '));
    }
    for (Map.Entry<String, String[]> e : sample(classInfo.entrySet(), 250)) {
      String[] info = e.getValue();
      if (info[1] == null) continue;
      setContext(null);
      String r = run(() -> {
        GenericClassDescriptor cd = GenericMain.parseClassSignature(e.getKey(), info[0]);
        if (cd == null) return "\\N";
        List<VarType> ifaces = new ArrayList<>();
        if (!info[2].isEmpty()) {
          for (String s : info[2].split(" ")) ifaces.add(new VarType(s, true));
        }
        cd.verifyTypes(new VarType(info[1], true), ifaces);
        return dumpClass(cd);
      });
      w.println("VC\t" + esc(e.getKey()) + "\t" + esc(info[0]) + "\t" + esc(info[1]) + "\t" + esc(info[2]) + "\t"
        + logged(r));
    }
  }

  // ---------------------------------------------------------------- code constants

  static void constants(PrintWriter w) throws Exception {
    List<Field> fields = new ArrayList<>(Arrays.asList(CodeConstants.class.getFields()));
    fields.sort(Comparator.comparing(Field::getName));
    for (Field f : fields) {
      if (Modifier.isStatic(f.getModifiers())) w.println("CC\t" + f.getName() + "\t" + esc(String.valueOf(f.get(null))));
    }
    String[][] calls = {
      {"java/lang/invoke/MethodHandle", "invokeExact"}, {"java/lang/invoke/MethodHandle", "invoke"},
      {"java/lang/invoke/MethodHandle", "invokeWithArguments"}, {"java/lang/invoke/VarHandle", "get"},
      {"java/lang/invoke/VarHandle", "set"}, {"java/lang/invoke/VarHandle", "getAndAddAcquire"},
      {"java/lang/invoke/VarHandle", "compareAndSet"}, {"java/lang/invoke/VarHandle", "compareAndExchangeRelease"},
      {"java/lang/invoke/VarHandle", "getAndBitwiseXor"}, {"java/lang/invoke/VarHandle", "weakCompareAndSetPlain"},
      {"java/lang/invoke/VarHandle", "foo"}, {"java/lang/Object", "get"}, {"java/lang/invoke/VarHandle", "getVolatile"},
    };
    for (String[] c : calls) {
      w.println("SP\t" + c[0] + "\t" + c[1] + "\t" + CodeConstants.isReturnPolymorphic(c[0], c[1]) + "\t"
        + CodeConstants.areParametersPolymorphic(c[0], c[1]));
    }
  }

  public static void main(String[] args) throws Exception {
    Path out = Paths.get(args[0]);
    Path classes = Paths.get(args[1]);
    setContext(null);
    harvest();
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("vartype.tsv")))) {
      vartype(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("descriptors.tsv")))) {
      descriptors(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("signatures.tsv")))) {
      signatures(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("generic_ops.tsv")))) {
      genericOps(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("lattice.tsv")))) {
      lattice(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("pools.tsv")))) {
      pools(w, classes);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("match.tsv")))) {
      match(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("constants.tsv")))) {
      constants(w);
    }
    try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(out.resolve("verify.tsv")))) {
      verify(w);
    }
  }
}
