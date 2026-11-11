package kuna.vftrace;

import org.jetbrains.java.decompiler.code.Instruction;
import org.jetbrains.java.decompiler.code.InstructionSequence;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructLocalVariableTableAttribute.LocalVariable;
import org.jetbrains.java.decompiler.struct.consts.LinkConstant;
import org.jetbrains.java.decompiler.struct.consts.PooledConstant;
import org.jetbrains.java.decompiler.struct.consts.PrimitiveConstant;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.TypeFamily;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericType;

import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Collection;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Canonical text dump of a method's statement tree (format: tools/vftrace/README.md). The dump never
 * asks any Vineflower object for an identity hash (which would shift the JVM's hash sequence) and never
 * calls anything that allocates statements or exprents (which would consume id counters): objects are
 * keyed by their int ids with == inside each bucket, and fields are read directly.
 */
public final class Dumper {
  private static final int ALL = Statement.STATEDGE_ALL;
  private static final int DIRECT_ALL = Statement.STATEDGE_DIRECT_ALL;
  private static final int[] LIST_KEYS = {
    DIRECT_ALL, StatEdge.TYPE_REGULAR, StatEdge.TYPE_EXCEPTION, StatEdge.TYPE_BREAK, StatEdge.TYPE_CONTINUE,
    StatEdge.TYPE_FINALLYEXIT
  };

  private static final Map<String, Field> FIELDS = new HashMap<>();
  /** -Dvftrace.rawpreds=1 keeps predecessor lists in list order (hash-sensitivity experiments). */
  private static final boolean RAW_PREDS = Boolean.getBoolean("vftrace.rawpreds")
    || "1".equals(System.getProperty("vftrace.rawpreds"));

  static {
    // Resolved up front on the driver's main thread. Class literals and forName(.., false, ..) load
    // without initializing, so no Vineflower static initializer runs (or hashes) on this thread.
    field(Statement.class, "mapSuccEdges");
    field(Statement.class, "mapPredEdges");
    field(Statement.class, "isLastAthrow");
    field(Statement.class, "continueSet");
    field(SwitchStatement.class, "scopedCaseStatements");
    field(RootStatement.class, "flags");
    field(ArrayExprent.class, "hardType");
    field(AssertExprent.class, "parameters");
    field(ConstExprent.class, "wasCondy");
    field(FieldExprent.class, "forceQualified");
    field(FieldExprent.class, "isQualifier");
    field(FieldExprent.class, "wasCondy");
    field(FunctionExprent.class, "implicitType");
    field(FunctionExprent.class, "needsCast");
    field(FunctionExprent.class, "disableNewlineGroupCreation");
    field(InvocationExprent.class, "canIgnoreBoxing");
    field(InvocationExprent.class, "desc");
    field(InvocationExprent.class, "remappedInstType");
    field(InvocationExprent.class, "isInvocationInstance");
    field(InvocationExprent.class, "isQualifier");
    field(InvocationExprent.class, "boxing");
    field(InvocationExprent.class, "wasLazyCondy");
    field(NewExprent.class, "isVarArgParam");
    field(NewExprent.class, "genericArgs");
    field(NewExprent.class, "inferredLambdaType");
    field(PatternExprent.class, "varType");
    field(SwitchExprent.class, "type");
    field(SwitchExprent.class, "fallthrough");
    field(SwitchExprent.class, "standalone");
    field(VarExprent.class, "varType");
    field(VarExprent.class, "isEffectivelyFinal");
    field(VarExprent.class, "writingPattern");
    field(VarExprent.class, "boundType");
    field(VarExprent.class, "isIntersectionType");
    field(VarExprent.class, "isCatchTempVar");
    field(YieldExprent.class, "retType");
    field(GenericType.class, "parent");
    field(GenericType.class, "arguments");
    field(GenericType.class, "wildcard");
    try {
      ClassLoader loader = Dumper.class.getClassLoader();
      Class<?> flags = Class.forName("org.jetbrains.java.decompiler.modules.decompiler.stats.RootStatement$ContentFlags", false, loader);
      field(flags, "hasTryCatch");
      field(flags, "hasLoops");
      field(flags, "hasSwitch");
      Class<?> box = Class.forName("org.jetbrains.java.decompiler.modules.decompiler.exps.InvocationExprent$BoxState", false, loader);
      field(box, "forceBoxing");
      field(box, "forceUnboxing");
      field(box, "keepCast");
    } catch (ClassNotFoundException e) {
      throw new IllegalStateException(e);
    }
  }

  private static void field(Class<?> cls, String name) {
    try {
      Field f = cls.getDeclaredField(name);
      f.setAccessible(true);
      FIELDS.put(cls.getName() + "#" + name, f);
    } catch (NoSuchFieldException e) {
      throw new IllegalStateException(cls.getName() + "." + name, e);
    }
  }

  private static Object get(Class<?> cls, Object obj, String name) {
    Field f = FIELDS.get(cls.getName() + "#" + name);
    if (f == null) {
      throw new IllegalStateException("unresolved field " + cls.getName() + "." + name);
    }
    try {
      return f.get(obj);
    } catch (IllegalAccessException e) {
      throw new IllegalStateException(e);
    }
  }

  private static boolean flag(Class<?> cls, Object obj, String name) {
    return (Boolean) get(cls, obj, name);
  }

  /** Forces class initialization (field resolution) on the caller's thread. */
  public static void init() {
  }

  private static final class Named<T> {
    final T obj;
    final String name;

    Named(T obj, String name) {
      this.obj = obj;
      this.name = name;
    }
  }

  private final StringBuilder out = new StringBuilder();
  private final HashMap<Integer, List<Named<Statement>>> statNames = new HashMap<>();
  private final List<Statement> order = new ArrayList<>();
  private final List<Integer> depths = new ArrayList<>();
  private final List<Statement> treeParents = new ArrayList<>();
  private final List<Statement> orphans = new ArrayList<>();
  private final HashMap<Integer, List<Named<Exprent>>> exprUses = new HashMap<>();
  private int sharedCount = 0;
  private Statement exit;

  private Dumper() {
  }

  public static String dump(RootStatement root) {
    Dumper d = new Dumper();
    d.run(root);
    return d.out.toString();
  }

  /**
   * The variable table of a method's VarProcessor (README "Variable table"): every pair named, a
   * parameter, a this-var, external or inherited, sorted by (var, version).
   */
  public static String vars(VarProcessor vp) {
    java.util.TreeSet<VarVersionPair> pairs = new java.util.TreeSet<>(
      java.util.Comparator.<VarVersionPair>comparingInt(p -> p.var).thenComparingInt(p -> p.version));
    pairs.addAll(vp.getUsedVarVersions());
    pairs.addAll(vp.getParams());
    pairs.addAll(vp.getThisVars().keySet());
    pairs.addAll(vp.getExternalVars());
    pairs.addAll(vp.getInheritedNames().keySet());
    StringBuilder s = new StringBuilder("vars");
    if (vp.nestedProcessed) s.append(" nested");
    s.append('\n');
    for (VarVersionPair p : pairs) {
      String name = vp.getVarName(p);
      s.append("  v ").append(p.var).append('.').append(p.version).append(' ')
        .append(name == null ? "~" : str(name)).append(" t=").append(vt(vp.getVarType(p)))
        .append(' ').append(vp.getVarFinal(p).name());
      String clash = vp.getClashingName(p);
      if (clash != null) s.append(" clash=").append(str(clash));
      if (vp.getParams().contains(p)) s.append(" param");
      String thisVar = vp.getThisVars().get(p);
      if (vp.getThisVars().containsKey(p)) s.append(" this=").append(thisVar == null ? "~" : str(thisVar));
      if (vp.getExternalVars().contains(p)) s.append(" ext");
      if (vp.getInheritedNames().containsKey(p)) {
        String inh = vp.getInheritedNames().get(p);
        s.append(" inh=").append(inh == null ? "~" : str(inh));
      }
      org.jetbrains.java.decompiler.util.Pair<String, VarVersionPair> src = vp.getVarSource(p);
      if (src != null) {
        s.append(" src=").append(src.a == null ? "~" : str(src.a)).append(':')
          .append(src.b == null ? "~" : src.b.var + "." + src.b.version);
      }
      LocalVariable lvt = vp.getVarLVT(p);
      if (lvt != null) {
        s.append(" lvt=").append(lvt.getName() == null ? "~" : str(lvt.getName())).append(':')
          .append(lvt.getDescriptor());
        if (lvt.getSignature() != null) s.append(':').append(lvt.getSignature());
        s.append('@').append(lvt.getStart()).append('-').append(lvt.getEnd());
      }
      s.append('\n');
    }
    List<String> clashing = new ArrayList<>();
    for (String c : vp.clashingNames()) clashing.add(c == null ? "~" : str(c));
    if (!clashing.isEmpty()) {
      java.util.Collections.sort(clashing);
      s.append("  clashing ").append(String.join(" ", clashing)).append('\n');
    }
    if (!vp.getSyntheticSemaphores().isEmpty()) {
      List<Integer> sem = new ArrayList<>(vp.getSyntheticSemaphores());
      java.util.Collections.sort(sem);
      s.append("  semaphores ").append(sem).append('\n');
    }
    return s.toString();
  }

  private void run(RootStatement root) {
    number(root, null, 0);
    exit = root.getDummyExit();
    if (exit != null && lookup(exit) == null) {
      register(exit, "x");
    }
    for (Statement st : order) {
      countExprents(st);
    }
    for (int i = 0; i < order.size(); i++) {
      statement(order.get(i), depths.get(i), treeParents.get(i));
    }
    if (exit != null) {
      statement(exit, 0, null);
    }
    List<String> orphanLines = new ArrayList<>();
    for (Statement o : orphans) {
      orphanLines.add(lookup(o) + " orphan " + kind(o));
    }
    orphanLines.sort(Dumper::compareRefs);
    for (String line : orphanLines) {
      out.append(line).append('\n');
    }
  }

  // ---------------------------------------------------------------------------------------------
  // statement naming

  private void register(Statement st, String name) {
    statNames.computeIfAbsent(st.id, k -> new ArrayList<>()).add(new Named<>(st, name));
  }

  private String lookup(Statement st) {
    List<Named<Statement>> bucket = statNames.get(st.id);
    if (bucket != null) {
      for (Named<Statement> n : bucket) {
        if (n.obj == st) {
          return n.name;
        }
      }
    }
    return null;
  }

  private void number(Statement st, Statement treeParent, int depth) {
    if (lookup(st) != null) {
      return;
    }
    register(st, "s" + order.size());
    order.add(st);
    depths.add(depth);
    treeParents.add(treeParent);
    for (Statement child : st.getStats()) {
      number(child, st, depth + 1);
    }
  }

  private String sid(Statement st) {
    if (st == null) {
      return "~";
    }
    String name = lookup(st);
    if (name == null) {
      // Named by raw id, not by encounter order, which can follow hash-set iteration.
      name = "o" + st.id;
      List<Named<Statement>> bucket = statNames.get(st.id);
      if (bucket != null) {
        name += "_" + bucket.size();
      }
      orphans.add(st);
      register(st, name);
    }
    return name;
  }

  // ---------------------------------------------------------------------------------------------
  // statements

  private static String kind(Statement st) {
    switch (st.type) {
      case ROOT: return "Root";
      case BASIC_BLOCK: return "Block";
      case SEQUENCE: return "Seq";
      case DUMMY_EXIT: return "Exit";
      case GENERAL: return "General";
      case IF: return "If";
      case DO: return "Do";
      case SWITCH: return "Switch";
      case SYNCHRONIZED: return "Sync";
      case TRY_CATCH: return "Try";
      case CATCH_ALL: return "CatchAll";
      default: return "Other";
    }
  }

  private void statement(Statement st, int depth, Statement treeParent) {
    String pad = " ".repeat(depth);
    String in = pad + "  ";
    StringBuilder h = new StringBuilder();
    h.append(pad).append(sid(st)).append(' ').append(kind(st)).append(" id=").append(st.id);

    List<Statement> stats = st.getStats();
    Statement first = st.getFirst();
    Statement expectedFirst = stats.isEmpty() ? null : stats.get(0);
    if (first != expectedFirst) {
      h.append(" first=").append(sid(first));
    }
    if (st.getParent() != treeParent) {
      h.append(" parent=").append(sid(st.getParent()));
    }
    if (st.getPost() != null) {
      h.append(" post=").append(sid(st.getPost()));
    }
    if (st.getLastBasicType() != Statement.LastBasicType.GENERAL) {
      h.append(" lbt=").append(st.getLastBasicType().name());
    }
    if (st.isMonitorEnter()) {
      h.append(" monenter");
    }
    if (flag(Statement.class, st, "isLastAthrow")) {
      h.append(" athrow");
    }
    if (st.containsMonitorExit()) {
      h.append(" monexit");
    }
    if (st.isCopied()) {
      h.append(" copied");
    }
    if (st.isPhantom()) {
      h.append(" phantom");
    }
    subclassHeader(st, h);
    out.append(h).append('\n');

    Object cont = get(Statement.class, st, "continueSet");
    if (cont == null) {
      out.append(in).append("cont ~\n");
    } else if (!((Collection<?>) cont).isEmpty()) {
      out.append(in).append("cont ").append(sortedStats((Collection<?>) cont)).append('\n');
    }
    if (!st.getLabelEdges().isEmpty()) {
      List<String> refs = new ArrayList<>();
      for (StatEdge e : st.getLabelEdges()) {
        refs.add(edgeRef(e));
      }
      refs.sort(Dumper::compareRefs);
      out.append(in).append("labels ").append(String.join(" ", refs)).append('\n');
    }

    edges(st, in);

    for (Exprent e : st.getVarDefinitions()) {
      out.append(in).append("def ").append(expr(e)).append('\n');
    }
    List<Exprent> exprents = st.getExprents();
    if (exprents != null) {
      out.append(in).append("exprents ").append(exprents.size()).append('\n');
      for (Exprent e : exprents) {
        out.append(in).append("- ").append(expr(e)).append('\n');
      }
    }
    subclassBody(st, in);
  }

  private String sortedStats(Collection<?> set) {
    List<String> names = new ArrayList<>();
    for (Object o : set) {
      names.add(sid((Statement) o));
    }
    names.sort(Dumper::compareRefs);
    return "[" + String.join(" ", names) + "]";
  }

  private void subclassHeader(Statement st, StringBuilder h) {
    if (st instanceof RootStatement root) {
      Object flags = get(RootStatement.class, root, "flags");
      String f = "";
      if (flag(flags.getClass(), flags, "hasTryCatch")) f += "T";
      if (flag(flags.getClass(), flags, "hasLoops")) f += "L";
      if (flag(flags.getClass(), flags, "hasSwitch")) f += "S";
      if (!f.isEmpty()) h.append(" flags=").append(f);
      if (root.addErrorComment) h.append(" errcomment");
      if (root.getDummyExit() != exit) h.append(" exit=").append(sid(root.getDummyExit()));
    } else if (st instanceof BasicBlockStatement bb) {
      if (bb.isRemovableMonitorexit()) h.append(" rmexit");
    } else if (st instanceof GeneralStatement g) {
      if (g.isPlaceholder()) h.append(" placeholder");
    } else if (st instanceof IfStatement is) {
      h.append(" iftype=").append(is.iftype);
      h.append(" ifstat=").append(sid(is.getIfstat()));
      h.append(" elsestat=").append(sid(is.getElsestat()));
      h.append(" ifedge=").append(edgeRef(is.getIfEdge()));
      h.append(" elseedge=").append(edgeRef(is.getElseEdge()));
      if (is.isNegated()) h.append(" negated");
      if (is.isPatternMatched()) h.append(" patmatched");
      if (is.hasPPMM()) h.append(" ppmm");
    } else if (st instanceof DoStatement ds) {
      h.append(" loop=").append(ds.getLooptype().name());
    } else if (st instanceof SwitchStatement ss) {
      List<String> cases = new ArrayList<>();
      for (Statement c : ss.getCaseStatements()) cases.add(sid(c));
      h.append(" cases=[").append(String.join(" ", cases)).append(']');
      h.append(" default=").append(edgeRef(ss.getDefaultEdge()));
      Collection<?> scoped = (Collection<?>) get(SwitchStatement.class, ss, "scopedCaseStatements");
      if (!scoped.isEmpty()) h.append(" scoped=").append(sortedStats(scoped));
    } else if (st instanceof SynchronizedStatement sy) {
      h.append(" body=").append(sid(sy.getBody()));
    } else if (st instanceof CatchAllStatement ca) {
      h.append(" handler=").append(sid(ca.getHandler()));
      if (ca.isFinally()) h.append(" finally");
    } else if (st instanceof DummyExitStatement de) {
      if (de.bytecode != null) h.append(" bc=").append(bits(de.bytecode));
    }
  }

  private void subclassBody(Statement st, String in) {
    if (st instanceof RootStatement root) {
      if (root.commentLines != null) {
        for (String c : root.commentLines) out.append(in).append("comment ").append(str(c)).append('\n');
      }
    } else if (st instanceof BasicBlockStatement bb) {
      if (st.getExprents() == null) {
        InstructionSequence seq = bb.getBlock().getSeq();
        StringBuilder s = new StringBuilder();
        for (int i = 0; i < seq.length(); i++) {
          Instruction ins = seq.getInstr(i);
          if (i > 0) s.append(' ');
          s.append(ins.startOffset).append(':').append(ins.opcode);
        }
        out.append(in).append("insn [").append(s).append("]\n");
      }
    } else if (st instanceof IfStatement is) {
      head(in, "head", is.getHeadexprentList());
    } else if (st instanceof DoStatement ds) {
      head(in, "init", ds.getInitExprentList());
      head(in, "cond", ds.getConditionExprentList());
      head(in, "inc", ds.getIncExprentList());
    } else if (st instanceof SwitchStatement ss) {
      head(in, "head", ss.getHeadexprentList());
      List<List<StatEdge>> caseEdges = ss.getCaseEdges();
      for (int i = 0; i < caseEdges.size(); i++) {
        List<String> refs = new ArrayList<>();
        for (StatEdge e : caseEdges.get(i)) refs.add(edgeRef(e));
        out.append(in).append("caseedges ").append(i).append(" [").append(String.join(" ", refs)).append("]\n");
      }
      List<List<Exprent>> caseValues = ss.getCaseValues();
      for (int i = 0; i < caseValues.size(); i++) {
        List<String> vals = new ArrayList<>();
        for (Exprent e : caseValues.get(i)) vals.add(e == null ? "default" : expr(e));
        out.append(in).append("casevalues ").append(i).append(" [").append(String.join(" ", vals)).append("]\n");
      }
      List<Exprent> guards = ss.getCaseGuards();
      for (int i = 0; i < guards.size(); i++) {
        out.append(in).append("guard ").append(i).append(' ').append(expr(guards.get(i))).append('\n');
      }
    } else if (st instanceof SynchronizedStatement sy) {
      head(in, "head", sy.getHeadexprentList());
    } else if (st instanceof CatchStatement cs) {
      List<List<String>> exc = cs.getExctStrings();
      for (int i = 0; i < exc.size(); i++) {
        out.append(in).append("catch ").append(i).append(" [").append(String.join(" ", exc.get(i))).append("]\n");
      }
      List<VarExprent> vars = cs.getVars();
      for (int i = 0; i < vars.size(); i++) {
        out.append(in).append("catchvar ").append(i).append(' ').append(expr(vars.get(i))).append('\n');
      }
      for (Exprent r : cs.getResources()) {
        out.append(in).append("resource ").append(expr(r)).append('\n');
      }
    } else if (st instanceof CatchAllStatement ca) {
      if (ca.getMonitor() != null) out.append(in).append("monitor ").append(expr(ca.getMonitor())).append('\n');
      for (VarExprent v : ca.getVars()) out.append(in).append("catchvar ").append(expr(v)).append('\n');
    }
  }

  /** One-slot head lists: Vineflower's [null] and kuna's empty list both read as absent. */
  private void head(String in, String key, List<Exprent> list) {
    Exprent e = list.isEmpty() ? null : list.get(0);
    if (e != null || list.size() > 1) {
      out.append(in).append(key);
      for (Exprent x : list) out.append(' ').append(expr(x));
      out.append('\n');
    }
  }

  // ---------------------------------------------------------------------------------------------
  // edges

  @SuppressWarnings("unchecked")
  private static List<StatEdge> edgeList(Statement st, boolean succ, int key) {
    Map<Integer, List<StatEdge>> map =
      (Map<Integer, List<StatEdge>>) get(Statement.class, st, succ ? "mapSuccEdges" : "mapPredEdges");
    return map.get(key);
  }

  private static String typeName(int type) {
    switch (type) {
      case StatEdge.TYPE_REGULAR: return "R";
      case StatEdge.TYPE_EXCEPTION: return "E";
      case StatEdge.TYPE_BREAK: return "B";
      case StatEdge.TYPE_CONTINUE: return "C";
      case StatEdge.TYPE_FINALLYEXIT: return "F";
      case DIRECT_ALL: return "D";
      default: return Integer.toString(type);
    }
  }

  private static boolean inList(int key, StatEdge e) {
    return key == DIRECT_ALL ? e.getType() != StatEdge.TYPE_EXCEPTION : e.getType() == key;
  }

  private static int indexOf(List<StatEdge> list, StatEdge e) {
    if (list != null) {
      for (int i = 0; i < list.size(); i++) {
        if (list.get(i) == e) return i;
      }
    }
    return -1;
  }

  private String edgeRef(StatEdge e) {
    if (e == null) {
      return "~";
    }
    int idx = indexOf(edgeList(e.getSource(), true, ALL), e);
    if (idx >= 0) {
      return sid(e.getSource()) + "." + idx;
    }
    return "*" + sid(e.getSource()) + typeName(e.getType()) + sid(e.getDestination());
  }

  private String edgeAttrs(StatEdge e) {
    StringBuilder s = new StringBuilder();
    s.append(typeName(e.getType())).append(' ').append(sid(e.getDestination()));
    if (e.closure != null) s.append(" c=").append(sid(e.closure));
    if (!e.labeled) s.append(" unlabeled");
    if (!e.explicit) s.append(" implicit");
    if (!e.canInline) s.append(" noinline");
    if (e.phantomContinue) s.append(" phantomcont");
    if (e.getExceptions() != null) s.append(" exc=[").append(String.join(" ", e.getExceptions())).append(']');
    return s.toString();
  }

  private void edges(Statement st, String in) {
    List<StatEdge> succ = edgeList(st, true, ALL);
    if (succ != null) {
      for (StatEdge e : succ) {
        out.append(in).append("> ").append(edgeAttrs(e));
        if (e.getSource() != st) out.append(" src=").append(sid(e.getSource()));
        out.append('\n');
      }
    }
    List<StatEdge> pred = edgeList(st, false, ALL);
    if (pred != null && !pred.isEmpty()) {
      List<String> refs = new ArrayList<>();
      for (StatEdge e : pred) {
        String r = edgeRef(e);
        if (e.getDestination() != st) r += "!dst=" + sid(e.getDestination());
        refs.add(r);
      }
      // Predecessor order is the one hash-order-dependent element of Vineflower's state (README).
      if (!RAW_PREDS) refs.sort(Dumper::compareRefs);
      out.append(in).append("< ").append(String.join(" ", refs)).append('\n');
    }
    subLists(st, in, true, succ);
    subLists(st, in, false, pred);
  }

  /** Per-type lists that are not exactly the ALL list filtered by type (order or membership). */
  private void subLists(Statement st, String in, boolean succ, List<StatEdge> all) {
    for (int key : LIST_KEYS) {
      List<StatEdge> actual = edgeList(st, succ, key);
      int n = actual == null ? 0 : actual.size();
      boolean same = true;
      int j = 0;
      if (all != null) {
        for (StatEdge e : all) {
          if (inList(key, e)) {
            if (j >= n || actual.get(j) != e) {
              same = false;
              break;
            }
            j++;
          }
        }
      }
      if (same && j == n) {
        continue;
      }
      List<String> refs = new ArrayList<>();
      for (int i = 0; i < n; i++) {
        StatEdge e = actual.get(i);
        int idx = succ ? indexOf(all, e) : -1;
        refs.add(idx >= 0 ? Integer.toString(idx) : edgeRef(e));
      }
      out.append(in).append(succ ? ">" : "<").append(typeName(key)).append(" [")
        .append(String.join(" ", refs)).append("]\n");
    }
  }

  /** Orders "s12" after "s3": canonical names sort by kind letter, then number, then the rest. */
  private static int compareRefs(String a, String b) {
    int c = Character.compare(a.charAt(0), b.charAt(0));
    if (c != 0) return c;
    long na = leadingNumber(a), nb = leadingNumber(b);
    if (na != nb) return Long.compare(na, nb);
    return a.compareTo(b);
  }

  private static long leadingNumber(String s) {
    long n = 0;
    int i = 1;
    boolean any = false;
    while (i < s.length() && Character.isDigit(s.charAt(i))) {
      n = n * 10 + (s.charAt(i) - '0');
      i++;
      any = true;
    }
    return any ? n : -1;
  }

  // ---------------------------------------------------------------------------------------------
  // exprents

  private void countExprents(Statement st) {
    for (Exprent e : st.getVarDefinitions()) countTree(e);
    if (st.getExprents() != null) for (Exprent e : st.getExprents()) countTree(e);
    if (st instanceof IfStatement is) countAll(is.getHeadexprentList());
    else if (st instanceof DoStatement ds) {
      countAll(ds.getInitExprentList());
      countAll(ds.getConditionExprentList());
      countAll(ds.getIncExprentList());
    } else if (st instanceof SwitchStatement ss) {
      countAll(ss.getHeadexprentList());
      for (List<Exprent> l : ss.getCaseValues()) countAll(l);
      countAll(ss.getCaseGuards());
    } else if (st instanceof SynchronizedStatement sy) countAll(sy.getHeadexprentList());
    else if (st instanceof CatchStatement cs) {
      for (Exprent e : cs.getVars()) countTree(e);
      countAll(cs.getResources());
    } else if (st instanceof CatchAllStatement ca) {
      if (ca.getMonitor() != null) countTree(ca.getMonitor());
      for (Exprent e : ca.getVars()) countTree(e);
    }
  }

  private void countAll(List<? extends Exprent> list) {
    for (Exprent e : list) if (e != null) countTree(e);
  }

  /** Counts occurrences by object; an exprent seen twice gets a share label when printed. */
  private void countTree(Exprent e) {
    List<Named<Exprent>> bucket = exprUses.computeIfAbsent(e.id, k -> new ArrayList<>());
    for (Named<Exprent> n : bucket) {
      if (n.obj == e) {
        if (n.name == null) {
          bucket.set(bucket.indexOf(n), new Named<>(e, ""));
        }
        return;
      }
    }
    bucket.add(new Named<>(e, null));
    for (Exprent child : children(e)) {
      if (child != null) countTree(child);
    }
  }

  /** Children in the order expr() prints them. */
  private static List<Exprent> children(Exprent e) {
    List<Exprent> l = new ArrayList<>();
    if (e instanceof AnnotationExprent a) l.addAll(a.getParValues());
    else if (e instanceof ArrayExprent a) { l.add(a.getArray()); l.add(a.getIndex()); }
    else if (e instanceof AssertExprent a) l.addAll(assertParams(a));
    else if (e instanceof AssignmentExprent a) { l.add(a.getLeft()); l.add(a.getRight()); }
    else if (e instanceof ExitExprent x) l.add(x.getValue());
    else if (e instanceof FieldExprent f) l.add(f.getInstance());
    else if (e instanceof FunctionExprent f) l.addAll(f.getLstOperands());
    else if (e instanceof IfExprent i) l.add(i.getCondition());
    else if (e instanceof InvocationExprent i) { l.add(i.getInstance()); l.addAll(i.getLstParameters()); }
    else if (e instanceof MonitorExprent m) l.add(m.getValue());
    else if (e instanceof NewExprent n) {
      l.add(n.getConstructor());
      l.addAll(n.getLstDims());
      l.addAll(n.getLstArrayElements());
    } else if (e instanceof PatternExprent p) l.addAll(p.getExprents());
    else if (e instanceof SwitchHeadExprent s) {
      l.add(s.getValue());
      for (List<Exprent> vals : s.getCaseValues()) l.addAll(vals);
    } else if (e instanceof YieldExprent y) l.add(y.getContent());
    return l;
  }

  @SuppressWarnings("unchecked")
  private static List<Exprent> assertParams(AssertExprent a) {
    return (List<Exprent>) get(AssertExprent.class, a, "parameters");
  }

  /** The share label of e ("&k" on first print, "^k" afterwards), or null when e occurs once. */
  private String share(Exprent e, boolean[] firstPrint) {
    List<Named<Exprent>> bucket = exprUses.get(e.id);
    if (bucket == null) return null;
    for (int i = 0; i < bucket.size(); i++) {
      Named<Exprent> n = bucket.get(i);
      if (n.obj == e) {
        if (n.name == null) return null;
        if (n.name.isEmpty()) {
          String label = Integer.toString(sharedCount++);
          bucket.set(i, new Named<>(e, label));
          firstPrint[0] = true;
          return label;
        }
        firstPrint[0] = false;
        return n.name;
      }
    }
    return null;
  }

  private String expr(Exprent e) {
    StringBuilder s = new StringBuilder();
    expr(e, s);
    return s.toString();
  }

  private void list(String key, List<? extends Exprent> l, StringBuilder s) {
    s.append(" (").append(key);
    for (Exprent x : l) {
      s.append(' ');
      expr(x, s);
    }
    s.append(')');
  }

  private void expr(Exprent e, StringBuilder s) {
    if (e == null) {
      s.append('~');
      return;
    }
    boolean[] firstPrint = new boolean[1];
    String label = share(e, firstPrint);
    if (label != null && !firstPrint[0]) {
      s.append('^').append(label);
      return;
    }
    s.append('(');
    if (label != null) s.append('&').append(label).append(' ');
    s.append(exprKind(e));
    if (e.bytecode != null && !e.bytecode.isEmpty()) s.append('@').append(bits(e.bytecode));

    if (e instanceof AnnotationExprent a) {
      s.append(' ').append(a.getClassName());
      List<String> names = a.getParNames();
      List<? extends Exprent> values = a.getParValues();
      for (int i = 0; i < names.size(); i++) {
        s.append(' ').append(names.get(i)).append('=');
        expr(values.get(i), s);
      }
    } else if (e instanceof ArrayExprent a) {
      s.append(' ').append(vt((VarType) get(ArrayExprent.class, a, "hardType")));
      s.append(' ');
      expr(a.getArray(), s);
      s.append(' ');
      expr(a.getIndex(), s);
    } else if (e instanceof AssertExprent a) {
      for (Exprent p : assertParams(a)) {
        s.append(' ');
        expr(p, s);
      }
    } else if (e instanceof AssignmentExprent a) {
      if (a.getCondType() != null) s.append(" op=").append(a.getCondType().name());
      s.append(' ');
      expr(a.getLeft(), s);
      s.append(' ');
      expr(a.getRight(), s);
    } else if (e instanceof ConstExprent c) {
      s.append(' ').append(vt(c.getConstType())).append(' ').append(constValue(c.getValue()));
      if (c.isBoolPermitted()) s.append(" boolok");
      if (flag(ConstExprent.class, c, "wasCondy")) s.append(" condy");
    } else if (e instanceof ExitExprent x) {
      s.append(' ').append(x.getExitType().name());
      if (x.getRetType() != null) s.append(" rt=").append(vt(x.getRetType()));
      s.append(' ');
      expr(x.getValue(), s);
    } else if (e instanceof FieldExprent f) {
      s.append(' ').append(f.getClassname()).append('.').append(f.getName()).append(' ')
        .append(f.getDescriptor() == null ? "~" : f.getDescriptor().descriptorString);
      if (f.isStatic()) s.append(" static");
      if (flag(FieldExprent.class, f, "forceQualified")) s.append(" forcequal");
      if (flag(FieldExprent.class, f, "isQualifier")) s.append(" qualifier");
      if (flag(FieldExprent.class, f, "wasCondy")) s.append(" condy");
      s.append(' ');
      expr(f.getInstance(), s);
    } else if (e instanceof FunctionExprent f) {
      s.append(' ').append(f.getFuncType().name());
      VarType it = (VarType) get(FunctionExprent.class, f, "implicitType");
      if (it != null) s.append(" it=").append(vt(it));
      if (!flag(FunctionExprent.class, f, "needsCast")) s.append(" nocast");
      if (flag(FunctionExprent.class, f, "disableNewlineGroupCreation")) s.append(" nonewline");
      for (Exprent x : f.getLstOperands()) {
        s.append(' ');
        expr(x, s);
      }
    } else if (e instanceof IfExprent i) {
      s.append(' ');
      expr(i.getCondition(), s);
    } else if (e instanceof InvocationExprent i) {
      invocation(i, s);
    } else if (e instanceof MonitorExprent m) {
      s.append(' ').append(m.getMonType().name());
      if (m.isRemovable()) s.append(" remove");
      s.append(' ');
      expr(m.getValue(), s);
    } else if (e instanceof NewExprent n) {
      s.append(' ').append(vt(n.getNewType()));
      if (n.isDirectArrayInit()) s.append(" directinit");
      if (flag(NewExprent.class, n, "isVarArgParam")) s.append(" vararg");
      if (n.isAnonymous()) s.append(" anon");
      if (n.isLambda()) s.append(" lambda");
      if (n.isMethodReference()) s.append(" methodref");
      if (n.isEnumConst()) s.append(" enumconst");
      @SuppressWarnings("unchecked")
      List<VarType> ga = (List<VarType>) get(NewExprent.class, n, "genericArgs");
      if (!ga.isEmpty()) s.append(" ga=").append(vts(ga));
      VarType ilt = (VarType) get(NewExprent.class, n, "inferredLambdaType");
      if (ilt != null) s.append(" ilt=").append(vt(ilt));
      s.append(' ');
      expr(n.getConstructor(), s);
      list("dims", n.getLstDims(), s);
      list("elems", n.getLstArrayElements(), s);
    } else if (e instanceof PatternExprent p) {
      s.append(' ').append(vt((VarType) get(PatternExprent.class, p, "varType")));
      PatternExprent.PatternData data = p.getData();
      if (data instanceof PatternExprent.PatternData.RecordPatternData r) {
        s.append(" record=").append(r.cl() == null ? "~" : r.cl().qualifiedName);
      }
      s.append(" bt=").append(vts(p.getBoundTypes()));
      for (Exprent x : p.getExprents()) {
        s.append(' ');
        expr(x, s);
      }
    } else if (e instanceof SwitchExprent sw) {
      s.append(' ').append(sid(sw.getBacking()));
      s.append(' ').append(vt((VarType) get(SwitchExprent.class, sw, "type")));
      if (flag(SwitchExprent.class, sw, "fallthrough")) s.append(" fallthrough");
      if (flag(SwitchExprent.class, sw, "standalone")) s.append(" standalone");
    } else if (e instanceof SwitchHeadExprent sh) {
      s.append(' ');
      expr(sh.getValue(), s);
      for (List<Exprent> vals : sh.getCaseValues()) {
        s.append(" [");
        for (int k = 0; k < vals.size(); k++) {
          if (k > 0) s.append(' ');
          if (vals.get(k) == null) s.append("default");
          else expr(vals.get(k), s);
        }
        s.append(']');
      }
    } else if (e instanceof VarExprent v) {
      var(v, s);
    } else if (e instanceof YieldExprent y) {
      s.append(' ').append(vt((VarType) get(YieldExprent.class, y, "retType")));
      s.append(' ');
      expr(y.getContent(), s);
    }
    s.append(')');
  }

  private static String exprKind(Exprent e) {
    switch (e.type) {
      case ANNOTATION: return "annotation";
      case ARRAY: return "array";
      case ASSERT: return "assert";
      case ASSIGNMENT: return "assign";
      case CONST: return "const";
      case EXIT: return "exit";
      case FIELD: return "field";
      case FUNCTION: return "func";
      case IF: return "if";
      case INVOCATION: return "invoke";
      case MONITOR: return "monitor";
      case NEW: return "new";
      case PATTERN: return "pattern";
      case SWITCH: return "switch";
      case SWITCH_HEAD: return "switchhead";
      case VAR: return "var";
      case YIELD: return "yield";
      default: return "other";
    }
  }

  private void invocation(InvocationExprent i, StringBuilder s) {
    s.append(' ').append(i.getInvocationType().name()).append(' ').append(i.getFunctype().name());
    s.append(' ').append(nz(i.getClassname())).append('.').append(nz(i.getName())).append(nz(i.getStringDescriptor()));
    if (i.isStatic()) s.append(" static");
    if (!flag(InvocationExprent.class, i, "canIgnoreBoxing")) s.append(" noignorebox");
    if (i.getInvokeDynamicClassSuffix() != null) s.append(" sfx=").append(i.getInvokeDynamicClassSuffix());
    StructMethod desc = (StructMethod) get(InvocationExprent.class, i, "desc");
    if (desc != null) s.append(" target=").append(desc.getClassQualifiedName()).append('.').append(desc.getName()).append(desc.getDescriptor());
    LinkConstant bsm = i.getBootstrapMethod();
    if (bsm != null) s.append(" bsm=").append(link(bsm));
    List<PooledConstant> bsa = i.getBootstrapArguments();
    if (bsa != null) {
      s.append(" bsa=[");
      for (int k = 0; k < bsa.size(); k++) {
        if (k > 0) s.append(' ');
        s.append(pooled(bsa.get(k)));
      }
      s.append(']');
    }
    if (!i.getGenericArgs().isEmpty()) s.append(" ga=").append(vts(i.getGenericArgs()));
    VarType rit = (VarType) get(InvocationExprent.class, i, "remappedInstType");
    if (rit != null) s.append(" rit=").append(vt(rit));
    if (i.forceGenericQualfication) s.append(" forcegeneric");
    Map<?, ?> gm = i.getGenericsMap();
    if (!gm.isEmpty()) {
      List<String> entries = new ArrayList<>();
      // NullableConcurrentHashMap stores null keys/values as private sentinel objects.
      for (Map.Entry<?, ?> en : gm.entrySet()) {
        Object k = en.getKey(), v = en.getValue();
        entries.add((k instanceof VarType kt ? vt(kt) : "~") + "=" + (v instanceof VarType vv ? vt(vv) : "~"));
      }
      entries.sort(String::compareTo);
      s.append(" gm={").append(String.join(" ", entries)).append('}');
    }
    if (flag(InvocationExprent.class, i, "isInvocationInstance")) s.append(" invinstance");
    if (flag(InvocationExprent.class, i, "isQualifier")) s.append(" qualifier");
    Object box = get(InvocationExprent.class, i, "boxing");
    if (flag(box.getClass(), box, "forceBoxing")) s.append(" forcebox");
    if (flag(box.getClass(), box, "forceUnboxing")) s.append(" forceunbox");
    if (flag(box.getClass(), box, "keepCast")) s.append(" keepcast");
    if (i.isSyntheticNullCheck()) s.append(" nullcheck");
    if (flag(InvocationExprent.class, i, "wasLazyCondy")) s.append(" lazycondy");
    s.append(' ');
    expr(i.getInstance(), s);
    for (Exprent p : i.getLstParameters()) {
      s.append(' ');
      expr(p, s);
    }
  }

  private void var(VarExprent v, StringBuilder s) {
    s.append(' ').append(v.getIndex()).append('.').append(v.getVersion());
    VarType raw = (VarType) get(VarExprent.class, v, "varType");
    s.append(" t=").append(vt(raw));
    VarProcessor proc = v.getProcessor();
    if (proc != null) {
      VarType pt = proc.getVarType(new VarVersionPair(v.getIndex(), v.getVersion()));
      if (pt != null) s.append(" pt=").append(vt(pt));
    } else {
      s.append(" noproc");
    }
    if (v.isDefinition()) s.append(" def");
    if (v.isClassDef()) s.append(" classdef");
    if (v.isStack()) s.append(" stack");
    LocalVariable lvt = v.getLVT();
    if (lvt != null) {
      s.append(" lvt=").append(lvt.getName()).append(':').append(lvt.getDescriptor())
        .append('@').append(lvt.getStart()).append('-').append(lvt.getEnd());
      if (lvt.getSignature() != null) s.append('<').append(lvt.getSignature()).append('>');
      VarVersionPair ver = lvt.getVersion();
      s.append('/').append(ver.var).append('.').append(ver.version);
    }
    Instruction backing = v.getBackingInstr();
    if (backing != null) s.append(" backing=").append(backing.startOffset).append(':').append(backing.opcode);
    if (flag(VarExprent.class, v, "isEffectivelyFinal")) s.append(" efffinal");
    if (flag(VarExprent.class, v, "writingPattern")) s.append(" writingpattern");
    VarType bound = (VarType) get(VarExprent.class, v, "boundType");
    if (bound != null) s.append(" bound=").append(vt(bound));
    if (flag(VarExprent.class, v, "isIntersectionType")) s.append(" intersection");
    if (flag(VarExprent.class, v, "isCatchTempVar")) s.append(" catchtemp");
  }

  // ---------------------------------------------------------------------------------------------
  // values

  private static String nz(String s) {
    return s == null ? "" : s;
  }

  static String link(LinkConstant l) {
    return l.type + ":" + nz(l.classname) + "." + nz(l.elementname) + nz(l.descriptor);
  }

  static String pooled(PooledConstant c) {
    if (c == null) return "~";
    if (c instanceof LinkConstant l) return link(l);
    if (c instanceof PrimitiveConstant p) return p.type + ":" + constValue(p.value) + (p.isArray ? "[]" : "");
    return Integer.toString(c.type);
  }

  static String constValue(Object v) {
    if (v == null) return "null";
    if (v instanceof Integer i) return Integer.toString(i);
    if (v instanceof Long l) return l + "L";
    if (v instanceof Float f) return "f:" + Integer.toHexString(Float.floatToRawIntBits(f));
    if (v instanceof Double d) return "d:" + Long.toHexString(Double.doubleToRawLongBits(d));
    if (v instanceof String str) return str(str);
    return "?" + v.getClass().getSimpleName();
  }

  private static final char[] HEX = "0123456789abcdef".toCharArray();

  static String str(String v) {
    StringBuilder s = new StringBuilder("\"");
    for (int i = 0; i < v.length(); i++) {
      char c = v.charAt(i);
      if (c == '"' || c == '\\') s.append('\\').append(c);
      else if (c >= 0x20 && c < 0x7f) s.append(c);
      else s.append("\\u").append(HEX[(c >> 12) & 15]).append(HEX[(c >> 8) & 15]).append(HEX[(c >> 4) & 15]).append(HEX[c & 15]);
    }
    return s.append('"').toString();
  }

  static String bits(BitSet b) {
    StringBuilder s = new StringBuilder();
    int i = b.nextSetBit(0);
    while (i >= 0) {
      int j = i;
      while (b.get(j + 1)) j++;
      if (s.length() > 0) s.append(',');
      s.append(i);
      if (j > i) s.append('-').append(j);
      i = b.nextSetBit(j + 1);
    }
    return s.toString();
  }

  static String vts(List<VarType> l) {
    StringBuilder s = new StringBuilder("[");
    for (int i = 0; i < l.size(); i++) {
      if (i > 0) s.append(' ');
      s.append(vt(l.get(i)));
    }
    return s.append(']').toString();
  }

  private static String charOf(CodeType t) {
    switch (t) {
      case BYTE: return "B";
      case CHAR: return "C";
      case DOUBLE: return "D";
      case FLOAT: return "F";
      case INT: return "I";
      case LONG: return "J";
      case SHORT: return "S";
      case BOOLEAN: return "Z";
      case VOID: return "V";
      case GROUP2EMPTY: return "G";
      case NOTINITIALIZED: return "N";
      case ADDRESS: return "A";
      case BYTECHAR: return "X";
      case SHORTCHAR: return "Y";
      case UNKNOWN: return "U";
      default: return null;
    }
  }

  private static TypeFamily family(CodeType t, int dim) {
    if (dim > 0) return TypeFamily.OBJECT;
    switch (t) {
      case BYTE: case BYTECHAR: case SHORTCHAR: case CHAR: case SHORT: case INT: return TypeFamily.INTEGER;
      case DOUBLE: return TypeFamily.DOUBLE;
      case FLOAT: return TypeFamily.FLOAT;
      case LONG: return TypeFamily.LONG;
      case BOOLEAN: return TypeFamily.BOOLEAN;
      case NULL: case OBJECT: case GENVAR: return TypeFamily.OBJECT;
      default: return TypeFamily.UNKNOWN;
    }
  }

  private static int stackSize(CodeType t, int dim) {
    if (dim > 0) return 1;
    switch (t) {
      case DOUBLE: case LONG: return 2;
      case VOID: case GROUP2EMPTY: return 0;
      default: return 1;
    }
  }

  static String vt(VarType t) {
    if (t == null) return "~";
    StringBuilder s = new StringBuilder();
    if (t instanceof GenericType g) {
      int wildcard = (Integer) get(GenericType.class, g, "wildcard");
      if (wildcard == GenericType.WILDCARD_EXTENDS) s.append('+');
      else if (wildcard == GenericType.WILDCARD_SUPER) s.append('-');
      else if (wildcard == GenericType.WILDCARD_UNBOUND) s.append('*');
      else if (wildcard != GenericType.WILDCARD_NO) s.append("w").append(wildcard);
    }
    s.append("[".repeat(Math.max(0, t.arrayDim)));
    if (t.arrayDim < 0) s.append("dim").append(t.arrayDim);
    switch (t.type) {
      case OBJECT:
        s.append('L').append(t.value).append(';');
        break;
      case GENVAR:
        s.append('T').append(t.value).append(';');
        break;
      case NULL:
        s.append("null");
        if (t.value != null) s.append(':').append(t.value);
        break;
      default: {
        String c = charOf(t.type);
        if (c != null && c.equals(t.value)) s.append(c);
        else s.append(t.type.name()).append(':').append(t.value == null ? "~" : t.value);
      }
    }
    if (t.typeFamily != family(t.type, t.arrayDim)) s.append("!f=").append(t.typeFamily.name());
    if (t.stackSize != stackSize(t.type, t.arrayDim)) s.append("!s=").append(t.stackSize);
    if (t instanceof GenericType g) {
      @SuppressWarnings("unchecked")
      List<VarType> args = (List<VarType>) get(GenericType.class, g, "arguments");
      s.append('<');
      for (int i = 0; i < args.size(); i++) {
        if (i > 0) s.append(',');
        s.append(args.get(i) == null ? "?" : vt(args.get(i)));
      }
      s.append('>');
      VarType parent = (VarType) get(GenericType.class, g, "parent");
      if (parent != null) s.append("{").append(vt(parent)).append('}');
    }
    return s.toString();
  }
}
